//! The cuEST context: the top-level handle every other object is derived
//! from.
//!
//! A [`CuestContext`] wraps a user-supplied [`Arc<CudaStream>`]. The stream
//! (and its cuBLAS/cuSolver resources) is bound to the cuEST handle once at
//! creation — cuEST has no per-call streams — so device buffers passed to
//! compute calls must be allocated on this stream.

use crate::error::{status_check, CuestError};
use crate::parameters::CuestParameters;
use cudarc::driver::CudaStream;
use cuest_ffi::ffi;
use std::ffi::{c_int, c_void, CString};
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::Arc;

/// Options for [`CuestContext::with_options`].
///
/// The defaults (all fields `None`) reproduce cuEST's default handle
/// parameters. For JIT-related options and semantics, see the cuEST
/// documentation.
#[derive(Debug, Default, Clone)]
pub struct CuestHandleOptions {
    /// JIT kernel cache directory.
    ///
    /// `None` uses the cuEST default
    /// (`~/.cuest_cache/cuest-cuda<N>-v<version>/`), which requires the
    /// `HOME` environment variable to be set; set this explicitly in
    /// sandboxes, CI, and tests. cuEST rejects unsafe or unwritable
    /// directories.
    pub jit_cache_dir: Option<PathBuf>,
    /// Number of parallel JIT precompile worker threads (>= 1, clamped to
    /// 256 by cuEST). cuEST default: 16.
    pub jit_compile_threads: Option<i32>,
    /// Maximum number of Gauss-Hermite quadrature points. cuEST default: 20.
    pub max_gauss_hermite: Option<u64>,
    /// Highest angular momentum for solid harmonics. cuEST default: 10.
    pub max_l_solid_harmonic: Option<u64>,
    /// Maximum number of Rys quadrature points (0 selects the largest
    /// available). cuEST default: 0.
    pub max_rys: Option<u64>,
}

/// cuEST math mode (see [`CuestContext::math_mode`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CuestMathMode {
    /// Math mode is heuristically chosen per function for maximum efficiency;
    /// some results may use mixed precision arithmetic.
    Default,
    /// Native FP64 compute is used everywhere.
    NativeFp64,
}

impl CuestMathMode {
    fn to_raw(self) -> ffi::cuestMathMode_t {
        match self {
            Self::Default => ffi::CUEST_DEFAULT_MATH_MODE,
            Self::NativeFp64 => ffi::CUEST_NATIVE_FP64_MATH_MODE,
        }
    }

    fn from_raw(raw: ffi::cuestMathMode_t) -> Result<Self, CuestError> {
        match raw {
            ffi::CUEST_DEFAULT_MATH_MODE => Ok(Self::Default),
            ffi::CUEST_NATIVE_FP64_MATH_MODE => Ok(Self::NativeFp64),
            _ => Err(CuestError::Rust(format!("unknown cuestMathMode_t value {}", raw.0))),
        }
    }
}

/// Shared context state; the last [`Arc`] reference destroys the cuEST handle.
pub(crate) struct ContextInner {
    handle: ffi::cuestHandle_t,
    stream: Arc<CudaStream>,
    version: (u32, u32, u32),
    /// Kept alive defensively; cuEST is not documented to copy the string.
    _jit_cache_dir: Option<CString>,
}

impl ContextInner {
    pub(crate) fn handle(&self) -> ffi::cuestHandle_t {
        self.handle
    }

    pub(crate) fn stream(&self) -> &Arc<CudaStream> {
        &self.stream
    }

    pub(crate) fn version(&self) -> (u32, u32, u32) {
        self.version
    }

    /// Verify a device buffer was allocated on this context's stream
    /// (`buffer.stream()`, obtained from any `DeviceSlice`).
    ///
    /// cuEST has a single stream bound at handle creation; using buffers from
    /// another stream would race with the cuEST kernels. Streams are compared
    /// by their underlying CUDA stream and context (`CudaStream::eq`), not by
    /// `Arc` identity — cudarc hands out fresh `Arc`s for e.g. the default
    /// stream on each call.
    pub(crate) fn check_stream(
        &self,
        stream: &Arc<CudaStream>,
        arg: &str,
    ) -> Result<(), CuestError> {
        if self.stream.as_ref() == stream.as_ref() {
            Ok(())
        } else {
            Err(CuestError::Rust(format!(
                "`{arg}` is allocated on a different CUDA stream than the cuEST context; \
                 allocate it on the context stream"
            )))
        }
    }
}

impl Drop for ContextInner {
    fn drop(&mut self) {
        // cuEST requires the handle's device to be current when destroying.
        // Drop cannot report errors; failures only leak GPU resources.
        let _ = self.stream.context().bind_to_thread();
        let _ = unsafe { ffi::cuestDestroy(self.handle) };
    }
}

/// A cuEST context: the library handle bound to one CUDA stream.
///
/// This is the root object of the API; all other objects are created from it
/// (directly or transitively) and keep it alive.
pub struct CuestContext {
    inner: Arc<ContextInner>,
}

impl CuestContext {
    /// Create a context on `stream` with default handle parameters.
    pub fn new(stream: &Arc<CudaStream>) -> Result<Self, CuestError> {
        Self::with_options(stream, &CuestHandleOptions::default())
    }

    /// Create a context on `stream` with explicit handle parameters.
    ///
    /// The stream's device is made current on the calling thread (the
    /// `cudaSetDevice` equivalent) before the handle is created, as cuEST
    /// requires.
    pub fn with_options(
        stream: &Arc<CudaStream>,
        options: &CuestHandleOptions,
    ) -> Result<Self, CuestError> {
        stream.context().bind_to_thread()?;

        let params = CuestParameters::create(ffi::CUEST_HANDLE_PARAMETERS)?;
        let attr = |a: ffi::cuestHandleParametersAttributes_t| a.0 as c_int;
        // Bind the stream to the handle; cuBLAS/cuSolver handles are left
        // NULL so cuEST allocates its own on this stream.
        params.configure_ptr(
            attr(ffi::CUEST_HANDLE_PARAMETERS_CUDASTREAM),
            stream.cu_stream() as *mut c_void,
        )?;
        if let Some(threads) = options.jit_compile_threads {
            params
                .configure_i32(attr(ffi::CUEST_HANDLE_PARAMETERS_JIT_COMPILE_THREADS), threads)?;
        }
        if let Some(value) = options.max_gauss_hermite {
            params.configure_u64(attr(ffi::CUEST_HANDLE_PARAMETERS_MAX_GAUSS_HERMITE), value)?;
        }
        if let Some(value) = options.max_l_solid_harmonic {
            params.configure_u64(attr(ffi::CUEST_HANDLE_PARAMETERS_MAX_L_SOLID_HARMONIC), value)?;
        }
        if let Some(value) = options.max_rys {
            params.configure_u64(attr(ffi::CUEST_HANDLE_PARAMETERS_MAX_RYS), value)?;
        }
        let jit_cache_dir = options.jit_cache_dir.as_deref().map(cstr_from_path).transpose()?;
        if let Some(dir) = &jit_cache_dir {
            params.configure_cstr(attr(ffi::CUEST_HANDLE_PARAMETERS_JIT_CACHE_DIR), dir)?;
        }

        let mut handle: ffi::cuestHandle_t = ptr::null_mut();
        status_check(unsafe { ffi::cuestCreate(params.raw(), &mut handle) }, "cuestCreate")?;

        let version = match query_version(handle) {
            Ok(version) => version,
            Err(err) => {
                let _ = unsafe { ffi::cuestDestroy(handle) };
                return Err(err);
            },
        };
        let needed = (ffi::CUEST_VER_MAJOR, ffi::CUEST_VER_MINOR, ffi::CUEST_VER_PATCH);
        if (version.0, version.1) != (needed.0, needed.1) {
            let _ = unsafe { ffi::cuestDestroy(handle) };
            return Err(CuestError::UnsupportedVersion { needed, found: version });
        }

        Ok(Self {
            inner: Arc::new(ContextInner {
                handle,
                stream: stream.clone(),
                version,
                _jit_cache_dir: jit_cache_dir,
            }),
        })
    }

    /// The CUDA stream this context's handle is bound to.
    pub fn stream(&self) -> &Arc<CudaStream> {
        self.inner.stream()
    }

    /// cuEST library version (major, minor, patch) reported at creation.
    pub fn version(&self) -> (u32, u32, u32) {
        self.inner.version()
    }

    /// The math mode of this handle.
    pub fn math_mode(&self) -> Result<CuestMathMode, CuestError> {
        let mut mode = CuestMathMode::Default.to_raw();
        status_check(
            unsafe { ffi::cuestGetMathMode(self.inner.handle(), &mut mode) },
            "cuestGetMathMode",
        )?;
        CuestMathMode::from_raw(mode)
    }

    /// Set the math mode of this handle.
    pub fn set_math_mode(&self, mode: CuestMathMode) -> Result<(), CuestError> {
        status_check(
            unsafe { ffi::cuestSetMathMode(self.inner.handle(), mode.to_raw()) },
            "cuestSetMathMode",
        )
    }

    /// The compute capability (major, minor) used for heuristics.
    pub fn compute_capability_target(&self) -> Result<(u32, u32), CuestError> {
        let (mut major, mut minor) = (0u32, 0u32);
        status_check(
            unsafe {
                ffi::cuestGetComputeCapabilityTarget(self.inner.handle(), &mut major, &mut minor)
            },
            "cuestGetComputeCapabilityTarget",
        )?;
        Ok((major, minor))
    }

    /// Override the compute capability used for heuristics (not for kernel
    /// execution or JIT compilation); see the cuEST documentation.
    pub fn set_compute_capability_target(&self, major: u32, minor: u32) -> Result<(), CuestError> {
        status_check(
            unsafe { ffi::cuestSetComputeCapabilityTarget(self.inner.handle(), major, minor) },
            "cuestSetComputeCapabilityTarget",
        )
    }

    /// The raw cuEST handle (escape hatch for API surface not wrapped yet).
    pub fn raw_handle(&self) -> ffi::cuestHandle_t {
        self.inner.handle()
    }

    pub(crate) fn inner(&self) -> &Arc<ContextInner> {
        &self.inner
    }
}

impl Clone for CuestContext {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl std::fmt::Debug for CuestContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CuestContext").field("version", &self.version()).finish_non_exhaustive()
    }
}

// SAFETY: see the "Thread safety" section of the crate documentation: the
// handle may be moved between threads, but a handle (and everything derived
// from it) must be used from one thread at a time — cuEST documents no
// internal synchronization.
unsafe impl Send for CuestContext {}

fn query_version(handle: ffi::cuestHandle_t) -> Result<(u32, u32, u32), CuestError> {
    let (mut major, mut minor, mut patch) = (0u32, 0u32, 0u32);
    status_check(unsafe { ffi::cuestGetMajorVersion(handle, &mut major) }, "cuestGetMajorVersion")?;
    status_check(unsafe { ffi::cuestGetMinorVersion(handle, &mut minor) }, "cuestGetMinorVersion")?;
    status_check(unsafe { ffi::cuestGetPatchVersion(handle, &mut patch) }, "cuestGetPatchVersion")?;
    Ok((major, minor, patch))
}

fn cstr_from_path(path: &Path) -> Result<CString, CuestError> {
    let string = path.to_str().ok_or_else(|| {
        CuestError::Rust(format!("JIT cache directory path is not valid UTF-8: {}", path.display()))
    })?;
    CString::new(string).map_err(|_| {
        CuestError::Rust(format!("JIT cache directory path contains NUL: {}", path.display()))
    })
}
