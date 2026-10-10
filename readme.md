# cuEST Rust bindings

Rust FFI bindings and wrappers for [cuEST](https://developer.nvidia.com/cuest)
(CUDA Electronic Structure Toolkit), NVIDIA's library for GPU-accelerated
electronic structure computations.

This project is not an official NVIDIA project.

## Workspace structure

```text
cuest-rs/
├── cuest-ffi/          # Raw FFI bindings (auto-generated, unsafe)
└── cuest-cudarc/       # Safe wrapper over cuest-ffi + cudarc
```

## Crate `cuest-ffi`

Raw, unsafe FFI bindings 1:1 with the cuEST C API, generated from the cuEST
public C headers by `cuest-ffi/scripts/generate_ffi.py`. Supports dynamic
library loading (default) and static linking. See `cuest-ffi/readme.md`.

## Crate `cuest-cudarc`

Safe wrapper around the raw bindings built on [cudarc], covering context
creation, AO basis construction (shells, basis, pair lists) and one-electron
integrals (overlap, kinetic, potential). See `cuest-cudarc/readme.md`.

[cudarc]: https://docs.rs/cudarc

## AI assistance

This project is developed with the help of AI assistance (mainly using Claude
Code with customized LLM models). See CLAUDE.md and git history for details.

## License

Apache-2.0 for this project. cuEST itself, including its headers and library,
is proprietary NVIDIA software licensed by NVIDIA and is not distributed with
this project.
