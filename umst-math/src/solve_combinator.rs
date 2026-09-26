// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Coalgebraic solve combinator: a budgeted unfold with typed outcomes.
//!
//! Termination is the second-law budget, never a compiled iteration cap.
//! Each step posts at least `k_B · T · ln 2` joules per erased bit (UCRS Landauer
//! ledger increment). The result is one of [`SolveOutcome`] — never a silent
//! best-effort value.
//!
//! Proof anchors: `LandauerBound` / `idealResetErasure`; Gate `clausiusDuhemFwd`.
//! DOI: 10.5281/zenodo.19159660

use crate::landauer::K_B;
use ordered_float::NotNan;

/// Why the combinator refuses to start or continue (not a solve outcome).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombinatorRefuse {
    /// A quantity was NaN or infinite.
    NonFiniteQuantity,
    /// Operator/problem budget was not strictly positive.
    NonPositiveBudget,
    /// Temperature was not strictly positive.
    NonPositiveTemperature,
    /// Problem-derived tolerance was not strictly positive.
    NonPositiveTolerance,
    /// Bits erased per step was not strictly positive (would stall the measure).
    NonPositiveBits,
}

/// Remaining energy, set by the operator or the problem. Never a compiled cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyBudget {
    joules: NotNan<f64>,
    temperature_k: NotNan<f64>,
}

impl EnergyBudget {
    /// Build a live budget. Both arguments must be finite and strictly positive.
    pub fn from_joules_at(
        joules: f64,
        temperature_k: f64,
    ) -> Result<Self, CombinatorRefuse> {
        let joules = finite_positive(joules, CombinatorRefuse::NonPositiveBudget)?;
        let temperature_k =
            finite_positive(temperature_k, CombinatorRefuse::NonPositiveTemperature)?;
        Ok(Self {
            joules,
            temperature_k,
        })
    }

    /// Joules still available.
    pub fn remaining_joules(self) -> NotNan<f64> {
        self.joules
    }

    /// Absolute temperature used for the Landauer floor (kelvin).
    pub fn temperature_k(self) -> NotNan<f64> {
        self.temperature_k
    }

    fn try_debit(self, cost: EnergySpent) -> Result<Self, CombinatorRefuse> {
        let next = self.joules.into_inner() - cost.joules;
        if next < 0.0 {
            return Err(CombinatorRefuse::NonPositiveBudget);
        }
        let joules = finite_non_negative(next)?;
        Ok(Self {
            joules,
            temperature_k: self.temperature_k,
        })
    }
}

/// Energy already spent. Commutative monoid under addition.
///
/// `joules` is finite and ≥ 0 by construction (`Eq` is the constructor invariant).
#[derive(Clone, Copy, Debug)]
pub struct EnergySpent {
    joules: f64,
}

impl PartialEq for EnergySpent {
    fn eq(&self, other: &Self) -> bool {
        self.joules == other.joules
    }
}

impl Eq for EnergySpent {}

impl EnergySpent {
    /// Additive identity (Defined 0 J).
    pub fn zero() -> Self {
        Self { joules: 0.0 }
    }

    fn from_finite_non_negative(joules: f64) -> Result<Self, CombinatorRefuse> {
        let nn = finite_non_negative(joules)?;
        Ok(Self {
            joules: nn.into_inner(),
        })
    }

    /// Joules recorded on the UCRS Landauer ledger for this spend.
    pub fn ucrs_ledger_joules(self) -> Result<NotNan<f64>, CombinatorRefuse> {
        finite_non_negative(self.joules)
    }

    /// Monoid operation. Parallel composition adds spends.
    pub fn plus(self, other: Self) -> Result<Self, CombinatorRefuse> {
        Self::from_finite_non_negative(self.joules + other.joules)
    }
}

/// Residual tolerance derived from the problem (conditioning × data uncertainty).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProblemTolerance {
    residual: NotNan<f64>,
}

impl ProblemTolerance {
    /// `tolerance = conditioning × data_uncertainty`. Neither factor may be non-positive.
    pub fn from_problem(
        conditioning: f64,
        data_uncertainty: f64,
    ) -> Result<Self, CombinatorRefuse> {
        let c = finite_positive(conditioning, CombinatorRefuse::NonPositiveTolerance)?;
        let u = finite_positive(data_uncertainty, CombinatorRefuse::NonPositiveTolerance)?;
        let residual = finite_positive(
            c.into_inner() * u.into_inner(),
            CombinatorRefuse::NonPositiveTolerance,
        )?;
        Ok(Self { residual })
    }

    /// Derived residual target.
    pub fn residual(self) -> NotNan<f64> {
        self.residual
    }
}

/// A-posteriori residual versus the problem-derived tolerance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidualCertificate {
    /// Measured residual (floats compute; exact/ball decide at N4).
    pub residual: NotNan<f64>,
    /// Problem-derived target.
    pub tolerance: ProblemTolerance,
    /// Whether `residual ≤ tolerance`.
    pub meets: bool,
}

impl ResidualCertificate {
    /// Compare a residual to the problem tolerance.
    pub fn of(residual: NotNan<f64>, tolerance: ProblemTolerance) -> Self {
        Self {
            residual,
            tolerance,
            meets: residual <= tolerance.residual,
        }
    }
}

/// Evidence that the residual stopped decreasing on the last lattice rung.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StallEvidence {
    /// Residual that failed to decrease.
    pub residual: NotNan<f64>,
    /// Residual before the stalled step.
    pub prior_residual: NotNan<f64>,
    /// Rung at which the lattice was exhausted.
    pub rung: StrategyRung,
}

/// Progress recorded when the budget is exhausted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressCertificate {
    /// Best residual reached.
    pub residual: NotNan<f64>,
    /// Energy posted to the UCRS ledger.
    pub spent: EnergySpent,
}

/// Typed solve result. Never an unconverged value dressed as an answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolveOutcome<X> {
    /// Problem-derived tolerance met, with a residual certificate.
    Converged {
        /// Certified state.
        x: X,
        /// Residual certificate.
        certificate: ResidualCertificate,
    },
    /// Residual stopped decreasing under every available strategy.
    Stalled {
        /// Best state reached.
        best: X,
        /// Stall evidence.
        evidence: StallEvidence,
    },
    /// Operator/problem energy budget exhausted.
    BudgetSpent {
        /// Best state reached.
        best: X,
        /// Energy posted to the UCRS ledger.
        spent: EnergySpent,
        /// Progress at exhaustion.
        progress_certificate: ProgressCertificate,
    },
}

/// Finite strategy lattice for linear solves (monotone ascent).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrategyRung {
    /// Point Jacobi.
    Jacobi,
    /// Block Jacobi.
    BlockJacobi,
    /// Algebraic multigrid.
    AlgebraicMultigrid,
    /// Geometric multigrid.
    GeometricMultigrid,
    /// Sparse direct (last rung).
    SparseDirect,
}

impl StrategyRung {
    /// Next rung, or `None` at the top of the lattice.
    pub fn escalate(self) -> Option<Self> {
        match self {
            Self::Jacobi => Some(Self::BlockJacobi),
            Self::BlockJacobi => Some(Self::AlgebraicMultigrid),
            Self::AlgebraicMultigrid => Some(Self::GeometricMultigrid),
            Self::GeometricMultigrid => Some(Self::SparseDirect),
            Self::SparseDirect => None,
        }
    }
}

/// Success fragment of `Thermo a = Budget → Either Refusal (a, Spent)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thermo<A> {
    /// Carried value.
    pub value: A,
    /// Energy spent to produce it.
    pub spent: EnergySpent,
}

impl<A> Thermo<A> {
    /// Monad unit.
    pub fn unit(value: A) -> Self {
        Self {
            value,
            spent: EnergySpent::zero(),
        }
    }

    /// Kleisli bind: sequential composition threads the spend monoid.
    pub fn bind<B, F>(self, f: F) -> Result<Thermo<B>, CombinatorRefuse>
    where
        F: FnOnce(A) -> Result<Thermo<B>, CombinatorRefuse>,
    {
        let next = f(self.value)?;
        Ok(Thermo {
            value: next.value,
            spent: self.spent.plus(next.spent)?,
        })
    }
}

/// Landauer floor for `bits_erased` at temperature `T`: `k_B · T · ln 2 · bits`.
pub fn landauer_step_joules(
    temperature_k: NotNan<f64>,
    bits_erased: NotNan<f64>,
) -> Result<EnergySpent, CombinatorRefuse> {
    let bits = bits_erased.into_inner();
    if bits <= 0.0 {
        return Err(CombinatorRefuse::NonPositiveBits);
    }
    let t = temperature_k.into_inner();
    if t <= 0.0 {
        return Err(CombinatorRefuse::NonPositiveTemperature);
    }
    let j = K_B * t * core::f64::consts::LN_2 * bits;
    let _ = finite_positive(j, CombinatorRefuse::NonFiniteQuantity)?;
    EnergySpent::from_finite_non_negative(j)
}

/// Coalgebraic unfold. Stops only for Converged, Stalled (lattice exhausted),
/// or BudgetSpent. No iteration ceiling.
pub fn unfold<S, ResidualOf, Step>(
    initial: S,
    residual_of: ResidualOf,
    mut step: Step,
    tolerance: ProblemTolerance,
    budget: EnergyBudget,
    bits_per_step: f64,
) -> Result<SolveOutcome<S>, CombinatorRefuse>
where
    S: Clone,
    ResidualOf: Fn(&S) -> Result<NotNan<f64>, CombinatorRefuse>,
    Step: FnMut(&S, StrategyRung) -> Result<S, CombinatorRefuse>,
{
    let bits = finite_positive(bits_per_step, CombinatorRefuse::NonPositiveBits)?;
    let mut state = initial;
    let mut residual = residual_of(&state)?;
    let mut spent = EnergySpent::zero();
    let mut remaining = budget;
    let mut strategy = StrategyRung::Jacobi;
    let initial_cert = ResidualCertificate::of(residual, tolerance);
    if initial_cert.meets {
        return Ok(SolveOutcome::Converged {
            x: state,
            certificate: initial_cert,
        });
    }

    loop {
        let cost = landauer_step_joules(remaining.temperature_k(), bits)?;
        match remaining.try_debit(cost) {
            Ok(next_budget) => {
                remaining = next_budget;
                spent = spent.plus(cost)?;
            }
            Err(_) => {
                return Ok(SolveOutcome::BudgetSpent {
                    best: state,
                    spent,
                    progress_certificate: ProgressCertificate { residual, spent },
                });
            }
        }

        let next = step(&state, strategy)?;
        let next_residual = residual_of(&next)?;
        let cert = ResidualCertificate::of(next_residual, tolerance);
        if cert.meets {
            return Ok(SolveOutcome::Converged {
                x: next,
                certificate: cert,
            });
        }

        if next_residual < residual {
            state = next;
            residual = next_residual;
            continue;
        }

        match strategy.escalate() {
            Some(next_rung) => {
                strategy = next_rung;
            }
            None => {
                return Ok(SolveOutcome::Stalled {
                    best: state,
                    evidence: StallEvidence {
                        residual: next_residual,
                        prior_residual: residual,
                        rung: strategy,
                    },
                });
            }
        }
    }
}

fn finite_positive(x: f64, on_non_positive: CombinatorRefuse) -> Result<NotNan<f64>, CombinatorRefuse> {
    let nn = NotNan::new(x).map_err(|_| CombinatorRefuse::NonFiniteQuantity)?;
    if nn.into_inner() <= 0.0 {
        return Err(on_non_positive);
    }
    Ok(nn)
}

fn finite_non_negative(x: f64) -> Result<NotNan<f64>, CombinatorRefuse> {
    let nn = NotNan::new(x).map_err(|_| CombinatorRefuse::NonFiniteQuantity)?;
    if nn.into_inner() < 0.0 {
        return Err(CombinatorRefuse::NonPositiveBudget);
    }
    Ok(nn)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn nn(x: f64) -> NotNan<f64> {
        NotNan::new(x).expect("test finite")
    }

    fn abs_residual(x: &f64) -> Result<NotNan<f64>, CombinatorRefuse> {
        NotNan::new(x.abs()).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
    }

    #[test]
    fn energy_spent_is_commutative_monoid() {
        let a = EnergySpent { joules: 1.5 };
        let b = EnergySpent { joules: 2.5 };
        let c = EnergySpent { joules: 3.5 };
        let z = EnergySpent::zero();
        assert_eq!(a.plus(z).expect("a+0"), a);
        assert_eq!(z.plus(a).expect("0+a"), a);
        assert_eq!(a.plus(b).expect("a+b"), b.plus(a).expect("b+a"));
        let left = a.plus(b).expect("a+b").plus(c).expect("(a+b)+c");
        let right = a.plus(b.plus(c).expect("b+c")).expect("a+(b+c)");
        assert_eq!(left, right);
    }

    #[test]
    fn thermo_left_and_right_identity() {
        let m = Thermo::unit(7_i32);
        let right = m
            .clone()
            .bind(|x| Ok(Thermo::unit(x)))
            .expect("right id");
        assert_eq!(right, Thermo::unit(7));
        let left = Thermo::unit(7)
            .bind(|x| Ok(Thermo { value: x + 1, spent: EnergySpent::zero() }))
            .expect("left id");
        assert_eq!(left.value, 8);
        assert_eq!(left.spent, EnergySpent::zero());
    }

    #[test]
    fn thermo_bind_associative_and_adds_spend() {
        let spend = |j: f64| EnergySpent { joules: j };
        let f = |x: i32| {
            Ok(Thermo {
                value: x + 1,
                spent: spend(1.0),
            })
        };
        let g = |x: i32| {
            Ok(Thermo {
                value: x * 2,
                spent: spend(2.0),
            })
        };
        let m = Thermo {
            value: 3,
            spent: spend(4.0),
        };
        let left = m
            .clone()
            .bind(f)
            .expect("f")
            .bind(g)
            .expect("g");
        let right = m
            .bind(|x| f(x)?.bind(g))
            .expect("assoc");
        assert_eq!(left, right);
        assert_eq!(left.value, 8);
        assert_eq!(left.spent, spend(7.0));
    }

    #[test]
    fn landauer_step_strictly_decreases_budget() {
        let t = nn(293.15);
        let bits = nn(1.0);
        let cost = landauer_step_joules(t, bits).expect("cost");
        let budget = EnergyBudget::from_joules_at(1e-18, 293.15).expect("budget");
        let before = budget.remaining_joules();
        let after = budget.try_debit(cost).expect("debit");
        assert!(after.remaining_joules() < before);
        assert!(cost.ucrs_ledger_joules().expect("finite").into_inner() > 0.0);
    }

    #[test]
    fn unfold_converges_with_problem_derived_tolerance() {
        let tolerance = ProblemTolerance::from_problem(4.0, 1e-3).expect("tol");
        let budget = EnergyBudget::from_joules_at(1e-12, 293.15).expect("B");
        let out = unfold(
            1.0_f64,
            abs_residual,
            |x, _rung| Ok(x * 0.5),
            tolerance,
            budget,
            1.0,
        )
        .expect("unfold");
        match out {
            SolveOutcome::Converged { x, certificate } => {
                assert!(certificate.meets);
                assert!(x.abs() <= tolerance.residual().into_inner());
            }
            other => panic!("expected Converged, got {other:?}"),
        }
    }

    #[test]
    fn unfold_stalls_when_lattice_exhausted() {
        let tolerance = ProblemTolerance::from_problem(2.0, 1e-6).expect("tol");
        let budget = EnergyBudget::from_joules_at(1e-12, 293.15).expect("B");
        let out = unfold(
            1.0_f64,
            abs_residual,
            |x, _rung| Ok(*x),
            tolerance,
            budget,
            1.0,
        )
        .expect("unfold");
        match out {
            SolveOutcome::Stalled { best, evidence } => {
                assert_eq!(best, 1.0);
                assert_eq!(evidence.rung, StrategyRung::SparseDirect);
                assert_eq!(evidence.residual, nn(1.0));
            }
            other => panic!("expected Stalled, got {other:?}"),
        }
    }

    #[test]
    fn unfold_budget_spent_when_landauer_floor_exceeds_remaining() {
        let tolerance = ProblemTolerance::from_problem(2.0, 1e-6).expect("tol");
        let budget = EnergyBudget::from_joules_at(1e-40, 293.15).expect("tiny B");
        let out = unfold(
            1.0_f64,
            abs_residual,
            |x, _rung| Ok(x * 0.5),
            tolerance,
            budget,
            1.0,
        )
        .expect("unfold");
        match out {
            SolveOutcome::BudgetSpent {
                best,
                spent,
                progress_certificate,
            } => {
                assert_eq!(best, 1.0);
                assert_eq!(spent, EnergySpent::zero());
                assert_eq!(progress_certificate.residual, nn(1.0));
            }
            other => panic!("expected BudgetSpent, got {other:?}"),
        }
    }

    #[test]
    fn unfold_refuses_compiled_style_zero_bits() {
        let tolerance = ProblemTolerance::from_problem(2.0, 1e-3).expect("tol");
        let budget = EnergyBudget::from_joules_at(1e-12, 293.15).expect("B");
        let err = unfold(1.0_f64, abs_residual, |x, _| Ok(*x), tolerance, budget, 0.0)
            .expect_err("zero bits");
        assert_eq!(err, CombinatorRefuse::NonPositiveBits);
    }
}
