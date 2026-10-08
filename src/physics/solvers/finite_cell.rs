// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Finite-cell (fictitious-domain) hexahedral elasticity on a tensor grid: statics, certified free-vibration
//! modes, mass properties and the driving-point apparent mass of a body given by a signed distance function.
//!
//! **Geometry.** A body implements [`OccupancyField`]: a signed distance (negative inside), a material index per
//! point, and the planar interfaces of its layers. The grid places node planes at every interface
//! ([`grid::TensorGrid::conforming`]), so a thin layer is whole cells and no cell straddles a layer interface.
//! Only the outline is immersed: a cell the outline cuts is integrated over its solid part alone.
//!
//! **Integration** ([`quadrature`]). Positive weights only. A whole cell takes 2×2×2 Gauss points. A cut cell whose
//! signed distance does not vary through its thickness is subdivided in the plane; each cut leaf is split into
//! four triangles at its centre, each triangle is clipped by the zero contour (located by bisection on the
//! distance), and each clipped piece is integrated by a collapsed-square 3×3 Gauss rule, exact for polynomials of
//! total degree four, which covers the trilinear mass integrand. The thickness direction takes 2-point Gauss,
//! exact for the trilinear integrands. A cell that varies through its thickness falls back to an octree with
//! point inclusion; such cells are counted and reported as approximate.
//!
//! **Elements** ([`element`]). Trilinear Q1, conforming, whose eigenvalues bound the exact ones from above
//! (min-max); and Q1 with nine incompatible modes (Wilson, Taylor, Doherty & Ghaboussi 1973; Taylor, Beresford &
//! Wilson 1976), statically condensed, free of the parasitic shear that locks thin Q1 layers in bending. On a cut
//! cell the incompatible-mode gradients are re-centred on the solid part, so the constant-strain patch test holds
//! there too. The gap between the two runs is a discretisation indicator.
//!
//! **No fictitious material.** Stiffness and mass integrate over the solid only. Nodes whose cells hold less than
//! a fraction `θ` of solid are aggregated onto a neighbouring well-posed cell (Badia, Verdugo & Martín 2018): their
//! values are the trilinear extrapolation of that cell's nodal values. The discrete space stays a subspace of the
//! continuous one, so the Rayleigh–Ritz bound survives.
//!
//! **Solve** ([`analysis`]). Statics by the profile `L D Lᵀ` of [`umst_math::profile_ldlt`]; modes by the
//! certified shift-invert Lanczos of [`umst_math::generalized_eigen`] with exact rigid-body deflation and a Sturm
//! count; the apparent mass at a point as the rigid-body term plus the elastic compliance by inertia relief.
//!
//! Units are SI throughout: metres, pascals, kilograms per cubic metre. The Voigt order is
//! `[xx, yy, zz, xy, yz, xz]` with engineering shear strains, as in `hex_elasticity`.
//!
//! `PHYSICS_GREEN` stays false: verification against closed forms is not validation against measured rows.

pub mod analysis;
pub mod assembly;
pub mod element;
pub mod grid;
pub mod quadrature;

use umst_math::profile_ldlt::{spd_factor, Clique, ProfilePattern, ProfileRefuse};

/// Verification against closed forms does not make a physics claim; measured rows decide.
pub const FINITE_CELL_PHYSICS_GREEN: bool = false;

/// A 6×6 stiffness in Voigt order `[xx, yy, zz, xy, yz, xz]` with engineering shear, in pascals.
pub type Voigt6 = [[f64; 6]; 6];

/// A body to discretise: geometry, material regions and planar layer interfaces, in metres.
pub trait OccupancyField {
    /// Axis-aligned bounds `(lo, hi)` enclosing the body.
    fn bounds(&self) -> ([f64; 3], [f64; 3]);
    /// Signed distance, negative inside the solid. Only its sign and its zero set are used.
    fn signed_distance(&self, p: [f64; 3]) -> f64;
    /// Material index at a point, `None` outside the solid.
    fn material_at(&self, p: [f64; 3]) -> Option<u16>;
    /// Planar interfaces normal to `axis` (`0`, `1` or `2`), where material changes across a plane.
    fn interface_planes(&self, axis: usize) -> Vec<f64>;
}

/// Why a finite-cell model was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FiniteCellRefuse {
    /// Bounds, spacings or tolerances are not finite and positive, or `lo ≥ hi`.
    InvalidGeometry,
    /// A material stiffness is not symmetric positive definite, or a density is not positive.
    InvalidMaterial {
        /// Index into the table.
        index: u16,
    },
    /// The field named a material the table does not hold.
    UnknownMaterial {
        /// The index named.
        index: u16,
    },
    /// No cell holds solid.
    EmptyBody,
    /// An ill-posed cell found no well-posed cell to aggregate onto within its layer.
    AggregationUnresolved,
    /// A support constrains an aggregated node whose masters are not all fixed alike (its value is theirs).
    SupportOnAggregatedNode,
    /// The stiffness with the given supports is singular: the supports leave a mechanism.
    Mechanism,
    /// A profile operation refused.
    Profile(ProfileRefuse),
    /// A point lies outside the solid of the model.
    PointOutsideBody,
    /// A vector has the wrong length.
    DimMismatch,
}

impl From<ProfileRefuse> for FiniteCellRefuse {
    fn from(e: ProfileRefuse) -> Self {
        Self::Profile(e)
    }
}

/// Stiffness and density per material index.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialTable {
    stiffness: Vec<Voigt6>,
    density: Vec<f64>,
}

impl MaterialTable {
    /// Admit a table whose stiffnesses are symmetric positive definite and whose densities are positive.
    ///
    /// Definiteness is decided by an `L D Lᵀ` factor (all pivots positive), not by a test on samples.
    ///
    /// # Errors
    /// [`FiniteCellRefuse::InvalidMaterial`] at the first failing entry; [`FiniteCellRefuse::DimMismatch`] when
    /// the two lists differ in length.
    pub fn try_new(stiffness: Vec<Voigt6>, density: Vec<f64>) -> Result<Self, FiniteCellRefuse> {
        if stiffness.len() != density.len() || stiffness.len() > usize::from(u16::MAX) {
            return Err(FiniteCellRefuse::DimMismatch);
        }
        let dofs: Vec<usize> = (0..6).collect();
        for (i, (d, rho)) in stiffness.iter().zip(&density).enumerate() {
            let index = u16::try_from(i).map_err(|_| FiniteCellRefuse::DimMismatch)?;
            let symmetric = (0..6).all(|r| (0..6).all(|c| d[r][c] == d[c][r]));
            let block: Vec<f64> = d.iter().flatten().copied().collect();
            let spd = ProfilePattern::from_cliques(6, [dofs.as_slice()])
                .and_then(|p| {
                    p.assemble([Clique {
                        dofs: &dofs,
                        block: &block,
                    }])
                })
                .and_then(|a| spd_factor(&a))
                .is_ok();
            if !(symmetric && spd && rho.is_finite() && *rho > 0.0) {
                return Err(FiniteCellRefuse::InvalidMaterial { index });
            }
        }
        Ok(Self { stiffness, density })
    }

    /// Stiffness of a material.
    ///
    /// # Errors
    /// [`FiniteCellRefuse::UnknownMaterial`] for an index outside the table.
    pub fn stiffness(&self, index: u16) -> Result<&Voigt6, FiniteCellRefuse> {
        self.stiffness
            .get(usize::from(index))
            .ok_or(FiniteCellRefuse::UnknownMaterial { index })
    }

    /// Density of a material.
    ///
    /// # Errors
    /// [`FiniteCellRefuse::UnknownMaterial`] for an index outside the table.
    pub fn density(&self, index: u16) -> Result<f64, FiniteCellRefuse> {
        self.density
            .get(usize::from(index))
            .copied()
            .ok_or(FiniteCellRefuse::UnknownMaterial { index })
    }
}

/// Isotropic stiffness in Voigt order from Young's modulus and Poisson's ratio.
#[must_use]
pub fn isotropic_stiffness(e: f64, nu: f64) -> Voigt6 {
    let lambda = e * nu / ((1.0 + nu) * (1.0 - nu - nu));
    let mu = e / (real(2) * (1.0 + nu));
    let mut d = [[0.0; 6]; 6];
    for (r, row) in d.iter_mut().enumerate().take(3) {
        for (c, v) in row.iter_mut().enumerate().take(3) {
            *v = if r == c { lambda + mu + mu } else { lambda };
        }
    }
    (3..6).for_each(|i| d[i][i] = mu);
    d
}

/// `usize` as `f64`; every count here is below 2⁵³.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub(crate) fn real(n: usize) -> f64 {
    n as f64
}

/// Gauss–Legendre points and weights on `[−1, 1]`, from integer arithmetic only.
#[must_use]
pub(crate) fn gauss_legendre(order: usize) -> Vec<(f64, f64)> {
    match order {
        1 => vec![(0.0, real(2))],
        2 => {
            let a = (1.0 / real(3)).sqrt();
            vec![(-a, 1.0), (a, 1.0)]
        }
        _ => {
            let a = (real(3) / real(5)).sqrt();
            let outer = real(5) / real(9);
            vec![(-a, outer), (0.0, real(8) / real(9)), (a, outer)]
        }
    }
}
