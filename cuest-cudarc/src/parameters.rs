//! Internal wrapper around the type-erased `cuestParameters_t` objects.
//!
//! Every cuEST creation and compute call takes a parameters handle created
//! with `cuestParametersCreate` and destroyed with `cuestParametersDestroy`
//! (right after use, per the cuEST lifetime rules).

use crate::error::{status_check, CuestError};
use cuest_ffi::ffi;
use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;

/// Owns a cuEST parameters handle of one specific parameter type.
///
/// The C API declares parameters as `void*`.
pub(crate) struct CuestParameters {
    raw: *mut c_void,
    type_: ffi::cuestParametersType_t,
}

impl CuestParameters {
    /// Create a parameters object with cuEST defaults.
    pub(crate) fn create(type_: ffi::cuestParametersType_t) -> Result<Self, CuestError> {
        let mut raw: *mut c_void = ptr::null_mut();
        status_check(
            unsafe { ffi::cuestParametersCreate(type_, &mut raw) },
            "cuestParametersCreate",
        )?;
        Ok(Self { raw, type_ })
    }

    /// The raw handle for FFI calls.
    pub(crate) fn raw(&self) -> *mut c_void {
        self.raw
    }

    fn configure_raw(
        &self,
        attribute: c_int,
        value: *const c_void,
        size: usize,
    ) -> Result<(), CuestError> {
        status_check(
            unsafe { ffi::cuestParametersConfigure(self.type_, self.raw, attribute, value, size) },
            "cuestParametersConfigure",
        )
    }

    pub(crate) fn configure_u64(&self, attribute: c_int, value: u64) -> Result<(), CuestError> {
        let bytes = value.to_ne_bytes();
        self.configure_raw(attribute, bytes.as_ptr() as *const c_void, bytes.len())
    }

    pub(crate) fn configure_i32(&self, attribute: c_int, value: i32) -> Result<(), CuestError> {
        let bytes = value.to_ne_bytes();
        self.configure_raw(attribute, bytes.as_ptr() as *const c_void, bytes.len())
    }

    /// Configure a `const char*` attribute: the value is the address of the
    /// string pointer (`&str, sizeof(char *)`), as in the cuEST C examples —
    /// not the string bytes themselves.
    pub(crate) fn configure_cstr(&self, attribute: c_int, value: &CStr) -> Result<(), CuestError> {
        let ptr: *const c_char = value.as_ptr();
        self.configure_raw(
            attribute,
            ptr::addr_of!(ptr) as *const c_void,
            size_of::<*const c_char>(),
        )
    }

    /// Configure a `cudaStream_t` attribute (a plain pointer value).
    pub(crate) fn configure_ptr(
        &self,
        attribute: c_int,
        value: *mut c_void,
    ) -> Result<(), CuestError> {
        self.configure_raw(
            attribute,
            ptr::addr_of!(value) as *const c_void,
            size_of::<*mut c_void>(),
        )
    }
}

impl Drop for CuestParameters {
    fn drop(&mut self) {
        // Drop cannot report failure; a leak here only wastes a small handle.
        let _ = unsafe { ffi::cuestParametersDestroy(self.type_, self.raw) };
    }
}
