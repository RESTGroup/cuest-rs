//! GPU tests of the wrapper mechanics that pure-Rust unit tests cannot
//! reach: `*_into` variants, buffer/stream validation, typed queries, handle
//! settings, and cross-object consistency checks.
//!
//! Requires a GPU and a local cuEST installation; run with:
//!
//! ```bash
//! CUEST_DYLOAD=/path/to/libcuest.so LD_LIBRARY_PATH=/usr/local/cuda/lib64 \
//!     cargo test -p cuest-cudarc --test test_gpu -- --ignored
//! ```

use cuest_cudarc::cudarc::driver::{CudaContext, CudaSlice, CudaStream};
use cuest_cudarc::prelude::*;
use cuest_cudarc::query;
use std::path::Path;
use std::sync::Arc;

/// H 1s STO-3G, as published (coefficients relative to normalized
/// primitives).
const H_EXPONENTS: [f64; 3] = [3.42525091, 0.62391373, 0.16885540];
const H_COEFFICIENTS: [f64; 3] = [0.15432897, 0.53532814, 0.44463454];

const H2_XYZ: &str = "2
H2
H 0.0 0.0 0.0
H 0.0 0.0 0.7408
";

fn context_and_stream() -> (CuestContext, Arc<CudaStream>) {
    let stream = CudaContext::new(0).expect("CUDA device 0").default_stream();
    let jit_cache_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/cuest-jit-cache");
    std::fs::create_dir_all(&jit_cache_dir).expect("create JIT cache dir");
    let options = CuestHandleOptions { jit_cache_dir: Some(jit_cache_dir), ..Default::default() };
    let context = CuestContext::with_options(&stream, &options).expect("create cuEST context");
    (context, stream)
}

fn h_shell(context: &CuestContext) -> CuestAOShell {
    let spec = CuestShellSpec::new(0, true, H_EXPONENTS.to_vec(), H_COEFFICIENTS.to_vec())
        .expect("shell spec")
        .normalized(1.0)
        .expect("normalize");
    CuestAOShell::new(context, &spec).expect("create shell")
}

fn to_host(stream: &Arc<CudaStream>, slice: &CudaSlice<f64>) -> Vec<f64> {
    let mut host = vec![0.0; slice.len()];
    stream.memcpy_dtoh(slice, &mut host).expect("device-to-host copy");
    stream.synchronize().expect("stream synchronize");
    host
}

#[test]
#[ignore = "requires a GPU and a local cuEST installation"]
fn gpu_handle_settings_and_queries() {
    let (context, _stream) = context_and_stream();

    // Runtime version matches the bindings (context creation would have
    // failed with `UnsupportedVersion` otherwise).
    let needed = (
        cuest_cudarc::cuest_ffi::ffi::CUEST_VER_MAJOR,
        cuest_cudarc::cuest_ffi::ffi::CUEST_VER_MINOR,
        cuest_cudarc::cuest_ffi::ffi::CUEST_VER_PATCH,
    );
    assert_eq!(context.version(), needed);

    // Math mode round trip.
    assert_eq!(context.math_mode().expect("get math mode"), CuestMathMode::Default);
    context.set_math_mode(CuestMathMode::NativeFp64).expect("set math mode");
    assert_eq!(context.math_mode().expect("get math mode"), CuestMathMode::NativeFp64);
    context.set_math_mode(CuestMathMode::Default).expect("set math mode");

    // Compute capability target round trip.
    let (major, minor) = context.compute_capability_target().expect("get compute capability");
    assert!(major > 0, "unexpected compute capability {major}.{minor}");
    context.set_compute_capability_target(major, minor).expect("set compute capability");
    assert_eq!(
        context.compute_capability_target().expect("get compute capability"),
        (major, minor)
    );

    // Typed queries agree with the cached accessors.
    let shell0 = h_shell(&context);
    let shell1 = h_shell(&context);
    assert_eq!(shell0.query::<query::AoshellL>().expect("query L"), 0);
    assert_eq!(shell0.query::<query::AoshellNumPrimitive>().expect("query nprim"), 3);
    assert_eq!(shell0.query::<query::AoshellNumAo>().expect("query nao"), 1);
    assert_eq!(shell0.query::<query::AoshellNumPure>().expect("query npure"), 1);
    assert_eq!(shell0.query::<query::AoshellNumCart>().expect("query ncart"), 1);
    assert_ne!(shell0.query::<query::AoshellIsPure>().expect("query is_pure"), 0);
    assert_eq!(shell0.num_ao(), 1);
    assert!(shell0.is_pure());
    assert_eq!(shell0.num_primitive(), 3);

    let s0 = [shell0];
    let s1 = [shell1];
    let basis = CuestAOBasis::new(&context, &[&s0[..], &s1[..]]).expect("create basis");
    assert_eq!(basis.num_atom(), 2);
    assert_eq!(basis.num_shell(), 2);
    assert_eq!(basis.num_ao(), 2);
    assert_eq!(basis.max_l(), 0);
    assert!(basis.is_pure());
    assert!(!basis.is_mixed());
    assert_eq!(basis.query::<query::AobasisNumAo>().expect("query nao"), basis.num_ao());
    assert_eq!(basis.query::<query::AobasisNumAtom>().expect("query natom"), 2);
    assert_eq!(
        basis.query::<query::AobasisNumPrimitive>().expect("query nprim"),
        basis.num_primitive()
    );
    assert_ne!(basis.query::<query::AobasisIsPure>().expect("query is_pure"), 0);
    assert_eq!(basis.query::<query::AobasisIsMixed>().expect("query is_mixed"), 0);

    // Escape hatch: the raw handles are usable handles.
    assert!(!context.raw_handle().is_null());
    assert!(!basis.raw_handle().is_null());
}

#[test]
#[ignore = "requires a GPU and a local cuEST installation"]
fn gpu_validation_and_into_variants() {
    let (context, stream) = context_and_stream();
    let xyz = Xyz::parse(H2_XYZ).expect("parse xyz");
    let shell0 = h_shell(&context);
    let shell1 = h_shell(&context);
    let s0 = [shell0];
    let s1 = [shell1];
    let basis = CuestAOBasis::new(&context, &[&s0[..], &s1[..]]).expect("create basis");
    let pair_list = CuestAOPairList::from_xyz(&basis, &xyz, 1.0e-14).expect("pair list");
    let plan = CuestOeIntPlan::new(&basis, &pair_list).expect("plan");
    let nao = plan.nao() as usize;
    assert_eq!(nao, 2);

    // `*_into` writes the same matrix as the auto-allocating variant.
    let mut out = stream.alloc_zeros::<f64>(nao * nao).expect("alloc");
    plan.overlap_into(&mut out).expect("overlap_into");
    let reference = plan.overlap().expect("overlap");
    let (got, expected) = (to_host(&stream, &out), to_host(&stream, &reference));
    for (g, e) in got.iter().zip(&expected) {
        assert!((g - e).abs() < 1.0e-14, "{g} vs {e}");
    }

    // Too-small output buffer is rejected.
    let mut small = stream.alloc_zeros::<f64>(nao * nao - 1).expect("alloc");
    let err = plan.overlap_into(&mut small).unwrap_err();
    assert!(matches!(err, CuestError::Rust(ref msg) if msg.contains("expected at least")), "{err}");

    // Buffers on another stream are rejected.
    let other_stream = stream.context().new_stream().expect("create stream");
    let mut other = other_stream.alloc_zeros::<f64>(nao * nao).expect("alloc");
    let err = plan.overlap_into(&mut other).unwrap_err();
    assert!(
        matches!(err, CuestError::Rust(ref msg) if msg.contains("different CUDA stream")),
        "{err}"
    );

    // Potential with explicit options, plus argument-size validation.
    let coords = stream.clone_htod(&xyz.coords_bohr_flat()).expect("htod coords");
    let charges = stream.clone_htod(&xyz.nuclear_attraction_charges()).expect("htod charges");
    let options = CuestPotentialComputeOptions {
        ffloat_usage_mode: Some(CuestFfloatUsageMode::Off),
        jit_usage_mode: Some(CuestJitUsageMode::Off),
    };
    let mut v = stream.alloc_zeros::<f64>(nao * nao).expect("alloc");
    plan.potential_with_options_into(&options, &mut v, &coords, &charges).expect("potential");
    let v = to_host(&stream, &v);
    for value in &v {
        assert!(value.is_finite(), "non-finite potential matrix entry: {value}");
    }

    let short_charges = stream.clone_htod(&[1.0f64]).expect("htod");
    let err = plan.potential(&coords, &short_charges).unwrap_err();
    assert!(matches!(err, CuestError::Rust(ref msg) if msg.contains("charge positions")), "{err}");

    // A pair list built from a different basis is rejected by the plan.
    let basis2 = CuestAOBasis::new(&context, &[&s0[..], &s1[..]]).expect("create basis");
    let pair_list2 = CuestAOPairList::from_xyz(&basis2, &xyz, 1.0e-14).expect("pair list");
    let err = CuestOeIntPlan::new(&basis, &pair_list2).unwrap_err();
    assert!(matches!(err, CuestError::Rust(ref msg) if msg.contains("different basis")), "{err}");

    // Geometry size mismatches are rejected before the FFI call.
    let one_atom = Xyz::parse("1\nH\nH 0 0 0\n").expect("parse");
    let err = CuestAOPairList::from_xyz(&basis, &one_atom, 1.0e-14).unwrap_err();
    assert!(matches!(err, CuestError::Rust(_)), "{err}");

    // Mixed pure/Cartesian shells are rejected.
    let cart_spec = CuestShellSpec::new(0, false, H_EXPONENTS.to_vec(), H_COEFFICIENTS.to_vec())
        .expect("spec")
        .normalized(1.0)
        .expect("normalize");
    let cart_shell = CuestAOShell::new(&context, &cart_spec).expect("cartesian shell");
    let pure_shell = h_shell(&context);
    let c0 = [cart_shell];
    let p1 = [pure_shell];
    let err = CuestAOBasis::new(&context, &[&c0[..], &p1[..]]).unwrap_err();
    assert!(matches!(err, CuestError::Rust(ref msg) if msg.contains("pure")), "{err}");

    // A shell spec with mismatched primitive lists is rejected (the fields
    // are public; cuEST would read out of bounds otherwise).
    let bad_spec =
        CuestShellSpec { l: 0, pure: true, exponents: vec![1.0, 2.0], coefficients: vec![1.0] };
    let err = CuestAOShell::new(&context, &bad_spec).unwrap_err();
    assert!(matches!(err, CuestError::Rust(ref msg) if msg.contains("coefficients")), "{err}");
}
