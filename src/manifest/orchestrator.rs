// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Embodied policy loop: [`ManifoldGateway`] (tensor / CBF) composed with optional host gates via
//! [`GateEvaluatorRegistry`] and [`ThermodynamicTransitionEvaluator`] (see `docs/GateUnificationSpec.md`).

use crate::ai::ppo::ManifoldGateway;
use crate::core::tensors::{ClausiusDuhemProof, UnifiedMaterialStateTensor, VerifiedUMST};
use crate::core::traits::IScienceCartridge;
use crate::gate::{
    AdmissibilityVerdict, GateEvaluator, GateEvaluatorRegistry, KleisliUnitEvaluator,
    ThermodynamicMixEvaluator, ThermodynamicMixFilter, ThermodynamicState,
    ThermodynamicStateSnapshot, ThermodynamicTransitionContext, ThermodynamicTransitionEvaluator,
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

impl<B: Backend, C: IScienceCartridge<B>> EmbodiedOrchestrator<B, C> {
    #[must_use]
    pub fn new(cartridge: C, temperature_k: f64, initial_credit_joules: f64) -> Self {
        Self {
            gateway: ManifoldGateway::new(cartridge, temperature_k, initial_credit_joules),
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
    pub fn with_mix_registry(mut self, registry: GateEvaluatorRegistry) -> Self {
        self.mix_gate_registry = registry;
        self
    }

    pub fn register_mix_evaluator(&mut self, ev: ThermodynamicMixEvaluator) {
        self.mix_gate_registry.register(ev);
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
    reg.register(ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new()));
    reg.register_kleisli(KleisliUnitEvaluator::new());
    reg
}

fn host_to_snapshot(s: &ThermodynamicState) -> ThermodynamicStateSnapshot {
    ThermodynamicStateSnapshot {
        density: s.density,
        temperature: s.temperature,
        free_energy: s.free_energy,
        entropy: s.entropy,
        hydration_degree: s.hydration_degree,
        strength: s.strength,
    }
}

fn transition_verdict_to_admissibility(tv: TransitionVerdict) -> AdmissibilityVerdict {
    if tv.admissible {
        AdmissibilityVerdict::Accepted
    } else if !tv.mass_conserved {
        AdmissibilityVerdict::MassViolation
    } else if !tv.energy_positive {
        AdmissibilityVerdict::NegativeDissipation
    } else {
        AdmissibilityVerdict::Unknown
    }
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
}
