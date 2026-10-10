# cuEST Rust bindings

[![CI](https://github.com/RESTGroup/cuest-rs/actions/workflows/clippy.yml/badge.svg)](https://github.com/RESTGroup/cuest-rs/actions/workflows/clippy.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![cuEST](https://img.shields.io/badge/cuEST-v0.2.2-76b900.svg)](https://developer.nvidia.com/cuest)

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

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/cuest-ffi.svg)](https://crates.io/crates/cuest-ffi) |
| API Document | [![API Documentation](https://docs.rs/cuest-ffi/badge.svg)](https://docs.rs/cuest-ffi) |
| Bound library | [![cuEST v0.2.2](https://img.shields.io/badge/cuEST-v0.2.2-76b900.svg)](https://developer.nvidia.com/cuest) |

Raw, unsafe FFI bindings 1:1 with the cuEST C API, generated from the cuEST
public C headers by `cuest-ffi/scripts/generate_ffi.py`. Supports dynamic
library loading (default) and static linking. See
[cuest-ffi/readme.md](cuest-ffi/readme.md).

## Crate `cuest-cudarc`

| Resources | Badges |
|--|--|
| Crate | [![Crate](https://img.shields.io/crates/v/cuest-cudarc.svg)](https://crates.io/crates/cuest-cudarc) |
| API Document | [![API Documentation](https://docs.rs/cuest-cudarc/badge.svg)](https://docs.rs/cuest-cudarc) |

Safe wrapper around the raw bindings built on [cudarc], covering context
creation, AO basis construction (shells, basis, pair lists) and one-electron
integrals (overlap, kinetic, potential). See
[cuest-cudarc/readme.md](cuest-cudarc/readme.md).

[cudarc]: https://docs.rs/cudarc

## Links

- cuEST (NVIDIA): [product page](https://developer.nvidia.com/cuest) ·
  [documentation](https://docs.nvidia.com/cuda/cuest/index.html)
- cuEST samples: [NVIDIA/CUDALibrarySamples](https://github.com/NVIDIA/CUDALibrarySamples/tree/main/cuEST)
- [cudarc] — safe CUDA driver API bindings used by `cuest-cudarc`
- REST electronic structure toolkit: [RESTGroup/rest](https://gitee.com/RESTGroup/rest)

## Changelogs

- [`cuest-ffi`](CHANGELOG-cuest-ffi.md)
- [`cuest-cudarc`](CHANGELOG-cuest-cudarc.md)

## AI assistance

This project is developed with the help of AI assistance (mainly using Claude
Code with customized LLM models). See CLAUDE.md and git history for details.

## License

Apache-2.0 for this project. cuEST itself, including its headers and library,
is proprietary NVIDIA software licensed by NVIDIA and is not distributed with
this project.
