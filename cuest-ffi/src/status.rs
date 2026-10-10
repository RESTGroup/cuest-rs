//! Helpers for `cuestStatus_t` return codes.

pub use crate::ffi::cuestStatus_t as CuestStatus;

/// Convert a cuEST status code to a human-readable static message.
///
/// Unknown (for example, future) codes map to a generic message.
pub fn status_to_str(status: CuestStatus) -> &'static str {
    use crate::ffi::*;
    match status {
        CUEST_STATUS_SUCCESS => "success",
        CUEST_STATUS_EXCEPTION => "internal exception",
        CUEST_STATUS_NULL_POINTER => "null pointer",
        CUEST_STATUS_INVALID_ARGUMENT => "invalid argument",
        CUEST_STATUS_INVALID_SIZE => "invalid size",
        CUEST_STATUS_INVALID_TYPE => "invalid type",
        CUEST_STATUS_INVALID_PARAMETER => "invalid parameter",
        CUEST_STATUS_INVALID_ATTRIBUTE => "invalid attribute",
        CUEST_STATUS_INVALID_HANDLE => "invalid handle",
        CUEST_STATUS_UNKNOWN_ERROR => "unknown error",
        CUEST_STATUS_UNSUPPORTED_ARGUMENT => "unsupported argument",
        CUEST_STATUS_UNSUPPORTED_ARCHITECTURE => "unsupported architecture",
        CUEST_STATUS_INVALID_PLAN => "invalid plan",
        CUEST_STATUS_HOME_NOT_FOUND => "home directory not found",
        _ => "unrecognized status code",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::*;

    #[test]
    fn cpu_status_to_str() {
        assert_eq!(status_to_str(CUEST_STATUS_SUCCESS), "success");
        assert_eq!(status_to_str(CUEST_STATUS_HOME_NOT_FOUND), "home directory not found");
        assert_eq!(status_to_str(CuestStatus(1234)), "unrecognized status code");
    }

    #[test]
    fn cpu_version_macros() {
        let version = CUEST_VER_MAJOR * 10000 + CUEST_VER_MINOR * 100 + CUEST_VER_PATCH;
        assert_eq!(CUEST_VERSION, version);
        // Binding canary: update together with a cuEST version bump (see
        // readme).
        assert_eq!((CUEST_VER_MAJOR, CUEST_VER_MINOR, CUEST_VER_PATCH), (0, 2, 2));
    }
}
