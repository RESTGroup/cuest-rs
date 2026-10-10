# cuEST FFI bindings

Raw, unsafe Rust FFI bindings (1:1 with the C API) for
[cuEST](https://developer.nvidia.com/cuest) (CUDA Electronic Structure
Toolkit), NVIDIA's library for GPU-accelerated electronic structure
computations.

This crate is not an official NVIDIA project. cuEST itself, including its
headers and shared library, is proprietary NVIDIA software and is not
distributed with this crate; obtain it from NVIDIA. Users should prefer a safe
wrapper crate (planned `cuest-cudarc`) over using this crate directly.

## Loading modes

- **Dynamic loading** (default, `dynamic_loading` feature): the cuEST shared
  library is loaded at runtime via `libloading`. Set the `CUEST_DYLOAD`
  environment variable to the shared library path, or let the crate search
  standard locations (`LD_LIBRARY_PATH`, conda/python environments including
  the `nvidia-cuest` pip wheel layout, system directories). See the
  `ffi_dynamic` module documentation for the full search order.
- **Static linking** (`--no-default-features`): links `libcuest` at build
  time. Set `CUEST_LIB_DIR` to the cuEST installation prefix if it is not in a
  standard location. Programs linking cuEST must also link cuBLAS and cuSolver
  from the CUDA toolkit (set `CUDA_HOME` if it is not in a standard location).

## Usage

```rust
use cuest_ffi::ffi::*;

let mut params = std::ptr::null_mut();
let status = unsafe { cuestParametersCreate(CUEST_HANDLE_PARAMETERS, &mut params) };
assert_eq!(status, CUEST_STATUS_SUCCESS);
```

`cuestStatus_t` return codes are the only error channel of the cuEST C API;
the `status` module provides `status_to_str` for human-readable messages.
Loading the library (`cuestCreate`) requires an NVIDIA GPU and a CUDA-capable
driver; JIT-compiled kernels are cached under `~/.cuest_cache` by default
(configurable via the `CUEST_HANDLE_PARAMETERS_JIT_CACHE_DIR` parameter).

## Bindings generation

Bindings are auto-generated from the cuEST public C headers by
`scripts/generate_ffi.py` (bindgen CLI + post-processing). Enums are generated
as `#[repr(transparent)]` newtypes with global constants (bindgen
`newtype_global` style): same C ABI, nominally typed, and any integer value
(including future enum members) remains valid. Headers are not
vendored in this repository; set `CUEST_INSTALL_DIR` to a local cuEST
installation and run the script to regenerate. Never edit the generated files
(`src/ffi_static.rs`, `src/ffi_dynamic/{ffi_base,dyload_compatible,dyload_initializer,dyload_struct}.rs`)
manually; `src/ffi_dynamic/mod.rs` (runtime library search) is hand-written.

## AI assistance

This project is developed with the help of AI assistance (mainly using Claude
Code with customized LLM models). See CLAUDE.md and git history for details.

## License

Apache-2.0. This project is not affiliated with NVIDIA Corporation. It
contains AI-generated bindings derived from the public cuEST C API;
the cuEST library and its headers are licensed separately by NVIDIA under the
NVIDIA cuEST license agreement.
