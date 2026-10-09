//! Safe Rust wrapper for [cuEST](https://developer.nvidia.com/cuest) (CUDA
//! Electronic Structure Toolkit) built on [cudarc].
//!
//! This crate wraps the raw [`cuest_ffi`] bindings into a safe API: cuEST
//! handles become owned Rust objects, user-allocated workspaces are managed
//! internally, every return code is checked, and device buffers are tied to
//! the cudarc stream the cuEST handle was created on.
//!
//! This crate is not an official NVIDIA project. cuEST itself is proprietary
//! NVIDIA software and is not distributed with this crate.
//!
//! # Design notes
//!
//! - A [`CuestContext`] wraps a user-supplied `Arc<CudaStream>`. The stream is
//!   bound once at `cuestCreate` (cuEST has no per-call streams), so every
//!   device buffer passed to a compute call must live on that stream; this is
//!   checked at runtime and reported as [`CuestError::Rust`] otherwise.
//! - Objects keep their dependencies alive (an [`Arc`](std::sync::Arc)-based
//!   keep-alive, not lifetime parameters), so cuEST's destruction order is
//!   enforced by normal `Drop` order.
//! - Device buffers for inputs are taken as `&impl DevicePtr<f64>` (any
//!   [`CudaSlice`](cudarc::driver::CudaSlice),
//!   [`CudaView`](cudarc::driver::CudaView) or
//!   [`CudaViewMut`](cudarc::driver::CudaViewMut)); outputs are either
//!   auto-allocated or written into an `&mut impl DevicePtrMut<f64>` (`*_into`
//!   methods). No synchronization is performed inside compute methods — the
//!   work is stream-ordered, as with cudarc itself.
//! - All calls are error-checked; a failing cuEST status becomes
//!   [`CuestError::Status`]. Buffer sizes the cuEST C API cannot validate (the
//!   caller contract) are validated here instead.
//! - The `unsafe` blocks are cuEST FFI calls (justified by owned handles, the
//!   cuEST call contracts, and the validations above) and one uninitialized
//!   scratch allocation (cuEST workspace buffers, matching the C samples'
//!   `cudaMalloc`).
//!
//! # Example
//!
//! ```no_run
//! use cuest_cudarc::prelude::*;
//! use cuest_cudarc::cudarc::driver::CudaContext;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let stream = CudaContext::new(0)?.default_stream();
//! let context = CuestContext::new(&stream)?;
//!
//! // H2 at 1.4 Bohr (0.7408 Angstrom), STO-3G, on one atom each.
//! let xyz = Xyz::parse(
//!     "2
//! H2
//! H 0.0 0.0 0.0
//! H 0.0 0.0 0.7408
//! ",
//! )?;
//!
//! let spec = CuestShellSpec::new(0, true, vec![3.42525091, 0.62391373, 0.16885540], vec![
//!     0.15432897, 0.53532814, 0.44463454,
//! ])?
//! .normalized(1.0)?;
//! let shells: Vec<CuestAOShell> =
//!     (0..xyz.len()).map(|_| CuestAOShell::new(&context, &spec)).collect::<Result<_, _>>()?;
//! let basis = CuestAOBasis::new(&context, &[&shells[0..1], &shells[1..2]])?;
//!
//! let pair_list = CuestAOPairList::from_xyz(&basis, &xyz, 1.0e-14)?;
//! let plan = CuestOeIntPlan::new(&basis, &pair_list)?;
//!
//! let s = plan.overlap()?; // CudaSlice<f64>, nao x nao, on `stream`
//! let t = plan.kinetic()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Loading modes
//!
//! The loading mode is selected through this crate's features (matching
//! `cuest-ffi`):
//!
//! - **Dynamic loading** (default, `dynamic_loading` feature): the cuEST shared
//!   library is loaded at runtime; set `CUEST_DYLOAD` or see
//!   [`cuest_ffi::ffi_dynamic`] for the search order.
//! - **Static linking** (`--no-default-features`): links `libcuest` at build
//!   time (`CUEST_LIB_DIR` overrides the search path).
//!
//! # Thread safety
//!
//! cuEST documents no thread-safety guarantees beyond the usage model of one
//! handle per stream. The types in this crate are therefore `Send` (they can
//! be moved between threads) but **not** `Sync`: a given handle (and any
//! object derived from it) must be used from one thread at a time. The
//! underlying CUDA stream is the real serialization point.
//!
//! # Escape hatch
//!
//! The raw bindings are re-exported as [`cuest_ffi`] for API surface this
//! crate does not wrap yet; every object also exposes its raw handle
//! (`raw_handle()`).

#![doc = include_str!("../readme.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(non_snake_case)]
// Deliberate: the shared per-object state (`Arc<...Inner>`) holds a raw cuEST
// handle and is therefore neither `Send` nor `Sync`; the public wrapper types
// add a documented `unsafe impl Send` instead (see "Thread safety" above).
#![allow(clippy::arc_with_non_send_sync)]

pub mod basis;
pub mod context;
pub mod error;
pub mod oeint;
pub mod pair_list;
pub mod query;
pub mod shell;
pub mod xyz;

pub(crate) mod parameters;
pub(crate) mod workspace;

/// The cudarc version this crate was built against.
pub use cudarc;
/// The raw cuEST FFI bindings this crate builds on (escape hatch).
pub use cuest_ffi;

pub use crate::basis::CuestAOBasis;
pub use crate::context::{CuestContext, CuestHandleOptions, CuestMathMode};
pub use crate::error::CuestError;
pub use crate::oeint::{
    CuestFfloatUsageMode, CuestJitUsageMode, CuestOeIntPlan, CuestPotentialComputeOptions,
};
pub use crate::pair_list::CuestAOPairList;
pub use crate::shell::{CuestAOShell, CuestShellSpec};
pub use crate::xyz::Xyz;

/// Common imports.
pub mod prelude {
    pub use crate::basis::CuestAOBasis;
    pub use crate::context::{CuestContext, CuestHandleOptions, CuestMathMode};
    pub use crate::error::CuestError;
    pub use crate::oeint::{
        CuestFfloatUsageMode, CuestJitUsageMode, CuestOeIntPlan, CuestPotentialComputeOptions,
    };
    pub use crate::pair_list::CuestAOPairList;
    pub use crate::shell::{CuestAOShell, CuestShellSpec};
    pub use crate::xyz::Xyz;
}
