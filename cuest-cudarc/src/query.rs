//! Typed queries of cuEST object attributes (`cuestQuery`).
//!
//! cuEST attributes are queried through one type-erased C function; the
//! attribute determines the value type. This module encodes that mapping in
//! the type system: each attribute is a marker type implementing
//! [`CuestAttribute`] (which carries the value type and the C id), plus an
//! object-scoped trait ([`AoshellAttribute`], [`AobasisAttribute`], ...) so
//! that only valid attribute/object combinations compile.
//!
//! The v1 table covers the AO shell and AO basis attributes; more markers are
//! pure transcription work. Unknown attributes can be reached through the raw
//! bindings ([`cuest_ffi`]).

use crate::error::{status_check, CuestError};
use cuest_ffi::ffi;
use std::ffi::{c_int, c_void};

mod sealed {
    pub trait Sealed {}
    impl Sealed for u64 {}
    impl Sealed for i32 {}
    impl Sealed for f64 {}
}

/// Value types an attribute can be read into.
///
/// Sealed: the set of types cuEST returns for scalars is closed (`uint64_t`,
/// `int32_t`, `double`).
pub trait AttributeValue: Copy + sealed::Sealed {
    /// Size in bytes cuEST expects for this type.
    const SIZE: usize;
    /// Interpret the value written by cuEST into `bytes`.
    fn from_bytes(bytes: &[u8]) -> Self;
}

fn read_ne<const N: usize>(bytes: &[u8]) -> [u8; N] {
    let mut buf = [0u8; N];
    buf.copy_from_slice(&bytes[..N]);
    buf
}

impl AttributeValue for u64 {
    const SIZE: usize = size_of::<u64>();
    fn from_bytes(bytes: &[u8]) -> Self {
        u64::from_ne_bytes(read_ne(bytes))
    }
}

impl AttributeValue for i32 {
    const SIZE: usize = size_of::<i32>();
    fn from_bytes(bytes: &[u8]) -> Self {
        i32::from_ne_bytes(read_ne(bytes))
    }
}

impl AttributeValue for f64 {
    const SIZE: usize = size_of::<f64>();
    fn from_bytes(bytes: &[u8]) -> Self {
        f64::from_ne_bytes(read_ne(bytes))
    }
}

/// A queryable attribute: its value type and its C attribute id.
pub trait CuestAttribute {
    /// The type cuEST returns for this attribute.
    type Value: AttributeValue;
    /// The `int attribute` id passed to `cuestQuery`.
    const ID: c_int;
}

/// Attribute of a `cuestAOShell_t` object.
pub trait AoshellAttribute: CuestAttribute {}

/// Attribute of a `cuestAOBasis_t` object.
pub trait AobasisAttribute: CuestAttribute {}

macro_rules! attribute {
    ($(#[$doc:meta])* $name:ident: $trait:ident, $value:ty, $id:expr) => {
        $(#[$doc])*
        pub struct $name;
        impl CuestAttribute for $name {
            type Value = $value;
            const ID: c_int = $id;
        }
        impl $trait for $name {}
    };
}

attribute!(
    /// Whether the shell uses pure (spherical) functions (nonzero if true).
    AoshellIsPure: AoshellAttribute, i32, ffi::CUEST_AOSHELL_IS_PURE.0 as c_int
);
attribute!(
    /// Angular momentum quantum number (L).
    AoshellL: AoshellAttribute, u64, ffi::CUEST_AOSHELL_L.0 as c_int
);
attribute!(
    /// Number of primitive Gaussians.
    AoshellNumPrimitive: AoshellAttribute, u64, ffi::CUEST_AOSHELL_NUM_PRIMITIVE.0 as c_int
);
attribute!(
    /// Number of basis functions (AOs, either pure or Cartesian as set).
    AoshellNumAo: AoshellAttribute, u64, ffi::CUEST_AOSHELL_NUM_AO.0 as c_int
);
attribute!(
    /// Number of pure (spherical) AOs.
    AoshellNumPure: AoshellAttribute, u64, ffi::CUEST_AOSHELL_NUM_PURE.0 as c_int
);
attribute!(
    /// Number of Cartesian AOs.
    AoshellNumCart: AoshellAttribute, u64, ffi::CUEST_AOSHELL_NUM_CART.0 as c_int
);

attribute!(
    /// Nonzero if all shells are pure (spherical), zero otherwise.
    AobasisIsPure: AobasisAttribute, i32, ffi::CUEST_AOBASIS_IS_PURE.0 as c_int
);
attribute!(
    /// Nonzero if all shells are Cartesian, zero otherwise.
    AobasisIsCart: AobasisAttribute, i32, ffi::CUEST_AOBASIS_IS_CART.0 as c_int
);
attribute!(
    /// Nonzero if shells are mixed spherical/Cartesian, zero otherwise.
    AobasisIsMixed: AobasisAttribute, i32, ffi::CUEST_AOBASIS_IS_MIXED.0 as c_int
);
attribute!(
    /// Number of basis function centers.
    AobasisNumAtom: AobasisAttribute, u64, ffi::CUEST_AOBASIS_NUM_ATOM.0 as c_int
);
attribute!(
    /// Total number of AO shells.
    AobasisNumShell: AobasisAttribute, u64, ffi::CUEST_AOBASIS_NUM_SHELL.0 as c_int
);
attribute!(
    /// Total number of basis functions (AOs).
    AobasisNumAo: AobasisAttribute, u64, ffi::CUEST_AOBASIS_NUM_AO.0 as c_int
);
attribute!(
    /// Total number of Cartesian basis functions.
    AobasisNumCart: AobasisAttribute, u64, ffi::CUEST_AOBASIS_NUM_CART.0 as c_int
);
attribute!(
    /// Total number of Gaussian primitives.
    AobasisNumPrimitive: AobasisAttribute, u64, ffi::CUEST_AOBASIS_NUM_PRIMITIVE.0 as c_int
);
attribute!(
    /// Maximum angular momentum (L) among shells.
    AobasisMaxL: AobasisAttribute, u64, ffi::CUEST_AOBASIS_MAX_L.0 as c_int
);

/// Run a `cuestQuery` for a scalar attribute of an object.
pub(crate) fn query_attribute<V: AttributeValue>(
    handle: ffi::cuestHandle_t,
    object_type: ffi::cuestType_t,
    object: *const c_void,
    attribute: c_int,
) -> Result<V, CuestError> {
    let mut buf = [0u8; size_of::<u64>()];
    status_check(
        unsafe {
            ffi::cuestQuery(
                handle,
                object_type,
                object,
                attribute,
                buf.as_mut_ptr() as *mut c_void,
                V::SIZE,
            )
        },
        "cuestQuery",
    )?;
    Ok(V::from_bytes(&buf))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_attribute_ids_match_c() {
        assert_eq!(<AoshellIsPure as CuestAttribute>::ID, 0);
        assert_eq!(<AoshellNumCart as CuestAttribute>::ID, 5);
        assert_eq!(<AobasisIsPure as CuestAttribute>::ID, 0);
        assert_eq!(<AobasisMaxL as CuestAttribute>::ID, 8);
    }

    #[test]
    fn cpu_value_sizes() {
        assert_eq!(<u64 as AttributeValue>::SIZE, 8);
        assert_eq!(<i32 as AttributeValue>::SIZE, 4);
        assert_eq!(<f64 as AttributeValue>::SIZE, 8);
    }

    #[test]
    fn cpu_from_bytes() {
        assert_eq!(u64::from_bytes(&7u64.to_ne_bytes()), 7);
        assert_eq!(i32::from_bytes(&(-3i32).to_ne_bytes()), -3);
        assert_eq!(f64::from_bytes(&1.5f64.to_ne_bytes()), 1.5);
        // Zero-padded tail (the query buffer is wider than the value).
        assert_eq!(i32::from_bytes(&[2, 0, 0, 0, 9, 9, 9, 9]), 2);
    }
}
