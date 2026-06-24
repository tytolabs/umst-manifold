// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Post-step gate evidence wiring for coupled THMC (`p5-thmc-wire` spike).
//!
//! **Scope:** lifts pre/post [`super::thmc::ThmcState`] nodal means into
//! [`crate::gate::transition_proposal::ThermodynamicStateSnapshot`] and routes through
//! [`crate::runtime::gate::CdTransitionCartridge::transition_evidence`] at the post-step hook.
//!
//! ## Attachment site (post-step)
//!
//! Gate evidence attaches **after** operator-split transport / mechanics / fracture in
//! [`super::thmc::ThmcSolver::step_experimental`], immediately before `state.time += dt`
//! and `Ok(state)`:
//!
//! ```text
//! ThmcSolver::step_experimental
//!   … operator-split (T, α) → h → u …
//!   … PhaseFieldFractureSolver::update_damage …
//!   → wire_gate_evidence_post_step   ← gate evidence hook (this module)
//!   → state.time += dt
//!   → Ok(state)
//! ```
//!
//! Future work: cartridge-backed evidence via [`crate::core::traits::IScienceCartridge`] and
//! manifest-driven closure literals (see [`docs/W9_PLAN.md`](../../../docs/W9_PLAN.md) and
//! [`docs/rfc/GATE_EVIDENCE.md`](../../../docs/rfc/GATE_EVIDENCE.md)).

#[cfg(feature = "thmc-coupled")]
use burn::tensor::backend::Backend;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::Tensor;

#[cfg(feature = "thmc-coupled")]
use crate::core::tensors::UnifiedMaterialStateTensor;
#[cfg(feature = "thmc-coupled")]
use crate::core::traits::IScienceCartridge;
#[cfg(feature = "thmc-coupled")]
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;
#[cfg(feature = "thmc-coupled")]
use crate::runtime::gate::evidence::{explain_cd_transition_host, ConstraintExplanation};
#[cfg(feature = "thmc-coupled")]
use crate::runtime::gate::{CdTransitionCartridge, GateCartridge, TransitionEvidence};

#[cfg(feature = "thmc-coupled")]
use super::thmc::{ThmcSolver, ThmcState};

/// Default intrinsic strength (MPa) for mix-calibrated lift — matches [`crate::gate::http_manifest::GateManifest::from`].
#[cfg(feature = "thmc-coupled")]
const THMC_GATE_LIFT_S_INTRINSIC_MPA: f64 = 80.0;

/// Witness returned at the post-step gate hook after [`CdTransitionCartridge`] evaluation.
#[cfg(feature = "thmc-coupled")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThmcStepGateEvidence {
    pub dt_seconds: f32,
    pub time_after: f32,
    pub transition: TransitionEvidence,
    /// Host Clausius–Duhem slack witness (immutable; feeds penalize accumulator).
    pub constraint: ConstraintExplanation,
    /// Static tag for CI / ledger honesty — not a cryptographic digest.
    pub wiring_tag: &'static str,
}

/// Post-step gate evidence hook for [`ThmcSolver`] orchestration (`thmc-coupled`).
///
/// Implementors may override [`Self::attach_gate_evidence`] to route cartridge-backed
/// transition evidence into the gate stack.
#[cfg(feature = "thmc-coupled")]
pub trait ThmcSolverStep<B: Backend> {
    /// Lift post-step [`ThmcState`] into gate evidence after physics advance.
    fn attach_gate_evidence<C: IScienceCartridge<B>>(
        &self,
        cartridge: &C,
        pre: &ThmcState<B>,
        post: &ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
        dt: f32,
    ) -> Result<ThmcStepGateEvidence, String>;
}

#[cfg(feature = "thmc-coupled")]
impl<B> ThmcSolverStep<B> for ThmcSolver
where
    B: Backend<FloatElem = f32>,
{
    fn attach_gate_evidence<C: IScienceCartridge<B>>(
        &self,
        cartridge: &C,
        pre: &ThmcState<B>,
        post: &ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
        dt: f32,
    ) -> Result<ThmcStepGateEvidence, String> {
        wire_gate_evidence_post_step(self, cartridge, pre, post, manifold, dt)
    }
}

/// Batch-mean scalar read from a `[B, N, F]` THMC plan tensor (host telemetry only).
#[cfg(feature = "thmc-coupled")]
fn thmc_tensor_batch_mean_f32<B>(tensor: &Tensor<B, 3>) -> Result<f32, String>
where
    B: Backend<FloatElem = f32>,
{
    let value: f32 = tensor.clone().mean().into_scalar();
    if value.is_finite() {
        Ok(value)
    } else {
        Err("thmc gate evidence: non-finite tensor mean".into())
    }
}

/// Lift nodal THMC means into a gate [`ThermodynamicStateSnapshot`] via mix calibration.
#[cfg(feature = "thmc-coupled")]
fn thmc_state_thermodynamic_snapshot<B>(
    state: &ThmcState<B>,
    _manifold: &UnifiedMaterialStateTensor<B>,
) -> Result<ThermodynamicStateSnapshot, String>
where
    B: Backend<FloatElem = f32>,
{
    let temperature_k = thmc_tensor_batch_mean_f32(&state.thermal.temperature)?;
    let reaction_extent = thmc_tensor_batch_mean_f32(&state.chemical.reaction_extent)?;
    let w_c = thmc_tensor_batch_mean_f32(&state.hydro.humidity)?;
    Ok(ThermodynamicStateSnapshot::from_mix_calibrated(
        f64::from(w_c),
        f64::from(reaction_extent),
        f64::from(temperature_k),
        THMC_GATE_LIFT_S_INTRINSIC_MPA,
    ))
}

/// Post-step gate evidence hook invoked from [`super::thmc::ThmcSolver::step_experimental`].
///
/// Lifts pre/post [`ThmcState`] snapshots and evaluates [`CdTransitionCartridge::transition_evidence`].
#[cfg(feature = "thmc-coupled")]
#[must_use]
pub fn wire_gate_evidence_post_step<B, C>(
    _solver: &ThmcSolver,
    _cartridge: &C,
    pre: &ThmcState<B>,
    post: &ThmcState<B>,
    manifold: &UnifiedMaterialStateTensor<B>,
    dt: f32,
) -> Result<ThmcStepGateEvidence, String>
where
    B: Backend<FloatElem = f32>,
    C: IScienceCartridge<B>,
{
    let old = thmc_state_thermodynamic_snapshot(pre, manifold)?;
    let new = thmc_state_thermodynamic_snapshot(post, manifold)?;
    let constraint = explain_cd_transition_host(&old, &new, f64::from(dt), 1e-6);
    let transition = CdTransitionCartridge.transition_evidence(&old, &new, f64::from(dt));
    Ok(ThmcStepGateEvidence {
        dt_seconds: dt,
        time_after: post.time,
        transition,
        constraint,
        wiring_tag: "p5-thmc-wire: CdTransitionCartridge::transition_evidence",
    })
}
