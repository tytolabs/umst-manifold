// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The unfold step on the passive route of the one SecondLaw predicate.
//!
//! A step `x₀ → x₊` over `τ > 0` is admitted when the discrete inequality of law L3 holds:
//!
//! `ψ(x₊) − ψ(x₀) + τ·φ((x₊ − x₀)/τ) ≤ W_in + k_B·T·I`, with `W_in = w(x₊) − w(x₀)`.
//!
//! φ is a [`PassivePhi`] of umst-cartridge-api, convex, non-negative and zero at zero by
//! construction (`UMST.Convex.PhiGrammar.phiExpr_passive_secondLaw`), and ψ is the free
//! energy the model states (a cartridge's `UMSTCartridge::free_energy` or a
//! `GsmComposite::free_energy`). The acceptance decision is the existing core gate
//! ([`core_gate`]): in rate form the slack of L3,
//! `(W_in + k_B·T·I − ΔF)/τ − φ((x₊ − x₀)/τ)`, is the net dissipation the gate judges against
//! its tolerance, together with the step's mass balance. A minimiser of the incremental
//! potential satisfies L3 (`UMST.UnfoldProxStepLaws.prox_step_le`, umst-formal cf08d05, parity
//! row `prox.prox_step_le`; a passive step with the mass condition is the `transition` case
//! of SecondLaw through `prox_step_passive_secondLaw`), and a candidate that breaks it is no
//! minimiser (`prox_step_refused`). The gate runs on every step whatever proposed it.
//!
//! [`Admitted`] carries a value and the balances of the steps that made it;
//! [`Admitted::and_then`] is Kleisli bind (law L1, `UMST.UnfoldKleisli.bind_assoc`: the
//! balances form a free monoid). Operator splitting passes several parts per macro step to
//! [`dissipative_unfold`], each admitted on its own. A JKO step is the proximal step in the
//! Wasserstein metric (Jordan, Kinderlehrer and Otto 1998, SIAM J. Math. Anal. 29(1)).
//!
//! Floating point enters at ψ, at [`PassivePhi::eval`] and at the gate; the gate tolerance is
//! the caller's stated bound on that rounding.

use std::cell::RefCell;

use ordered_float::NotNan;
use umst_cartridge_api::convex_phi::PassivePhi;
use umst_cartridge_api::ConvexPhiEvalError;
use umst_gate::core_gate::{core_gate, CoreGateOutcome, ScalarConstitutiveResponse};
use umst_math::solve_combinator::{CombinatorRefuse, StepEnergyMeter, UnfoldStop};
use umst_math::solve_kleisli::{nested_unfold, Converged, NestedRefuse};

/// How a step was proposed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepKind {
    /// Minimiser of `ψ − w + τ·φ` (minimising movement).
    Proximal,
    /// Proximal step in the Wasserstein metric (JKO).
    Jko,
}

/// One step of a passive model: ψ, φ, the work functional, the information term and the
/// proposer of the next state.
pub trait DissipativeStep {
    /// `ψ(x)`.
    fn free_energy(&self, x: &[f64]) -> f64;
    /// The passive dissipation potential φ over rates.
    fn dissipation(&self) -> &PassivePhi;
    /// `w(x)`; the supplied work of a step is `w(x₊) − w(x₀)`.
    fn work(&self, x: &[f64]) -> f64;
    /// `k_B·T·I` of the step; zero for a step without measurement.
    fn information(&self, _x0: &[f64], _x1: &[f64]) -> f64 {
        0.0
    }
    /// Whether the step conserves mass; true for a state that carries none.
    fn mass_conserved(&self, _x0: &[f64], _x1: &[f64]) -> bool {
        true
    }
    /// How [`Self::propose`] defines the next state.
    fn kind(&self) -> StepKind;
    /// The proposed next state (proximal or JKO minimiser, or any proposer).
    fn propose(&self, x0: &[f64], tau: f64) -> Result<Vec<f64>, CombinatorRefuse>;
}

/// The energy balance of one admitted step.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StepBalance {
    delta_psi: f64,
    dissipated: f64,
    work_in: f64,
    information: f64,
    kind: StepKind,
}

impl StepBalance {
    /// `ψ(x₊) − ψ(x₀)`.
    pub fn delta_psi(self) -> f64 {
        self.delta_psi
    }
    /// `τ·φ((x₊ − x₀)/τ)`.
    pub fn dissipated(self) -> f64 {
        self.dissipated
    }
    /// `w(x₊) − w(x₀)`.
    pub fn work_in(self) -> f64 {
        self.work_in
    }
    /// `k_B·T·I`.
    pub fn information(self) -> f64 {
        self.information
    }
    /// How the step was proposed.
    pub fn kind(self) -> StepKind {
        self.kind
    }
}

/// Why a step is not admitted.
#[derive(Clone, Debug, PartialEq)]
pub enum DissipationStepRefuse {
    /// `τ` is not finite and strictly positive.
    NonPositiveStep,
    /// The proposed state has another length than the current one.
    ShapeMismatch {
        /// Current length.
        expected: usize,
        /// Proposed length.
        found: usize,
    },
    /// ψ, w, the information term or a rate is not finite.
    NonFinite,
    /// The information term is negative.
    NegativeInformation,
    /// φ refused the rate.
    Phi(ConvexPhiEvalError),
    /// The proposer refused.
    Proposer(CombinatorRefuse),
    /// The core gate rejected the step.
    Rejected {
        /// The gate's verdict and terms.
        outcome: CoreGateOutcome,
        /// The step's balance.
        balance: StepBalance,
    },
    /// The outer unfold stalled, spent its budget or refused.
    Outer(NestedRefuse),
}

/// Admit `x₀ → x₁` of `step` over `tau` through the core gate, or refuse it.
pub fn admit<S: DissipativeStep + ?Sized>(
    step: &S,
    x0: &[f64],
    x1: &[f64],
    tau: f64,
    tolerance: f64,
) -> Result<StepBalance, DissipationStepRefuse> {
    if !(tau.is_finite() && tau > 0.0) {
        return Err(DissipationStepRefuse::NonPositiveStep);
    }
    if x1.len() != x0.len() {
        return Err(DissipationStepRefuse::ShapeMismatch {
            expected: x0.len(),
            found: x1.len(),
        });
    }
    let rate: Vec<f64> = x0.iter().zip(x1).map(|(a, b)| (b - a) / tau).collect();
    let phi = step
        .dissipation()
        .eval(&rate)
        .map_err(DissipationStepRefuse::Phi)?;
    let information = step.information(x0, x1);
    let balance = StepBalance {
        delta_psi: step.free_energy(x1) - step.free_energy(x0),
        dissipated: tau * phi,
        work_in: step.work(x1) - step.work(x0),
        information,
        kind: step.kind(),
    };
    let terms = [
        balance.delta_psi,
        balance.dissipated,
        balance.work_in,
        information,
    ];
    if terms.iter().chain(&rate).any(|t| !t.is_finite()) {
        return Err(DissipationStepRefuse::NonFinite);
    }
    if information < 0.0 {
        return Err(DissipationStepRefuse::NegativeInformation);
    }
    let slack = (balance.work_in + information - balance.delta_psi) / tau - phi;
    let outcome = core_gate(
        &ScalarConstitutiveResponse {
            dissipation: slack,
            power_input: 0.0,
        },
        step.mass_conserved(x0, x1),
        tolerance,
    );
    if outcome.is_accepted() {
        Ok(balance)
    } else {
        Err(DissipationStepRefuse::Rejected { outcome, balance })
    }
}

/// A value with the balances of the admitted steps that produced it.
#[derive(Clone, Debug, PartialEq)]
pub struct Admitted<X> {
    value: X,
    steps: Vec<StepBalance>,
}

impl<X> Admitted<X> {
    /// Monad unit: no step.
    pub fn unit(value: X) -> Self {
        Self {
            value,
            steps: Vec::new(),
        }
    }

    /// The state reached.
    pub fn value(&self) -> &X {
        &self.value
    }

    /// The balances of the admitted steps, in order.
    pub fn steps(&self) -> &[StepBalance] {
        &self.steps
    }

    /// Kleisli bind: run `f` on the value and append its balances.
    pub fn and_then<Y, F>(self, f: F) -> Result<Admitted<Y>, DissipationStepRefuse>
    where
        F: FnOnce(X) -> Result<Admitted<Y>, DissipationStepRefuse>,
    {
        let Self { value, mut steps } = self;
        let next = f(value)?;
        steps.extend(next.steps);
        Ok(Admitted {
            value: next.value,
            steps,
        })
    }
}

/// Propose and admit one step of `step` from `x0`.
pub fn admitted_step<S: DissipativeStep + ?Sized>(
    step: &S,
    x0: Vec<f64>,
    tau: f64,
    tolerance: f64,
) -> Result<Admitted<Vec<f64>>, DissipationStepRefuse> {
    let x1 = step
        .propose(&x0, tau)
        .map_err(DissipationStepRefuse::Proposer)?;
    let balance = admit(step, &x0, &x1, tau, tolerance)?;
    Ok(Admitted {
        value: x1,
        steps: vec![balance],
    })
}

/// Run macro steps under `unfold`, each the parts in order (one part for a plain step,
/// several for operator splitting), every part admitted by the core gate. A refused part
/// stops the unfold with its typed refusal.
pub fn dissipative_unfold<R, Meter>(
    parts: &[&dyn DissipativeStep],
    residual_of: R,
    initial: Vec<f64>,
    tau: f64,
    tolerance: f64,
    stop: UnfoldStop,
    meter: &Meter,
) -> Result<Converged<Admitted<Vec<f64>>>, DissipationStepRefuse>
where
    R: Fn(&[f64]) -> Result<NotNan<f64>, CombinatorRefuse>,
    Meter: StepEnergyMeter,
{
    let cause: RefCell<Option<DissipationStepRefuse>> = RefCell::new(None);
    let outcome = nested_unfold(
        Admitted::unit(initial),
        |a: &Admitted<Vec<f64>>| residual_of(a.value()),
        |a: &Admitted<Vec<f64>>, _rung| {
            parts
                .iter()
                .try_fold(a.clone(), |acc, part| {
                    acc.and_then(|x| admitted_step(*part, x, tau, tolerance))
                })
                .map_err(|refuse| {
                    *cause.borrow_mut() = Some(refuse);
                    CombinatorRefuse::StepRefused
                })
        },
        stop,
        meter,
    );
    outcome.map_err(|outer| match cause.borrow_mut().take() {
        Some(inner) => inner,
        None => DissipationStepRefuse::Outer(outer),
    })
}
