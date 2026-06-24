// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Post-step gate evidence wiring for coupled THMC (`p5-thmc-wire` spike).
//!
//! **Scope:** documents the attachment site and exposes a trait stub only — no cartridge
//! evidence extraction, no [`crate::gate::TransitionGateEvaluator`] call, and no
//! `GateCartridge::transition_evidence` implementation (W9 Phase B).
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
//! Future work lifts pre/post [`super::thmc::ThmcState`] scalar snapshots into
//! [`crate::gate::transition_proposal::ThermodynamicStateSnapshot`] and routes through
//! [`crate::gate::TransitionGateEvaluator`] once `GateCartridge::transition_evidence`
//! lands (see [`docs/W9_PLAN.md`](../../../docs/W9_PLAN.md) and
//! [`docs/rfc/GATE_EVIDENCE.md`](../../../docs/rfc/GATE_EVIDENCE.md)).

#[cfg(feature = "thmc-coupled")]
use burn::tensor::backend::Backend;

#[cfg(feature = "thmc-coupled")]
use crate::core::tensors::UnifiedMaterialStateTensor;
#[cfg(feature = "thmc-coupled")]
use crate::core::traits::IScienceCartridge;

#[cfg(feature = "thmc-coupled")]
use super::thmc::{ThmcSolver, ThmcState};

/// Placeholder witness returned at the post-step gate hook until Phase B evidence lands.
#[cfg(feature = "thmc-coupled")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThmcStepGateEvidence {
    pub dt_seconds: f32,
    pub time_after: f32,
    /// Static tag for CI / ledger honesty — not a cryptographic digest.
    pub wiring_tag: &'static str,
}

/// Post-step gate evidence hook for [`ThmcSolver`] orchestration (`thmc-coupled`).
///
/// Implementors may override [`Self::attach_gate_evidence`] to route cartridge-backed
/// transition evidence into the gate stack; the default is a documented no-op stub.
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
    ) -> ThmcStepGateEvidence;
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
    ) -> ThmcStepGateEvidence {
        wire_gate_evidence_post_step(self, cartridge, pre, post, manifold, dt)
    }
}

/// Post-step gate evidence hook invoked from [`super::thmc::ThmcSolver::step_experimental`].
///
/// Default: returns a tagged stub; does **not** call the cartridge or gate evaluators.
#[cfg(feature = "thmc-coupled")]
#[must_use]
pub fn wire_gate_evidence_post_step<B, C>(
    _solver: &ThmcSolver,
    _cartridge: &C,
    pre: &ThmcState<B>,
    post: &ThmcState<B>,
    _manifold: &UnifiedMaterialStateTensor<B>,
    dt: f32,
) -> ThmcStepGateEvidence
where
    B: Backend<FloatElem = f32>,
    C: IScienceCartridge<B>,
{
    let _ = (pre, post);
    ThmcStepGateEvidence {
        dt_seconds: dt,
        time_after: post.time,
        wiring_tag: "p5-thmc-wire: awaiting GateCartridge::transition_evidence",
    }
}
