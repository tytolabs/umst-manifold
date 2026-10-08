// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Coalgebraic solve combinator: a budgeted unfold with typed outcomes.
//!
//! Termination is the second-law budget, never a compiled iteration cap.
//! Each step debits **measured** energy (elapsed × package power from a
//! UCRS/powermetrics reader). Landauer `k_B · T · ln 2 · bits` is a certified
//! floor: measured must be ≥ floor. No reader ⇒ [`CombinatorRefuse::Unmeasured`].
//! Stall is judged over a problem-derived window, not a single residual bump.
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
    /// No UCRS/powermetrics package-power reader is available.
    Unmeasured,
    /// Measured step energy was below the certified Landauer floor.
    MeasuredBelowFloor,
    /// Progress window length was not strictly positive.
    NonPositiveWindow,
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

/// Whether step energy was measured live or bounded above from elapsed × ceiling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySpendProvenance {
    /// Elapsed × package power from a reader or injected meter.
    Measured,
    /// Elapsed × operator-supplied package power ceiling (conservative upper bound).
    BoundedAbove,
}

/// Energy already spent. Commutative monoid under addition.
///
/// `joules` is finite and ≥ 0 by construction (`Eq` is the constructor invariant).
#[derive(Clone, Copy, Debug)]
pub struct EnergySpent {
    joules: f64,
    provenance: EnergySpendProvenance,
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
        Self {
            joules: 0.0,
            provenance: EnergySpendProvenance::Measured,
        }
    }

    /// How this spend was obtained.
    pub fn provenance(self) -> EnergySpendProvenance {
        self.provenance
    }

    fn from_finite_non_negative(joules: f64) -> Result<Self, CombinatorRefuse> {
        let nn = finite_non_negative(joules)?;
        Ok(Self {
            joules: nn.into_inner(),
            provenance: EnergySpendProvenance::Measured,
        })
    }

    fn from_bounded_finite_non_negative(joules: f64) -> Result<Self, CombinatorRefuse> {
        let nn = finite_non_negative(joules)?;
        Ok(Self {
            joules: nn.into_inner(),
            provenance: EnergySpendProvenance::BoundedAbove,
        })
    }

    /// Joules recorded on the UCRS Landauer ledger for this spend.
    pub fn ucrs_ledger_joules(self) -> Result<NotNan<f64>, CombinatorRefuse> {
        finite_non_negative(self.joules)
    }

    /// Monoid operation. Parallel composition adds spends.
    pub fn plus(self, other: Self) -> Result<Self, CombinatorRefuse> {
        let provenance = match (self.provenance, other.provenance) {
            (EnergySpendProvenance::Measured, EnergySpendProvenance::Measured) => {
                EnergySpendProvenance::Measured
            }
            _ => EnergySpendProvenance::BoundedAbove,
        };
        let nn = finite_non_negative(self.joules + other.joules)?;
        Ok(Self {
            joules: nn.into_inner(),
            provenance,
        })
    }

    fn is_below(self, floor: Self) -> bool {
        self.joules < floor.joules
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
    /// Residual before the stalled window.
    pub prior_residual: NotNan<f64>,
    /// Rung at which the lattice was exhausted.
    pub rung: StrategyRung,
    /// Problem-derived progress window that was exhausted.
    pub window: ProblemProgressWindow,
}

/// Consecutive non-improving steps allowed before stall/escalate (e.g. Krylov restart).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProblemProgressWindow {
    length: u32,
}

impl ProblemProgressWindow {
    /// Window from a Krylov restart length (or any problem-derived count ≥ 1).
    pub fn from_krylov_restart(restart_length: u32) -> Result<Self, CombinatorRefuse> {
        if restart_length == 0 {
            return Err(CombinatorRefuse::NonPositiveWindow);
        }
        Ok(Self {
            length: restart_length,
        })
    }

    /// Window length in steps.
    pub fn length(self) -> u32 {
        self.length
    }
}

/// Instantaneous package power from the UCRS / powermetrics reader.
pub trait PackagePowerReader {
    /// Package power in watts, or [`CombinatorRefuse::Unmeasured`].
    fn package_power_watts(&self) -> Result<NotNan<f64>, CombinatorRefuse>;
}

/// Measures the energy of one solver step. Never guesses.
pub trait StepEnergyMeter {
    /// Run `step` and return the value plus the measured spend.
    fn measure<F, T>(&self, step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<T, CombinatorRefuse>;
}

/// Typed absence of a package-power reader.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnmeasuredMeter {}

impl StepEnergyMeter for UnmeasuredMeter {
    fn measure<F, T>(&self, _step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<T, CombinatorRefuse>,
    {
        Err(CombinatorRefuse::Unmeasured)
    }
}

/// Test / injected meter that posts a known joule spend (not a live guess).
#[derive(Clone, Copy, Debug)]
pub struct FixedJouleMeter {
    joules: f64,
}

impl FixedJouleMeter {
    /// Finite non-negative joules posted for every measured step.
    pub fn from_joules(joules: f64) -> Result<Self, CombinatorRefuse> {
        let _ = finite_non_negative(joules)?;
        Ok(Self { joules })
    }
}

impl StepEnergyMeter for FixedJouleMeter {
    fn measure<F, T>(&self, step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<T, CombinatorRefuse>,
    {
        let value = step()?;
        Ok((value, EnergySpent::from_finite_non_negative(self.joules)?))
    }
}

/// Live meter: elapsed seconds × package watts from [`PackagePowerReader`].
#[derive(Clone, Copy, Debug)]
pub struct MeasuredPackageMeter<R> {
    reader: R,
}

impl<R: PackagePowerReader> MeasuredPackageMeter<R> {
    /// Bind a reader. Absence is [`UnmeasuredMeter`], not this type.
    pub fn from_reader(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: PackagePowerReader> StepEnergyMeter for MeasuredPackageMeter<R> {
    fn measure<F, T>(&self, step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<T, CombinatorRefuse>,
    {
        let watts = self.reader.package_power_watts()?;
        let t0 = std::time::Instant::now();
        let value = step()?;
        let joules = watts.into_inner() * t0.elapsed().as_secs_f64();
        Ok((value, EnergySpent::from_finite_non_negative(joules)?))
    }
}

/// Conservative meter: energy ≤ elapsed monotonic time × operator-supplied package ceiling.
///
/// Registry row [`solve_combinator_macos_package_power_ceiling_watts`] is unmeasured until
/// the operator passes watts from a `sudo powermetrics` sample. Without a ceiling use
/// [`UnmeasuredMeter`].
#[derive(Clone, Copy, Debug)]
pub struct BoundedPackageMeter {
    ceiling_watts: NotNan<f64>,
}

impl BoundedPackageMeter {
    /// Ceiling watts from an operator powermetrics sample (machine-specific; not invented here).
    pub fn from_operator_ceiling_watts(watts: f64) -> Result<Self, CombinatorRefuse> {
        let ceiling_watts = finite_positive(watts, CombinatorRefuse::NonFiniteQuantity)?;
        Ok(Self { ceiling_watts })
    }
}

impl StepEnergyMeter for BoundedPackageMeter {
    fn measure<F, T>(&self, step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<T, CombinatorRefuse>,
    {
        let t0 = std::time::Instant::now();
        let value = step()?;
        let joules = self.ceiling_watts.into_inner() * t0.elapsed().as_secs_f64();
        Ok((value, EnergySpent::from_bounded_finite_non_negative(joules)?))
    }
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

/// Stop policy of [`unfold`]: the residual target, the energy budget, the progress window
/// and the bits erased per step that set the Landauer floor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnfoldStop {
    tolerance: ProblemTolerance,
    budget: EnergyBudget,
    window: ProblemProgressWindow,
    bits_per_step: NotNan<f64>,
}

impl UnfoldStop {
    /// Bind the stop policy. `bits_per_step` must be finite and strictly positive.
    pub fn new(
        tolerance: ProblemTolerance,
        budget: EnergyBudget,
        window: ProblemProgressWindow,
        bits_per_step: f64,
    ) -> Result<Self, CombinatorRefuse> {
        let bits_per_step = finite_positive(bits_per_step, CombinatorRefuse::NonPositiveBits)?;
        Ok(Self {
            tolerance,
            budget,
            window,
            bits_per_step,
        })
    }

    /// Problem-derived residual target.
    pub fn tolerance(self) -> ProblemTolerance {
        self.tolerance
    }

    /// Energy available to the unfold.
    pub fn budget(self) -> EnergyBudget {
        self.budget
    }

    /// Consecutive non-improving steps allowed before escalation.
    pub fn window(self) -> ProblemProgressWindow {
        self.window
    }

    /// Bits erased per step (sets the Landauer floor).
    pub fn bits_per_step(self) -> NotNan<f64> {
        self.bits_per_step
    }
}

/// Coalgebraic unfold. Stops only for Converged, Stalled (lattice exhausted),
/// or BudgetSpent. No iteration ceiling. Debits measured energy, not Landauer×bits.
pub fn unfold<S, ResidualOf, Step, Meter>(
    initial: S,
    residual_of: ResidualOf,
    mut step: Step,
    stop: UnfoldStop,
    meter: &Meter,
) -> Result<SolveOutcome<S>, CombinatorRefuse>
where
    S: Clone,
    ResidualOf: Fn(&S) -> Result<NotNan<f64>, CombinatorRefuse>,
    Step: FnMut(&S, StrategyRung) -> Result<S, CombinatorRefuse>,
    Meter: StepEnergyMeter,
{
    let UnfoldStop {
        tolerance,
        budget,
        window,
        bits_per_step: bits,
    } = stop;
    let mut state = initial;
    let mut residual = residual_of(&state)?;
    let mut best = state.clone();
    let mut best_residual = residual;
    let mut spent = EnergySpent::zero();
    let mut remaining = budget;
    let mut strategy = StrategyRung::Jacobi;
    let mut stale = 0_u32;
    let initial_cert = ResidualCertificate::of(residual, tolerance);
    if initial_cert.meets {
        return Ok(SolveOutcome::Converged {
            x: state,
            certificate: initial_cert,
        });
    }

    loop {
        let floor = landauer_step_joules(remaining.temperature_k(), bits)?;

        let (next, cost) = meter.measure(|| step(&state, strategy))?;
        if cost.is_below(floor) {
            return Err(CombinatorRefuse::MeasuredBelowFloor);
        }
        match remaining.try_debit(cost) {
            Ok(next_budget) => {
                remaining = next_budget;
                spent = spent.plus(cost)?;
            }
            Err(_) => {
                return Ok(SolveOutcome::BudgetSpent {
                    best,
                    spent,
                    progress_certificate: ProgressCertificate {
                        residual: best_residual,
                        spent,
                    },
                });
            }
        }

        state = next;
        residual = residual_of(&state)?;
        let cert = ResidualCertificate::of(residual, tolerance);
        if cert.meets {
            return Ok(SolveOutcome::Converged {
                x: state,
                certificate: cert,
            });
        }

        if residual < best_residual {
            best = state.clone();
            best_residual = residual;
            stale = 0;
            continue;
        }

        stale = stale.saturating_add(1);
        if stale < window.length() {
            continue;
        }
        stale = 0;
        match strategy.escalate() {
            Some(next_rung) => {
                strategy = next_rung;
            }
            None => {
                return Ok(SolveOutcome::Stalled {
                    best,
                    evidence: StallEvidence {
                        residual,
                        prior_residual: best_residual,
                        rung: strategy,
                        window,
                    },
                });
            }
        }
    }
}

fn finite_positive(x: f64, on_non_positive: CombinatorRefuse) -> Result<NotNan<f64>, CombinatorRefuse> {
    let nn = finite(x)?;
    if nn.into_inner() <= 0.0 {
        return Err(on_non_positive);
    }
    Ok(nn)
}

fn finite_non_negative(x: f64) -> Result<NotNan<f64>, CombinatorRefuse> {
    let nn = finite(x)?;
    if nn.into_inner() < 0.0 {
        return Err(CombinatorRefuse::NonPositiveBudget);
    }
    Ok(nn)
}

/// `NotNan` admits ±∞; a combinator quantity must be finite.
fn finite(x: f64) -> Result<NotNan<f64>, CombinatorRefuse> {
    if !x.is_finite() {
        return Err(CombinatorRefuse::NonFiniteQuantity);
    }
    NotNan::new(x).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
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

    fn paid_meter() -> FixedJouleMeter {
        FixedJouleMeter::from_joules(1e-18).expect("above floor")
    }

    fn one_step_window() -> ProblemProgressWindow {
        ProblemProgressWindow::from_krylov_restart(1).expect("window")
    }

    fn measured_joules(j: f64) -> EnergySpent {
        EnergySpent::from_finite_non_negative(j).expect("test joules")
    }

    #[test]
    fn energy_spent_is_commutative_monoid() {
        let a = measured_joules(1.5);
        let b = measured_joules(2.5);
        let c = measured_joules(3.5);
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
        let spend = measured_joules;
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
            UnfoldStop::new(tolerance, budget, one_step_window(), 1.0)
                .expect("stop"),
            &paid_meter(),
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
            UnfoldStop::new(tolerance, budget, one_step_window(), 1.0)
                .expect("stop"),
            &paid_meter(),
        )
        .expect("unfold");
        match out {
            SolveOutcome::Stalled { best, evidence } => {
                assert_eq!(best, 1.0);
                assert_eq!(evidence.rung, StrategyRung::SparseDirect);
                assert_eq!(evidence.residual, nn(1.0));
                assert_eq!(evidence.window.length(), 1);
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
            UnfoldStop::new(tolerance, budget, one_step_window(), 1.0)
                .expect("stop"),
            &paid_meter(),
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
        let err = UnfoldStop::new(tolerance, budget, one_step_window(), 0.0)
            .expect_err("zero bits");
        assert_eq!(err, CombinatorRefuse::NonPositiveBits);
    }

    #[test]
    fn infinite_quantities_refused_as_non_finite() {
        let tolerance = ProblemTolerance::from_problem(2.0, 1e-3).expect("tol");
        let budget = EnergyBudget::from_joules_at(1e-12, 293.15).expect("B");
        assert_eq!(
            UnfoldStop::new(tolerance, budget, one_step_window(), f64::INFINITY),
            Err(CombinatorRefuse::NonFiniteQuantity)
        );
        assert_eq!(
            EnergyBudget::from_joules_at(f64::INFINITY, 293.15),
            Err(CombinatorRefuse::NonFiniteQuantity)
        );
        assert_eq!(
            ProblemTolerance::from_problem(f64::INFINITY, 1e-3),
            Err(CombinatorRefuse::NonFiniteQuantity)
        );
        assert!(FixedJouleMeter::from_joules(f64::INFINITY).is_err());
    }

    #[test]
    fn unfold_refuses_unmeasured_when_no_reader() {
        let tolerance = ProblemTolerance::from_problem(2.0, 1e-3).expect("tol");
        let budget = EnergyBudget::from_joules_at(1e-12, 293.15).expect("B");
        let err = unfold(
            1.0_f64,
            abs_residual,
            |x, _| Ok(x * 0.5),
            UnfoldStop::new(tolerance, budget, one_step_window(), 1.0)
                .expect("stop"),
            &UnmeasuredMeter {},
        )
        .expect_err("unmeasured");
        assert_eq!(err, CombinatorRefuse::Unmeasured);
    }

    #[test]
    fn unfold_debits_measured_joules_not_landauer_times_bits() {
        let tolerance = ProblemTolerance::from_problem(2.0, 1e-3).expect("tol");
        let meter = FixedJouleMeter::from_joules(3.0).expect("3 J");
        let floor = landauer_step_joules(nn(293.15), nn(1.0)).expect("floor");
        assert!(floor.ucrs_ledger_joules().expect("finite").into_inner() < 1e-15);
        // One debit per measured step (Landauer is a floor check only). Identity stall needs
        // 5 rungs × 3 J = 15 J; budget 10 J allows 3 full debits (9 J) then BudgetSpent on the 4th.
        let budget_tight = EnergyBudget::from_joules_at(10.0, 293.15).expect("B");
        let out = unfold(
            1.0_f64,
            abs_residual,
            |x, _| Ok(*x),
            UnfoldStop::new(tolerance, budget_tight, one_step_window(), 1.0)
                .expect("stop"),
            &meter,
        )
        .expect("unfold");
        match out {
            SolveOutcome::BudgetSpent { spent, .. } => {
                assert_eq!(
                    spent.ucrs_ledger_joules().expect("finite").into_inner(),
                    9.0
                );
                assert_eq!(spent.provenance(), EnergySpendProvenance::Measured);
            }
            other => panic!("expected BudgetSpent for 10 J / 3 J per step, got {other:?}"),
        }
        let posts = std::cell::Cell::new(0_u32);
        let counting = CountingMeter {
            inner: meter,
            posts: &posts,
        };
        let budget_stall = EnergyBudget::from_joules_at(16.0, 293.15).expect("B stall");
        let out = unfold(
            1.0_f64,
            abs_residual,
            |x, _| Ok(*x),
            UnfoldStop::new(tolerance, budget_stall, one_step_window(), 1.0)
                .expect("stop"),
            &counting,
        )
        .expect("count");
        match out {
            SolveOutcome::Stalled { .. } => {
                assert_eq!(posts.get(), 5);
                assert!(3.0 * f64::from(posts.get()) > floor.ucrs_ledger_joules().expect("f").into_inner() * 1e10);
            }
            other => panic!("expected Stalled after 5×3 J with 16 J budget, got {other:?}"),
        }
    }

    #[test]
    fn unfold_refuses_measured_below_landauer_floor() {
        let tolerance = ProblemTolerance::from_problem(2.0, 1e-3).expect("tol");
        let budget = EnergyBudget::from_joules_at(1e-12, 293.15).expect("B");
        let err = unfold(
            1.0_f64,
            abs_residual,
            |x, _| Ok(x * 0.5),
            UnfoldStop::new(tolerance, budget, one_step_window(), 1.0)
                .expect("stop"),
            &FixedJouleMeter::from_joules(1e-40).expect("tiny"),
        )
        .expect_err("below floor");
        assert_eq!(err, CombinatorRefuse::MeasuredBelowFloor);
    }

    const NON_MONO_RESIDUALS: [f64; 4] = [1.0, 1.3, 0.8, 0.4];

    fn residual_at_index(idx: &usize) -> Result<NotNan<f64>, CombinatorRefuse> {
        let r = NON_MONO_RESIDUALS
            .get(*idx)
            .copied()
            .ok_or(CombinatorRefuse::NonFiniteQuantity)?;
        NotNan::new(r).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
    }

    fn advance_index(idx: &usize, _rung: StrategyRung) -> Result<usize, CombinatorRefuse> {
        if *idx + 1 >= NON_MONO_RESIDUALS.len() {
            Ok(*idx)
        } else {
            Ok(idx.saturating_add(1))
        }
    }

    #[test]
    fn unfold_non_monotone_sequence_converges_with_window_two() {
        // Pre: residuals [1.0, 1.3, 0.8, 0.4]; tolerance 0.5 so 0.4 converges; window 2 tolerates one bump.
        let tolerance = ProblemTolerance::from_problem(1.0, 0.5).expect("tol");
        assert!(tolerance.residual().into_inner() >= 0.4);
        let budget = EnergyBudget::from_joules_at(1e-12, 293.15).expect("B");
        let window = ProblemProgressWindow::from_krylov_restart(2).expect("restart 2");
        let out = unfold(
            0_usize,
            residual_at_index,
            advance_index,
            UnfoldStop::new(tolerance, budget, window, 1.0)
                .expect("stop"),
            &paid_meter(),
        )
        .expect("unfold");
        match out {
            SolveOutcome::Converged { x, certificate } => {
                assert_eq!(x, 3);
                assert!(certificate.meets);
                assert_eq!(certificate.residual, nn(0.4));
            }
            other => panic!("expected Converged, got {other:?}"),
        }
    }

    #[test]
    fn unfold_non_monotone_sequence_escalates_with_window_one() {
        // Pre: same sequence; window 1 escalates on the 1.3 bump (state still advances to index 1).
        let tolerance = ProblemTolerance::from_problem(1.0, 0.5).expect("tol");
        let budget = EnergyBudget::from_joules_at(1e-12, 293.15).expect("B");
        let window = one_step_window();
        let strategies = std::cell::Cell::new(StrategyRung::Jacobi);
        let out = unfold(
            0_usize,
            residual_at_index,
            |idx, rung| {
                strategies.set(rung);
                advance_index(idx, rung)
            },
            UnfoldStop::new(tolerance, budget, window, 1.0)
                .expect("stop"),
            &paid_meter(),
        )
        .expect("unfold");
        match out {
            SolveOutcome::Converged { x, .. } => {
                assert_eq!(x, 3);
                assert_ne!(strategies.get(), StrategyRung::Jacobi);
            }
            other => panic!("expected Converged after escalation past bump, got {other:?}"),
        }
    }

    #[test]
    fn bounded_package_meter_labels_spend_bounded_above() {
        let meter = BoundedPackageMeter::from_operator_ceiling_watts(42.0).expect("ceiling");
        let (_v, spent) = meter
            .measure(|| Ok(1_i32))
            .expect("bounded measure");
        assert_eq!(spent.provenance(), EnergySpendProvenance::BoundedAbove);
    }

    struct CountingMeter<'a> {
        inner: FixedJouleMeter,
        posts: &'a std::cell::Cell<u32>,
    }

    impl StepEnergyMeter for CountingMeter<'_> {
        fn measure<F, T>(&self, step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
        where
            F: FnOnce() -> Result<T, CombinatorRefuse>,
        {
            self.posts.set(self.posts.get().saturating_add(1));
            self.inner.measure(step)
        }
    }
}
