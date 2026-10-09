//! AO pair lists: sparsity discovery for a basis at a fixed geometry.

use crate::basis::CuestAOBasis;
use crate::context::ContextInner;
use crate::error::{status_check, CuestError};
use crate::parameters::CuestParameters;
use crate::workspace::{new_descriptor, Workspace};
use crate::xyz::Xyz;
use cuest_ffi::ffi;
use std::ptr;
use std::sync::Arc;

/// Owned state of one AO pair list handle.
pub(crate) struct PairListInner {
    pub(crate) context: Arc<ContextInner>,
    pub(crate) handle: ffi::cuestAOPairList_t,
    /// The basis this pair list was built from (cuEST requires the
    /// pair list to outlive plans created from it, and the basis to outlive
    /// the pair list).
    pub(crate) basis: CuestAOBasis,
    _persistent_workspace: Workspace,
    threshold_pq: f64,
}

impl Drop for PairListInner {
    fn drop(&mut self) {
        // cuEST requires the handle's device to be current for destroy.
        // Drop cannot report errors; failures only leak GPU resources.
        let _ = self.context.stream().context().bind_to_thread();
        let _ = unsafe { ffi::cuestAOPairListDestroy(self.handle) };
    }
}

/// A cuEST AO pair list object (`cuestAOPairList_t`).
///
/// A pair list captures the significant shell pairs of a basis at one
/// geometry and one screening threshold. It keeps its [`CuestAOBasis`]
/// alive, and plans created from it keep it alive in turn.
pub struct CuestAOPairList {
    inner: Arc<PairListInner>,
}

impl CuestAOPairList {
    /// Create a pair list for `basis` at `xyz_bohr` (flat, row-major, one
    /// `[x, y, z]` row per atom, in Bohr) with the given screening threshold
    /// `threshold_pq`.
    ///
    /// cuEST recommends thresholds between `1e-10` and `1e-16`.
    /// [`CuestAOPairList::from_xyz`] is the convenience form for an
    /// [`Xyz`] geometry.
    pub fn new(
        basis: &CuestAOBasis,
        xyz_bohr: &[f64],
        threshold_pq: f64,
    ) -> Result<Self, CuestError> {
        let num_atoms = basis.num_atom();
        if xyz_bohr.len() != 3 * num_atoms as usize {
            return Err(CuestError::Rust(format!(
                "geometry has {} coordinates, but the basis has {num_atoms} atoms (expected {})",
                xyz_bohr.len(),
                3 * num_atoms
            )));
        }
        if threshold_pq < 0.0 || threshold_pq.is_nan() {
            return Err(CuestError::Rust(format!(
                "pair screening threshold {threshold_pq} is invalid (must be >= 0)"
            )));
        }

        let context = basis.inner().context.clone();
        let params = CuestParameters::create(ffi::CUEST_AOPAIRLIST_PARAMETERS)?;
        let (mut persistent_desc, mut temporary_desc) = (new_descriptor(), new_descriptor());
        status_check(
            unsafe {
                ffi::cuestAOPairListCreateWorkspaceQuery(
                    context.handle(),
                    basis.raw_handle(),
                    num_atoms,
                    xyz_bohr.as_ptr(),
                    threshold_pq,
                    params.raw(),
                    &mut persistent_desc,
                    &mut temporary_desc,
                    ptr::null_mut(),
                )
            },
            "cuestAOPairListCreateWorkspaceQuery",
        )?;

        let stream = context.stream().clone();
        let persistent = Workspace::new(&stream, &persistent_desc)?;
        let temporary = Workspace::new(&stream, &temporary_desc)?;
        let mut handle: ffi::cuestAOPairList_t = ptr::null_mut();
        {
            let (mut persistent_raw, _persistent_guard) = persistent.raw(&stream);
            let (mut temporary_raw, _temporary_guard) = temporary.raw(&stream);
            status_check(
                unsafe {
                    ffi::cuestAOPairListCreate(
                        context.handle(),
                        basis.raw_handle(),
                        num_atoms,
                        xyz_bohr.as_ptr(),
                        threshold_pq,
                        params.raw(),
                        &mut persistent_raw,
                        &mut temporary_raw,
                        &mut handle,
                    )
                },
                "cuestAOPairListCreate",
            )?;
        }
        // The temporary workspace is no longer needed.

        Ok(Self {
            inner: Arc::new(PairListInner {
                context,
                handle,
                basis: basis.clone(),
                _persistent_workspace: persistent,
                threshold_pq,
            }),
        })
    }

    /// Create a pair list for `basis` from an [`Xyz`] geometry (Angstrom
    /// input; converted to Bohr internally).
    pub fn from_xyz(
        basis: &CuestAOBasis,
        xyz: &Xyz,
        threshold_pq: f64,
    ) -> Result<Self, CuestError> {
        Self::new(basis, &xyz.coords_bohr_flat(), threshold_pq)
    }

    /// The screening threshold this pair list was built with.
    pub fn threshold_pq(&self) -> f64 {
        self.inner.threshold_pq
    }

    /// The basis this pair list was built from.
    pub fn basis(&self) -> &CuestAOBasis {
        &self.inner.basis
    }

    /// The raw cuEST pair list handle (escape hatch).
    pub fn raw_handle(&self) -> ffi::cuestAOPairList_t {
        self.inner.handle
    }

    pub(crate) fn inner(&self) -> &Arc<PairListInner> {
        &self.inner
    }
}

impl Clone for CuestAOPairList {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl std::fmt::Debug for CuestAOPairList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CuestAOPairList")
            .field("num_atom", &self.basis().num_atom())
            .field("num_ao", &self.basis().num_ao())
            .field("threshold_pq", &self.threshold_pq())
            .finish_non_exhaustive()
    }
}

// SAFETY: see the "Thread safety" section of the crate documentation.
unsafe impl Send for CuestAOPairList {}
