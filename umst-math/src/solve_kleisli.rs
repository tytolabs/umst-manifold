// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Nested solves composed as Kleisli arrows under [`unfold`].
//!
//! A nested solve is an arrow `X → Nested<Y>`, where [`Nested`] is
//! `Result<Certified<Y>, NestedRefuse>`: the error monad over a writer whose
//! monoid is the pair (spend ledger, tolerance chain), both under concatenation.
//! [`Certified::and_then`] is bind; [`compose`] is the Kleisli composite `f >=> g`.
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
//! Laws: bind associativity, the budget monoid (sum of measured spends ≥ sum of
//! Landauer floors) and error propagation over rational intervals are the
//! statements of cell `FORMAL-UNFOLD-KLEISLI-LAWS` (`umst-formal/Lean/UnfoldKleisli.lean`,
//! four-language parity); the pin test follows that cell. Until then they are
//! property-tested in `tests/unfold_kleisli_compose.rs`.

use crate::solve_combinator::{
    two_sum, unfold, CombinatorRefuse, EnergySpendProvenance, EnergySpent, ProgressCertificate,
    ResidualCertificate, SolveOutcome, StallEvidence, StepEnergyMeter, StrategyRung, UnfoldStop,
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
    /// Enclosure of the sum of the solve's step spends, in joules.
    pub joules: SumEnclosure,
    /// `Measured` only when every step was measured.
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

    /// Enclosure of the total spend: a left fold of the entries.
    pub fn total(&self) -> Result<SumEnclosure, CombinatorRefuse> {
        self.0
            .iter()
            .try_fold(SumEnclosure::zero(), |acc, e| acc.plus(e.joules))
    }

    /// `Measured` only when every entry is measured; the empty ledger is `Measured`.
    pub fn provenance(&self) -> EnergySpendProvenance {
        if self
            .0
            .iter()
            .all(|e| e.provenance == EnergySpendProvenance::Measured)
        {
            EnergySpendProvenance::Measured
        } else {
            EnergySpendProvenance::BoundedAbove
        }
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
        self.0.iter().all(|c| c.meets)
    }

    /// Enclosure of the sum of the links' residuals.
    pub fn residual_sum(&self) -> Result<SumEnclosure, CombinatorRefuse> {
        self.0.iter().try_fold(SumEnclosure::zero(), |acc, c| {
            acc.plus(SumEnclosure::point(c.residual.into_inner())?)
        })
    }

    /// Enclosure of the sum of the links' tolerances: the error budget of the chain
    /// when errors propagate additively.
    pub fn tolerance_sum(&self) -> Result<SumEnclosure, CombinatorRefuse> {
        self.0.iter().try_fold(SumEnclosure::zero(), |acc, c| {
            acc.plus(SumEnclosure::point(c.tolerance.residual().into_inner())?)
        })
    }
}

/// A converged composite: its value, the spend of every nested solve and the chain of
/// their certificates.
#[derive(Clone, Debug, PartialEq)]
pub struct Certified<X> {
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
pub type Nested<X> = Result<Certified<X>, NestedRefuse>;

impl<X> Certified<X> {
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

    /// Take the certified value.
    pub fn into_value(self) -> X {
        self.value
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
            Ok(next) => Ok(Certified {
                value: next.value,
                ledger: ledger.concat(next.ledger),
                chain: chain.concat(next.chain),
            }),
            Err(refuse) => Err(refuse.after(ledger, chain)),
        }
    }

    /// Functor map on the value; spends and certificates unchanged.
    pub fn map<Y, F: FnOnce(X) -> Y>(self, f: F) -> Certified<Y> {
        Certified {
            value: f(self.value),
            ledger: self.ledger,
            chain: self.chain,
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

/// Method form of bind on [`Nested`], so composites chain left to right.
pub trait NestedBind<X> {
    /// [`and_then`] as a method.
    fn bind<Y, F>(self, f: F) -> Nested<Y>
    where
        F: FnOnce(X) -> Nested<Y>;
}

impl<X> NestedBind<X> for Nested<X> {
    fn bind<Y, F>(self, f: F) -> Nested<Y>
    where
        F: FnOnce(X) -> Nested<Y>,
    {
        and_then(self, f)
    }
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
        Ok(SolveOutcome::Converged { x, certificate }) => Ok(Certified {
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
/// posts the step spends into one enclosure.
#[derive(Debug)]
pub struct SubMeter<'m, M> {
    outer: &'m M,
    posted: Cell<SumEnclosure>,
    bounded: Cell<bool>,
}

impl<'m, M: StepEnergyMeter> SubMeter<'m, M> {
    /// A fresh sub-meter over `outer` with nothing posted.
    pub fn over(outer: &'m M) -> Self {
        Self {
            outer,
            posted: Cell::new(SumEnclosure::zero()),
            bounded: Cell::new(false),
        }
    }

    /// The spend posted so far, as one ledger entry.
    pub fn spend(&self) -> NestedSpend {
        NestedSpend {
            joules: self.posted.get(),
            provenance: if self.bounded.get() {
                EnergySpendProvenance::BoundedAbove
            } else {
                EnergySpendProvenance::Measured
            },
        }
    }
}

impl<M: StepEnergyMeter> StepEnergyMeter for SubMeter<'_, M> {
    fn measure<F, T>(&self, step: F) -> Result<(T, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<T, CombinatorRefuse>,
    {
        let (value, cost) = self.outer.measure(step)?;
        let joules = cost.ucrs_ledger_joules()?.into_inner();
        self.posted
            .set(self.posted.get().plus(SumEnclosure::point(joules)?)?);
        if cost.provenance() == EnergySpendProvenance::BoundedAbove {
            self.bounded.set(true);
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
    let sub = SubMeter::over(meter);
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
