//! One-electron integral plans and computation (overlap, kinetic, potential).

use crate::basis::CuestAOBasis;
use crate::context::ContextInner;
use crate::error::{status_check, CuestError};
use crate::pair_list::CuestAOPairList;
use crate::parameters::CuestParameters;
use crate::workspace::{new_descriptor, Workspace};
use cudarc::driver::{CudaSlice, DevicePtr, DevicePtrMut};
use cuest_ffi::ffi;
use std::ffi::c_int;
use std::ptr;
use std::sync::Arc;

/// Precision usage mode for compute calls (cuEST "ffloat").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CuestFfloatUsageMode {
    /// Use ffloat compute when it is available and expected to be faster on
    /// this GPU architecture (the cuEST default).
    Default,
    /// Request ffloat (reduced-precision) compute.
    On,
    /// Request FP64 compute.
    Off,
}

impl CuestFfloatUsageMode {
    fn to_raw(self) -> ffi::cuestFfloatUsageMode_t {
        match self {
            Self::Default => ffi::CUEST_FFLOAT_USAGE_MODE_DEFAULT,
            Self::On => ffi::CUEST_FFLOAT_USAGE_MODE_ON,
            Self::Off => ffi::CUEST_FFLOAT_USAGE_MODE_OFF,
        }
    }
}

/// JIT usage mode for compute calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CuestJitUsageMode {
    /// Use JIT-compiled kernels. Requires the cuEST CUDA 13.x package;
    /// requesting this on a CUDA 12 package returns an error.
    On,
    /// Use AOT-compiled kernels.
    Off,
}

impl CuestJitUsageMode {
    fn to_raw(self) -> ffi::cuestJITUsageMode_t {
        match self {
            Self::On => ffi::CUEST_JIT_USAGE_MODE_ON,
            Self::Off => ffi::CUEST_JIT_USAGE_MODE_OFF,
        }
    }
}

/// Options for [`CuestOeIntPlan::potential_with_options`].
///
/// All fields are `Option`s: `None` leaves the cuEST default in place. (This
/// matters for [`CuestJitUsageMode`]: the cuEST default is JIT on CUDA 13.x
/// packages and AOT on CUDA 12 packages, and explicitly requesting JIT on a
/// CUDA 12 package fails.)
#[derive(Debug, Default, Clone)]
pub struct CuestPotentialComputeOptions {
    /// Precision usage mode; `None` = compute precision chosen heuristically.
    pub ffloat_usage_mode: Option<CuestFfloatUsageMode>,
    /// JIT usage mode; `None` = package default (JIT on CUDA 13.x).
    pub jit_usage_mode: Option<CuestJitUsageMode>,
}

/// Owned state of one one-electron integral plan handle.
struct PlanInner {
    context: Arc<ContextInner>,
    handle: ffi::cuestOEIntPlan_t,
    /// The basis and pair list the plan was built from (cuEST requires both
    /// to outlive the plan).
    _basis: CuestAOBasis,
    _pair_list: CuestAOPairList,
    _persistent_workspace: Workspace,
    nao: u64,
}

impl PlanInner {
    /// Size the temporary workspace with `query` and allocate it on the
    /// context's stream.
    fn temporary_workspace(
        &self,
        query: impl FnOnce(&mut ffi::cuestWorkspaceDescriptor_t) -> ffi::cuestStatus_t,
        name: &'static str,
    ) -> Result<Workspace, CuestError> {
        let mut desc = new_descriptor();
        status_check(query(&mut desc), name)?;
        Workspace::new(self.context.stream(), &desc)
    }
}

impl Drop for PlanInner {
    fn drop(&mut self) {
        // cuEST requires the handle's device to be current for destroy.
        // Drop cannot report errors; failures only leak GPU resources.
        let _ = self.context.stream().context().bind_to_thread();
        let _ = unsafe { ffi::cuestOEIntPlanDestroy(self.handle) };
    }
}

/// A cuEST one-electron integral plan (`cuestOEIntPlan_t`).
///
/// Built from a basis and a pair list (both kept alive by the plan), it
/// caches the intermediates needed to compute the overlap, kinetic, and
/// potential matrices. All compute methods are stream-ordered: they enqueue
/// work on the context's stream and return without synchronizing.
pub struct CuestOeIntPlan {
    inner: Arc<PlanInner>,
}

impl CuestOeIntPlan {
    /// Create a plan for `basis`/`pair_list`.
    ///
    /// The pair list must have been built from `basis`; returns
    /// [`CuestError::Rust`] otherwise.
    pub fn new(basis: &CuestAOBasis, pair_list: &CuestAOPairList) -> Result<Self, CuestError> {
        if !Arc::ptr_eq(basis.inner(), pair_list.inner().basis.inner()) {
            return Err(CuestError::Rust(
                "pair list was built from a different basis than the one given".to_string(),
            ));
        }
        let context = basis.inner().context.clone();
        let params = CuestParameters::create(ffi::CUEST_OEINTPLAN_PARAMETERS)?;
        let (mut persistent_desc, mut temporary_desc) = (new_descriptor(), new_descriptor());
        status_check(
            unsafe {
                ffi::cuestOEIntPlanCreateWorkspaceQuery(
                    context.handle(),
                    basis.raw_handle(),
                    pair_list.raw_handle(),
                    params.raw(),
                    &mut persistent_desc,
                    &mut temporary_desc,
                    ptr::null_mut(),
                )
            },
            "cuestOEIntPlanCreateWorkspaceQuery",
        )?;

        let stream = context.stream().clone();
        let persistent = Workspace::new(&stream, &persistent_desc)?;
        let temporary = Workspace::new(&stream, &temporary_desc)?;
        let mut handle: ffi::cuestOEIntPlan_t = ptr::null_mut();
        {
            let (mut persistent_raw, _persistent_guard) = persistent.raw(&stream);
            let (mut temporary_raw, _temporary_guard) = temporary.raw(&stream);
            status_check(
                unsafe {
                    ffi::cuestOEIntPlanCreate(
                        context.handle(),
                        basis.raw_handle(),
                        pair_list.raw_handle(),
                        params.raw(),
                        &mut persistent_raw,
                        &mut temporary_raw,
                        &mut handle,
                    )
                },
                "cuestOEIntPlanCreate",
            )?;
        }
        // The temporary workspace is no longer needed.

        Ok(Self {
            inner: Arc::new(PlanInner {
                context,
                handle,
                _basis: basis.clone(),
                _pair_list: pair_list.clone(),
                _persistent_workspace: persistent,
                nao: basis.num_ao(),
            }),
        })
    }

    /// Number of basis functions (the side of the output matrices).
    pub fn nao(&self) -> u64 {
        self.inner.nao
    }

    /// Compute the AO overlap matrix `S` (nao x nao) into a new device
    /// buffer allocated on the context's stream.
    pub fn overlap(&self) -> Result<CudaSlice<f64>, CuestError> {
        let mut out = self.alloc_output()?;
        self.overlap_into(&mut out)?;
        Ok(out)
    }

    /// Compute the AO overlap matrix `S` into `out` (at least nao x nao
    /// elements on the context's stream).
    pub fn overlap_into(&self, out: &mut impl DevicePtrMut<f64>) -> Result<(), CuestError> {
        self.inner.context.check_stream(out.stream(), "out")?;
        check_output_len(self.inner.nao, out.len())?;
        let params = CuestParameters::create(ffi::CUEST_OVERLAPCOMPUTE_PARAMETERS)?;
        let temp = self.inner.temporary_workspace(
            |desc| unsafe {
                ffi::cuestOverlapComputeWorkspaceQuery(
                    self.inner.context.handle(),
                    self.inner.handle,
                    params.raw(),
                    desc,
                    ptr::null_mut(),
                )
            },
            "cuestOverlapComputeWorkspaceQuery",
        )?;

        let stream = self.inner.context.stream();
        let (out_ptr, _out_guard) = out.device_ptr_mut(stream);
        let (mut ws, _ws_guard) = temp.raw(stream);
        status_check(
            unsafe {
                ffi::cuestOverlapCompute(
                    self.inner.context.handle(),
                    self.inner.handle,
                    params.raw(),
                    &mut ws,
                    out_ptr as *mut f64,
                )
            },
            "cuestOverlapCompute",
        )
    }

    /// Compute the AO kinetic energy matrix `T` (nao x nao) into a new device
    /// buffer allocated on the context's stream.
    pub fn kinetic(&self) -> Result<CudaSlice<f64>, CuestError> {
        let mut out = self.alloc_output()?;
        self.kinetic_into(&mut out)?;
        Ok(out)
    }

    /// Compute the AO kinetic energy matrix `T` into `out` (at least nao x nao
    /// elements on the context's stream).
    pub fn kinetic_into(&self, out: &mut impl DevicePtrMut<f64>) -> Result<(), CuestError> {
        self.inner.context.check_stream(out.stream(), "out")?;
        check_output_len(self.inner.nao, out.len())?;
        let params = CuestParameters::create(ffi::CUEST_KINETICCOMPUTE_PARAMETERS)?;
        let temp = self.inner.temporary_workspace(
            |desc| unsafe {
                ffi::cuestKineticComputeWorkspaceQuery(
                    self.inner.context.handle(),
                    self.inner.handle,
                    params.raw(),
                    desc,
                    ptr::null_mut(),
                )
            },
            "cuestKineticComputeWorkspaceQuery",
        )?;

        let stream = self.inner.context.stream();
        let (out_ptr, _out_guard) = out.device_ptr_mut(stream);
        let (mut ws, _ws_guard) = temp.raw(stream);
        status_check(
            unsafe {
                ffi::cuestKineticCompute(
                    self.inner.context.handle(),
                    self.inner.handle,
                    params.raw(),
                    &mut ws,
                    out_ptr as *mut f64,
                )
            },
            "cuestKineticCompute",
        )
    }

    /// Compute the AO potential energy matrix `V` for `charges` at `xyz`
    /// (both on the context's stream) into a new device buffer.
    ///
    /// `xyz` is flat row-major coordinates (3 per charge, in Bohr), `charges`
    /// the matching charge magnitudes. cuEST evaluates the potential with a
    /// positive unit test charge: for nuclear attraction integrals use the
    /// *negated* nuclear charges, e.g.
    /// [`Xyz::nuclear_attraction_charges`](crate::Xyz::nuclear_attraction_charges).
    pub fn potential(
        &self,
        xyz: &impl DevicePtr<f64>,
        charges: &impl DevicePtr<f64>,
    ) -> Result<CudaSlice<f64>, CuestError> {
        self.potential_with_options(&CuestPotentialComputeOptions::default(), xyz, charges)
    }

    /// [`CuestOeIntPlan::potential`] into `out` (at least nao x nao elements).
    pub fn potential_into(
        &self,
        out: &mut impl DevicePtrMut<f64>,
        xyz: &impl DevicePtr<f64>,
        charges: &impl DevicePtr<f64>,
    ) -> Result<(), CuestError> {
        self.potential_with_options_into(
            &CuestPotentialComputeOptions::default(),
            out,
            xyz,
            charges,
        )
    }

    /// [`CuestOeIntPlan::potential`] with explicit compute options.
    pub fn potential_with_options(
        &self,
        options: &CuestPotentialComputeOptions,
        xyz: &impl DevicePtr<f64>,
        charges: &impl DevicePtr<f64>,
    ) -> Result<CudaSlice<f64>, CuestError> {
        let mut out = self.alloc_output()?;
        self.potential_with_options_into(options, &mut out, xyz, charges)?;
        Ok(out)
    }

    /// [`CuestOeIntPlan::potential_with_options`] into `out`.
    pub fn potential_with_options_into(
        &self,
        options: &CuestPotentialComputeOptions,
        out: &mut impl DevicePtrMut<f64>,
        xyz: &impl DevicePtr<f64>,
        charges: &impl DevicePtr<f64>,
    ) -> Result<(), CuestError> {
        self.inner.context.check_stream(out.stream(), "out")?;
        self.inner.context.check_stream(xyz.stream(), "xyz")?;
        self.inner.context.check_stream(charges.stream(), "charges")?;
        check_output_len(self.inner.nao, out.len())?;
        if xyz.len() == 0 || xyz.len() % 3 != 0 {
            return Err(CuestError::Rust(format!(
                "`xyz` has {} coordinates; expected 3 per charge",
                xyz.len()
            )));
        }
        let num_charges = xyz.len() / 3;
        if charges.len() != num_charges {
            return Err(CuestError::Rust(format!(
                "`charges` has {} entries for {num_charges} charge positions",
                charges.len()
            )));
        }

        let params = CuestParameters::create(ffi::CUEST_POTENTIALCOMPUTE_PARAMETERS)?;
        if let Some(mode) = options.ffloat_usage_mode {
            params.configure_i32(
                ffi::CUEST_POTENTIALCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE.0 as c_int,
                mode.to_raw().0 as i32,
            )?;
        }
        if let Some(mode) = options.jit_usage_mode {
            params.configure_i32(
                ffi::CUEST_POTENTIALCOMPUTE_PARAMETERS_JIT_USAGE_MODE.0 as c_int,
                mode.to_raw().0 as i32,
            )?;
        }

        let num_charges = num_charges as u64;
        let temp = self.inner.temporary_workspace(
            |desc| unsafe {
                ffi::cuestPotentialComputeWorkspaceQuery(
                    self.inner.context.handle(),
                    self.inner.handle,
                    params.raw(),
                    desc,
                    num_charges,
                    ptr::null(),
                    ptr::null(),
                    ptr::null_mut(),
                )
            },
            "cuestPotentialComputeWorkspaceQuery",
        )?;

        let stream = self.inner.context.stream();
        let (xyz_ptr, _xyz_guard) = xyz.device_ptr(stream);
        let (charges_ptr, _charges_guard) = charges.device_ptr(stream);
        let (out_ptr, _out_guard) = out.device_ptr_mut(stream);
        let (mut ws, _ws_guard) = temp.raw(stream);
        status_check(
            unsafe {
                ffi::cuestPotentialCompute(
                    self.inner.context.handle(),
                    self.inner.handle,
                    params.raw(),
                    &mut ws,
                    num_charges,
                    xyz_ptr as *const f64,
                    charges_ptr as *const f64,
                    out_ptr as *mut f64,
                )
            },
            "cuestPotentialCompute",
        )
    }

    /// Allocate a nao x nao zero-initialized output on the context's stream.
    fn alloc_output(&self) -> Result<CudaSlice<f64>, CuestError> {
        let count = (self.inner.nao * self.inner.nao) as usize;
        Ok(self.inner.context.stream().alloc_zeros::<f64>(count)?)
    }

    /// The raw cuEST plan handle (escape hatch).
    pub fn raw_handle(&self) -> ffi::cuestOEIntPlan_t {
        self.inner.handle
    }
}

impl Clone for CuestOeIntPlan {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl std::fmt::Debug for CuestOeIntPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CuestOeIntPlan").field("nao", &self.nao()).finish_non_exhaustive()
    }
}

// SAFETY: see the "Thread safety" section of the crate documentation.
unsafe impl Send for CuestOeIntPlan {}

/// The cuEST C API cannot check output buffer sizes; do it here.
fn check_output_len(nao: u64, len: usize) -> Result<(), CuestError> {
    let needed = (nao * nao) as usize;
    if len < needed {
        Err(CuestError::Rust(format!(
            "output matrix has {len} elements, expected at least {needed} (nao = {nao})"
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_output_len() {
        assert!(check_output_len(3, 9).is_ok());
        assert!(check_output_len(3, 10).is_ok());
        assert!(check_output_len(3, 8).is_err());
        assert!(check_output_len(0, 0).is_ok());
    }
}
