// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Element matrices of an axis-aligned hexahedral cell over the quadrature of its solid part.
//!
//! Local corner `a = ix + 2 iy + 4 iz` sits at `ξ_a = (2 ix − 1, 2 iy − 1, 2 iz − 1)`; `N_a = Π_i (1 + ξ_a,i ξ_i) / 2`.
//! Degree of freedom `3a + c` is the displacement component `c` of corner `a`.

use umst_math::profile_ldlt::{spd_factor, Clique, ProfilePattern};

use super::quadrature::CellQuadrature;
use super::{real, FiniteCellRefuse, MaterialTable};

/// The element formulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElementKind {
    /// Trilinear, conforming: eigenvalues bound the exact ones from above.
    Q1,
    /// Trilinear with nine incompatible modes, statically condensed: free of parasitic shear in bending.
    Q1E9,
}

/// Stiffness (24×24, row-major) and scalar consistent mass (8×8, row-major; the 24×24 mass is `M ⊗ I₃`).
#[derive(Clone, Debug, PartialEq)]
pub struct ElementMatrices {
    /// Stiffness.
    pub k: Vec<f64>,
    /// Scalar consistent mass.
    pub m: Vec<f64>,
    /// The incompatible modes were dropped because their block was singular on this solid part.
    pub condensation_fallback: bool,
}

const SHEAR: [[usize; 3]; 3] = [[0, 3, 5], [3, 1, 4], [5, 4, 2]];

fn corner(a: usize) -> [f64; 3] {
    std::array::from_fn(|i| if (a >> i) & 1 == 0 { -1.0 } else { 1.0 })
}

/// Shape values and physical gradients at local coordinates `xi` of a cell with edges `h`.
fn shape(xi: [f64; 3], h: [f64; 3]) -> ([f64; 8], [[f64; 3]; 8]) {
    let two = real(2);
    let eight = real(8);
    let n = std::array::from_fn(|a| {
        let s = corner(a);
        (0..3).map(|i| 1.0 + s[i] * xi[i]).product::<f64>() / eight
    });
    let g = std::array::from_fn(|a| {
        let s = corner(a);
        std::array::from_fn(|d| {
            let others: f64 = (0..3)
                .filter(|&i| i != d)
                .map(|i| 1.0 + s[i] * xi[i])
                .product();
            s[d] * others / eight * two / h[d]
        })
    });
    (n, g)
}

/// Columns of the strain–displacement matrix: column `col` holds the six strains of unit dof `col`.
fn b_columns(grad: &[[f64; 3]]) -> Vec<[f64; 6]> {
    grad.iter()
        .flat_map(|g| {
            (0..3).map(move |c| {
                let mut e = [0.0; 6];
                (0..3).for_each(|d| e[SHEAR[c][d]] += g[d]);
                e
            })
        })
        .collect()
}

/// `Σ_q w Bᵀ D B` for column sets `left` and `right`, accumulated into `out` (row-major `left × right`).
fn accumulate(out: &mut [f64], left: &[[f64; 6]], right: &[[f64; 6]], d: &[[f64; 6]; 6], w: f64) {
    let db: Vec<[f64; 6]> = right
        .iter()
        .map(|col| std::array::from_fn(|r| (0..6).map(|s| d[r][s] * col[s]).sum()))
        .collect();
    let nr = right.len();
    left.iter().enumerate().for_each(|(i, bl)| {
        db.iter().enumerate().for_each(|(j, dbj)| {
            out[i * nr + j] += w * (0..6).map(|r| bl[r] * dbj[r]).sum::<f64>()
        });
    });
}

/// Element matrices of a cell with edges `h` over its solid quadrature.
///
/// # Errors
/// [`FiniteCellRefuse::UnknownMaterial`] for a material outside the table; a profile refusal from the condensation
/// solve other than a singular block (which falls back to Q1 for this cell and is flagged).
pub fn element_matrices(
    h: [f64; 3],
    quad: &CellQuadrature,
    table: &MaterialTable,
    kind: ElementKind,
) -> Result<ElementMatrices, FiniteCellRefuse> {
    let mut kuu = vec![0.0; 24 * 24];
    let mut m = vec![0.0; 64];
    let mut kua = vec![0.0; 24 * 9];
    let mut kaa = vec![0.0; 81];
    // Mean enhanced gradient over the solid, so the re-centred modes integrate to zero there (patch test).
    let mean: [f64; 3] = std::array::from_fn(|i| {
        quad.points
            .iter()
            .map(|q| q.weight * enhanced_gradient(q.xi[i], h[i]))
            .sum::<f64>()
            / quad.volume
    });
    for q in &quad.points {
        let d = table.stiffness(q.material)?;
        let rho = table.density(q.material)?;
        let (n, g) = shape(q.xi, h);
        let bu = b_columns(&g);
        accumulate(&mut kuu, &bu, &bu, d, q.weight);
        (0..8).for_each(|a| (0..8).for_each(|b| m[a * 8 + b] += q.weight * rho * n[a] * n[b]));
        if kind == ElementKind::Q1E9 {
            let ge: [f64; 3] = std::array::from_fn(|i| enhanced_gradient(q.xi[i], h[i]) - mean[i]);
            // Mode (i, c): displacement component c times (1 − ξ_i²); its gradient points along axis i.
            let ba: Vec<[f64; 6]> = (0..3)
                .flat_map(|i| {
                    (0..3).map(move |c| {
                        let mut e = [0.0; 6];
                        e[SHEAR[c][i]] += ge[i];
                        e
                    })
                })
                .collect();
            accumulate(&mut kua, &bu, &ba, d, q.weight);
            accumulate(&mut kaa, &ba, &ba, d, q.weight);
        }
    }
    if kind == ElementKind::Q1 || quad.points.is_empty() {
        return Ok(ElementMatrices {
            k: kuu,
            m,
            condensation_fallback: false,
        });
    }
    let dofs: Vec<usize> = (0..9).collect();
    let factor = ProfilePattern::from_cliques(9, [dofs.as_slice()])
        .and_then(|p| {
            p.assemble([Clique {
                dofs: &dofs,
                block: &kaa,
            }])
        })
        .and_then(|a| spd_factor(&a));
    let Ok(factor) = factor else {
        return Ok(ElementMatrices {
            k: kuu,
            m,
            condensation_fallback: true,
        });
    };
    // K = Kuu − Kua Kaa⁻¹ Kau, one solve per displacement dof.
    let x: Vec<Vec<f64>> = (0..24)
        .map(|j| factor.solve(&(0..9).map(|a| kua[j * 9 + a]).collect::<Vec<f64>>()))
        .collect::<Result<_, _>>()?;
    let k = (0..24 * 24)
        .map(|idx| {
            let (i, j) = (idx / 24, idx % 24);
            kuu[idx] - (0..9).map(|a| kua[i * 9 + a] * x[j][a]).sum::<f64>()
        })
        .collect::<Vec<f64>>();
    // Symmetrise the rounding of the two triangular products.
    let k = (0..24 * 24)
        .map(|idx| (k[idx] + k[(idx % 24) * 24 + idx / 24]) / real(2))
        .collect();
    Ok(ElementMatrices {
        k,
        m,
        condensation_fallback: false,
    })
}

/// `∂(1 − ξ²)/∂x = −2ξ · 2/h`.
fn enhanced_gradient(xi: f64, h: f64) -> f64 {
    -real(4) * xi / h
}

/// Shape values of the cell at local coordinates (also outside `[−1, 1]³`, for extrapolation).
#[must_use]
pub fn shape_values(xi: [f64; 3]) -> [f64; 8] {
    shape(xi, [1.0; 3]).0
}
