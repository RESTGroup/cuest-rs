//! FFI bindings for cuEST (CUDA Electronic Structure Toolkit).
//!
//! This crate provides raw, unsafe FFI bindings to the cuEST C API,
//! one-to-one with the C declarations. Prefer a safe wrapper (e.g.
//! `cuest-cudarc`) over using this crate directly.
//!
//! The bindings are auto-generated from the cuEST public C headers; do not
//! edit the generated files manually. See `scripts/generate_ffi.py`.
//!
//! cuEST itself is NVIDIA proprietary software and is not distributed with
//! this crate; see the readme for how to obtain it and point this crate to it.
//!
//! # Loading Modes
//!
//! - **Dynamic loading** (default, `dynamic_loading` feature): `libcuest` is
//!   loaded at runtime via `libloading`. Set `CUEST_DYLOAD` to the library
//!   path, or let the crate search standard locations (see `ffi_dynamic` module
//!   documentation).
//! - **Static linking** (`--no-default-features`): links `libcuest` at build
//!   time. Set `CUEST_LIB_DIR` if the library is in a nonstandard location.

// Doxygen markers in the cuEST header comments (`[in]`, `[out]`, `<N>`, ...)
// are not rustdoc links or HTML; keep the original comment text as-is.
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(non_snake_case)]

#[cfg(not(feature = "dynamic_loading"))]
pub mod ffi_static;
#[cfg(not(feature = "dynamic_loading"))]
pub use ffi_static as ffi;

#[cfg(feature = "dynamic_loading")]
pub mod ffi_dynamic;
#[cfg(feature = "dynamic_loading")]
pub use ffi_dynamic as ffi;

pub mod status;
