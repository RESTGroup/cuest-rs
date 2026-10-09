# Generated FFI files of `cuest-ffi`

**Never edit manually** (they are overwritten by regeneration):

- `cuest-ffi/src/ffi_static.rs`
- `cuest-ffi/src/ffi_dynamic/ffi_base.rs`
- `cuest-ffi/src/ffi_dynamic/dyload_struct.rs`
- `cuest-ffi/src/ffi_dynamic/dyload_initializer.rs`
- `cuest-ffi/src/ffi_dynamic/dyload_compatible.rs`

Hand-written files that may be edited: `cuest-ffi/src/ffi_dynamic/mod.rs`
(runtime library search), `cuest-ffi/src/lib.rs`, `cuest-ffi/src/status.rs`,
`cuest-ffi/build.rs`, `cuest-ffi/tests/*`.

## Regeneration

The bindings are generated from the cuEST public C headers; headers are **not**
vendored in this repository (proprietary NVIDIA software).

```bash
cd cuest-ffi/scripts
CUEST_INSTALL_DIR=/path/to/cuest-installation python3 generate_ffi.py
```

The script runs `bindgen` + tree-sitter post-processing and `cargo fmt`; the
generated files are committed. Requirements: `bindgen` CLI, python packages
`tree-sitter` and `tree-sitter-rust`.

## Version bumps

When pointing `CUEST_INSTALL_DIR` at a new cuEST version:

1. Re-run the generation script (above).
2. Update the version canary in `cuest-ffi/src/status.rs`
   (`test_version_macros`) and any version notes in `cuest-ffi/readme.md`.
3. Check the upstream release notes for behavioral changes (e.g. changed
   numerical schemes, removed enum values) and for any struct/function
   signature diff in the generated files (`git diff` after regeneration).
4. There are no `api-vX_Y` cargo features: the crate binds exactly one cuEST
   API version at a time. Add version features only when a second version must
   be supported simultaneously.
