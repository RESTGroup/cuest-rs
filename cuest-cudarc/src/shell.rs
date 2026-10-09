//! Atomic orbital (AO) Gaussian shells.
//!
//! A [`CuestAOShell`] wraps a `cuestAOShell_t` handle built from a
//! [`CuestShellSpec`]. Shells are host-side objects: they are inputs to
//! [`CuestAOBasis::new`](crate::CuestAOBasis::new) and may be dropped as
//! soon as the basis has been created.

use crate::context::{ContextInner, CuestContext};
use crate::error::{status_check, CuestError};
use crate::parameters::CuestParameters;
use crate::query::{self, AoshellAttribute};
use cuest_ffi::ffi;
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

/// Specification of one contracted Gaussian shell.
///
/// `exponents` and `coefficients` are the primitive lists cuEST uses
/// *verbatim*: coefficients multiply **raw** (unnormalized) primitives, so
/// coefficients taken from a standard basis file (e.g. `.gbs`), which are
/// relative to *normalized* primitives, must be converted with
/// [`CuestShellSpec::normalized`] (cuEST's documented normalization). Both
/// lists are in atomic units.
#[derive(Debug, Clone, PartialEq)]
pub struct CuestShellSpec {
    /// Angular momentum quantum number (s = 0, p = 1, d = 2, ...).
    pub l: u64,
    /// Whether the shell uses pure (spherical) functions instead of
    /// Cartesian ones. All shells of a basis must agree.
    pub pure: bool,
    /// Primitive Gaussian exponents (all positive).
    pub exponents: Vec<f64>,
    /// Contraction coefficients, one per exponent, used verbatim.
    pub coefficients: Vec<f64>,
}

impl CuestShellSpec {
    /// Build a shell spec from primitive lists.
    ///
    /// Returns [`CuestError::Rust`] if the lists are empty or have different
    /// lengths.
    pub fn new(
        l: u64,
        pure: bool,
        exponents: Vec<f64>,
        coefficients: Vec<f64>,
    ) -> Result<Self, CuestError> {
        check_primitive_lists(exponents.len(), coefficients.len())?;
        Ok(Self { l, pure, exponents, coefficients })
    }

    /// Convert the coefficients to cuEST's convention, normalized to
    /// `normalization` (usually 1.0).
    ///
    /// Input coefficients are interpreted as relative to *normalized*
    /// primitives (the convention of standard basis files); the output folds
    /// in the primitive normalization that cuEST's verbatim convention
    /// requires (see [`normalize_coefficients`]). Calling this twice is
    /// therefore wrong — the result is already in cuEST's convention.
    pub fn normalized(&self, normalization: f64) -> Result<Self, CuestError> {
        let coefficients =
            normalize_coefficients(self.l, &self.exponents, &self.coefficients, normalization)?;
        Ok(Self { coefficients, ..self.clone() })
    }

    /// Number of primitives in the shell.
    pub fn num_primitive(&self) -> u64 {
        self.exponents.len() as u64
    }
}

/// Normalize contraction coefficients following the cuEST convention.
///
/// cuEST uses contraction coefficients verbatim against raw (unnormalized)
/// primitive Gaussians, so coefficients from standard basis files (relative
/// to normalized primitives) must first be brought to the "finished"
/// primitive-and-L normalized form documented by cuEST:
///
/// ```text
/// d   = prod_{k=1..L} (2k - 1)
/// c_k = c_k^raw * sqrt( 2^L * (2 e_k)^(L + 3/2) / (pi^(3/2) * d) )
/// V   = sum_ij c_i^raw c_j^raw * ( 2 sqrt(e_i e_j) / (e_i + e_j) )^(L + 3/2)
/// c_k^N = c_k * sqrt( normalization / V )
/// ```
///
/// This mirrors `compute_normalized_coefficients` from the cuEST C examples
/// and the formula in the cuEST documentation (`api_methods.rst`,
/// "Primitive Basis Function Normalization").
///
/// Returns [`CuestError::Rust`] for invalid input: empty or mismatched lists,
/// non-positive exponents, `L >= 9`, non-positive `normalization`, or a
/// degenerate contraction.
pub fn normalize_coefficients(
    l: u64,
    exponents: &[f64],
    coefficients: &[f64],
    normalization: f64,
) -> Result<Vec<f64>, CuestError> {
    check_primitive_lists(exponents.len(), coefficients.len())?;
    if l >= 9 {
        return Err(CuestError::Rust(format!("cuEST cannot normalize L = {l} (L < 9 required)")));
    }
    if normalization <= 0.0 || normalization.is_nan() {
        return Err(CuestError::Rust(format!("normalization {normalization} is not positive")));
    }
    for (i, &e) in exponents.iter().enumerate() {
        if e <= 0.0 || e.is_nan() {
            return Err(CuestError::Rust(format!("exponent {i} = {e} is not positive")));
        }
    }

    // d = prod_{k=1..L} (2k - 1); the empty product for L = 0 is 1.
    let d: f64 = (1..=l).map(|k| (2 * k - 1) as f64).product();
    let two_l = 2f64.powi(l as i32);
    let pi32 = std::f64::consts::PI.powf(1.5);
    let scale = (two_l / (pi32 * d)).sqrt();
    let l32 = l as f64 + 1.5;

    let mut normalized: Vec<f64> = (0..exponents.len())
        .map(|k| coefficients[k] * scale * (2.0 * exponents[k]).powf(l32).sqrt())
        .collect();

    // V = self-overlap of the raw contraction.
    let mut v = 0.0;
    for (&ei, &ci) in exponents.iter().zip(coefficients) {
        for (&ej, &cj) in exponents.iter().zip(coefficients) {
            v += ci * cj * (2.0 * (ei * ej).sqrt() / (ei + ej)).powf(l32);
        }
    }
    if v <= 0.0 || v.is_nan() {
        return Err(CuestError::Rust(
            "shell contraction has non-positive self-overlap".to_string(),
        ));
    }

    let q = (normalization / v).sqrt();
    for c in &mut normalized {
        *c *= q;
    }
    Ok(normalized)
}

/// Validate that a shell's primitive lists are non-empty and equally long.
///
/// cuEST reads `exponents.len()` coefficients from both arrays, so mismatched
/// lists would read out of bounds in the C API.
fn check_primitive_lists(exponents_len: usize, coefficients_len: usize) -> Result<(), CuestError> {
    if exponents_len == 0 || exponents_len != coefficients_len {
        return Err(CuestError::Rust(format!(
            "shell has {exponents_len} exponents but {coefficients_len} coefficients"
        )));
    }
    Ok(())
}

/// Owned state of one AO shell handle.
struct ShellInner {
    context: Arc<ContextInner>,
    handle: ffi::cuestAOShell_t,
    spec: CuestShellSpec,
    num_ao: u64,
    num_pure: u64,
    num_cart: u64,
}

impl Drop for ShellInner {
    fn drop(&mut self) {
        // Drop cannot report errors; failures only leak a host object.
        let _ = unsafe { ffi::cuestAOShellDestroy(self.handle) };
    }
}

/// A cuEST AO shell object (`cuestAOShell_t`).
///
/// See [`CuestShellSpec`] for the input data and normalization conventions.
pub struct CuestAOShell {
    inner: Arc<ShellInner>,
}

impl CuestAOShell {
    /// Create a shell from `spec` on the given context.
    ///
    /// The spec's primitive lists are validated again here: the fields of
    /// [`CuestShellSpec`] are public, and cuEST reads `exponents.len()`
    /// coefficients from both arrays.
    pub fn new(context: &CuestContext, spec: &CuestShellSpec) -> Result<Self, CuestError> {
        check_primitive_lists(spec.exponents.len(), spec.coefficients.len())?;
        let context_inner = context.inner().clone();
        let params = CuestParameters::create(ffi::CUEST_AOSHELL_PARAMETERS)?;
        let mut handle: ffi::cuestAOShell_t = ptr::null_mut();
        status_check(
            unsafe {
                ffi::cuestAOShellCreate(
                    context_inner.handle(),
                    spec.pure as i32,
                    spec.l,
                    spec.num_primitive(),
                    spec.exponents.as_ptr(),
                    spec.coefficients.as_ptr(),
                    params.raw(),
                    &mut handle,
                )
            },
            "cuestAOShellCreate",
        )?;

        match query_new_attributes(&context_inner, handle) {
            Ok((num_ao, num_pure, num_cart)) => Ok(Self {
                inner: Arc::new(ShellInner {
                    context: context_inner,
                    handle,
                    spec: spec.clone(),
                    num_ao,
                    num_pure,
                    num_cart,
                }),
            }),
            Err(err) => {
                let _ = unsafe { ffi::cuestAOShellDestroy(handle) };
                Err(err)
            },
        }
    }

    /// The specification this shell was created from.
    pub fn spec(&self) -> &CuestShellSpec {
        &self.inner.spec
    }

    /// Angular momentum quantum number (L).
    pub fn l(&self) -> u64 {
        self.inner.spec.l
    }

    /// Whether the shell uses pure (spherical) functions.
    pub fn is_pure(&self) -> bool {
        self.inner.spec.pure
    }

    /// Number of primitive Gaussians.
    pub fn num_primitive(&self) -> u64 {
        self.inner.spec.num_primitive()
    }

    /// Number of basis functions of this shell (pure or Cartesian as set).
    pub fn num_ao(&self) -> u64 {
        self.inner.num_ao
    }

    /// Number of pure (spherical) AOs.
    pub fn num_pure(&self) -> u64 {
        self.inner.num_pure
    }

    /// Number of Cartesian AOs.
    pub fn num_cart(&self) -> u64 {
        self.inner.num_cart
    }

    /// Query an attribute through the typed attribute mechanism.
    pub fn query<A: AoshellAttribute>(&self) -> Result<A::Value, CuestError> {
        query::query_attribute(
            self.inner.context.handle(),
            ffi::CUEST_AOSHELL,
            self.inner.handle as *const c_void,
            A::ID,
        )
    }

    /// The raw cuEST shell handle (escape hatch).
    pub fn raw_handle(&self) -> ffi::cuestAOShell_t {
        self.inner.handle
    }

    pub(crate) fn context(&self) -> &Arc<ContextInner> {
        &self.inner.context
    }
}

impl Clone for CuestAOShell {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl std::fmt::Debug for CuestAOShell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CuestAOShell")
            .field("l", &self.l())
            .field("pure", &self.is_pure())
            .field("num_primitive", &self.num_primitive())
            .field("num_ao", &self.num_ao())
            .finish_non_exhaustive()
    }
}

// SAFETY: see the "Thread safety" section of the crate documentation.
unsafe impl Send for CuestAOShell {}

fn query_new_attributes(
    context: &ContextInner,
    handle: ffi::cuestAOShell_t,
) -> Result<(u64, u64, u64), CuestError> {
    let query = |id: std::ffi::c_int| {
        query::query_attribute::<u64>(
            context.handle(),
            ffi::CUEST_AOSHELL,
            handle as *const c_void,
            id,
        )
    };
    Ok((
        query(ffi::CUEST_AOSHELL_NUM_AO.0 as std::ffi::c_int)?,
        query(ffi::CUEST_AOSHELL_NUM_PURE.0 as std::ffi::c_int)?,
        query(ffi::CUEST_AOSHELL_NUM_CART.0 as std::ffi::c_int)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Norm of a normalized primitive: `|N_k g_k| = 1` where `g_k` is the raw
    /// primitive `x^l e^{-e r^2}`.
    fn primitive_norm(l: u64, e: f64) -> f64 {
        let d: f64 = (1..=l).map(|k| (2 * k - 1) as f64).product();
        (2f64.powi(l as i32) * (2.0 * e).powf(l as f64 + 1.5)
            / (std::f64::consts::PI.powf(1.5) * d))
            .sqrt()
    }

    /// Overlap of two normalized primitives.
    fn primitive_overlap(l: u64, ei: f64, ej: f64) -> f64 {
        (2.0 * (ei * ej).sqrt() / (ei + ej)).powf(l as f64 + 1.5)
    }

    /// Self-overlap of `sum_k c_k phi_k` where `phi_k` are *normalized*
    /// primitives (the convention of standard basis files).
    fn self_overlap_normalized_primitives(l: u64, exponents: &[f64], coefficients: &[f64]) -> f64 {
        let mut v = 0.0;
        for (&ei, &ci) in exponents.iter().zip(coefficients) {
            for (&ej, &cj) in exponents.iter().zip(coefficients) {
                v += ci * cj * primitive_overlap(l, ei, ej);
            }
        }
        v
    }

    /// Self-overlap of `sum_k c_k g_k` where `g_k` are *raw* primitives
    /// (cuEST's convention: coefficients are used verbatim). The overlap of
    /// two raw primitives is that of the normalized ones, divided by
    /// `N_i N_j`.
    fn self_overlap_raw_primitives(l: u64, exponents: &[f64], coefficients: &[f64]) -> f64 {
        let mut v = 0.0;
        for (&ei, &ci) in exponents.iter().zip(coefficients) {
            for (&ej, &cj) in exponents.iter().zip(coefficients) {
                v += ci * cj * primitive_overlap(l, ei, ej)
                    / (primitive_norm(l, ei) * primitive_norm(l, ej));
            }
        }
        v
    }

    #[test]
    fn test_normalize_coefficients() {
        // H 1s STO-3G coefficients as published: relative to *normalized*
        // primitives, with the contraction ~unit-normalized (to the basis
        // file's precision).
        let exponents = [3.42525091, 0.62391373, 0.16885540];
        let published = [0.15432897, 0.53532814, 0.44463454];
        let nearly_one = self_overlap_normalized_primitives(0, &exponents, &published);
        assert!(
            (nearly_one - 1.0).abs() < 1e-4,
            "published coefficients are ~normalized: {nearly_one}"
        );

        // Conversion yields cuEST-convention coefficients whose contraction
        // (coefficients against *raw* primitives) has the requested norm.
        let normalized = normalize_coefficients(0, &exponents, &published, 1.0).unwrap();
        assert!((self_overlap_raw_primitives(0, &exponents, &normalized) - 1.0).abs() < 1e-12);

        // A p shell, with a non-unit normalization factor.
        let normalized = normalize_coefficients(1, &exponents, &published, 2.5).unwrap();
        assert!((self_overlap_raw_primitives(1, &exponents, &normalized) - 2.5).abs() < 1e-12);

        // Scaling the input coefficients only rescales the output.
        let scaled: Vec<f64> = published.iter().map(|c| c * 3.0).collect();
        let normalized_scaled = normalize_coefficients(1, &exponents, &scaled, 1.0).unwrap();
        assert!(
            (self_overlap_raw_primitives(1, &exponents, &normalized_scaled) - 1.0).abs() < 1e-12
        );
    }

    #[test]
    fn test_normalize_coefficients_errors() {
        let exps = [1.0];
        assert!(normalize_coefficients(0, &[], &[], 1.0).is_err());
        assert!(normalize_coefficients(0, &exps, &[1.0, 2.0], 1.0).is_err());
        assert!(normalize_coefficients(9, &exps, &[1.0], 1.0).is_err());
        assert!(normalize_coefficients(0, &exps, &[1.0], 0.0).is_err());
        assert!(normalize_coefficients(0, &[-1.0], &[1.0], 1.0).is_err());
    }

    #[test]
    fn test_check_primitive_lists() {
        assert!(check_primitive_lists(3, 3).is_ok());
        assert!(check_primitive_lists(0, 0).is_err());
        assert!(check_primitive_lists(2, 3).is_err());
    }

    #[test]
    fn test_shell_spec() {
        let spec = CuestShellSpec::new(0, true, vec![1.0, 2.0], vec![0.5, 0.5]).unwrap();
        assert_eq!(spec.num_primitive(), 2);
        let normalized = spec.normalized(1.0).unwrap();
        assert_eq!(normalized.l, 0);
        assert_eq!(normalized.coefficients.len(), 2);
        assert_ne!(normalized.coefficients, spec.coefficients);
        assert!(CuestShellSpec::new(0, true, vec![], vec![]).is_err());
        assert!(CuestShellSpec::new(0, true, vec![1.0], vec![1.0, 2.0]).is_err());
    }
}
