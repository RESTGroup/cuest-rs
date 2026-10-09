//! Host/device scratch workspaces required by cuEST object creation and
//! compute calls.
//!
//! cuEST asks the caller to allocate both buffers of a `cuestWorkspace_t`
//! with the exact sizes reported by a `...WorkspaceQuery` call. A
//! [`Workspace`] owns both buffers:
//!
//! - the host buffer is a `Vec<u64>` (8-byte aligned, rounded up in size);
//! - the device buffer is a cudarc [`CudaSlice<u8>`](CudaSlice), allocated on
//!   the context's stream so that allocation and the cuEST kernels writing into
//!   it are stream-ordered.
//!
//! Persistent workspaces are owned by the object created with them;
//! temporary workspaces are allocated at the start of a
//! `...WorkspaceQuery`/create (or compute) sequence and dropped at the end of
//! the call.

use crate::error::CuestError;
use cudarc::driver::{CudaSlice, CudaStream, DevicePtr, SyncOnDrop};
use cuest_ffi::ffi;
use std::sync::Arc;

/// A zero-initialized workspace descriptor for `...WorkspaceQuery` output.
pub(crate) fn new_descriptor() -> ffi::cuestWorkspaceDescriptor_t {
    ffi::cuestWorkspaceDescriptor_t { hostBufferSizeInBytes: 0, deviceBufferSizeInBytes: 0 }
}

pub(crate) struct Workspace {
    host: Vec<u64>,
    host_bytes: usize,
    device: Option<CudaSlice<u8>>,
    device_bytes: usize,
}

impl Workspace {
    /// Allocate both buffers of `desc` on `stream`.
    pub(crate) fn new(
        stream: &Arc<CudaStream>,
        desc: &ffi::cuestWorkspaceDescriptor_t,
    ) -> Result<Self, CuestError> {
        let host_bytes = desc.hostBufferSizeInBytes;
        let device_bytes = desc.deviceBufferSizeInBytes;
        // u64 elements guarantee 8-byte alignment; the byte size is rounded
        // up, which is safe: cuEST only requires at least `host_bytes`.
        let host = vec![0u64; host_bytes.div_ceil(8)];
        // SAFETY: the workspace is opaque scratch memory; cuEST is entitled
        // to read it only after writing it (the C samples allocate it with
        // plain cudaMalloc, i.e. uninitialized).
        let device = if device_bytes > 0 {
            Some(unsafe { stream.alloc::<u8>(device_bytes)? })
        } else {
            None
        };
        Ok(Self { host, host_bytes, device, device_bytes })
    }

    /// Build the raw `cuestWorkspace_t` for an FFI call.
    ///
    /// If a device buffer exists, the returned [`SyncOnDrop`] guard (which
    /// borrows the buffer and the stream) must stay alive until the FFI call
    /// that consumes the pointer has been issued.
    pub(crate) fn raw<'a>(
        &'a self,
        stream: &'a Arc<CudaStream>,
    ) -> (ffi::cuestWorkspace_t, Option<SyncOnDrop<'a>>) {
        let (device_ptr, guard) = match &self.device {
            Some(device) => {
                let (ptr, guard) = device.device_ptr(stream);
                (ptr as usize, Some(guard))
            },
            None => (0, None),
        };
        let host_ptr = if self.host_bytes > 0 { self.host.as_ptr() as usize } else { 0 };
        let raw = ffi::cuestWorkspace_t {
            hostBuffer: host_ptr,
            hostBufferSizeInBytes: self.host_bytes,
            deviceBuffer: device_ptr,
            deviceBufferSizeInBytes: self.device_bytes,
        };
        (raw, guard)
    }
}
