// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Statics, certified free-vibration modes and the driving-point apparent mass of an assembled body.
//!
//! **Apparent mass.** For a free body and a force along `n` at `p`, the receptance at circular frequency `ω` is
//! `α(ω) = −(1/ω²) Σ_r φ_r(p)² + c(p) + Σ_k φ_k(p)² (1/(ω_k² − ω²) − 1/ω_k²)` (mass-normalised modes, values
//! along `n`), with `r` over the rigid-body modes, `k` over the computed elastic modes, and `c` the elastic
//! compliance of the free body by inertia relief: the load minus its rigid-body inertia, `P f = f − M R (RᵀMR)⁻¹
//! Rᵀ f`, solved on a statically determinate 3-2-1 support and projected `M`-orthogonally to the rigid modes. The
//! modal term corrects the static compliance of the computed modes for their dynamics (the mode-acceleration
//! method). The apparent mass is `−1/(ω² α)`, and tends to the rigid-body effective mass `1/Σ_r φ_r(p)²` as
//! `ω → 0` (Cross 1999, *Am. J. Phys.* 67, 692).

use umst_math::generalized_eigen::{
    lowest_eigenpairs, Deflation, EigenRefuse, EigenRequest, GeneralizedEigenSolution, Pencil,
};
use umst_math::profile_ldlt::{spd_factor, Clique, ProfilePattern, ProfileRefuse, SpdFactor};
use umst_math::solve_combinator::{EnergyBudget, ProblemTolerance, SolveOutcome, StepEnergyMeter};

use super::assembly::{assemble, Discretisation, System};
use super::{real, FiniteCellRefuse};

fn mechanism(e: ProfileRefuse) -> FiniteCellRefuse {
    match e {
        ProfileRefuse::NotPositiveDefinite { .. } | ProfileRefuse::NearZeroPivot { .. } => {
            FiniteCellRefuse::Mechanism
        }
        other => FiniteCellRefuse::Profile(other),
    }
}

/// Factor the stiffness of a supported system; a singular stiffness is a mechanism.
///
/// # Errors
/// [`FiniteCellRefuse::Mechanism`] when the supports leave a rigid motion.
pub fn stiffness_factor(system: &System) -> Result<SpdFactor, FiniteCellRefuse> {
    spd_factor(system.stiffness()).map_err(mechanism)
}

/// Displacements under nodal forces `f` (one per degree of freedom).
///
/// # Errors
/// [`FiniteCellRefuse::Mechanism`] for a singular stiffness; [`FiniteCellRefuse::DimMismatch`] for a wrong length.
pub fn static_displacement(system: &System, f: &[f64]) -> Result<Vec<f64>, FiniteCellRefuse> {
    if f.len() != system.n() {
        return Err(FiniteCellRefuse::DimMismatch);
    }
    Ok(stiffness_factor(system)?.solve(f)?)
}

/// Consistent nodal forces of a point force at `p`: each node takes the force times its shape value there.
///
/// # Errors
/// [`FiniteCellRefuse::PointOutsideBody`] when `p` lies in no occupied cell.
pub fn point_load(
    disc: &Discretisation,
    system: &System,
    p: [f64; 3],
    force: [f64; 3],
) -> Result<Vec<f64>, FiniteCellRefuse> {
    let weights = disc.interpolation_at(p)?;
    Ok(weights
        .iter()
        .fold(vec![0.0; system.n()], |mut f, &(node, w)| {
            (0..3).for_each(|c| {
                if let Some(d) = system.dof(node, c) {
                    f[d] += w * force[c];
                }
            });
            f
        }))
}

/// The six rigid-body motions about the centre of mass, as vectors of degrees of freedom: three translations, then
/// rotations about `x`, `y` and `z`.
#[must_use]
pub fn rigid_body_modes(disc: &Discretisation, system: &System) -> Vec<Vec<f64>> {
    let c = disc.mass_properties().centroid;
    (0..6)
        .map(|r| {
            disc.free_nodes()
                .fold(vec![0.0; system.n()], |mut v, node| {
                    let p = disc.node_point(node);
                    let x: [f64; 3] = std::array::from_fn(|i| p[i] - c[i]);
                    let u: [f64; 3] = match r {
                        0..=2 => std::array::from_fn(|i| if i == r { 1.0 } else { 0.0 }),
                        // e_axis × x
                        3 => [0.0, -x[2], x[1]],
                        4 => [x[2], 0.0, -x[0]],
                        _ => [-x[1], x[0], 0.0],
                    };
                    (0..3).for_each(|i| {
                        if let Some(d) = system.dof(node, i) {
                            v[d] = u[i];
                        }
                    });
                    v
                })
        })
        .collect()
}

/// What modal analysis to run.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModeRequest {
    /// Elastic modes wanted (rigid-body modes of a free body are deflated and reported apart).
    pub wanted: usize,
    /// Spectral shift `σ = −ω_σ²`, below the first elastic eigenvalue.
    pub shift: f64,
    /// Relative Ritz residual target.
    pub tolerance: ProblemTolerance,
}

/// The lowest elastic modes with certified eigenvalue intervals. A free body's rigid-body modes are deflated
/// exactly and re-certified in [`GeneralizedEigenSolution::deflated`].
///
/// # Errors
/// [`EigenRefuse`] from the eigensolver.
pub fn modes<M: StepEnergyMeter>(
    disc: &Discretisation,
    system: &System,
    request: &ModeRequest,
    budget: EnergyBudget,
    meter: &M,
) -> Result<SolveOutcome<GeneralizedEigenSolution>, EigenRefuse> {
    let deflation = if system.is_supported() {
        Vec::new()
    } else {
        rigid_body_modes(disc, system)
            .into_iter()
            .map(|vector| Deflation { vector })
            .collect()
    };
    let eig = EigenRequest {
        wanted: request.wanted,
        shift: request.shift,
        deflation,
        tolerance: request.tolerance,
    };
    lowest_eigenpairs(
        Pencil {
            k: system.stiffness(),
            m: system.mass(),
        },
        &eig,
        budget,
        meter,
    )
}

/// Frequency in hertz of an eigenvalue `λ = ω²` (a negative rounding of a rigid mode reads as zero).
#[must_use]
pub fn frequency_hz(lambda: f64) -> f64 {
    lambda.max(0.0).sqrt() / (real(2) * std::f64::consts::PI)
}

/// Apparent mass at one point and direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApparentMass {
    /// Rigid-body effective mass `1/Σ_r φ_r(p)²` (kg).
    pub rigid_mass: f64,
    /// Elastic compliance of the free body by inertia relief (m/N).
    pub compliance: f64,
    /// Apparent mass at the requested frequency (kg).
    pub apparent_mass: f64,
}

/// Inertia-relief solver of a free body: a factor of the stiffness on a statically determinate 3-2-1 support and
/// the rigid-body Gram matrix.
pub struct InertiaRelief<'a> {
    disc: &'a Discretisation,
    free: &'a System,
    pinned: System,
    factor: SpdFactor,
    rigid: Vec<Vec<f64>>,
    m_rigid: Vec<Vec<f64>>,
    gram: SpdFactor,
}

impl<'a> InertiaRelief<'a> {
    /// Build for a free system. Supports: all components at the free node of least coordinate along the longest
    /// axis (`A`); at the node of greatest coordinate (`B`), the two components most nearly normal to `AB`; at the
    /// node farthest from line `AB` (`C`), the component along which a rotation about `AB` moves it most.
    ///
    /// # Errors
    /// [`FiniteCellRefuse::Mechanism`] when the support does not remove the rigid motions; profile refusals.
    pub fn new(disc: &'a Discretisation, free: &'a System) -> Result<Self, FiniteCellRefuse> {
        let nodes: Vec<(usize, [f64; 3])> =
            disc.free_nodes().map(|n| (n, disc.node_point(n))).collect();
        let counts = disc.grid().node_counts();
        let long = (0..3).max_by_key(|&i| counts[i]).unwrap_or(0);
        let by = |key: &dyn Fn(&[f64; 3]) -> f64| {
            nodes
                .iter()
                .copied()
                .max_by(|a, b| key(&a.1).total_cmp(&key(&b.1)))
                .ok_or(FiniteCellRefuse::EmptyBody)
        };
        let a = by(&|p| -p[long])?;
        let b = by(&|p| p[long])?;
        let d: [f64; 3] = std::array::from_fn(|i| b.1[i] - a.1[i]);
        let cross = |p: &[f64; 3]| -> [f64; 3] {
            let r: [f64; 3] = std::array::from_fn(|i| p[i] - a.1[i]);
            [
                d[1] * r[2] - d[2] * r[1],
                d[2] * r[0] - d[0] * r[2],
                d[0] * r[1] - d[1] * r[0],
            ]
        };
        let c = by(&|p| cross(p).iter().map(|v| v * v).sum())?;
        let mut b_axes = [0, 1, 2];
        b_axes.sort_by(|&i, &j| d[i].abs().total_cmp(&d[j].abs()));
        let rc = cross(&c.1);
        let c_axis = (0..3)
            .max_by(|&i, &j| rc[i].abs().total_cmp(&rc[j].abs()))
            .unwrap_or(2);
        let fixed = move |p: [f64; 3]| -> [bool; 3] {
            if p == a.1 {
                [true; 3]
            } else if p == b.1 {
                std::array::from_fn(|i| i == b_axes[0] || i == b_axes[1])
            } else if p == c.1 {
                std::array::from_fn(|i| i == c_axis)
            } else {
                [false; 3]
            }
        };
        let pinned = assemble(disc, &fixed)?;
        let factor = stiffness_factor(&pinned)?;
        let rigid = rigid_body_modes(disc, free);
        let m_rigid: Vec<Vec<f64>> = rigid
            .iter()
            .map(|r| free.mass().mul(r))
            .collect::<Result<_, _>>()?;
        let g: Vec<f64> = (0..36)
            .map(|ij| dot(&rigid[ij / 6], &m_rigid[ij % 6]))
            .collect();
        let dofs: Vec<usize> = (0..6).collect();
        let gram = ProfilePattern::from_cliques(6, [dofs.as_slice()])
            .and_then(|p| {
                p.assemble([Clique {
                    dofs: &dofs,
                    block: &g,
                }])
            })
            .and_then(|m| spd_factor(&m))
            .map_err(mechanism)?;
        Ok(Self {
            disc,
            free,
            pinned,
            factor,
            rigid,
            m_rigid,
            gram,
        })
    }

    /// `Rᵀ v` for the six rigid modes.
    fn rigid_dot(&self, v: &[f64]) -> Vec<f64> {
        self.rigid.iter().map(|r| dot(r, v)).collect()
    }

    /// Elastic compliance along `n` at `p` and the rigid-body inverse mass `Σ_r φ_r(p)²` there.
    ///
    /// # Errors
    /// [`FiniteCellRefuse::PointOutsideBody`]; profile refusals.
    pub fn compliance(&self, p: [f64; 3], n: [f64; 3]) -> Result<(f64, f64), FiniteCellRefuse> {
        let f = point_load(self.disc, self.free, p, n)?;
        // Equilibrate: f − M R G⁻¹ Rᵀ f.
        let coeff = self.gram.solve(&self.rigid_dot(&f))?;
        let pf: Vec<f64> = (0..f.len())
            .map(|i| f[i] - (0..6).map(|r| self.m_rigid[r][i] * coeff[r]).sum::<f64>())
            .collect();
        let to_pinned = self.map(self.free, &self.pinned, &pf);
        let u_pinned = self.factor.solve(&to_pinned)?;
        let u = self.map(&self.pinned, self.free, &u_pinned);
        // Project out the rigid part M-orthogonally: u − R G⁻¹ Rᵀ M u.
        let mu = self.free.mass().mul(&u)?;
        let c = self.gram.solve(&self.rigid_dot(&mu))?;
        let u_el: Vec<f64> = (0..u.len())
            .map(|i| u[i] - (0..6).map(|r| self.rigid[r][i] * c[r]).sum::<f64>())
            .collect();
        let along = |v: &[f64]| -> Result<f64, FiniteCellRefuse> {
            let x = self.disc.value_at(self.free, v, p)?;
            Ok((0..3).map(|i| x[i] * n[i]).sum())
        };
        let r_at: Vec<f64> = self
            .rigid
            .iter()
            .map(|r| along(r))
            .collect::<Result<_, _>>()?;
        let inv_mass = dot(&r_at, &self.gram.solve(&r_at)?);
        Ok((along(&u_el)?, inv_mass))
    }

    fn map(&self, from: &System, to: &System, v: &[f64]) -> Vec<f64> {
        self.disc
            .free_nodes()
            .fold(vec![0.0; to.n()], |mut out, node| {
                (0..3).for_each(|c| {
                    if let (Some(i), Some(j)) = (from.dof(node, c), to.dof(node, c)) {
                        out[j] = v[i];
                    }
                });
                out
            })
    }

    /// Apparent mass at `p` along `n` at circular frequency `omega > 0`, with the computed elastic modes.
    ///
    /// # Errors
    /// As [`Self::compliance`]; [`FiniteCellRefuse::InvalidGeometry`] for a non-positive frequency.
    pub fn apparent_mass(
        &self,
        modes: &GeneralizedEigenSolution,
        p: [f64; 3],
        n: [f64; 3],
        omega: f64,
    ) -> Result<ApparentMass, FiniteCellRefuse> {
        if !(omega.is_finite() && omega > 0.0) {
            return Err(FiniteCellRefuse::InvalidGeometry);
        }
        let (compliance, inv_mass) = self.compliance(p, n)?;
        let w2 = omega * omega;
        let modal: f64 = modes
            .pairs
            .iter()
            .map(|pair| {
                let x = self.disc.value_at(self.free, &pair.vector, p)?;
                let phi: f64 = (0..3).map(|i| x[i] * n[i]).sum();
                let wk2 = pair.lambda;
                Ok(phi * phi * (1.0 / (wk2 - w2) - 1.0 / wk2))
            })
            .sum::<Result<f64, FiniteCellRefuse>>()?;
        let alpha = -inv_mass / w2 + compliance + modal;
        Ok(ApparentMass {
            rigid_mass: inv_mass.recip(),
            compliance,
            apparent_mass: -1.0 / (w2 * alpha),
        })
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
