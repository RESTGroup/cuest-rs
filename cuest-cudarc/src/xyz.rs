//! Minimal XYZ geometry parsing and unit helpers.
//!
//! cuEST works in atomic units (Bohr); coordinates in XYZ files are in
//! Angstrom. [`Xyz`] keeps the parsed (Angstrom) coordinates and converts on
//! demand.

use crate::error::CuestError;
use std::path::Path;

/// Angstrom -> Bohr conversion factor.
///
/// The cuEST examples (and PSI4, cf. `unit_conversions.py` in the cuEST
/// Python SCF examples) use `a0 = 0.52917720859 Angstrom`.
pub const ANGSTROM_TO_BOHR: f64 = 1.0 / 0.52917720859;

/// Element symbols indexed by atomic number; index 0 is the `X` dummy atom
/// used for ghost centers in some XYZ files.
const ELEMENT_SYMBOLS: [&str; 119] = [
    "X", "H", "HE", "LI", "BE", "B", "C", "N", "O", "F", "NE", "NA", "MG", "AL", "SI", "P", "S",
    "CL", "AR", "K", "CA", "SC", "TI", "V", "CR", "MN", "FE", "CO", "NI", "CU", "ZN", "GA", "GE",
    "AS", "SE", "BR", "KR", "RB", "SR", "Y", "ZR", "NB", "MO", "TC", "RU", "RH", "PD", "AG", "CD",
    "IN", "SN", "SB", "TE", "I", "XE", "CS", "BA", "LA", "CE", "PR", "ND", "PM", "SM", "EU", "GD",
    "TB", "DY", "HO", "ER", "TM", "YB", "LU", "HF", "TA", "W", "RE", "OS", "IR", "PT", "AU", "HG",
    "TL", "PB", "BI", "PO", "AT", "RN", "FR", "RA", "AC", "TH", "PA", "U", "NP", "PU", "AM", "CM",
    "BK", "CF", "ES", "FM", "MD", "NO", "LR", "RF", "DB", "SG", "BH", "HS", "MT", "DS", "RG", "CN",
    "NH", "FL", "MC", "LV", "TS", "OG",
];

/// Atomic number of an element symbol (case-insensitive).
pub fn atomic_number(symbol: &str) -> Result<u32, CuestError> {
    let symbol = symbol.to_uppercase();
    ELEMENT_SYMBOLS
        .iter()
        .position(|&element| element == symbol)
        .map(|index| index as u32)
        .ok_or_else(|| CuestError::Rust(format!("unknown element symbol `{symbol}`")))
}

/// A parsed XYZ geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct Xyz {
    symbols: Vec<String>,
    atomic_numbers: Vec<u32>,
    coords_angstrom: Vec<[f64; 3]>,
}

impl Xyz {
    /// Parse XYZ-formatted text (atom count line, comment line, then one
    /// `symbol x y z` line per atom with the coordinates in Angstrom).
    ///
    /// Extra columns after the coordinates are ignored.
    pub fn parse(text: &str) -> Result<Self, CuestError> {
        let mut lines = text.lines();
        let count_line =
            lines.next().ok_or_else(|| CuestError::Rust("XYZ input is empty".to_string()))?;
        let num_atoms: usize = count_line.trim().parse().map_err(|_| {
            CuestError::Rust(format!(
                "first XYZ line is not an atom count: `{}`",
                count_line.trim()
            ))
        })?;
        if num_atoms == 0 {
            return Err(CuestError::Rust("XYZ input contains no atoms".to_string()));
        }
        lines
            .next()
            .ok_or_else(|| CuestError::Rust("XYZ input has no comment line".to_string()))?;

        let mut symbols = Vec::with_capacity(num_atoms);
        let mut atomic_numbers = Vec::with_capacity(num_atoms);
        let mut coords_angstrom = Vec::with_capacity(num_atoms);
        for (index, line) in lines.enumerate() {
            if symbols.len() == num_atoms {
                break;
            }
            let mut fields = line.split_whitespace();
            let parse_error = |what: &str| {
                CuestError::Rust(format!(
                    "XYZ atom line {}: cannot parse {what}: `{line}`",
                    index + 3
                ))
            };
            let symbol = fields.next().ok_or_else(|| parse_error("element symbol"))?;
            let mut coord = [0.0; 3];
            for value in &mut coord {
                *value = fields
                    .next()
                    .and_then(|f| f.parse().ok())
                    .ok_or_else(|| parse_error("coordinate"))?;
            }
            symbols.push(symbol.to_uppercase());
            atomic_numbers.push(atomic_number(symbol)?);
            coords_angstrom.push(coord);
        }
        if symbols.len() != num_atoms {
            return Err(CuestError::Rust(format!(
                "XYZ declares {num_atoms} atoms but only {} atom lines follow",
                symbols.len()
            )));
        }
        Ok(Self { symbols, atomic_numbers, coords_angstrom })
    }

    /// Parse an XYZ file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, CuestError> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path).map_err(|err| {
            CuestError::Rust(format!("cannot read XYZ file `{}`: {err}", path.display()))
        })?;
        Self::parse(&text)
    }

    /// Number of atoms.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Whether the geometry contains no atoms (never true for a parsed file).
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Element symbols (upper case), one per atom.
    pub fn symbols(&self) -> &[String] {
        &self.symbols
    }

    /// Atomic numbers, one per atom.
    pub fn atomic_numbers(&self) -> &[u32] {
        &self.atomic_numbers
    }

    /// Coordinates in Angstrom, one `[x, y, z]` per atom.
    pub fn coords_angstrom(&self) -> &[[f64; 3]] {
        &self.coords_angstrom
    }

    /// Coordinates in Bohr, one `[x, y, z]` per atom.
    pub fn coords_bohr(&self) -> Vec<[f64; 3]> {
        self.coords_angstrom
            .iter()
            .map(|coord| coord.map(|value| value * ANGSTROM_TO_BOHR))
            .collect()
    }

    /// Coordinates in Bohr, flat and row-major (the layout cuEST expects).
    pub fn coords_bohr_flat(&self) -> Vec<f64> {
        self.coords_angstrom
            .iter()
            .flat_map(|coord| coord.iter().map(|value| value * ANGSTROM_TO_BOHR))
            .collect()
    }

    /// Nuclear charges `+Z`, one per atom.
    pub fn nuclear_charges(&self) -> Vec<f64> {
        self.atomic_numbers.iter().map(|&z| z as f64).collect()
    }

    /// Charges for the cuEST potential convention, i.e. `-Z` per atom.
    ///
    /// cuEST evaluates the potential with a positive unit test charge
    /// (`V_pq = sum_A q_A <p|1/|r - R_A||q>`); passing `-Z` yields the
    /// nuclear attraction matrix.
    pub fn nuclear_attraction_charges(&self) -> Vec<f64> {
        self.atomic_numbers.iter().map(|&z| -(z as f64)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const H2O: &str = "3
water
O 0.000000 0.000000 0.000000
H 0.000000 0.757000 0.587000
H 0.000000 -0.757000 0.587000
";

    #[test]
    fn test_parse() {
        let xyz = Xyz::parse(H2O).unwrap();
        assert_eq!(xyz.len(), 3);
        assert_eq!(xyz.symbols(), ["O", "H", "H"]);
        assert_eq!(xyz.atomic_numbers(), [8, 1, 1]);
        assert_eq!(xyz.coords_angstrom()[1], [0.0, 0.757, 0.587]);
        assert_eq!(xyz.nuclear_charges(), [8.0, 1.0, 1.0]);
        assert_eq!(xyz.nuclear_attraction_charges(), [-8.0, -1.0, -1.0]);
    }

    #[test]
    fn test_bohr_conversion() {
        let xyz = Xyz::parse(H2O).unwrap();
        let bohr = xyz.coords_bohr_flat();
        assert_eq!(bohr.len(), 9);
        assert!((bohr[4] - 0.757 / 0.52917720859).abs() < 1e-15);
        assert_eq!(bohr[0], 0.0);
    }

    #[test]
    fn test_parse_tolerance_and_errors() {
        // Lower case symbols, extra columns, comments.
        let xyz = Xyz::parse("1\ncomment\nh 1.0 2.0 3.0 extra\n").unwrap();
        assert_eq!(xyz.symbols(), ["H"]);
        assert_eq!(xyz.coords_angstrom()[0], [1.0, 2.0, 3.0]);

        assert!(Xyz::parse("").is_err());
        assert!(Xyz::parse("notanumber\ncomment\n").is_err());
        assert!(Xyz::parse("0\ncomment\n").is_err());
        assert!(Xyz::parse("1\n").is_err());
        assert!(Xyz::parse("1\ncomment\n").is_err()); // missing atom line
        assert!(Xyz::parse("2\ncomment\nH 0 0 0\n").is_err()); // too few atoms
        assert!(Xyz::parse("1\ncomment\nQq 0 0 0\n").is_err()); // unknown element
        assert!(Xyz::parse("1\ncomment\nH 0 0\n").is_err()); // missing coordinate
    }

    #[test]
    fn test_atomic_number() {
        assert_eq!(atomic_number("H").unwrap(), 1);
        assert_eq!(atomic_number("og").unwrap(), 118);
        assert_eq!(atomic_number("X").unwrap(), 0);
        assert!(atomic_number("Qq").is_err());
    }
}
