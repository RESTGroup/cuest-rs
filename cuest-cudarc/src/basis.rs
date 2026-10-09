//! Atomic orbital (AO) basis sets.

use crate::context::{ContextInner, CuestContext};
use crate::error::{status_check, CuestError};
use crate::parameters::CuestParameters;
use crate::query::{self, AobasisAttribute};
use crate::shell::CuestAOShell;
use crate::workspace::{new_descriptor, Workspace};
use cuest_ffi::ffi;
use std::ffi::{c_int, c_void};
use std::ptr;
use std::sync::Arc;

/// Owned state of one AO basis handle.
pub(crate) struct BasisInner {
    pub(crate) context: Arc<ContextInner>,
    pub(crate) handle: ffi::cuestAOBasis_t,
    /// Persistent workspace owned by the basis (freed after the basis is
    /// destroyed — field drop order).
    _persistent_workspace: Workspace,
    attrs: BasisAttributes,
}

impl Drop for BasisInner {
    fn drop(&mut self) {
        // cuEST requires the handle's device to be current for destroy.
        // Drop cannot report errors; failures only leak GPU resources.
        let _ = self.context.stream().context().bind_to_thread();
        let _ = unsafe { ffi::cuestAOBasisDestroy(self.handle) };
    }
}

/// Cached attribute values queried at basis creation.
struct BasisAttributes {
    is_pure: bool,
    is_cart: bool,
    is_mixed: bool,
    num_atom: u64,
    num_shell: u64,
    num_ao: u64,
    num_cart: u64,
    num_primitive: u64,
    max_l: u64,
}

/// A cuEST AO basis set object (`cuestAOBasis_t`).
///
/// Built from AO shells grouped per atom; the shells may be dropped as soon
/// as the basis exists (cuEST copies their data verbatim at creation).
/// A basis keeps its [`CuestContext`] alive, and all objects derived from it
/// (pair lists, plans) keep the basis alive in turn.
pub struct CuestAOBasis {
    inner: Arc<BasisInner>,
}

impl CuestAOBasis {
    /// Create a basis from shells grouped per atom (`shells[atom][shell]`).
    ///
    /// All shells must belong to `context` and use the same `pure`/Cartesian
    /// setting; each atom needs at least one shell. Returns
    /// [`CuestError::Rust`] otherwise.
    pub fn new(context: &CuestContext, shells: &[&[CuestAOShell]]) -> Result<Self, CuestError> {
        if shells.is_empty() {
            return Err(CuestError::Rust("basis requires at least one atom".to_string()));
        }
        if shells.iter().any(|per_atom| per_atom.is_empty()) {
            return Err(CuestError::Rust("every atom requires at least one shell".to_string()));
        }
        let context_inner = context.inner().clone();
        for shell in shells.iter().flat_map(|per_atom| per_atom.iter()) {
            if !Arc::ptr_eq(shell.context(), &context_inner) {
                return Err(CuestError::Rust(
                    "shell was created on a different cuEST context than the basis".to_string(),
                ));
            }
        }
        let pure = shells[0][0].is_pure();
        if shells.iter().flat_map(|per_atom| per_atom.iter()).any(|shell| shell.is_pure() != pure) {
            return Err(CuestError::Rust(
                "cuEST AO bases cannot mix pure (spherical) and Cartesian shells".to_string(),
            ));
        }

        let num_atoms = shells.len() as u64;
        let num_shells_per_atom: Vec<u64> =
            shells.iter().map(|per_atom| per_atom.len() as u64).collect();
        let flat_shells: Vec<ffi::cuestAOShell_t> =
            shells.iter().flat_map(|per_atom| per_atom.iter().map(|s| s.raw_handle())).collect();

        let params = CuestParameters::create(ffi::CUEST_AOBASIS_PARAMETERS)?;
        let (mut persistent_desc, mut temporary_desc) = (new_descriptor(), new_descriptor());
        status_check(
            unsafe {
                ffi::cuestAOBasisCreateWorkspaceQuery(
                    context_inner.handle(),
                    num_atoms,
                    num_shells_per_atom.as_ptr(),
                    flat_shells.as_ptr(),
                    params.raw(),
                    &mut persistent_desc,
                    &mut temporary_desc,
                    ptr::null_mut(),
                )
            },
            "cuestAOBasisCreateWorkspaceQuery",
        )?;

        let stream = context_inner.stream().clone();
        let persistent = Workspace::new(&stream, &persistent_desc)?;
        let temporary = Workspace::new(&stream, &temporary_desc)?;
        let mut handle: ffi::cuestAOBasis_t = ptr::null_mut();
        {
            let (mut persistent_raw, _persistent_guard) = persistent.raw(&stream);
            let (mut temporary_raw, _temporary_guard) = temporary.raw(&stream);
            status_check(
                unsafe {
                    ffi::cuestAOBasisCreate(
                        context_inner.handle(),
                        num_atoms,
                        num_shells_per_atom.as_ptr(),
                        flat_shells.as_ptr(),
                        params.raw(),
                        &mut persistent_raw,
                        &mut temporary_raw,
                        &mut handle,
                    )
                },
                "cuestAOBasisCreate",
            )?;
        }
        // The temporary workspace is no longer needed.

        match query_new_attributes(&context_inner, handle) {
            Ok(attrs) => Ok(Self {
                inner: Arc::new(BasisInner {
                    context: context_inner,
                    handle,
                    _persistent_workspace: persistent,
                    attrs,
                }),
            }),
            Err(err) => {
                let _ = unsafe { ffi::cuestAOBasisDestroy(handle) };
                Err(err)
            },
        }
    }

    /// Number of basis function centers (atoms).
    pub fn num_atom(&self) -> u64 {
        self.inner.attrs.num_atom
    }

    /// Total number of AO shells.
    pub fn num_shell(&self) -> u64 {
        self.inner.attrs.num_shell
    }

    /// Total number of basis functions (AOs).
    pub fn num_ao(&self) -> u64 {
        self.inner.attrs.num_ao
    }

    /// Total number of Cartesian basis functions.
    pub fn num_cart(&self) -> u64 {
        self.inner.attrs.num_cart
    }

    /// Total number of Gaussian primitives.
    pub fn num_primitive(&self) -> u64 {
        self.inner.attrs.num_primitive
    }

    /// Maximum angular momentum (L) among the shells.
    pub fn max_l(&self) -> u64 {
        self.inner.attrs.max_l
    }

    /// Whether all shells are pure (spherical).
    pub fn is_pure(&self) -> bool {
        self.inner.attrs.is_pure
    }

    /// Whether all shells are Cartesian.
    pub fn is_cart(&self) -> bool {
        self.inner.attrs.is_cart
    }

    /// Whether the shells mix spherical and Cartesian functions (rejected by
    /// cuEST, so this is false for any successfully created basis).
    pub fn is_mixed(&self) -> bool {
        self.inner.attrs.is_mixed
    }

    /// Query an attribute through the typed attribute mechanism.
    pub fn query<A: AobasisAttribute>(&self) -> Result<A::Value, CuestError> {
        query::query_attribute(
            self.inner.context.handle(),
            ffi::CUEST_AOBASIS,
            self.inner.handle as *const c_void,
            A::ID,
        )
    }

    /// The raw cuEST basis handle (escape hatch).
    pub fn raw_handle(&self) -> ffi::cuestAOBasis_t {
        self.inner.handle
    }

    pub(crate) fn inner(&self) -> &Arc<BasisInner> {
        &self.inner
    }
}

impl Clone for CuestAOBasis {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl std::fmt::Debug for CuestAOBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CuestAOBasis")
            .field("num_atom", &self.num_atom())
            .field("num_shell", &self.num_shell())
            .field("num_ao", &self.num_ao())
            .field("max_l", &self.max_l())
            .field("pure", &self.is_pure())
            .finish_non_exhaustive()
    }
}

// SAFETY: see the "Thread safety" section of the crate documentation.
unsafe impl Send for CuestAOBasis {}

fn query_new_attributes(
    context: &ContextInner,
    handle: ffi::cuestAOBasis_t,
) -> Result<BasisAttributes, CuestError> {
    let query_u64 = |id: c_int| {
        query::query_attribute::<u64>(
            context.handle(),
            ffi::CUEST_AOBASIS,
            handle as *const c_void,
            id,
        )
    };
    let query_i32 = |id: c_int| {
        query::query_attribute::<i32>(
            context.handle(),
            ffi::CUEST_AOBASIS,
            handle as *const c_void,
            id,
        )
    };
    Ok(BasisAttributes {
        is_pure: query_i32(ffi::CUEST_AOBASIS_IS_PURE.0 as c_int)? != 0,
        is_cart: query_i32(ffi::CUEST_AOBASIS_IS_CART.0 as c_int)? != 0,
        is_mixed: query_i32(ffi::CUEST_AOBASIS_IS_MIXED.0 as c_int)? != 0,
        num_atom: query_u64(ffi::CUEST_AOBASIS_NUM_ATOM.0 as c_int)?,
        num_shell: query_u64(ffi::CUEST_AOBASIS_NUM_SHELL.0 as c_int)?,
        num_ao: query_u64(ffi::CUEST_AOBASIS_NUM_AO.0 as c_int)?,
        num_cart: query_u64(ffi::CUEST_AOBASIS_NUM_CART.0 as c_int)?,
        num_primitive: query_u64(ffi::CUEST_AOBASIS_NUM_PRIMITIVE.0 as c_int)?,
        max_l: query_u64(ffi::CUEST_AOBASIS_MAX_L.0 as c_int)?,
    })
}
