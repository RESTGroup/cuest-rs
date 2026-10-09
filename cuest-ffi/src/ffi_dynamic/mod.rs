//! FFI module for cuEST (dynamic loading).
//!
//! This module provides dynamic loading support. It is compiled only when the
//! `dynamic_loading` feature is enabled (see `lib.rs`).
//!
//! # Rule and guide of dynamic loading
//!
//! By enabling the `dynamic_loading` feature (which is the default), the crate
//! will attempt to load the cuEST shared library at runtime. The loading
//! process will search for the library in multiple locations, by the following
//! order:
//!
//! 1. User-defined candidates via environment variables `CUEST_DYLOAD_CUEST` or
//!    `CUEST_DYLOAD`. Values may be library paths or directories.
//! 2. `LD_LIBRARY_PATH` style discovery via environment variables
//!    `LD_LIBRARY_PATH` (Linux), `DYLD_LIBRARY_PATH` and
//!    `DYLD_FALLBACK_LIBRARY_PATH` (macOS), `PATH` (Windows). Note we are not
//!    distinguishing different operating systems, so all these environment
//!    variables will be checked on all platforms.
//! 3. Python interpreter path discovery: for each python interpreter found, the
//!    library is expected in the `lib` directory of the python installation
//!    (e.g. `/path/lib/libcuest.so` for a python at `/path/bin/python`), or
//!    under the `nvidia/cuest` pip wheel layout (e.g.
//!    `/path/lib/python3.*/site-packages/nvidia/cuest/lib/libcuest.so`).
//!    - The python interpreter path of `CUEST_PYTHON_PATH` environment
//!      variable, if set.
//!    - The conda prefix path of `CONDA_PREFIX` environment variable, if set.
//!    - The python interpreter path in `PATH` environment variable, if exists.
//!      Will first check `python`, then `python3`.
//! 4. Standard system candidates, such as `libcuest.so` in some common library
//!    directories such as `/usr/lib`, `/usr/local/lib`, and `/lib`.
//!
//! For API developer, if you want to check the library loading sequence, you
//! can try the following code:
//! ```no_run
//! let candidates = unsafe { &cuest_ffi::ffi_dynamic::dyload_lib().__libraries_path };
//! println!("Library loading candidates: {candidates:#?}");
//! ```

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub const MOD_NAME: &str = module_path!();
pub const LIB_NAME: &str = "CUEST";
pub const LIB_NAME_SHOW: &str = "cuEST";
pub const LIB_NAME_LINK: &str = "cuest";

mod dynamic_loading_specific {
    use super::*;
    use libloading::Library;
    use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
    use std::fmt::Debug;
    use std::sync::OnceLock;

    /// Candidate file names of the cuEST shared library.
    fn lib_names() -> Vec<String> {
        vec![
            format!("{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
            format!("{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}.0"),
        ]
    }

    /// Detect Python interpreter path and return the corresponding lib
    /// directory. Uses OnceLock pattern for lazy initialization.
    static PYTHON_LIB_PATH: OnceLock<Vec<String>> = OnceLock::new();

    fn extract_base_from_python_bin(python_bin: &str) -> Option<String> {
        // If python is at /path/to/bin/python, the base is /path/to
        let bin_path = std::path::Path::new(python_bin);
        bin_path.parent()?.parent().map(|base| base.to_string_lossy().to_string())
    }

    fn push_python_base(base: &str, lib_paths: &mut Vec<String>) {
        let lib_path = std::path::Path::new(base).join("lib");
        if !lib_path.exists() {
            return;
        }
        lib_paths.push(lib_path.to_string_lossy().to_string());

        // NVIDIA pip wheels place the library at
        // `lib/python3.*/{site,dist}-packages/nvidia/cuest/lib`.
        if let Ok(entries) = std::fs::read_dir(&lib_path) {
            for entry in entries.flatten() {
                if !entry.file_name().to_string_lossy().starts_with("python") {
                    continue;
                }
                for site in ["site-packages", "dist-packages"] {
                    let wheel_lib = entry.path().join(site).join("nvidia/cuest/lib");
                    if wheel_lib.exists() {
                        lib_paths.push(wheel_lib.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    fn detect_python_lib_paths() -> Vec<String> {
        PYTHON_LIB_PATH
            .get_or_init(|| {
                let mut lib_paths = vec![];

                // 1. Check explicit environment variable first
                if let Ok(python_path) = std::env::var("CUEST_PYTHON_PATH") {
                    if let Some(base) = extract_base_from_python_bin(&python_path) {
                        push_python_base(&base, &mut lib_paths);
                    }
                }

                // 2. Check conda prefix, if it exists
                if let Ok(conda_prefix) = std::env::var("CONDA_PREFIX") {
                    push_python_base(&conda_prefix, &mut lib_paths);
                }

                // 3. Try to find python in PATH (first `python`, then
                //    `python3`)
                if let Ok(paths) = std::env::var("PATH") {
                    for name in ["python", "python3"] {
                        for path in paths.split(":") {
                            let python_bin = format!("{path}/{name}");
                            if std::path::Path::new(&python_bin).exists() {
                                if let Some(base) = extract_base_from_python_bin(&python_bin) {
                                    push_python_base(&base, &mut lib_paths);
                                }
                            }
                        }
                    }
                }

                lib_paths.dedup();
                lib_paths
            })
            .clone()
    }

    fn get_lib_candidates() -> Vec<String> {
        let names = lib_names();
        let mut candidates = vec![];

        // User-defined candidates via environment variables; entries may be
        // library paths or directories.
        for env_var in [format!("CUEST_DYLOAD_{LIB_NAME}").as_str(), "CUEST_DYLOAD"] {
            if let Ok(path) = std::env::var(env_var) {
                for path in path.split(":").filter(|p| !p.is_empty()) {
                    if std::path::Path::new(path).is_dir() {
                        for name in &names {
                            candidates.push(format!("{path}/{name}"));
                        }
                    } else {
                        candidates.push(path.to_string());
                    }
                }
            }
        }

        // LD_LIBRARY_PATH style discovery
        let mut dirs = vec![];
        for env_var in [
            "LD_LIBRARY_PATH",            // linux
            "DYLD_LIBRARY_PATH",          // macos
            "DYLD_FALLBACK_LIBRARY_PATH", // macos
            "PATH",                       // windows
        ] {
            if let Ok(paths) = std::env::var(env_var) {
                dirs.extend(paths.split(":").map(|path| path.to_string()));
            }
        }

        // Python interpreter path discovery (cached)
        dirs.extend(detect_python_lib_paths());

        // Standard system candidates
        dirs.extend(["/usr/lib".to_string(), "/usr/local/lib".to_string(), "/lib".to_string()]);

        for dir in dirs {
            for name in &names {
                candidates.push(format!("{dir}/{name}"));
            }
        }

        // Bare names, letting the dynamic loader search default locations
        candidates.extend(names);
        candidates
    }

    fn check_lib_loaded(lib: &DyLoadLib) -> bool {
        lib.cuestCreate.is_some()
    }

    fn panic_no_lib_found<S: Debug>(candidates: &[S], err_msg: &str) -> ! {
        panic!(
            r#"
This happens in module `{MOD_NAME}`.
Unable to dynamically load the {LIB_NAME_SHOW} (`{LIB_NAME_LINK}`) shared library.
Candidates: {candidates:#?}

Please check:
- If dynamic-loading is not desired, disable the `dynamic_loading` feature in Cargo.toml.
- Use environment variable `CUEST_DYLOAD_{LIB_NAME}` or `CUEST_DYLOAD` to specify the library path.
- If `lib{LIB_NAME_LINK}.so` is installed on your system.
- If `LD_LIBRARY_PATH` is set correctly.
- Python interpreter path discovery: if Python is at `/path/bin/python`, the
  library is expected at `/path/lib/libcuest.so` or under the pip wheel layout
  `/path/lib/python3.*/site-packages/nvidia/cuest/lib/libcuest.so`.

Error message(s):
{err_msg}
"#
        )
    }

    fn panic_condition_not_met<S: Debug>(candidates: &[S]) -> ! {
        panic!(
            r#"
This happens in module `{MOD_NAME}`.
Library loaded but condition not met: `cuestCreate` not found.
Found libraries: {candidates:#?}

Please check that the loaded library is a valid cuEST library.
"#
        )
    }

    pub unsafe fn dyload_lib() -> &'static DyLoadLib {
        static LIB: OnceLock<DyLoadLib> = OnceLock::new();

        LIB.get_or_init(|| {
            let candidates = get_lib_candidates();
            let (mut libraries, mut libraries_path) = (vec![], vec![]);
            let mut err_msg = String::new();
            for candidate in &candidates {
                match Library::new(candidate) {
                    Ok(l) => {
                        libraries.push(l);
                        libraries_path.push(candidate.to_string());
                    },
                    Err(e) => err_msg.push_str(&format!("Failed to load `{candidate}`: {e}\n")),
                }
            }
            let lib = DyLoadLib::new(libraries, libraries_path);
            if lib.__libraries.is_empty() {
                panic_no_lib_found(&candidates, &err_msg);
            }
            if !check_lib_loaded(&lib) {
                panic_condition_not_met(&lib.__libraries_path);
            }
            lib
        })
    }
}

pub use dynamic_loading_specific::*;

/* #region general configuration */

pub(crate) mod ffi_base;
pub use ffi_base::*;

pub(crate) mod dyload_compatible;
pub(crate) mod dyload_initializer;
pub(crate) mod dyload_struct;

pub use dyload_compatible::*;
pub use dyload_struct::*;

/* #endregion */
