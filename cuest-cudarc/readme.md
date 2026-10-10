# cuest-cudarc

Safe Rust wrapper for [cuEST](https://developer.nvidia.com/cuest) (CUDA
Electronic Structure Toolkit) on top of [cudarc](https://docs.rs/cudarc), the
safe CUDA driver API crate.

This crate is not an official NVIDIA project. cuEST itself, including its
headers and shared library, is proprietary NVIDIA software and is not
distributed with this crate; obtain it from NVIDIA. The raw bindings live in
the `cuest-ffi` crate of this workspace.

## Usage

```rust,no_run
use cuest_cudarc::prelude::*;
use cuest_cudarc::cudarc::driver::CudaContext;

let stream = CudaContext::new(0).unwrap().default_stream();
let context = CuestContext::new(&stream).unwrap();

// H2, STO-3G: one s shell per atom.
let xyz = Xyz::parse("2\nH2\nH 0.0 0.0 0.0\nH 0.0 0.0 0.7408\n").unwrap();
let spec = CuestShellSpec::new(
    0,
    true,
    vec![3.42525091, 0.62391373, 0.16885540],
    vec![0.15432897, 0.53532814, 0.44463454],
)
.unwrap()
.normalized(1.0)
.unwrap();
let shells: Vec<CuestAOShell> =
    (0..xyz.len()).map(|_| CuestAOShell::new(&context, &spec).unwrap()).collect();
let basis = CuestAOBasis::new(&context, &[&shells[0..1], &shells[1..2]]).unwrap();

let pair_list = CuestAOPairList::from_xyz(&basis, &xyz, 1.0e-14).unwrap();
let plan = CuestOeIntPlan::new(&basis, &pair_list).unwrap();

let s = plan.overlap().unwrap(); // CudaSlice<f64>, nao x nao, on `stream`
let t = plan.kinetic().unwrap();
```

The wrapper owns all cuEST handles (with correct destruction order enforced by
`Arc`-based keep-alives), allocates and frees the host/device workspaces
required by every create/compute call, validates the buffer sizes the cuEST C
API cannot check, and verifies that every device buffer lives on the CUDA
stream the context is bound to. Compute methods are stream-ordered and do not
synchronize; use cudarc to read results back.

## Scope

Wrapped by this crate today (the v1 slice):

- `CuestContext` (handle creation, math mode, compute-capability target,
  version queries);
- `CuestAOShell` / `CuestAOBasis` / `CuestAOPairList` (basis construction with
  the cuEST coefficient normalization);
- `CuestOeIntPlan` with overlap (`S`), kinetic (`T`) and potential (`V`)
  matrix computation;
- an XYZ parser for geometries (Angstrom input, Bohr conversion).

Not wrapped yet (use the raw [`cuest_ffi`](https://docs.rs/cuest-ffi) bindings
through the escape hatches — every object exposes `raw_handle()`): density
fitting (J/K), exchange-correlation, ECP, grids, PCM, multipole/nabla/angular
momentum integrals and all derivative computations.

### Contraction coefficients

cuEST uses primitive exponents and coefficients *verbatim*: the coefficients
must include the primitive normalization (relative to **raw**, unnormalized
primitives), which is *not* the convention of standard basis files. Use
`CuestShellSpec::normalized(normalization)` (resp.
`cuest_cudarc::shell::normalize_coefficients`) to convert coefficients taken
from a basis file (`.gbs` and friends); do not normalize twice.

## Loading modes

- **Dynamic loading** (default, `dynamic_loading` feature): the cuEST shared
  library is loaded at runtime; set `CUEST_DYLOAD` to the library path, or let
  `cuest-ffi` search standard locations.
- **Static linking** (`--no-default-features`): links `libcuest` at build time
  (`CUEST_LIB_DIR` overrides the search path).

## Thread safety

Types are `Send` but not `Sync`: handles may be moved between threads but must
be used from one thread at a time (the CUDA stream is the serialization
point). cuEST documents no further thread-safety guarantees.

## Testing

`cargo test` runs the pure-Rust unit tests (no GPU, no cuEST library needed).
The end-to-end tests are `#[ignore]`d and need a GPU plus a local cuEST
installation:

```bash
CUEST_DYLOAD=/path/to/libcuest.so LD_LIBRARY_PATH=/usr/local/cuda/lib64 \
    cargo test -p cuest-cudarc -- --ignored
```

They compare `S`/`T`/`V` matrices for H2/STO-3G and H2O/def2-SVP against
pyscf reference values in `tests/regression/reference.toml` (regenerate with
`tests/regression/gen_reference.py`). The tests set the JIT kernel cache to
`target/cuest-jit-cache` (cuEST otherwise requires `HOME`); in restricted
environments pass `CuestHandleOptions::jit_cache_dir` explicitly.

## AI assistance

This project is developed with the help of AI assistance (mainly using Claude
Code with customized LLM models). See CLAUDE.md and git history for details.

## License

Apache-2.0. This project is not affiliated with NVIDIA Corporation; the cuEST
library and its headers are licensed separately by NVIDIA under the NVIDIA
cuEST license agreement.
