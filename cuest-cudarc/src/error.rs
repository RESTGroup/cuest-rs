//! Error types for cuEST operations.

use cudarc::driver::DriverError;
use cuest_ffi::status::{status_to_str, CuestStatus};
use std::fmt;

/// Error type for all cuEST wrapper operations.
///
/// This enum is `#[non_exhaustive]`: new variants may be added in patch
/// releases, so downstream matches must include a wildcard (`_`) arm.
#[derive(Debug)]
#[non_exhaustive]
pub enum CuestError {
    /// A cuEST C API call returned a non-success status code.
    ///
    /// `context` names the failing call (or wrapper-level operation).
    Status {
        /// The status code returned by cuEST.
        status: CuestStatus,
        /// Name of the failing call, e.g. `"cuestAOBasisCreate"`.
        context: &'static str,
    },
    /// A CUDA operation performed through cudarc failed.
    Cuda(DriverError),
    /// An error originating in this wrapper: invalid arguments (sizes,
    /// mismatched streams, unknown enum values), parse failures, etc.
    Rust(String),
    /// The loaded cuEST library version does not match the C API these
    /// bindings were generated from.
    ///
    /// `needed` is the (major, minor, patch) version of the bindings, `found`
    /// the version reported by the loaded library.
    UnsupportedVersion { needed: (u32, u32, u32), found: (u32, u32, u32) },
}

impl fmt::Display for CuestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CuestError::Status { status, context } => {
                write!(
                    f,
                    "{context} failed: {} (cuEST status {})",
                    status_to_str(*status),
                    status.0
                )
            },
            CuestError::Cuda(err) => write!(f, "cudarc error: {err}"),
            CuestError::Rust(msg) => write!(f, "{msg}"),
            CuestError::UnsupportedVersion { needed, found } => write!(
                f,
                "these bindings require cuEST {}.{}.{}, but the loaded library is {}.{}.{}",
                needed.0, needed.1, needed.2, found.0, found.1, found.2
            ),
        }
    }
}

impl std::error::Error for CuestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CuestError::Cuda(err) => Some(err),
            _ => None,
        }
    }
}

impl From<DriverError> for CuestError {
    fn from(err: DriverError) -> Self {
        CuestError::Cuda(err)
    }
}

/// Convert a cuEST status code into `Result`, attaching the call name.
pub(crate) fn status_check(status: CuestStatus, context: &'static str) -> Result<(), CuestError> {
    if status == cuest_ffi::ffi::CUEST_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(CuestError::Status { status, context })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cuest_ffi::ffi::{CUEST_STATUS_INVALID_ARGUMENT, CUEST_STATUS_SUCCESS};

    #[test]
    fn cpu_status_check() {
        assert!(status_check(CUEST_STATUS_SUCCESS, "f").is_ok());
        let err = status_check(CUEST_STATUS_INVALID_ARGUMENT, "cuestF").unwrap_err();
        match err {
            CuestError::Status { status, context } => {
                assert_eq!(status, CUEST_STATUS_INVALID_ARGUMENT);
                assert_eq!(context, "cuestF");
            },
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn cpu_display() {
        let err = CuestError::Status {
            status: CUEST_STATUS_INVALID_ARGUMENT,
            context: "cuestAOBasisCreate",
        };
        assert_eq!(err.to_string(), "cuestAOBasisCreate failed: invalid argument (cuEST status 3)");
        let err = CuestError::UnsupportedVersion { needed: (0, 2, 2), found: (0, 3, 0) };
        assert_eq!(
            err.to_string(),
            "these bindings require cuEST 0.2.2, but the loaded library is 0.3.0"
        );
        assert!(std::error::Error::source(&err).is_none());
    }
}
