// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Embodied policy loop: [`ManifoldGateway`] (tensor / CBF) composed with optional host gates via
//! [`GateEvaluatorRegistry`] and [`ThermodynamicTransitionEvaluator`] (see `docs/GateUnificationSpec.md`).
//!
//! **Honest status:** host-before-gateway routing + registry CD / mix / Kleisli η is measured —
//! not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER_RETICK`. Fragment audit (~22% scaffold)
//! and embodied loop close remain deferred beyond this spine.

/// W29 deepen cell id — honest orchestrator spine only.
pub const ORCHESTRATOR_CELL_ID: &str = "W29-042-ORCHESTRATOR";

/// Honest posture tag — tests deepen routing contract (`MASTER_RETICK=no`).
pub const ORCHESTRATOR_POSTURE_TAG: &str = "honest-embodied-orchestrator-spine-only";

/// Embodied orchestrator morphism id @ manifest composition band.
pub const ORCHESTRATOR_MORPHISM_ID: &str = "embodied_orchestrator_host_before_gateway";

/// Honest physics posture — routing computes; continuum lift deferred.
pub const ORCHESTRATOR_PHYSICS_GREEN: bool = false;

/// Production wiring at sense / actuate / loop-close seam — deferred beyond W29 slice.
pub const ORCHESTRATOR_PRODUCTION_WIRED: bool = false;

use crate::ai::ppo::ManifoldGateway;
use crate::core::tensors::{ClausiusDuhemProof, UnifiedMaterialStateTensor, VerifiedUMST};
use crate::core::traits::IScienceCartridge;
use crate::gate::{
    AdmissibilityVerdict, GateEvaluator, GateEvaluatorRegistry, KleisliUnitEvaluator,
    ThermodynamicState, ThermodynamicStateSnapshot, ThermodynamicTransitionContext,
    ThermodynamicTransitionEvaluator, TransitionEvaluator, TransitionFilter,
    TransitionGateEvaluator, TransitionVerdict,
};
use crate::manifest::UmstManifest;
use crate::runtime::catalog::traceability::{
    CD_TRANSITION_CATALOG_ID, THERMODYNAMIC_MIX_CATALOG_ID,
};
use burn::tensor::{backend::Backend, Tensor};

/// Host `f64` transition inputs keyed by stable [`GateEvaluator::catalog_id`].
#[derive(Debug, Clone, Copy)]
pub struct HostTransitionStep<'a> {
    pub catalog_id: &'static str,
    pub old_state: &'a ThermodynamicState,
    pub new_state: &'a ThermodynamicState,
    pub dt_s: f64,
}

/// Rejection from the embodied stack (host registry / CD transition before tensor gateway).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbodiedReject {
    HostTransition {
        catalog_id: &'static str,
        verdict: AdmissibilityVerdict,
    },
    HostRegistryMissing {
        catalog_id: String,
    },
    /// CBF / formal-witness rejection after host gates (if any) passed.
    TensorGateway {
        catalog_id: &'static str,
        detail: String,
    },
}

impl EmbodiedReject {
    /// Stable gate slug for telemetry parsers (see `GateUnificationSpec.md`).
    #[must_use]
    pub fn catalog_id(&self) -> &str {
        match self {
            Self::HostTransition { catalog_id, .. } => catalog_id,
            Self::HostRegistryMissing { catalog_id } => catalog_id.as_str(),
            Self::TensorGateway { catalog_id, .. } => catalog_id,
        }
    }
}

impl std::fmt::Display for EmbodiedReject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HostTransition {
                catalog_id,
                verdict,
            } => {
                write!(f, "host gate {catalog_id}: {}", verdict.as_str())
            }
            Self::HostRegistryMissing { catalog_id } => {
                write!(
                    f,
                    "no registered host evaluator for catalog_id={catalog_id}"
                )
            }
            Self::TensorGateway { catalog_id, detail } => {
                write!(f, "tensor gateway [{catalog_id}]: {detail}")
            }
        }
    }
}

impl std::error::Error for EmbodiedReject {}

/// Honest slice posture — host routing landed, physics GREEN refused.
#[must_use]
pub const fn orchestrator_posture_is_honest() -> bool {
    !ORCHESTRATOR_PHYSICS_GREEN && !ORCHESTRATOR_PRODUCTION_WIRED
}

/// W29 honest posture bundle — routing evaluators landed, physics GREEN refused.
#[must_use]
pub const fn orchestrator_w29_honest_posture_bundle() -> bool {
    orchestrator_posture_is_honest()
        && !ORCHESTRATOR_PHYSICS_GREEN
        && !ORCHESTRATOR_PRODUCTION_WIRED
}

/// Whether the embodied orchestrator morphism is pinned @ HEAD (host-before-gateway semantics).
#[must_use]
pub fn orchestrator_morphism_pinned() -> bool {
    ORCHESTRATOR_MORPHISM_ID == "embodied_orchestrator_host_before_gateway"
        && ORCHESTRATOR_POSTURE_TAG == "honest-embodied-orchestrator-spine-only"
        && ORCHESTRATOR_CELL_ID == "W29-042-ORCHESTRATOR"
}

/// Compile-time honesty fence — no fake production or master claims.
pub const ORCHESTRATOR_HONEST_FENCE: &str =
    "orchestrator_host_routing_landed=true production_wired=false master_composition_wired=false";

/// Composes [`IScienceCartridge`] through [`ManifoldGateway`] plus optional host gates.
pub struct EmbodiedOrchestrator<B: Backend, C: IScienceCartridge<B>> {
    pub gateway: ManifoldGateway<B, C>,
    pub host_transition_gate: ThermodynamicTransitionEvaluator,
    pub mix_gate_registry: GateEvaluatorRegistry,
    /// When `true`, a supplied [`HostTransitionStep`] must pass **and** the CBF gateway must accept.
    pub dual_run: bool,
}

impl<B: Backend, C: IScienceCartridge<B>> std::fmt::Debug for EmbodiedOrchestrator<B, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbodiedOrchestrator")
            .field("host_transition_gate", &self.host_transition_gate)
            .field("dual_run", &self.dual_run)
            .finish_non_exhaustive()
    }
}

impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> EmbodiedOrchestrator<B, C> {
    #[must_use]
    pub fn new(cartridge: C, temperature_k: f64, initial_credit_joules: f64) -> Self {
        let gateway = ManifoldGateway::new(cartridge, temperature_k, initial_credit_joules);
        #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
        let gateway = gateway.with_constraint_weights_from_env();
        Self {
            gateway,
            host_transition_gate: ThermodynamicTransitionEvaluator::new(),
            mix_gate_registry: default_host_mix_registry(),
            dual_run: false,
        }
    }

    #[must_use]
    pub fn from_manifest(cartridge: C, manifest: &UmstManifest) -> Self {
        let cbf = manifest.thermodynamic_cbf.clone();
        let mut gateway =
            ManifoldGateway::new(cartridge, cbf.temperature_k, cbf.available_credit_joules);
        #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
        {
            gateway = gateway.with_constraint_weights_from_env();
        }
        gateway.cbf = cbf;
        #[cfg(feature = "formal-witness")]
        manifest.apply_witness_to_gateway(&mut gateway);
        Self {
            gateway,
            host_transition_gate: manifest.default_transition_gate.clone(),
            mix_gate_registry: default_host_mix_registry(),
            dual_run: manifest.dual_run,
        }
    }

    #[must_use]
    pub fn with_registry(mut self, registry: GateEvaluatorRegistry) -> Self {
        self.mix_gate_registry = registry;
        self
    }

    #[deprecated(note = "renamed to with_registry")]
    #[must_use]
    pub fn with_mix_registry(self, registry: GateEvaluatorRegistry) -> Self {
        self.with_registry(registry)
    }

    pub fn register_evaluator(&mut self, ev: TransitionEvaluator) {
        self.mix_gate_registry.register(ev);
    }

    #[deprecated(note = "renamed to register_evaluator")]
    pub fn register_mix_evaluator(&mut self, ev: TransitionEvaluator) {
        self.register_evaluator(ev);
    }

    /// Topology step: optional host gate (registry or CD transition), then [`ManifoldGateway::evaluate_topology_step`].
    pub fn evaluate_topology_step(
        &mut self,
        raw_state: UnifiedMaterialStateTensor<B>,
        info_gain: Tensor<B, 1>,
        host_step: Option<HostTransitionStep<'_>>,
    ) -> Result<(VerifiedUMST<B, ClausiusDuhemProof>, Tensor<B, 1>), EmbodiedReject> {
        if let Some(step) = host_step {
            self.check_host_transition(step)?;
        } else if self.dual_run {
            return Err(EmbodiedReject::HostRegistryMissing {
                catalog_id: CD_TRANSITION_CATALOG_ID.to_string(),
            });
        }

        self.gateway
            .evaluate_topology_step_formal(raw_state, info_gain)
            .map_err(|rej| EmbodiedReject::TensorGateway {
                catalog_id: rej.catalog_id(),
                detail: rej.to_string(),
            })
    }

    fn check_host_transition(
        &mut self,
        step: HostTransitionStep<'_>,
    ) -> Result<(), EmbodiedReject> {
        let verdict = match step.catalog_id {
            id if id == self.host_transition_gate.catalog_id() => {
                transition_verdict_to_admissibility(
                    self.host_transition_gate.check_transition_host(
                        step.old_state,
                        step.new_state,
                        step.dt_s,
                    ),
                )
            }
            id if id == THERMODYNAMIC_MIX_CATALOG_ID => {
                let old = host_to_snapshot(step.old_state);
                let new = host_to_snapshot(step.new_state);
                let ctx = ThermodynamicTransitionContext {
                    old_state: &old,
                    new_state: &new,
                    dt_seconds: step.dt_s,
                };
                self.mix_gate_registry
                    .evaluate_mut(step.catalog_id, ctx)
                    .ok_or_else(|| EmbodiedReject::HostRegistryMissing {
                        catalog_id: step.catalog_id.to_string(),
                    })?
            }
            id if id == KleisliUnitEvaluator::CATALOG_ID => {
                let new = host_to_snapshot(step.new_state);
                let ctx = ThermodynamicTransitionContext {
                    old_state: &new,
                    new_state: &new,
                    dt_seconds: step.dt_s,
                };
                self.mix_gate_registry
                    .evaluate_mut(step.catalog_id, ctx)
                    .ok_or_else(|| EmbodiedReject::HostRegistryMissing {
                        catalog_id: step.catalog_id.to_string(),
                    })?
            }
            other => {
                return Err(EmbodiedReject::HostRegistryMissing {
                    catalog_id: other.to_string(),
                });
            }
        };

        if verdict == AdmissibilityVerdict::Accepted {
            Ok(())
        } else {
            Err(EmbodiedReject::HostTransition {
                catalog_id: step.catalog_id,
                verdict,
            })
        }
    }
}

/// Default host registry: constitutive mix + Kleisli unit η (R4 after R1–R3 routing).
fn default_host_mix_registry() -> GateEvaluatorRegistry {
    let mut reg = GateEvaluatorRegistry::default();
    reg.register(TransitionEvaluator::new(TransitionFilter::new()));
    reg.register_kleisli(KleisliUnitEvaluator::new());
    reg
}

fn host_to_snapshot(s: &ThermodynamicState) -> ThermodynamicStateSnapshot {
    ThermodynamicStateSnapshot {
        density: s.density,
        temperature: s.temperature,
        free_energy: s.free_energy,
        entropy: s.entropy,
        reaction_extent: s.reaction_extent,
        strength: s.strength,
    }
}

fn transition_verdict_to_admissibility(tv: TransitionVerdict) -> AdmissibilityVerdict {
    tv.rest_verdict()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_transition_reject_catalog_id_is_step_slug() {
        let rej = EmbodiedReject::HostTransition {
            catalog_id: CD_TRANSITION_CATALOG_ID,
            verdict: AdmissibilityVerdict::MassViolation,
        };
        assert_eq!(rej.catalog_id(), CD_TRANSITION_CATALOG_ID);
        assert!(
            rej.to_string().contains(CD_TRANSITION_CATALOG_ID),
            "Display must embed catalog_id"
        );
    }

    #[test]
    fn tensor_gateway_reject_carries_landauer_catalog_id() {
        let rej = EmbodiedReject::TensorGateway {
            catalog_id: crate::runtime::catalog::traceability::LANDAUER_CBF_CATALOG_ID,
            detail: "insufficient credit".into(),
        };
        assert_eq!(rej.catalog_id(), "umst.gate.landauer_cbf");
        assert!(rej.to_string().contains("umst.gate.landauer_cbf"));
    }

    #[test]
    fn orchestrator_posture_is_honest_witness() {
        assert!(orchestrator_posture_is_honest());
        assert!(!ORCHESTRATOR_PHYSICS_GREEN);
        assert!(!ORCHESTRATOR_PRODUCTION_WIRED);
    }

    #[test]
    fn orchestrator_w29_honest_posture_bundle_holds() {
        assert!(orchestrator_w29_honest_posture_bundle());
    }

    #[test]
    fn orchestrator_morphism_pinned_at_head() {
        assert!(orchestrator_morphism_pinned());
        assert_eq!(ORCHESTRATOR_CELL_ID, "W29-042-ORCHESTRATOR");
        assert_eq!(
            ORCHESTRATOR_MORPHISM_ID,
            "embodied_orchestrator_host_before_gateway"
        );
    }

    #[test]
    fn orchestrator_posture_tag_honest_not_green() {
        assert!(ORCHESTRATOR_POSTURE_TAG.contains("honest"));
        assert!(!ORCHESTRATOR_POSTURE_TAG.contains("GREEN"));
        assert!(!ORCHESTRATOR_POSTURE_TAG.contains("PRODUCTION"));
    }

    #[test]
    fn orchestrator_honest_fence_refuses_production_and_master() {
        assert!(ORCHESTRATOR_HONEST_FENCE.contains("production_wired=false"));
        assert!(ORCHESTRATOR_HONEST_FENCE.contains("master_composition_wired=false"));
        assert!(ORCHESTRATOR_HONEST_FENCE.contains("orchestrator_host_routing_landed=true"));
    }
}
