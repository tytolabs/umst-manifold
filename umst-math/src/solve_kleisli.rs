// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Nested solves composed as Kleisli arrows under [`unfold`].
//!
//! A nested solve is an arrow `X → Nested<Y>`, where [`Nested`] is
//! `Result<Converged<Y>, NestedRefuse>`: the error monad over a writer whose
//! monoid is the pair (spend ledger, tolerance chain), both under concatenation.
//! [`Converged::and_then`] is bind; [`compose`] is the Kleisli composite `f >=> g`.
//!
//! - **Converged only continues.** An inner solve that ends `Stalled` or
//!   `BudgetSpent`, or refuses, is the outer solve's refusal ([`NestedRefuse`]),
//!   carrying the spends and certificates gathered up to that point.
//! - **One sub-meter per nested solve.** [`nested_unfold`] runs [`unfold`] through a
//!   [`SubMeter`] over the caller's meter and posts that solve's spend as one
//!   ledger entry, an enclosure `[lo, hi]` of the binary64 step spends.
//! - **Budget shares.** An outer budget is split into inner shares with
//!   [`EnergyBudget::split`], which refuses any split whose shares do not add back
//!   exactly, so the outer budget is the sum of the inner ones.
//! - **Tolerance chain.** Each converged nested solve appends its
//!   [`ResidualCertificate`] to the [`ToleranceChain`].
//!
//! Bind associativity and the unit laws hold exactly: ledgers and chains are
//! free monoids (sequences), and totals are a left fold over them. With the steer
//! fold (`umst-adk`) consuming ledgers as outcome rows, `unfold` followed by the fold
//! forms a hylomorphism (six-fibre blueprint §12.6).
//!
//! Consumer: `umst_manifold::solve::implicit_adjoint` runs the forward solve through
//! [`nested_unfold`] and reads the chain's [`ToleranceChain::residual_sum`] and
//! [`ToleranceChain::tolerance_sum`] as the residual budget of the state its adjoint
//! differentiates.
//!
//! Laws: bind associativity, the budget monoid (sum of measured spends ≥ sum of
//! Landauer floors) and error propagation over rational intervals are the
//! statements of cell `FORMAL-UNFOLD-KLEISLI-LAWS` (`umst-formal/Lean/UnfoldKleisli.lean`,
//! four-language parity); the pin test follows that cell. Until then they are
//! property-tested in `tests/unfold_kleisli_compose.rs`.

use crate::solve_combinator::{
    landauer_step_joules, two_sum, unfold, CombinatorRefuse, EnergySpendProvenance, EnergySpent,
    ProgressCertificate, ResidualCertificate, SolveOutcome, StallEvidence, StepEnergyMeter,
    StrategyRung, UnfoldStop,
};
use core::cell::Cell;
use ordered_float::NotNan;

/// Enclosure `[lo, hi]` of a sum of finite non-negative binary64 values.
///
/// Each addition widens outward by one unit in the last place only when the
/// two-sum error is non-zero, so `lo ≤ Σ ≤ hi` holds for the real sum.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SumEnclosure {
    lo: f64,
    hi: f64,
}

impl SumEnclosure {
    /// The empty sum `[0, 0]`.
    pub fn zero() -> Self {
        Self { lo: 0.0, hi: 0.0 }
    }

    /// The point enclosure of one finite non-negative value.
    pub fn point(x: f64) -> Result<Self, CombinatorRefuse> {
        let x = non_negative(x)?;
        Ok(Self { lo: x, hi: x })
    }

    /// Lower end.
    pub fn lo(self) -> f64 {
        self.lo
    }

    /// Upper end.
    pub fn hi(self) -> f64 {
        self.hi
    }

    /// Whether `x` lies in `[lo, hi]`.
    pub fn contains(self, x: f64) -> bool {
        self.lo <= x && x <= self.hi
    }

    /// Outward-rounded sum of two enclosures.
    pub fn plus(self, other: Self) -> Result<Self, CombinatorRefuse> {
        let (lo, lo_err) = two_sum(self.lo, other.lo);
        let (hi, hi_err) = two_sum(self.hi, other.hi);
        let lo = if lo_err < 0.0 { next_down(lo) } else { lo };
        let hi = if hi_err > 0.0 { next_up(hi) } else { hi };
        Ok(Self {
            lo: non_negative(lo)?,
            hi: non_negative(hi)?,
        })
    }
}

/// The metered spend of one nested solve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NestedSpend {
    /// Enclosure of the sum of the solve's measured step spends, in joules.
    pub joules: SumEnclosure,
    /// Enclosure of the sum of the solve's Landauer floors `k_B·T·ln 2·b`, one per step,
    /// in joules. A measured entry is honest when `floor ≤ joules` (law L2,
    /// `UMST.UnfoldKleisli.bind_honest`: honest spends bind to an honest spend).
    pub floor: SumEnclosure,
    /// The meet of the step grades: `Measured` only when every step was measured.
    pub provenance: EnergySpendProvenance,
}

/// One entry per nested solve, in order.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SpendLedger(Vec<NestedSpend>);

impl SpendLedger {
    /// The empty ledger (monoid identity).
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// A ledger of one nested solve.
    pub fn single(entry: NestedSpend) -> Self {
        Self(vec![entry])
    }

    /// Entries, one per nested solve.
    pub fn entries(&self) -> &[NestedSpend] {
        &self.0
    }

    /// Monoid operation: concatenation.
    pub fn concat(mut self, other: Self) -> Self {
        self.0.extend(other.0);
        self
    }

    /// Enclosure of the total spend: a left fold of the entries;
    /// [`CombinatorRefuse::Unmeasured`] when an entry is absent, since then no total exists.
    pub fn total(&self) -> Result<SumEnclosure, CombinatorRefuse> {
        if self.provenance() == EnergySpendProvenance::Absent {
            return Err(CombinatorRefuse::Unmeasured);
        }
        self.0
            .iter()
            .try_fold(SumEnclosure::zero(), |acc, e| acc.plus(e.joules))
    }

    /// The meet of the entries' grades; the empty ledger is `Measured`.
    pub fn provenance(&self) -> EnergySpendProvenance {
        self.0
            .iter()
            .map(|e| e.provenance)
            .fold(EnergySpendProvenance::Measured, Ord::min)
    }
}

/// Residual certificates of the converged nested solves, in order.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ToleranceChain(Vec<ResidualCertificate>);

impl ToleranceChain {
    /// The empty chain (monoid identity).
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// A chain of one certificate.
    pub fn single(certificate: ResidualCertificate) -> Self {
        Self(vec![certificate])
    }

    /// Certificates, one per converged nested solve.
    pub fn links(&self) -> &[ResidualCertificate] {
        &self.0
    }

    /// Monoid operation: concatenation.
    pub fn concat(mut self, other: Self) -> Self {
        self.0.extend(other.0);
        self
    }

    /// Whether every link meets its tolerance.
    pub fn meets(&self) -> bool {
        self.0.iter().all(|c| c.meets())
    }

    /// Enclosure of the sum of the links' residuals.
    pub fn residual_sum(&self) -> Result<SumEnclosure, CombinatorRefuse> {
        self.0.iter().try_fold(SumEnclosure::zero(), |acc, c| {
            acc.plus(SumEnclosure::point(c.residual().into_inner())?)
        })
    }

    /// Enclosure of the sum of the links' tolerances: the error budget of the chain
    /// when errors propagate additively.
    pub fn tolerance_sum(&self) -> Result<SumEnclosure, CombinatorRefuse> {
        self.0.iter().try_fold(SumEnclosure::zero(), |acc, c| {
            acc.plus(SumEnclosure::point(c.tolerance().residual().into_inner())?)
        })
    }
}

/// A converged composite: its value, the spend of every nested solve and the chain of
/// their certificates.
///
/// The fields are private; a composite with a certificate comes only from [`lift`] of a
/// `Converged` outcome of `unfold` (through [`nested_unfold`]) and from bind. The unit
/// carries no certificate, so its chain is empty and a consumer that needs a converged
/// state refuses it. No functor map is offered: a value changed after the solve would
/// keep certificates that do not describe it.
#[derive(Clone, Debug, PartialEq)]
pub struct Converged<X> {
    value: X,
    ledger: SpendLedger,
    chain: ToleranceChain,
}

/// Why a composite of nested solves stopped short of `Converged`.
///
/// The ledger and chain cover every nested solve run up to and including the one
/// that stopped; the chain holds the certificates of those that converged.
#[derive(Clone, Debug, PartialEq)]
pub enum NestedRefuse {
    /// An inner solve refused (meter absent, measured below floor, invalid quantity).
    Refused {
        /// The inner refusal.
        cause: CombinatorRefuse,
        /// Spends so far.
        ledger: SpendLedger,
        /// Certificates so far.
        chain: ToleranceChain,
    },
    /// An inner solve stalled.
    Stalled {
        /// The inner stall evidence.
        evidence: StallEvidence,
        /// Spends so far.
        ledger: SpendLedger,
        /// Certificates so far.
        chain: ToleranceChain,
    },
    /// An inner solve spent its budget share.
    BudgetSpent {
        /// The inner progress at exhaustion.
        progress: ProgressCertificate,
        /// Spends so far.
        ledger: SpendLedger,
        /// Certificates so far.
        chain: ToleranceChain,
    },
}

impl NestedRefuse {
    /// Spends of the nested solves run before the stop, the stopped one included.
    pub fn ledger(&self) -> &SpendLedger {
        match self {
            Self::Refused { ledger, .. }
            | Self::Stalled { ledger, .. }
            | Self::BudgetSpent { ledger, .. } => ledger,
        }
    }

    /// Certificates of the nested solves that converged before the stop.
    pub fn chain(&self) -> &ToleranceChain {
        match self {
            Self::Refused { chain, .. }
            | Self::Stalled { chain, .. }
            | Self::BudgetSpent { chain, .. } => chain,
        }
    }

    fn after(self, prior_ledger: SpendLedger, prior_chain: ToleranceChain) -> Self {
        match self {
            Self::Refused {
                cause,
                ledger,
                chain,
            } => Self::Refused {
                cause,
                ledger: prior_ledger.concat(ledger),
                chain: prior_chain.concat(chain),
            },
            Self::Stalled {
                evidence,
                ledger,
                chain,
            } => Self::Stalled {
                evidence,
                ledger: prior_ledger.concat(ledger),
                chain: prior_chain.concat(chain),
            },
            Self::BudgetSpent {
                progress,
                ledger,
                chain,
            } => Self::BudgetSpent {
                progress,
                ledger: prior_ledger.concat(ledger),
                chain: prior_chain.concat(chain),
            },
        }
    }
}

/// The Kleisli object: a converged composite or the refusal that stopped it.
pub type Nested<X> = Result<Converged<X>, NestedRefuse>;

impl<X> Converged<X> {
    /// Monad unit: a value with no spend and no certificate.
    pub fn unit(value: X) -> Self {
        Self {
            value,
            ledger: SpendLedger::empty(),
            chain: ToleranceChain::empty(),
        }
    }

    /// The certified value.
    pub fn value(&self) -> &X {
        &self.value
    }

    /// One spend entry per nested solve.
    pub fn ledger(&self) -> &SpendLedger {
        &self.ledger
    }

    /// One certificate per nested solve.
    pub fn chain(&self) -> &ToleranceChain {
        &self.chain
    }

    /// Kleisli bind: run `f` on the converged value; concatenate ledgers and chains;
    /// an inner refusal becomes this composite's refusal with the spends so far.
    pub fn and_then<Y, F>(self, f: F) -> Nested<Y>
    where
        F: FnOnce(X) -> Nested<Y>,
    {
        let Self {
            value,
            ledger,
            chain,
        } = self;
        match f(value) {
            Ok(next) => Ok(Converged {
                value: next.value,
                ledger: ledger.concat(next.ledger),
                chain: chain.concat(next.chain),
            }),
            Err(refuse) => Err(refuse.after(ledger, chain)),
        }
    }
}

/// Bind on [`Nested`]: a refusal passes through unchanged.
pub fn and_then<X, Y, F>(m: Nested<X>, f: F) -> Nested<Y>
where
    F: FnOnce(X) -> Nested<Y>,
{
    m.and_then(|c| c.and_then(f))
}

/// Kleisli composite `f >=> g`.
pub fn compose<X, Y, Z, F, G>(f: F, g: G) -> impl FnOnce(X) -> Nested<Z>
where
    F: FnOnce(X) -> Nested<Y>,
    G: FnOnce(Y) -> Nested<Z>,
{
    move |x| and_then(f(x), g)
}

/// Lift one `unfold` result and its metered spend into [`Nested`].
pub fn lift<X>(
    outcome: Result<SolveOutcome<X>, CombinatorRefuse>,
    spend: NestedSpend,
) -> Nested<X> {
    let ledger = SpendLedger::single(spend);
    let chain = ToleranceChain::empty();
    match outcome {
        Ok(SolveOutcome::Converged { x, certificate }) => Ok(Converged {
            value: x,
            ledger,
            chain: ToleranceChain::single(certificate),
        }),
        Ok(SolveOutcome::Stalled { evidence, .. }) => Err(NestedRefuse::Stalled {
            evidence,
            ledger,
            chain,
        }),
        Ok(SolveOutcome::BudgetSpent {
            progress_certificate,
            ..
        }) => Err(NestedRefuse::BudgetSpent {
            progress: progress_certificate,
            ledger,
            chain,
        }),
        Err(cause) => Err(NestedRefuse::Refused {
            cause,
            ledger,
            chain,
        }),
    }
}

/// The meter of one nested solve: delegates each step to the outer meter and
/// posts the step spends into one enclosure and each step's Landauer floor into another;
/// the entry's grade is the meet of the step grades, and an absent step adds no joules to
/// the spend enclosure (its floor is still posted).
#[derive(Debug)]
pub struct SubMeter<'m, M> {
    outer: &'m M,
    floor_per_step: f64,
    posted: Cell<SumEnclosure>,
    floors: Cell<SumEnclosure>,
    grade: Cell<EnergySpendProvenance>,
}

impl<'m, M: StepEnergyMeter> SubMeter<'m, M> {
    /// A fresh sub-meter over `outer` whose steps each carry the Landauer floor `floor`.
    pub fn over(outer: &'m M, floor: EnergySpent) -> Result<Self, CombinatorRefuse> {
        Ok(Self {
            outer,
            floor_per_step: floor.ucrs_ledger_joules()?.into_inner(),
            posted: Cell::new(SumEnclosure::zero()),
            floors: Cell::new(SumEnclosure::zero()),
            grade: Cell::new(EnergySpendProvenance::Measured),
        })
    }

    /// The spend posted so far, as one ledger entry.
    pub fn spend(&self) -> NestedSpend {
        NestedSpend {
            joules: self.posted.get(),
            floor: self.floors.get(),
            provenance: self.grade.get(),
        }
    }
}

impl<M: StepEnergyMeter> StepEnergyMeter for SubMeter<'_, M> {
    fn measure<F, T>(&self, step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<T, CombinatorRefuse>,
    {
        let (value, cost) = self.outer.measure(step)?;
        self.grade.set(self.grade.get().min(cost.provenance()));
        self.floors.set(
            self.floors
                .get()
                .plus(SumEnclosure::point(self.floor_per_step)?)?,
        );
        if cost.provenance() != EnergySpendProvenance::Absent {
            let joules = cost.ucrs_ledger_joules()?.into_inner();
            self.posted
                .set(self.posted.get().plus(SumEnclosure::point(joules)?)?);
        }
        Ok((value, cost))
    }
}

/// Run one nested `unfold` through its own sub-meter and lift the result.
pub fn nested_unfold<S, ResidualOf, Step, Meter>(
    initial: S,
    residual_of: ResidualOf,
    step: Step,
    stop: UnfoldStop,
    meter: &Meter,
) -> Nested<S>
where
    S: Clone,
    ResidualOf: Fn(&S) -> Result<NotNan<f64>, CombinatorRefuse>,
    Step: FnMut(&S, StrategyRung) -> Result<S, CombinatorRefuse>,
    Meter: StepEnergyMeter,
{
    let floor = match landauer_step_joules(stop.budget().temperature_k(), stop.bits_per_step()) {
        Ok(floor) => floor,
        Err(cause) => {
            return Err(NestedRefuse::Refused {
                cause,
                ledger: SpendLedger::empty(),
                chain: ToleranceChain::empty(),
            })
        }
    };
    let sub = match SubMeter::over(meter, floor) {
        Ok(sub) => sub,
        Err(cause) => {
            return Err(NestedRefuse::Refused {
                cause,
                ledger: SpendLedger::empty(),
                chain: ToleranceChain::empty(),
            })
        }
    };
    let outcome = unfold(initial, residual_of, step, stop, &sub);
    lift(outcome, sub.spend())
}

fn non_negative(x: f64) -> Result<f64, CombinatorRefuse> {
    if !x.is_finite() {
        return Err(CombinatorRefuse::NonFiniteQuantity);
    }
    if x < 0.0 {
        return Err(CombinatorRefuse::NonPositiveBudget);
    }
    // `-0.0 + 0.0 = +0.0`: the bit-level successor below assumes the positive zero.
    Ok(x + 0.0)
}

/// Next binary64 above a finite `x ≥ 0`.
fn next_up(x: f64) -> f64 {
    f64::from_bits(x.to_bits() + 1)
}

/// Next binary64 below a finite `x > 0`; `0` stays `0` (sums of non-negatives).
fn next_down(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        f64::from_bits(x.to_bits() - 1)
    }
}
