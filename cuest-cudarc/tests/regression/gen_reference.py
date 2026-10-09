#!/usr/bin/env python3
"""Generate `tests/regression/reference.toml`: pyscf reference values for the
cuEST wrapper regression tests.

Usage:
    python3 gen_reference.py [output_path]

Requires pyscf (tested with 2.14) and numpy.

For each case the script dumps:

- the geometry (Angstrom) and the basis shell data in the *standard basis
  file* coefficient convention (coefficients relative to normalized
  primitives), which is what `CuestShellSpec::normalized` expects; general
  contractions are fanned out into one shell per coefficient column;
- the AO overlap (S), kinetic (T) and nuclear attraction (V) matrices in the
  spherical (pure) AO basis, flat row-major, in pyscf's AO ordering.
"""

import sys

import numpy as np
import pyscf
from pyscf import gto

CASES = [
    dict(
        name="h2_sto3g",
        basis="sto-3g",
        geometry="H 0.0 0.0 0.0\nH 0.0 0.0 0.7408",
        threshold_pq=1.0e-14,
    ),
    dict(
        name="h2o_def2svp",
        basis="def2-svp",
        geometry="O 0.0 0.0 0.0\nH 0.0 0.757 0.587\nH 0.0 -0.757 0.587",
        threshold_pq=1.0e-14,
    ),
]

# pyscf represents each basis entry as `[l, row_1, row_2, ...]` where each row
# is `[exponent, coefficient_1, coefficient_2, ...]`: one row per primitive,
# one coefficient per contraction column. AO ordering is atom order, then
# entry order, then column order. Each contraction column becomes one cuEST
# shell (keeping the full primitive list, like pyscf's internal expansion).
def dump_shells(mol):
    shells = []
    for atom in range(mol.natm):
        symbol = mol.atom_symbol(atom)
        for entry in mol._basis[symbol]:
            l = int(entry[0])
            rows = [[float(value) for value in row] for row in entry[1:]]
            ncolumn = len(rows[0]) - 1
            assert all(len(row) == ncolumn + 1 for row in rows), entry
            exponents = [row[0] for row in rows]
            for column in range(ncolumn):
                shells.append(
                    dict(
                        atom=atom,
                        l=l,
                        exponents=exponents,
                        coefficients=[row[1 + column] for row in rows],
                    )
                )
    return shells


def internal_shells(mol):
    """(l, number of primitives) per shell in pyscf's expansion order."""
    return [(int(row[1]), int(row[2])) for row in mol._bas]


# pyscf's AO ordering within an l-shell differs from cuEST's "CCA" solid
# harmonic ordering (cuEST documentation, `gaussian_basis.rst`: solid
# harmonics are ordered by magnetic quantum number as `0, 1c, 1s, 2c, 2s, ...`
# and `P` is `{z, x, y}` in solid harmonics). The per-shell permutations
# below map cuEST-local indices to pyscf-local indices; the pyscf side was
# verified numerically (`pyscf.gto.eval_gto` on a sphere: pyscf's p shells are
# `(x, y, z)`, its d shells `(xy, yz, z2, xz, x2-y2)`).
#   cuEST p: (z, x, y)        <- pyscf local (2, 0, 1)
#   cuEST d: (z2, xz, yz, x2-y2, xy) <- pyscf local (2, 3, 1, 4, 0)
PYSCF_TO_CUEST_ORDER = {0: [0], 1: [2, 0, 1], 2: [2, 3, 1, 4, 0]}


def cuest_ao_order(shells):
    """Map cuEST AO index -> pyscf AO index for the given shell layout."""
    order = []
    offset = 0
    for shell in shells:
        l = shell["l"]
        if l not in PYSCF_TO_CUEST_ORDER:
            raise RuntimeError(f"no pyscf/cuEST AO mapping for l = {l}")
        order.extend(offset + i for i in PYSCF_TO_CUEST_ORDER[l])
        offset += 2 * l + 1
    return order


def fmt(value):
    if isinstance(value, float):
        return repr(value)
    if isinstance(value, (list, tuple)):
        return "[" + ", ".join(fmt(v) for v in value) + "]"
    return repr(value)


def dump_case(case):
    mol = gto.M(
        atom=case["geometry"],
        basis=case["basis"],
        unit="Angstrom",
        cart=False,
        verbose=0,
    )
    shells = dump_shells(mol)
    dumped = [(shell["l"], len(shell["exponents"])) for shell in shells]
    internal = internal_shells(mol)
    assert dumped == internal, (case["name"], dumped, internal)
    # The dumped (spherical) shells must span the same AO space as pyscf's.
    assert sum(2 * shell["l"] + 1 for shell in shells) == mol.nao_nr()

    nao = mol.nao_nr()
    # Reference matrices, reordered from pyscf's AO ordering into cuEST's CCA
    # ordering (see PYSCF_TO_CUEST_ORDER above): M_cuest[a, b] is the overlap
    # of cuEST AOs a and b, i.e. pyscf AOs order[a] and order[b].
    order = cuest_ao_order(shells)
    assert sorted(order) == list(range(nao))
    s = mol.intor("int1e_ovlp")[np.ix_(order, order)]
    t = mol.intor("int1e_kin")[np.ix_(order, order)]
    v = mol.intor("int1e_nuc")[np.ix_(order, order)]
    symbols = [mol.atom_symbol(i) for i in range(mol.natm)]
    xyz = np.array([mol.atom_coord(i, unit="Angstrom") for i in range(mol.natm)])
    assert s.shape == (nao, nao)

    lines = []
    lines.append("[[cases]]")
    lines.append(f"name = {fmt(case['name'])}")
    lines.append(f"basis = {fmt(case['basis'])}")
    lines.append("pure = true")
    lines.append(f"threshold_pq = {fmt(case['threshold_pq'])}")
    lines.append(f"symbols = {fmt(symbols)}")
    lines.append(f"xyz_angstrom = {fmt([float(x) for x in xyz.ravel()])}")
    lines.append(f"nao = {nao}")
    for key, matrix in (("s", s), ("t", t), ("v", v)):
        lines.append(f"{key} = {fmt([float(x) for x in matrix.ravel()])}")
    for shell in shells:
        lines.append("")
        lines.append("[[cases.shells]]")
        lines.append(f"atom = {shell['atom']}")
        lines.append(f"l = {shell['l']}")
        lines.append(f"exponents = {fmt(shell['exponents'])}")
        lines.append(f"coefficients = {fmt(shell['coefficients'])}")
    lines.append("")
    return lines


def main():
    output = sys.argv[1] if len(sys.argv) > 1 else "reference.toml"
    lines = [
        "# Auto-generated by gen_reference.py; do not edit manually.",
        f"# pyscf {pyscf.__version__}",
        "",
    ]
    for case in CASES:
        lines.extend(dump_case(case))
        lines.append("")
    with open(output, "w") as fh:
        fh.write("\n".join(lines))
    print(f"wrote {output}")


if __name__ == "__main__":
    main()
