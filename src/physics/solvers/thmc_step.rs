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
use crate::physics::error::PhysicsError;
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

/// Default intrinsic strength (MPa) for mix-calibrated gate lift (240 MPa scale).
/// Override via [`ThmcSolver::with_gate_intrinsic_strength_mpa`] when a cartridge supplies its own scale.
#[cfg(feature = "thmc-coupled")]
pub const THMC_GATE_LIFT_S_INTRINSIC_MPA_DEFAULT: f64 = 240.0;

/// Injectable transition witness — default host CD cartridge.
#[cfg(feature = "thmc-coupled")]
pub const DEFAULT_GATE_CARTRIDGE: &CdTransitionCartridge = &CdTransitionCartridge;

#[cfg(feature = "thmc-coupled")]
#[allow(deprecated)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[deprecated(
    since = "0.1.0",
    note = "use ThmcSolver::gate_cartridge and with_gate_cartridge instead"
)]
pub enum TransitionGateWitness {
    /// Host Clausius–Duhem cartridge (default).
    #[default]
    HostCd,
}

#[cfg(feature = "thmc-coupled")]
#[allow(deprecated)]
impl TransitionGateWitness {
    /// Resolve to a [`GateCartridge`] witness implementation.
    #[must_use]
    pub fn cartridge(self) -> &'static dyn GateCartridge {
        match self {
            Self::HostCd => DEFAULT_GATE_CARTRIDGE,
        }
    }
}
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
        _cartridge: &C,
        pre: &ThmcState<B>,
        post: &ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
        dt: f32,
    ) -> Result<ThmcStepGateEvidence, PhysicsError>;
}

#[cfg(feature = "thmc-coupled")]
impl<B> ThmcSolverStep<B> for ThmcSolver
where
    B: Backend<FloatElem = f32>,
{
    fn attach_gate_evidence<C: IScienceCartridge<B>>(
        &self,
        _cartridge: &C,
        pre: &ThmcState<B>,
        post: &ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
        dt: f32,
    ) -> Result<ThmcStepGateEvidence, PhysicsError> {
        wire_gate_evidence_post_step(
            self,
            self.gate_cartridge,
            pre,
            post,
            manifold,
            dt,
            self.gate_intrinsic_strength_mpa,
        )
    }
}

/// Batch-mean scalar read from a `[B, N, F]` THMC plan tensor (host telemetry only).
#[cfg(feature = "thmc-coupled")]
fn thmc_tensor_batch_mean_f32<B>(tensor: &Tensor<B, 3>) -> Result<f32, PhysicsError>
where
    B: Backend<FloatElem = f32>,
{
    let value: f32 = tensor.clone().mean().into_scalar();
    if value.is_finite() {
        Ok(value)
    } else {
        Err(PhysicsError::NonFinite {
            context: "thmc gate evidence: non-finite tensor mean",
        })
    }
}

/// Lift nodal THMC means into a gate [`ThermodynamicStateSnapshot`] via mix calibration.
#[cfg(feature = "thmc-coupled")]
fn thmc_state_thermodynamic_snapshot<B>(
    state: &ThmcState<B>,
    _manifold: &UnifiedMaterialStateTensor<B>,
    s_intrinsic_mpa: f64,
) -> Result<ThermodynamicStateSnapshot, PhysicsError>
where
    B: Backend<FloatElem = f32>,
{
    let temperature_k = thmc_tensor_batch_mean_f32(state.thermal.temperature.as_tensor())?;
    let reaction_extent = thmc_tensor_batch_mean_f32(state.chemical.reaction_extent.as_tensor())?;
    let w_c = thmc_tensor_batch_mean_f32(state.hydro.humidity.as_tensor())?;
    Ok(ThermodynamicStateSnapshot::from_mix_calibrated(
        f64::from(w_c),
        f64::from(reaction_extent),
        f64::from(temperature_k),
        s_intrinsic_mpa,
    ))
}

/// Post-step gate evidence hook invoked from [`super::thmc::ThmcSolver::step_experimental`].
///
/// Lifts pre/post [`ThmcState`] snapshots and evaluates [`GateCartridge::transition_evidence`].
#[cfg(feature = "thmc-coupled")]
pub fn wire_gate_evidence_post_step<B>(
    _solver: &ThmcSolver,
    gate: &'static dyn GateCartridge,
    pre: &ThmcState<B>,
    post: &ThmcState<B>,
    manifold: &UnifiedMaterialStateTensor<B>,
    dt: f32,
    s_intrinsic_mpa: f64,
) -> Result<ThmcStepGateEvidence, PhysicsError>
where
    B: Backend<FloatElem = f32>,
{
    let old = thmc_state_thermodynamic_snapshot(pre, manifold, s_intrinsic_mpa)?;
    let new = thmc_state_thermodynamic_snapshot(post, manifold, s_intrinsic_mpa)?;
    let constraint = explain_cd_transition_host(&old, &new, f64::from(dt), 1e-6);
    let transition = gate.transition_evidence(&old, &new, f64::from(dt));
    Ok(ThmcStepGateEvidence {
        dt_seconds: dt,
        time_after: post.time,
        transition,
        constraint,
        wiring_tag: "p5-thmc-wire: GateCartridge::transition_evidence",
    })
}

// ---------------------------------------------------------------------------
// W29-088 deepen — honest posture fence (research wire; no GREEN invent)
// ---------------------------------------------------------------------------

/// W29 deepen cell — THMC post-step gate evidence wire.
#[cfg(feature = "thmc-coupled")]
pub const W29_THMC_STEP_DEEPEN_CELL: &str = "W29-088-THMC_STEP";

/// Honest posture tag — post-step CD transition evidence research lane.
#[cfg(feature = "thmc-coupled")]
pub const THMC_STEP_POSTURE_TAG: &str = "honest-thmc-step-gate-evidence-research-lane";

/// Honest physics posture — gate evidence wire passes unit tests; does not certify fleet physics GREEN.
#[cfg(feature = "thmc-coupled")]
pub const THMC_STEP_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by p5-thmc-wire alone.
#[cfg(feature = "thmc-coupled")]
pub const THMC_STEP_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
#[cfg(feature = "thmc-coupled")]
pub const THMC_STEP_MASTER: bool = false;

/// Whether post-step [`wire_gate_evidence_post_step`] + [`ThmcSolverStep`] are landed.
#[cfg(feature = "thmc-coupled")]
pub const THMC_STEP_GATE_EVIDENCE_WIRE_LANDED: bool = true;

/// Default mix-calibrated intrinsic strength pin (cement SSOT, MPa).
#[cfg(feature = "thmc-coupled")]
pub const THMC_STEP_DEFAULT_S_INTRINSIC_MPA: f64 = THMC_GATE_LIFT_S_INTRINSIC_MPA_DEFAULT;

/// Static wiring tag substring required on every [`ThmcStepGateEvidence`].
#[cfg(feature = "thmc-coupled")]
pub const THMC_STEP_WIRING_TAG_NEEDLE: &str = "GateCartridge::transition_evidence";

/// Honest deepen fence for meta / fleet probes.
#[cfg(feature = "thmc-coupled")]
pub const THMC_STEP_HONEST_FENCE: &str = concat!(
    "gate_evidence_wire_landed=true ",
    "host_cd_cartridge_default=true ",
    "s_intrinsic_mpa_default=240 ",
    "production_wired=false ",
    "master_composition_wired=false ",
    "physics_green=false"
);

/// Typed probe for THMC step gate-evidence posture honesty.
#[cfg(feature = "thmc-coupled")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThmcStepPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub gate_evidence_wire_landed: bool,
    pub default_s_intrinsic_mpa: f64,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for THMC post-step gate evidence.
#[cfg(feature = "thmc-coupled")]
#[must_use]
pub fn thmc_step_honest_posture_bundle() -> ThmcStepPostureProbe {
    ThmcStepPostureProbe {
        physics_green: THMC_STEP_PHYSICS_GREEN,
        production_wired: THMC_STEP_PRODUCTION_WIRED,
        master: THMC_STEP_MASTER,
        gate_evidence_wire_landed: THMC_STEP_GATE_EVIDENCE_WIRE_LANDED,
        default_s_intrinsic_mpa: THMC_STEP_DEFAULT_S_INTRINSIC_MPA,
        honest_fence: THMC_STEP_HONEST_FENCE,
        posture_tag: THMC_STEP_POSTURE_TAG,
        deepen_cell: W29_THMC_STEP_DEEPEN_CELL,
    }
}

/// Post-step gate wire landed with production/master/physics-green honestly open.
#[cfg(feature = "thmc-coupled")]
#[must_use]
pub fn thmc_step_posture_honest(probe: &ThmcStepPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.gate_evidence_wire_landed
        && (probe.default_s_intrinsic_mpa - 240.0).abs() < 1e-12
        && probe.honest_fence.contains("gate_evidence_wire_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("master_composition_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

#[cfg(all(test, feature = "thmc-coupled"))]
mod tests {
    use super::*;
    use crate::core::tensors::UnifiedMaterialStateTensor;
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
    use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;
    use crate::runtime::gate::AdmissibilityToken;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn dev() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    struct Stub;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for Stub {
        fn compute_all(&self, mix: &crate::core::tensors::MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
            let d = mix.fractions.device();
            PhysicalResult {
                free_energy: Tensor::zeros([1, 1], &d),
                dissipation: Tensor::zeros([1, 1], &d),
                safety_margin: Tensor::zeros([1, 1], &d),
                cost: Tensor::zeros([1, 1], &d),
                damage: Tensor::zeros([1, 1], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, 1], &d),
            }
        }

        fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
            let d = m.scalar_features.device();
            let n = m.scalar_features.dims()[0];
            PhysicalResult {
                free_energy: Tensor::zeros([1, n], &d),
                dissipation: Tensor::zeros([1, n], &d),
                safety_margin: Tensor::zeros([1, n], &d),
                cost: Tensor::zeros([1, n], &d),
                damage: Tensor::zeros([1, n], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, n], &d),
            }
        }
    }

    fn umst(n: usize) -> UnifiedMaterialStateTensor<B> {
        let device = dev();
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let coords: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &device);
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
            &device,
        );
        let faces_b2: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &device);
        UnifiedMaterialStateTensor {
            coords,
            edges_b1,
            faces_b2,
            scalar_features: Tensor::<B, 2>::zeros([n, f], &device),
            vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &device),
            matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &device),
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions: None,
            displacement_bc_mask: Tensor::<B, 3>::ones([n, 3, 1], &device),
            policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &device),
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    fn mk_state(
        device: &NdArrayDevice,
        n: usize,
        temp: f32,
        humidity: f32,
        alpha: f32,
        time: f32,
    ) -> ThmcState<B> {
        ThmcState::from_tensors(
            Tensor::full([1, n, 1], temp, device),
            Tensor::full([1, n, 1], humidity, device),
            Tensor::zeros([1, n, 3], device),
            Tensor::full([1, n, 1], alpha, device),
            Tensor::zeros([1, n, 1], device),
            time,
        )
    }

    #[test]
    fn thmc_step_honest_posture_refuses_green_production_master() {
        let probe = thmc_step_honest_posture_bundle();
        assert!(thmc_step_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(probe.gate_evidence_wire_landed);
        assert_eq!(probe.deepen_cell, W29_THMC_STEP_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, THMC_STEP_POSTURE_TAG);
        assert!((probe.default_s_intrinsic_mpa - 240.0).abs() < 1e-12);
    }

    #[test]
    fn thmc_step_default_gate_lift_strength_is_240_mpa() {
        assert!((THMC_GATE_LIFT_S_INTRINSIC_MPA_DEFAULT - 240.0).abs() < 1e-12);
        let solver = ThmcSolver::default();
        assert!((solver.gate_intrinsic_strength_mpa - 240.0).abs() < 1e-9);
    }

    #[test]
    fn thmc_step_default_gate_cartridge_is_host_cd() {
        let solver = ThmcSolver::default();
        assert!(std::ptr::eq(
            solver.gate_cartridge as *const dyn GateCartridge,
            DEFAULT_GATE_CARTRIDGE as *const dyn GateCartridge
        ));
        #[allow(deprecated)]
        {
            assert_eq!(
                TransitionGateWitness::default().cartridge() as *const dyn GateCartridge,
                DEFAULT_GATE_CARTRIDGE as *const dyn GateCartridge
            );
        }
    }

    #[test]
    fn thmc_step_wire_identity_transition_is_admissible() {
        let n = 2usize;
        let manifold = umst(n);
        let device = dev();
        let pre = mk_state(&device, n, 293.0_f32, 0.5_f32, 0.42_f32, 0.0_f32);
        let post = pre.clone();
        let solver = ThmcSolver::default();
        let evidence = wire_gate_evidence_post_step(
            &solver,
            solver.gate_cartridge,
            &pre,
            &post,
            &manifold,
            1.0_f32,
            solver.gate_intrinsic_strength_mpa,
        )
        .expect(
            "wire_gate_evidence_post_step identity pre=post must lift admissible CD evidence (FP §6 Track G THMC step deepen)",
        );
        assert_eq!(evidence.transition.catalog_id, CD_TRANSITION_CATALOG_ID);
        assert_eq!(
            evidence.transition.admissibility,
            AdmissibilityToken::Admissible
        );
        assert!(evidence.wiring_tag.contains(THMC_STEP_WIRING_TAG_NEEDLE));
        assert!((evidence.dt_seconds - 1.0).abs() < f32::EPSILON);
        assert!((evidence.time_after - post.time).abs() < f32::EPSILON);
    }

    #[test]
    fn thmc_step_attach_gate_evidence_trait_matches_wire_hook() {
        let n = 2usize;
        let manifold = umst(n);
        let device = dev();
        let pre = mk_state(&device, n, 293.0_f32, 0.5_f32, 0.42_f32, 1.25_f32);
        let post = mk_state(&device, n, 294.0_f32, 0.48_f32, 0.43_f32, 1.25_f32);
        let solver = ThmcSolver::default();
        let stub = Stub;
        let via_trait = ThmcSolverStep::attach_gate_evidence(
            &solver, &stub, &pre, &post, &manifold, 0.5_f32,
        )
        .expect(
            "ThmcSolverStep::attach_gate_evidence must succeed on finite THMC plan means (FP §6 Track G THMC step deepen)",
        );
        let via_wire = wire_gate_evidence_post_step(
            &solver,
            solver.gate_cartridge,
            &pre,
            &post,
            &manifold,
            0.5_f32,
            solver.gate_intrinsic_strength_mpa,
        )
        .expect(
            "wire_gate_evidence_post_step must succeed on finite THMC plan means (FP §6 Track G THMC step deepen)",
        );
        assert_eq!(via_trait.transition.catalog_id, via_wire.transition.catalog_id);
        assert_eq!(
            via_trait.transition.admissibility,
            via_wire.transition.admissibility
        );
        assert_eq!(via_trait.wiring_tag, via_wire.wiring_tag);
        assert!((via_trait.dt_seconds - 0.5).abs() < f32::EPSILON);
        assert!((via_trait.time_after - 1.25).abs() < f32::EPSILON);
    }

    #[test]
    fn thmc_step_nonfinite_tensor_mean_refuses_lift() {
        let device = dev();
        let bad: Tensor<B, 3> = Tensor::full([1, 2, 1], f32::NAN, &device);
        let err = thmc_tensor_batch_mean_f32(&bad).expect_err(
            "thmc_tensor_batch_mean_f32 must refuse non-finite mean (FP §6 Track G THMC step deepen)",
        );
        assert!(
            matches!(err, PhysicsError::NonFinite { .. }),
            "expected NonFinite, got {err:?}"
        );
    }
}
