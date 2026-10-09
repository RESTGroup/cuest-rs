//! Build script for `cuest-ffi`.
//!
//! Link flags are emitted only in static linking mode (when the
//! `dynamic_loading` feature is disabled). In dynamic loading mode (the
//! default) nothing is linked at build time; `libcuest` is loaded at runtime.
//!
//! Missing libraries produce warnings instead of build failures, so that
//! `cargo check` and documentation builds succeed without cuEST installed.

use std::path::{Path, PathBuf};

/// Split an environment variable path list into individual paths.
fn split_paths(paths: &str) -> Vec<String> {
    let split_char = if cfg!(windows) { ";" } else { ":" };
    paths.split(split_char).map(|path| path.to_string()).collect()
}

/// Library directories appended to every root candidate. Covers common Linux,
/// macOS and Windows layouts and CUDA toolkit stub locations.
///
/// Code modified from cudarc's build.rs.
const LIB_CANDIDATES: &[&str] = &[
    "",
    "lib",
    "lib/stubs",
    "lib/x64",
    "lib/Win32",
    "lib/x86_64",
    "lib/x86_64-linux-gnu",
    "lib64",
    "lib64/stubs",
    "targets/x86_64-linux",
    "targets/x86_64-linux/lib",
    "targets/x86_64-linux/lib/stubs",
];

/// Existing link search directories under the given roots.
///
/// Environment variables win over typical locations: when any environment
/// variable in `env_candidates` is set, only those roots are searched.
///
/// Code modified from cudarc's build.rs (via dftd3-src).
fn link_search_paths(env_candidates: &[&str], typical_locations: &[&str]) -> Vec<PathBuf> {
    let env_roots = env_candidates
        .iter()
        .filter_map(|var| std::env::var(var).ok())
        .flat_map(|path| split_paths(&path))
        .filter(|path| !path.is_empty())
        .collect::<Vec<_>>();
    let roots = if env_roots.is_empty() {
        typical_locations.iter().map(|path| path.to_string()).collect()
    } else {
        env_roots
    };

    let mut paths = vec![];
    for root in roots {
        for lib in LIB_CANDIDATES {
            let path = Path::new(&root).join(lib);
            if path.is_dir() {
                let path = std::fs::canonicalize(&path).unwrap_or(path);
                if !paths.contains(&path) {
                    paths.push(path);
                }
            }
        }
    }
    paths
}

/// Whether a directory contains a cuEST library file.
fn dir_contains_libcuest(path: &Path) -> bool {
    std::fs::read_dir(path)
        .map(|entries| {
            entries.filter_map(Result::ok).any(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                name.starts_with("libcuest.") || name == "cuest.lib"
            })
        })
        .unwrap_or(false)
}

fn main() {
    if std::env::var("CARGO_FEATURE_DYNAMIC_LOADING").is_ok() {
        return;
    }

    let env_cuest =
        ["CUEST_LIB_DIR", "REST_EXT_DIR", "LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH", "PATH"];
    let env_cuda = ["CUDA_HOME", "CUDA_PATH", "CUDA_ROOT", "CUDA_TOOLKIT_ROOT_DIR"];
    for env_var in env_cuest.iter().chain(env_cuda.iter()) {
        if let Ok(path) = std::env::var(env_var) {
            println!("cargo:rerun-if-env-changed={env_var}");
            println!("cargo:rerun-if-changed={path}");
        }
    }

    let paths_cuest =
        link_search_paths(&env_cuest, &["/usr", "/usr/local", "/usr/local/share", "/opt"]);
    let paths_cuda = link_search_paths(&env_cuda, &[
        "/usr/local/cuda",
        "/opt/cuda",
        "/usr/lib/cuda",
        "/usr",
        "/usr/local",
    ]);
    for path in paths_cuest.iter().chain(paths_cuda.iter()) {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    if !paths_cuest.iter().any(|path| dir_contains_libcuest(path)) {
        println!(
            "cargo:warning=libcuest library not found in the candidate search paths; \
             static linking will likely fail. Set CUEST_LIB_DIR (or REST_EXT_DIR) to the \
             cuEST installation prefix, or keep the default `dynamic_loading` feature."
        );
    }

    // Programs that link cuEST must also link cuBLAS and cuSolver
    // (see the cuEST installation documentation).
    println!("cargo:rustc-link-lib=cuest");
    println!("cargo:rustc-link-lib=cublas");
    println!("cargo:rustc-link-lib=cusolver");
}
