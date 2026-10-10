# Changelog

## v0.1.0 -- 2026-10-09

Initial release of `cuest-ffi`, the raw cuEST bindings.

- Auto-generated Rust FFI bindings (1:1 with the C API) for cuEST v0.2.2,
  produced by `scripts/generate_ffi.py` (bindgen + tree-sitter post-processing).
- Loading modes: dynamic loading via `libloading` (default, `dynamic_loading`
  feature) with runtime library search (`CUEST_DYLOAD`, `LD_LIBRARY_PATH`,
  conda/python environments, system paths); static linking via
  `--no-default-features` (`build.rs` emits the link flags for `cuest`, and the
  mandatory `cublas`/`cusolver` co-links).
- `status` module with `status_to_str` for human-readable `cuestStatus_t`
  return codes.
- Enums generated as `#[repr(transparent)]` newtypes with global constants, so
  any integer value (including future enum members) stays valid.
