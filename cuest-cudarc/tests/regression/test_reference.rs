//! End-to-end tests of the cuEST wrapper against pyscf reference matrices.
//!
//! For every case in `reference.toml` the full pipeline runs: context ->
//! shells (with normalization) -> basis -> pair list -> one-electron integral
//! plan -> S/T/V computation. The computed matrices are compared against the
//! pyscf values element-wise.

use cuest_cudarc::cudarc::driver::{CudaContext, CudaSlice, CudaStream};
use cuest_cudarc::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use toml::Value;

/// Absolute and relative tolerance (mirrors numpy `allclose`).
///
/// cuEST's S/T/V matrices agree with pyscf to |diff| <= 8e-9 for H2/STO-3G and
/// <= 4e-8 for H2O/def2-SVP (the kernels are not bitwise FP64, even in
/// `NativeFp64` math mode). The tolerance is set just above that noise but
/// far below any real convention/ordering error, which would be O(0.1).
const ATOL: f64 = 1.0e-7;
const RTOL: f64 = 1.0e-8;

fn reference_document() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/regression/reference.toml");
    let text = std::fs::read_to_string(&path).expect("read reference.toml");
    toml::from_str(&text).expect("parse reference.toml")
}

fn float_list(value: &Value) -> Vec<f64> {
    value.as_array().expect("array").iter().map(|v| v.as_float().expect("float")).collect()
}

fn integer(value: &Value) -> i64 {
    value.as_integer().expect("integer")
}

fn string_list(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(|v| v.as_str().expect("string").to_string())
        .collect()
}

fn jit_cache_dir() -> PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/cuest-jit-cache");
    std::fs::create_dir_all(&dir).expect("create JIT cache dir");
    dir
}

/// Element-wise `allclose` with a diagnostic on the worst element.
fn allclose(got: &[f64], expected: &[f64], nao: usize, label: &str) {
    assert_eq!(got.len(), expected.len(), "{label}: length mismatch");
    let (mut worst_index, mut worst_diff) = (0, 0.0);
    for (index, (g, e)) in got.iter().zip(expected).enumerate() {
        let diff = (g - e).abs();
        if diff > worst_diff {
            (worst_index, worst_diff) = (index, diff);
        }
    }
    let tolerance = ATOL + RTOL * expected[worst_index].abs();
    assert!(
        worst_diff <= tolerance,
        "{label}: worst element [{}, {}] (index {worst_index}): got {}, expected {} \
         (|diff| = {worst_diff} > {tolerance})\nfirst row got:      {:?}\nfirst row expected: {:?}",
        worst_index / nao,
        worst_index % nao,
        got[worst_index],
        expected[worst_index],
        &got[..nao],
        &expected[..nao],
    );
}

fn to_host(stream: &Arc<CudaStream>, slice: &CudaSlice<f64>) -> Vec<f64> {
    let mut host = vec![0.0; slice.len()];
    stream.memcpy_dtoh(slice, &mut host).expect("device-to-host copy");
    stream.synchronize().expect("stream synchronize");
    host
}

/// Dump a computed matrix as plain text (debugging aid, gated by
/// `CUEST_TEST_DUMP_DIR` at the call site; the matrix must be square).
fn dump_matrix(dir: &Path, name: &str, matrix: &[f64]) {
    std::fs::create_dir_all(dir).expect("create dump dir");
    let nao = (matrix.len() as f64).sqrt().round() as usize;
    assert_eq!(nao * nao, matrix.len());
    let text: String = matrix
        .chunks(nao)
        .map(|row| row.iter().map(|v| format!("{v:.17e}")).collect::<Vec<_>>().join(" ") + "\n")
        .collect();
    std::fs::write(dir.join(format!("{name}.txt")), text).expect("write dump");
}

/// Run the full pipeline for one case and return the host S/T/V matrices.
fn run_case(case: &Value) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let name = case["name"].as_str().expect("case name");
    let pure = case["pure"].as_bool().expect("pure flag");
    let threshold_pq = case["threshold_pq"].as_float().expect("threshold_pq");
    let symbols = string_list(&case["symbols"]);
    let xyz_angstrom = float_list(&case["xyz_angstrom"]);
    let natm = symbols.len();

    // Rebuild XYZ text so the geometry goes through the public parser.
    let mut xyz_text = format!("{natm}\n{name}\n");
    for (i, symbol) in symbols.iter().enumerate() {
        xyz_text.push_str(&format!(
            "{symbol} {} {} {}\n",
            xyz_angstrom[3 * i],
            xyz_angstrom[3 * i + 1],
            xyz_angstrom[3 * i + 2]
        ));
    }
    let xyz = Xyz::parse(&xyz_text).expect("parse geometry");

    let stream = CudaContext::new(0).expect("CUDA device 0").default_stream();
    let options = CuestHandleOptions { jit_cache_dir: Some(jit_cache_dir()), ..Default::default() };
    let context = CuestContext::with_options(&stream, &options).expect("create cuEST context");
    // FP64 everywhere, so the reference comparison is not limited by
    // reduced-precision (ffloat) compute.
    context.set_math_mode(CuestMathMode::NativeFp64).expect("set math mode");

    let mut shells_per_atom: Vec<Vec<CuestAOShell>> = (0..natm).map(|_| Vec::new()).collect();
    for shell in case["shells"].as_array().expect("shells") {
        let atom = integer(&shell["atom"]) as usize;
        let l = integer(&shell["l"]) as u64;
        let exponents = float_list(&shell["exponents"]);
        let coefficients = float_list(&shell["coefficients"]);
        let spec = CuestShellSpec::new(l, pure, exponents, coefficients)
            .expect("shell spec")
            .normalized(1.0)
            .expect("normalize coefficients");
        shells_per_atom[atom].push(CuestAOShell::new(&context, &spec).expect("create shell"));
    }
    let shells_ref: Vec<&[CuestAOShell]> =
        shells_per_atom.iter().map(|shells| shells.as_slice()).collect();
    let basis = CuestAOBasis::new(&context, &shells_ref).expect("create basis");
    assert_eq!(basis.num_ao(), integer(&case["nao"]) as u64, "{name}: nao");

    let pair_list = CuestAOPairList::from_xyz(&basis, &xyz, threshold_pq).expect("pair list");
    let plan = CuestOeIntPlan::new(&basis, &pair_list).expect("oeint plan");

    let s = plan.overlap().expect("overlap");
    let t = plan.kinetic().expect("kinetic");
    let charges = stream.clone_htod(&xyz.nuclear_attraction_charges()).expect("htod charges");
    let coords = stream.clone_htod(&xyz.coords_bohr_flat()).expect("htod coords");
    let v = plan.potential(&coords, &charges).expect("potential");

    let (s, t, v) = (to_host(&stream, &s), to_host(&stream, &t), to_host(&stream, &v));
    if let Some(dir) = std::env::var_os("CUEST_TEST_DUMP_DIR").map(PathBuf::from) {
        dump_matrix(&dir, &format!("{name}.s"), &s);
        dump_matrix(&dir, &format!("{name}.t"), &t);
        dump_matrix(&dir, &format!("{name}.v"), &v);
    }
    (s, t, v)
}

#[test]
#[ignore = "requires a GPU and a local cuEST installation"]
fn test_reference_matrices() {
    let document = reference_document();
    let cases = document["cases"].as_array().expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let nao = integer(&case["nao"]) as usize;
        let (s, t, v) = run_case(case);
        allclose(&s, &float_list(&case["s"]), nao, &format!("{name}: S"));
        allclose(&t, &float_list(&case["t"]), nao, &format!("{name}: T"));
        allclose(&v, &float_list(&case["v"]), nao, &format!("{name}: V"));
    }
}
