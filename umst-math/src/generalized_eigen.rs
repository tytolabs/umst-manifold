// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Certified lowest eigenpairs of a symmetric pencil `K x = λ M x` (`K` positive semidefinite, `M` positive
//! definite, both on one [`SymmetricProfile`]).
//!
//! **Method.** Shift-invert Lanczos in the `M` inner product on `A = (K − σM)⁻¹ M`, with `σ` below the wanted
//! spectrum, full re-orthogonalisation, and exact deflation of known eigenvectors (the rigid-body modes of a free
//! body). Each step is one step of the budgeted [`unfold`]: the state is the Krylov basis held in shared immutable
//! vectors, the residual is the largest relative Ritz residual estimate of the wanted pairs, and the stop is the
//! problem tolerance, a stall over a problem-derived window, or the energy budget.
//!
//! **Certificates, per pair.** With `x` normalised in `M`, Rayleigh quotient `ρ`, and `δ = ‖K x − ρ M x‖_{M⁻¹}`:
//!
//! - *Weinstein*: some eigenvalue of the pencil lies in `[ρ − δ, ρ + δ]`;
//! - *Kahan (clusters)*: overlapping Weinstein intervals may share one eigenvalue, so overlapping pairs are merged
//!   into a cluster, replaced by the Rayleigh–Ritz pairs of their span, and bounded together: `k` eigenvalues lie
//!   within `‖K X − M X H‖_F` of the `k` Ritz values (Parlett 1998, ch. 11);
//! - *Kato–Temple*: if `(a, b)` holds exactly one eigenvalue `λ*` and `a < ρ < b`, then
//!   `ρ − δ²/(b − ρ) ≤ λ* ≤ ρ + δ²/(ρ − a)`, issued only for isolated pairs under a certified count, with `σ` as
//!   the lower gap end of the first.
//!
//! Every interval is widened by `η`, an allowance for the rounding of the computed `ρ` (Higham 2002, §3.1). It is
//! an allowance and not a floating-point proof: the rounding of the residual vector is not bounded separately.
//!
//! **Completeness.** By Sylvester's law of inertia the number of negative pivots of `K − τM` is the number of
//! eigenvalues below `τ` ([`crate::profile_ldlt::LdltFactor::inertia`]), for the factored matrix up to the
//! perturbation of an unpivoted `L D Lᵀ`. With the clusters disjoint, `τ` above the last interval and below the
//! next Ritz value, a count equal to the number of pairs proves no eigenvalue was missed.
//!
//! References: Parlett, *The Symmetric Eigenvalue Problem* (SIAM 1998), §3 (Weinstein), §10 (Kato–Temple), §13
//! (Lanczos); Ericsson & Ruhe, *Math. Comp.* 35 (1980) 1251–1268 (spectral transformation Lanczos); Bathe,
//! *Finite Element Procedures* (2014) §11.4.3 (Sturm sequence check).

use std::rc::Rc;

use ordered_float::NotNan;

use crate::profile_ldlt::{
    ldlt, spd_factor, usize_to_f64, ProfileRefuse, SpdFactor, SymmetricProfile,
};
use crate::solve_combinator::{
    unfold, CombinatorRefuse, EnergyBudget, ProblemProgressWindow, ProblemTolerance, SolveOutcome,
    StepEnergyMeter, UnfoldStop,
};

/// Why the eigensolver refused to start or to finish its certificates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EigenRefuse {
    /// A profile operation refused.
    Profile(ProfileRefuse),
    /// The budgeted unfold refused.
    Combinator(CombinatorRefuse),
    /// `M` is not positive definite (first non-positive pivot).
    MassNotPositiveDefinite {
        /// Row of the pivot.
        row: usize,
    },
    /// `K − σM` has negative pivots: eigenvalues lie below the shift, so `σ` is not below the wanted spectrum.
    ShiftNotBelowSpectrum {
        /// Eigenvalues below `σ`.
        below: usize,
    },
    /// No pair was requested, or more were requested than the deflated space holds.
    WantedOutOfRange,
    /// A deflation vector has the wrong length, is zero, or depends on the others.
    DeflationInvalid,
    /// The tridiagonal Ritz problem did not converge within one bit of precision per sweep.
    TridiagonalNoConvergence,
}

impl From<ProfileRefuse> for EigenRefuse {
    fn from(e: ProfileRefuse) -> Self {
        Self::Profile(e)
    }
}

impl From<CombinatorRefuse> for EigenRefuse {
    fn from(e: CombinatorRefuse) -> Self {
        Self::Combinator(e)
    }
}

/// A known eigenvector removed from the iteration (for example a rigid-body mode of a free body). Its eigenvalue
/// is not taken on trust: the solution re-certifies it against the pencil.
#[derive(Clone, Debug, PartialEq)]
pub struct Deflation {
    /// The eigenvector (any normalisation).
    pub vector: Vec<f64>,
}

/// What to compute.
#[derive(Clone, Debug, PartialEq)]
pub struct EigenRequest {
    /// Number of lowest eigenpairs wanted after deflation.
    pub wanted: usize,
    /// Spectral shift `σ`, strictly below every wanted eigenvalue (negative for a free body).
    pub shift: f64,
    /// Known eigenpairs projected out of the iteration.
    pub deflation: Vec<Deflation>,
    /// Stopping rule: the relative Ritz residual of the shift-invert operator, from the problem. It does not fix
    /// the width of the pencil intervals, which each pair reports from its own residual.
    pub tolerance: ProblemTolerance,
}

/// Which bound produced a pair's interval.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EigenBound {
    /// `[ρ − δ, ρ + δ]`.
    Weinstein,
    /// Kato–Temple under a certified count, intersected with Weinstein.
    KatoTemple,
    /// A cluster of overlapping intervals bounded together by Kahan's theorem: the interval holds as many
    /// eigenvalues as the cluster has members.
    Cluster,
}

/// An eigenpair with a certified interval for its eigenvalue.
#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedEigenpair {
    /// Rayleigh quotient `ρ = xᵀKx / xᵀMx`.
    pub lambda: f64,
    /// Lower bound of the eigenvalue: certified up to the floating-point allowance in `rounding`.
    pub lower: f64,
    /// Upper bound of the eigenvalue: certified up to the floating-point allowance in `rounding`.
    pub upper: f64,
    /// The bound used.
    pub bound: EigenBound,
    /// `δ = ‖K x − ρ M x‖_{M⁻¹}` for the `M`-normalised vector.
    pub residual: f64,
    /// Floating-point allowance `η` on the computed `ρ` (an allowance for the rounding of `ρ`, not a proof that
    /// covers the rounding of the residual; see the module documentation).
    pub rounding: f64,
    /// The eigenvector, normalised so `xᵀ M x = 1`.
    pub vector: Rc<[f64]>,
}

/// Whether the computed pairs are provably the lowest ones.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Completeness {
    /// `K − τM` has exactly `below` negative pivots, equal to the deflated plus the computed pairs.
    Certified {
        /// The Sturm shift.
        tau: f64,
        /// Eigenvalues below `τ`.
        below: usize,
    },
    /// The count disagrees: an eigenvalue below `τ` was not found.
    Missed {
        /// The Sturm shift.
        tau: f64,
        /// Eigenvalues below `τ`.
        below: usize,
        /// Deflated plus computed pairs.
        found: usize,
    },
    /// No admissible `τ` (the last wanted interval meets the next Ritz value, or both trial shifts hit a
    /// near-zero pivot).
    Unchecked,
}

/// The result: deflated pairs with their residual certificates, the computed pairs in ascending order, and the
/// completeness certificate.
#[derive(Clone, Debug, PartialEq)]
pub struct GeneralizedEigenSolution {
    /// The deflated pairs, re-certified against the pencil.
    pub deflated: Vec<CertifiedEigenpair>,
    /// The computed pairs, lowest first.
    pub pairs: Vec<CertifiedEigenpair>,
    /// Sturm count certificate.
    pub completeness: Completeness,
    /// Wanted pairs not delivered (an invariant subspace or the budget ended the iteration first).
    pub shortfall: usize,
}

/// The pencil `(K, M)`.
#[derive(Clone, Copy, Debug)]
pub struct Pencil<'a> {
    /// Stiffness, symmetric positive semidefinite.
    pub k: &'a SymmetricProfile,
    /// Mass, symmetric positive definite.
    pub m: &'a SymmetricProfile,
}

/// Vectors held in shared immutable storage, so cloning a basis copies pointers.
type Basis = Vec<Rc<[f64]>>;

/// Krylov state: an `M`-orthonormal basis, its `M` images, and the Lanczos tridiagonal.
#[derive(Clone, Debug)]
struct Krylov {
    q: Vec<Rc<[f64]>>,
    mq: Vec<Rc<[f64]>>,
    alpha: Vec<f64>,
    beta: Vec<f64>,
    exhausted: bool,
}

/// Fixed context of one solve.
struct Context<'a> {
    shifted: crate::profile_ldlt::LdltFactor,
    z: Vec<Rc<[f64]>>,
    mz: Vec<Rc<[f64]>>,
    m: &'a SymmetricProfile,
    wanted: usize,
    capacity: usize,
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn axpy(y: &[f64], a: f64, x: &[f64]) -> Vec<f64> {
    y.iter().zip(x).map(|(yi, xi)| yi + a * xi).collect()
}

fn scale(x: &[f64], a: f64) -> Vec<f64> {
    x.iter().map(|v| v * a).collect()
}

/// `w` minus its `M`-projection on each basis vector, given the basis and its `M` images.
fn orthogonalise(w: Vec<f64>, basis: &[Rc<[f64]>], mbasis: &[Rc<[f64]>]) -> Vec<f64> {
    basis.iter().zip(mbasis).fold(w, |w, (q, mq)| {
        let c = dot(&w, mq);
        axpy(&w, -c, q)
    })
}

impl Context<'_> {
    /// `M`-orthogonalise against the deflated space and the basis, twice (Kahan–Parlett "twice is enough").
    fn reorthogonalise(&self, w: Vec<f64>, k: &Krylov) -> Vec<f64> {
        let once = orthogonalise(orthogonalise(w, &self.z, &self.mz), &k.q, &k.mq);
        orthogonalise(orthogonalise(once, &self.z, &self.mz), &k.q, &k.mq)
    }

    fn step(&self, k: &Krylov) -> Result<Krylov, CombinatorRefuse> {
        if k.exhausted {
            return Ok(k.clone());
        }
        let j = k.alpha.len();
        let qj = &k.q[j];
        let mqj = &k.mq[j];
        let w = self
            .shifted
            .solve(mqj)
            .map_err(|_| CombinatorRefuse::NonFiniteQuantity)?;
        let alpha = dot(&w, mqj);
        let w = axpy(&w, -alpha, qj);
        let w = match j {
            0 => w,
            _ => axpy(&w, -k.beta[j - 1], &k.q[j - 1]),
        };
        let w = self.reorthogonalise(w, k);
        let mw = self
            .m
            .mul(&w)
            .map_err(|_| CombinatorRefuse::NonFiniteQuantity)?;
        let beta = dot(&w, &mw).max(0.0).sqrt();
        let scale_of_t = k
            .alpha
            .iter()
            .chain(std::iter::once(&alpha))
            .fold(0.0_f64, |s, a| s.max(a.abs()));
        let breakdown = beta <= f64::EPSILON * usize_to_f64(w.len()) * scale_of_t;
        let mut next = k.clone();
        next.alpha.push(alpha);
        if breakdown || next.alpha.len() >= self.capacity {
            next.exhausted = true;
            return Ok(next);
        }
        next.beta.push(beta);
        next.q.push(Rc::from(scale(&w, beta.recip())));
        next.mq.push(Rc::from(scale(&mw, beta.recip())));
        Ok(next)
    }

    /// Largest relative Ritz residual estimate `|β s_last| / |θ|` over the wanted pairs; zero on an invariant
    /// subspace; the largest finite `f64` until enough Ritz values exist.
    fn residual(&self, k: &Krylov) -> Result<NotNan<f64>, CombinatorRefuse> {
        let not_yet = NotNan::new(f64::MAX).map_err(|_| CombinatorRefuse::NonFiniteQuantity)?;
        let m = k.alpha.len();
        if m < self.wanted {
            return if k.exhausted {
                NotNan::new(0.0).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
            } else {
                Ok(not_yet)
            };
        }
        let (theta, s) = tridiagonal_eigen(&k.alpha, &k.beta[..m - 1])
            .map_err(|_| CombinatorRefuse::NonFiniteQuantity)?;
        let tail = if k.exhausted || k.beta.len() < m {
            0.0
        } else {
            k.beta[m - 1]
        };
        let worst = (0..self.wanted).fold(0.0_f64, |worst, r| {
            let i = m - 1 - r;
            let est = (tail * s[m - 1][i]).abs() / theta[i].abs().max(f64::MIN_POSITIVE);
            worst.max(est)
        });
        NotNan::new(worst).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
    }
}

/// Lowest `request.wanted` eigenpairs of `(K, M)` with certificates, as a budgeted unfold.
///
/// The progress window is twice the number wanted: Lanczos with a spectral transformation typically converges
/// the `p` extreme pairs within about `2p` steps (Parlett 1998 §13.4), so a longer flat stretch is a stall.
///
/// # Errors
/// [`EigenRefuse`] when `M` is not positive definite, `σ` is not below the spectrum, the request or deflation is
/// malformed, a factorisation refuses, or the unfold refuses.
pub fn lowest_eigenpairs<Meter: StepEnergyMeter>(
    pencil: Pencil<'_>,
    request: &EigenRequest,
    budget: EnergyBudget,
    meter: &Meter,
) -> Result<SolveOutcome<GeneralizedEigenSolution>, EigenRefuse> {
    let n = pencil.k.n();
    if pencil.m.n() != n {
        return Err(ProfileRefuse::DimMismatch.into());
    }
    let mfac = spd_factor(pencil.m).map_err(|e| match e {
        ProfileRefuse::NotPositiveDefinite { row } | ProfileRefuse::NearZeroPivot { row } => {
            EigenRefuse::MassNotPositiveDefinite { row }
        }
        other => EigenRefuse::Profile(other),
    })?;
    let shifted = ldlt(&pencil.k.combine(1.0, pencil.m, -request.shift)?)?;
    let below = shifted.inertia().negative;
    if below > 0 {
        return Err(EigenRefuse::ShiftNotBelowSpectrum { below });
    }
    let (z, mz) = deflation_basis(pencil.m, &request.deflation, n)?;
    let capacity = n - z.len();
    if request.wanted == 0 || request.wanted > capacity {
        return Err(EigenRefuse::WantedOutOfRange);
    }
    let ctx = Context {
        shifted,
        z,
        mz,
        m: pencil.m,
        wanted: request.wanted,
        capacity,
    };
    let start = start_vector(&ctx, n)?;
    let window_len = u32::try_from(request.wanted.saturating_mul(2)).unwrap_or(u32::MAX);
    let window = ProblemProgressWindow::from_krylov_restart(window_len)?;
    // One Lanczos step erases the bits of one new basis vector.
    let bits = usize_to_f64(n) * f64::from(f64::MANTISSA_DIGITS);
    let outcome = unfold(
        start,
        |k| ctx.residual(k),
        |k, _rung| ctx.step(k),
        UnfoldStop::new(request.tolerance, budget, window, bits)?,
        meter,
    )?;
    let finish = |k: Krylov| finalise(&ctx, pencil, &mfac, request, &k);
    Ok(match outcome {
        SolveOutcome::Converged { x, certificate } => SolveOutcome::Converged {
            x: finish(x)?,
            certificate,
        },
        SolveOutcome::Stalled { best, evidence } => SolveOutcome::Stalled {
            best: finish(best)?,
            evidence,
        },
        SolveOutcome::BudgetSpent {
            best,
            spent,
            progress_certificate,
        } => SolveOutcome::BudgetSpent {
            best: finish(best)?,
            spent,
            progress_certificate,
        },
    })
}

/// `M`-orthonormalise the deflation vectors (twice) and return them with their `M` images.
fn deflation_basis(
    m: &SymmetricProfile,
    deflation: &[Deflation],
    n: usize,
) -> Result<(Basis, Basis), EigenRefuse> {
    deflation.iter().try_fold(
        (Vec::new(), Vec::new()),
        |(mut z, mut mz): (Basis, Basis), d| {
            if d.vector.len() != n || d.vector.iter().any(|v| !v.is_finite()) {
                return Err(EigenRefuse::DeflationInvalid);
            }
            let norm0 = dot(&d.vector, &m.mul(&d.vector)?).max(0.0).sqrt();
            let w = orthogonalise(orthogonalise(d.vector.clone(), &z, &mz), &z, &mz);
            let mw = m.mul(&w)?;
            let norm = dot(&w, &mw).max(0.0).sqrt();
            // A vector that loses all but rounding of its M-norm to the others is dependent on them.
            if norm <= f64::EPSILON.sqrt() * norm0 || norm == 0.0 {
                return Err(EigenRefuse::DeflationInvalid);
            }
            z.push(Rc::from(scale(&w, norm.recip())));
            mz.push(Rc::from(scale(&mw, norm.recip())));
            Ok((z, mz))
        },
    )
}

/// A deterministic start: the low-discrepancy sequence `frac(i·φ)` with `φ` the golden ratio conjugate, pushed
/// through `A` once and orthogonalised against the deflated space, so no eigenvector is favoured by structure.
fn start_vector(ctx: &Context<'_>, n: usize) -> Result<Krylov, EigenRefuse> {
    let five = usize_to_f64(5);
    let phi = (five.sqrt() - 1.0) / usize_to_f64(2);
    let raw: Vec<f64> = (0..n)
        .map(|i| (usize_to_f64(i + 1) * phi).fract())
        .collect();
    let v = ctx.shifted.solve(&ctx.m.mul(&raw)?)?;
    let v = orthogonalise(orthogonalise(v, &ctx.z, &ctx.mz), &ctx.z, &ctx.mz);
    let mv = ctx.m.mul(&v)?;
    let norm = dot(&v, &mv).max(0.0).sqrt();
    if norm == 0.0 || !norm.is_finite() {
        return Err(EigenRefuse::DeflationInvalid);
    }
    Ok(Krylov {
        q: vec![Rc::from(scale(&v, norm.recip()))],
        mq: vec![Rc::from(scale(&mv, norm.recip()))],
        alpha: Vec::new(),
        beta: Vec::new(),
        exhausted: false,
    })
}

/// Rayleigh quotient `ρ` and the certificate radius of an `M`-normalised `x`: `δ = ‖K x − ρ M x‖_{M⁻¹}` plus the
/// floating-point allowance `η = γ_k (|x|ᵀ|K||x| + |ρ| |x|ᵀ|M||x|) / xᵀMx`, `γ_k = k u / (1 − k u)` with
/// `k = n + w` (dot length plus the widest row) and `u` the unit roundoff (Higham, *Accuracy and Stability of
/// Numerical Algorithms*, 2nd ed., 2002, §3.1). The allowance covers the rounding of the computed `ρ`, so an
/// exact eigenpair yields an interval that still contains its eigenvalue.
fn rayleigh(
    pencil: Pencil<'_>,
    mfac: &SpdFactor,
    x: &[f64],
) -> Result<(f64, f64, f64), EigenRefuse> {
    let kx = pencil.k.mul(x)?;
    let mx = pencil.m.mul(x)?;
    let xmx = dot(x, &mx);
    let rho = dot(x, &kx) / xmx;
    let r = axpy(&kx, -rho, &mx);
    let delta = (mfac.inverse_norm_sq(&r)? / xmx).sqrt();
    let k = usize_to_f64(x.len() + pencil.k.max_row_width().max(pencil.m.max_row_width()));
    let u = f64::EPSILON / usize_to_f64(2);
    let gamma = k * u / (1.0 - k * u);
    let eta = gamma * (pencil.k.abs_quadratic(x) + rho.abs() * pencil.m.abs_quadratic(x)) / xmx;
    Ok((rho, delta, eta))
}

fn finalise(
    ctx: &Context<'_>,
    pencil: Pencil<'_>,
    mfac: &SpdFactor,
    request: &EigenRequest,
    k: &Krylov,
) -> Result<GeneralizedEigenSolution, EigenRefuse> {
    let deflated = ctx
        .z
        .iter()
        .map(|z| {
            let (rho, delta, eta) = rayleigh(pencil, mfac, z)?;
            Ok(weinstein(rho, delta, eta, Rc::clone(z)))
        })
        .collect::<Result<Vec<_>, EigenRefuse>>()?;
    let m = k.alpha.len();
    if m == 0 {
        return Ok(GeneralizedEigenSolution {
            deflated,
            pairs: Vec::new(),
            completeness: Completeness::Unchecked,
            shortfall: request.wanted,
        });
    }
    let (theta, s) = tridiagonal_eigen(&k.alpha, &k.beta[..m - 1])?;
    // Largest θ first is lowest λ = σ + 1/θ; keep positive θ only (λ > σ).
    let order: Vec<usize> = (0..m).rev().filter(|&i| theta[i] > 0.0).collect();
    let ritz = |i: usize| -> Vec<f64> {
        (0..m).fold(vec![0.0_f64; pencil.k.n()], |x, row| {
            axpy(&x, s[row][i], &k.q[row])
        })
    };
    let take = request.wanted.min(order.len());
    let mut pairs = order[..take]
        .iter()
        .map(|&i| {
            let x = ritz(i);
            let (rho, delta, eta) = rayleigh(pencil, mfac, &x)?;
            Ok(weinstein(rho, delta, eta, Rc::from(x)))
        })
        .collect::<Result<Vec<_>, EigenRefuse>>()?;
    pairs.sort_by(|a, b| a.lambda.total_cmp(&b.lambda));
    let next_ritz = order.get(take).map(|&i| request.shift + theta[i].recip());
    let deflated = isolate(pencil, mfac, deflated)?;
    // Rayleigh–Ritz rotation can reorder a cluster's members; report every list lowest first.
    let mut deflated = deflated;
    deflated.sort_by(|a, b| a.lambda.total_cmp(&b.lambda));
    let mut pairs = isolate(pencil, mfac, pairs)?;
    pairs.sort_by(|a, b| a.lambda.total_cmp(&b.lambda));
    let disjoint = disjoint_groups(&deflated, &pairs);
    let completeness = if disjoint {
        sturm_certificate(pencil, &deflated, &pairs, next_ritz)
    } else {
        Completeness::Unchecked
    };
    let pairs = match completeness {
        Completeness::Certified { tau, .. } => kato_temple(&deflated, pairs, tau, request.shift),
        _ => pairs,
    };
    Ok(GeneralizedEigenSolution {
        deflated,
        shortfall: request.wanted - pairs.len(),
        pairs,
        completeness,
    })
}

fn weinstein(rho: f64, delta: f64, eta: f64, vector: Rc<[f64]>) -> CertifiedEigenpair {
    CertifiedEigenpair {
        lambda: rho,
        lower: rho - delta - eta,
        upper: rho + delta + eta,
        bound: EigenBound::Weinstein,
        residual: delta,
        rounding: eta,
        vector,
    }
}

/// Merge pairs whose intervals overlap into clusters until the clusters are disjoint. A cluster of `k` pairs is
/// replaced by the Rayleigh–Ritz pairs of its span, `M`-orthonormalised, and every member carries the cluster
/// interval `[min μ − r, max μ + r]` with `r = ‖K X − M X H‖_F / (1 − e) + max η`, `e` the measured
/// `M`-orthogonality defect of `X`. By Kahan's theorem (Parlett 1998, ch. 11) `k` eigenvalues of the pencil lie
/// within `‖R‖₂ ≤ ‖R‖_F` of the `k` values `μ`, so the cluster interval holds `k` eigenvalues counted with
/// multiplicity. Each merge removes a cluster boundary, so the merging ends after at most one pass per pair.
fn isolate(
    pencil: Pencil<'_>,
    mfac: &SpdFactor,
    pairs: Vec<CertifiedEigenpair>,
) -> Result<Vec<CertifiedEigenpair>, EigenRefuse> {
    let mut groups: Vec<Vec<CertifiedEigenpair>> = pairs.into_iter().map(|p| vec![p]).collect();
    loop {
        let before = groups.len();
        groups = groups
            .into_iter()
            .fold(Vec::new(), |mut acc: Vec<Vec<CertifiedEigenpair>>, g| {
                let overlaps = acc.last().is_some_and(|prev| {
                    let prev_hi = prev
                        .iter()
                        .map(|p| p.upper)
                        .fold(f64::NEG_INFINITY, f64::max);
                    let lo = g.iter().map(|p| p.lower).fold(f64::INFINITY, f64::min);
                    lo <= prev_hi
                });
                match acc.last_mut() {
                    Some(prev) if overlaps => prev.extend(g),
                    _ => acc.push(g),
                }
                acc
            });
        groups = groups
            .into_iter()
            .map(|g| {
                if g.len() > 1 {
                    cluster_bound(pencil, mfac, &g)
                } else {
                    Ok(g)
                }
            })
            .collect::<Result<_, _>>()?;
        if groups.len() == before {
            return Ok(groups.into_iter().flatten().collect());
        }
    }
}

fn cluster_bound(
    pencil: Pencil<'_>,
    mfac: &SpdFactor,
    group: &[CertifiedEigenpair],
) -> Result<Vec<CertifiedEigenpair>, EigenRefuse> {
    // M-orthonormalise the members (twice), then Rayleigh–Ritz on their span.
    let (x, mx) = group.iter().try_fold(
        (Vec::new(), Vec::new()),
        |(mut x, mut mx): (Basis, Basis), p| {
            let w = orthogonalise(orthogonalise(p.vector.to_vec(), &x, &mx), &x, &mx);
            let mw = pencil.m.mul(&w)?;
            let norm = dot(&w, &mw).max(0.0).sqrt();
            if norm == 0.0 || !norm.is_finite() {
                return Err(EigenRefuse::DeflationInvalid);
            }
            x.push(Rc::from(scale(&w, norm.recip())));
            mx.push(Rc::from(scale(&mw, norm.recip())));
            Ok((x, mx))
        },
    )?;
    let k = x.len();
    let kx: Vec<Vec<f64>> = x
        .iter()
        .map(|v| pencil.k.mul(v))
        .collect::<Result<_, _>>()?;
    // H = XᵀKX, symmetrised: the two triangles are rounded apart, and Jacobi needs an exactly symmetric matrix.
    let h: Vec<Vec<f64>> = (0..k)
        .map(|i| {
            (0..k)
                .map(|j| (dot(&x[i], &kx[j]) + dot(&x[j], &kx[i])) / usize_to_f64(2))
                .collect()
        })
        .collect();
    let defect = (0..k)
        .flat_map(|i| (0..k).map(move |j| (i, j)))
        .map(|(i, j)| (dot(&x[i], &mx[j]) - if i == j { 1.0 } else { 0.0 }).powi(2))
        .sum::<f64>()
        .sqrt();
    let (mu, v) = symmetric_eigen_small(&h)?;
    // R = K X − M X H, column by column, in the M⁻¹ norm.
    let r_f = (0..k)
        .map(|j| {
            let col: Vec<f64> = (0..kx[j].len())
                .map(|row| kx[j][row] - (0..k).map(|i| mx[i][row] * h[i][j]).sum::<f64>())
                .collect();
            mfac.inverse_norm_sq(&col)
        })
        .sum::<Result<f64, _>>()?
        .sqrt();
    let eta = group.iter().map(|p| p.rounding).fold(0.0_f64, f64::max);
    let radius = if defect < 1.0 {
        r_f / (1.0 - defect) + eta
    } else {
        f64::INFINITY
    };
    let lo = mu.iter().copied().fold(f64::INFINITY, f64::min) - radius;
    let hi = mu.iter().copied().fold(f64::NEG_INFINITY, f64::max) + radius;
    Ok((0..k)
        .map(|c| {
            let y = (0..k).fold(vec![0.0_f64; x[0].len()], |acc, i| {
                axpy(&acc, v[i][c], &x[i])
            });
            CertifiedEigenpair {
                lambda: mu[c],
                lower: lo,
                upper: hi,
                bound: EigenBound::Cluster,
                residual: r_f,
                rounding: eta,
                vector: Rc::from(y),
            }
        })
        .collect())
}

/// Whether the deflated and computed intervals, taken in order, never overlap except inside one cluster.
fn disjoint_groups(deflated: &[CertifiedEigenpair], pairs: &[CertifiedEigenpair]) -> bool {
    let all: Vec<&CertifiedEigenpair> = deflated.iter().chain(pairs).collect();
    all.windows(2).all(|w| {
        let same_cluster = w[0].bound == EigenBound::Cluster
            && w[1].bound == EigenBound::Cluster
            && w[0].lower == w[1].lower;
        same_cluster || w[1].lower > w[0].upper
    })
}

/// Eigenvalues and eigenvectors (columns) of a small dense symmetric matrix by cyclic Jacobi rotations, one sweep
/// per bit of `f64` precision at most (Jacobi converges quadratically once the off-diagonal is small).
///
/// # Errors
/// [`EigenRefuse::TridiagonalNoConvergence`] when the sweeps run out.
fn symmetric_eigen_small(a: &[Vec<f64>]) -> Result<(Vec<f64>, Vec<Vec<f64>>), EigenRefuse> {
    let n = a.len();
    let mut a: Vec<Vec<f64>> = a.to_vec();
    let mut v: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    // Converged when every off-diagonal entry is below rounding of the whole matrix (its Frobenius norm).
    let frobenius = a.iter().flatten().map(|x| x * x).sum::<f64>().sqrt();
    let floor = f64::EPSILON * frobenius;
    for _ in 0..f64::MANTISSA_DIGITS {
        let off = (0..n)
            .flat_map(|i| (0..n).filter(move |&j| j != i).map(move |j| (i, j)))
            .map(|(i, j)| a[i][j].abs())
            .fold(0.0, f64::max);
        if off <= floor {
            return Ok(((0..n).map(|i| a[i][i]).collect(), v));
        }
        for p in 0..n {
            for q in p + 1..n {
                if a[p][q].abs() <= floor {
                    continue;
                }
                let tau = (a[q][q] - a[p][p]) / (a[p][q] + a[p][q]);
                let t = tau.signum() / (tau.abs() + (1.0 + tau * tau).sqrt());
                let t = if tau == 0.0 { 1.0 } else { t };
                let c = (1.0 + t * t).sqrt().recip();
                let s = t * c;
                for row in &mut a {
                    let (akp, akq) = (row[p], row[q]);
                    row[p] = c * akp - s * akq;
                    row[q] = s * akp + c * akq;
                }
                for k in 0..n {
                    let (apk, aqk) = (a[p][k], a[q][k]);
                    a[p][k] = c * apk - s * aqk;
                    a[q][k] = s * apk + c * aqk;
                }
                // The rotation annihilates a_pq; set it to zero so rounding cannot hold the sweep above its floor.
                a[p][q] = 0.0;
                a[q][p] = 0.0;
                for row in &mut v {
                    let (vp, vq) = (row[p], row[q]);
                    row[p] = c * vp - s * vq;
                    row[q] = s * vp + c * vq;
                }
            }
        }
    }
    Err(EigenRefuse::TridiagonalNoConvergence)
}

/// Count eigenvalues below a shift between the last interval and the next Ritz value. Tries the midpoint and, on
/// a near-zero pivot, the lower third of the gap. Called only when the intervals are disjoint clusters, so each
/// interval holds as many eigenvalues as it has members.
fn sturm_certificate(
    pencil: Pencil<'_>,
    deflated: &[CertifiedEigenpair],
    pairs: &[CertifiedEigenpair],
    next_ritz: Option<f64>,
) -> Completeness {
    let Some(last) = pairs.last() else {
        return Completeness::Unchecked;
    };
    let hi = last.upper;
    let next = next_ritz.unwrap_or(hi + (hi - last.lower).max(hi.abs() * f64::EPSILON.sqrt()));
    if next <= hi {
        return Completeness::Unchecked;
    }
    let found = deflated.len() + pairs.len();
    let gap = next - hi;
    let three = usize_to_f64(3);
    [gap / usize_to_f64(2), gap / three]
        .iter()
        .find_map(|offset| {
            let tau = hi + offset;
            let shifted = pencil.k.combine(1.0, pencil.m, -tau).ok()?;
            let below = ldlt(&shifted).ok()?.inertia().negative;
            Some(if below == found {
                Completeness::Certified { tau, below }
            } else {
                Completeness::Missed { tau, below, found }
            })
        })
        .unwrap_or(Completeness::Unchecked)
}

/// Tighten each isolated (non-cluster) interval with Kato–Temple, using the neighbouring intervals as the gap ends;
/// below the first, the shift `σ`, under which the entry check certified no eigenvalue. Valid only under a
/// certified count of disjoint clusters, which isolates each such eigenvalue. The theorem holds for the exact
/// Rayleigh quotient, within `η` of the computed one: `[ρ − η − d²/(b − ρ − η), ρ + η + d²/(ρ − η − a)]`,
/// `d = δ + η`.
fn kato_temple(
    deflated: &[CertifiedEigenpair],
    pairs: Vec<CertifiedEigenpair>,
    tau: f64,
    sigma: f64,
) -> Vec<CertifiedEigenpair> {
    let floor = deflated.iter().map(|d| d.upper).fold(sigma, f64::max);
    let uppers: Vec<f64> = pairs.iter().map(|p| p.upper).collect();
    let lowers: Vec<f64> = pairs.iter().map(|p| p.lower).collect();
    pairs
        .into_iter()
        .enumerate()
        .map(|(i, p)| {
            if p.bound == EigenBound::Cluster {
                return p;
            }
            let a = if i == 0 { floor } else { uppers[i - 1] };
            let b = lowers.get(i + 1).copied().unwrap_or(tau);
            let (lo_rho, hi_rho) = (p.lambda - p.rounding, p.lambda + p.rounding);
            let d = p.residual + p.rounding;
            if !(a < lo_rho && hi_rho < b) {
                return p;
            }
            let lower = (lo_rho - d * d / (b - hi_rho)).max(p.lower);
            let upper = (hi_rho + d * d / (lo_rho - a)).min(p.upper);
            CertifiedEigenpair {
                lower,
                upper,
                bound: EigenBound::KatoTemple,
                ..p
            }
        })
        .collect()
}

/// Eigenvalues (ascending) and eigenvectors (columns of the returned row-major matrix) of the symmetric
/// tridiagonal matrix with diagonal `alpha` and off-diagonal `beta`, by the implicit QL method with Wilkinson-type
/// shifts (Bowdler, Martin, Reinsch & Wilkinson, *Numer. Math.* 11 (1968) 293–306).
///
/// Each eigenvalue is allowed one sweep per bit of `f64` precision: shifted QL converges at least linearly and in
/// practice cubically, so a longer run is a failure, not slow progress.
///
/// # Errors
/// [`EigenRefuse::TridiagonalNoConvergence`] when an eigenvalue exceeds its sweeps, or a length mismatch.
pub fn tridiagonal_eigen(
    alpha: &[f64],
    beta: &[f64],
) -> Result<(Vec<f64>, Vec<Vec<f64>>), EigenRefuse> {
    let n = alpha.len();
    if n == 0 || beta.len() + 1 != n {
        return Err(EigenRefuse::Profile(ProfileRefuse::DimMismatch));
    }
    let mut d = alpha.to_vec();
    let mut e: Vec<f64> = beta.iter().copied().chain(std::iter::once(0.0)).collect();
    let mut v: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    let sweeps = f64::MANTISSA_DIGITS;
    let mut f = 0.0_f64;
    let mut tst1 = 0.0_f64;
    for l in 0..n {
        tst1 = tst1.max(d[l].abs() + e[l].abs());
        let m = (l..n)
            .find(|&m| e[m].abs() <= f64::EPSILON * tst1)
            .unwrap_or(n - 1);
        if m > l {
            let mut iter = 0_u32;
            loop {
                iter += 1;
                if iter > sweeps {
                    return Err(EigenRefuse::TridiagonalNoConvergence);
                }
                let g = d[l];
                let p = (d[l + 1] - g) / (e[l] + e[l]);
                let r = p.hypot(1.0);
                let r = if p < 0.0 { -r } else { r };
                d[l] = e[l] / (p + r);
                d[l + 1] = e[l] * (p + r);
                let dl1 = d[l + 1];
                let h = g - d[l];
                d.iter_mut().skip(l + 2).for_each(|di| *di -= h);
                f += h;
                let mut p = d[m];
                let (mut c, mut c2, mut c3) = (1.0_f64, 1.0_f64, 1.0_f64);
                let el1 = e[l + 1];
                let (mut s, mut s2) = (0.0_f64, 0.0_f64);
                for i in (l..m).rev() {
                    c3 = c2;
                    c2 = c;
                    s2 = s;
                    let g = c * e[i];
                    let h = c * p;
                    let r = p.hypot(e[i]);
                    e[i + 1] = s * r;
                    s = e[i] / r;
                    c = p / r;
                    p = c * d[i] - s * g;
                    d[i + 1] = h + s * (c * g + s * d[i]);
                    for row in &mut v {
                        let h = row[i + 1];
                        row[i + 1] = s * row[i] + c * h;
                        row[i] = c * row[i] - s * h;
                    }
                }
                p = -s * s2 * c3 * el1 * e[l] / dl1;
                e[l] = s * p;
                d[l] = c * p;
                if e[l].abs() <= f64::EPSILON * tst1 {
                    break;
                }
            }
        }
        d[l] += f;
        e[l] = 0.0;
    }
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| d[a].total_cmp(&d[b]));
    let values: Vec<f64> = idx.iter().map(|&i| d[i]).collect();
    let vectors: Vec<Vec<f64>> = v
        .iter()
        .map(|row| idx.iter().map(|&i| row[i]).collect())
        .collect();
    if values.iter().any(|x| !x.is_finite()) {
        return Err(EigenRefuse::TridiagonalNoConvergence);
    }
    Ok((values, vectors))
}
