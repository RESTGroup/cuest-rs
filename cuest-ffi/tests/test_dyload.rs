//! Smoke test of the dynamic loading path against a real cuEST installation.
//!
//! Requires the cuEST shared library together with its CUDA dependencies:
//!
//! ```bash
//! CUEST_DYLOAD=/path/to/libcuest.so LD_LIBRARY_PATH=/usr/local/cuda/lib64 \
//!     cargo test -p cuest-ffi -- --ignored
//! ```

#![cfg(feature = "dynamic_loading")]

use cuest_ffi::ffi::*;

/// Create and destroy a handle parameters object; no GPU work is involved.
#[test]
#[ignore = "requires a local cuEST installation and CUDA libraries"]
fn test_parameters_create_destroy() {
    unsafe {
        let mut params: cuestHandleParameters_t = std::ptr::null_mut();
        let status = cuestParametersCreate(CUEST_HANDLE_PARAMETERS, &mut params);
        assert_eq!(status, CUEST_STATUS_SUCCESS, "{}", cuest_ffi::status::status_to_str(status));
        assert!(!params.is_null());

        let status = cuestParametersDestroy(CUEST_HANDLE_PARAMETERS, params);
        assert_eq!(status, CUEST_STATUS_SUCCESS, "{}", cuest_ffi::status::status_to_str(status));
    }
}
