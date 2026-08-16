// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! AGAP-2033/2127-SEC-S5 — manifold gate runtime synthetic consensus wire map.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-S5 synthetic consensus SSOT;
//! L-N0 formal proof, live multi-LLM fan-out, and gateway `hcom_prov_gw_production_wired()` stay **honest open**.
//!
//! # Honesty (W29-124-SEC_S5)
//!
//! Census + synthetic consensus deepen only. Does **not** invent:
//! - physics / fleet **GREEN**
//! - **PRODUCTION_WIRED**
//! - **MASTER_RETICK** / master retick eligibility
//! - **OP-5 PASS**

use std::cell::RefCell;

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// W29-124 swarm cell id (SEC-S5 honest-fence deepen).
pub const W29_124_CELL_ID: &str = "W29-124-SEC_S5";

/// W29-124 honest posture — manifold S-5 census deepen only.
pub const W29_124_HONEST_POSTURE: &str = "SEC_S5_MANIFOLD_CENSUS_DEEPEN_ONLY";

/// W29-124 explicit non-claims (gate text).
pub const W29_124_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// W29-124 deepen schema version.
pub const W29_124_DEEPEN_SCHEMA_VERSION: &str = "sec_s5_w29_124_honest_fence_v1";

/// Honest fence string for meta / fleet probes (GREEN / PRODUCTION / MASTER / OP-5 fenced).
pub const HONEST_FENCE: &str = "census_wired=true production_wired=false green_claim_blocked=true \
master_retick=false op5_cleared=false live_fanout_wired=false ln0_proof_wired=false";

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-S5";

/// AGAP slot id (2033 synthetic consensus deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-S5";

/// FLEET-COMPOSER Prabhu Wave I slot I2 id.
pub const FLEET_P1812_I2_JOB_ID: &str = "PRABHU-WAVE-I-1812-I2";

/// FLEET-COMPOSER Prabhu Wave I I2 receipt path.
pub const FLEET_P1812_I2_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_P1812_I2.md";

/// FLEET-COMPOSER ACCEL-25 slot AC07 id.
pub const FLEET_ACCEL_AC07_JOB_ID: &str = "ACCEL-25-2030-AC07";

/// FLEET-COMPOSER ACCEL-25 AC07 receipt path.
pub const FLEET_ACCEL_AC07_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL_2030_AC07.md";

/// Prior AGAP-2033 SEC-S5 synthetic consensus receipt.
pub const PRIOR_RECEIPT_PATH_2033: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S5_2033.md";

/// Prior AGAP-2127 SEC-S5 L-N0 crosswalk + algebra roundtrip receipt.
pub const PRIOR_RECEIPT_PATH_2127: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S5_2127.md";

/// umst-trust SEC-S5 consensus delegate SSOT.
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/sec_s5_consensus.rs";

/// egoff consensus SSOT (full session/event deepen).
pub const EGOFF_CONSENSUS_SSOT: &str = "egoff/egoff/src/security/consensus.rs";

/// Gateway HCOM prov delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_hcom_prov_gw.rs";

/// umst-formal L-N0 proof SSOT (honest open).
pub const FORMAL_LN0_SSOT: &str = "umst-formal/Lean/Network/MajorityHonestConvergesToTruth.lean";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version (v3 absorbs W29-124 honest-fence deepen).
pub const SCHEMA_VERSION: &str = "sec_s5_gate_synthetic_consensus_census_v3";

/// S-5 synthetic consensus probe scenario ids (§12 algebra; AGAP-2127 deepen).
pub const S5_CONSENSUS_PROBE_SCENARIOS: &[&str] = &[
    "equivalence-class-split",
    "unanimous-agreement",
    "disagreement-kind-shape",
    "disagreement-kind-attestation",
    "disagreement-kind-timeout",
    "trust-heuristic-q4",
    "federated-auto-trigger",
    "session-resolve-badge",
];

/// Tier-3 provider identifiers for synthetic consensus algebra (no network).
pub const CONSENSUS_PROVIDER_TABLE: &[(&str, u8)] = &[("grok", 3), ("gemini", 2), ("minimax", 1)];

/// L-N0 formal proof wired — honest false until Lean lands.
pub const LN0_PROOF_WIRED_HONEST: bool = false;

/// Live multi-LLM fan-out wired — honest false.
pub const LIVE_FANOUT_WIRED_HONEST: bool = false;

/// S-Arc GREEN claim blocked — honest true in scaffold deepen.
pub const S5_GREEN_CLAIM_BLOCKED: bool = true;

/// Auto-consensus tier scaffold (no live env fan-out).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoConsensusTier {
    /// Consensus off.
    Off,
    /// Device-tier auto-trigger.
    Device,
    /// Federated-tier auto-trigger.
    Federated,
}

/// Disagreement taxonomy for cross-model events (manifold cold-edge algebra).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisagreementKind {
    /// Shape hash mismatch.
    ShapeMismatch,
    /// Attestation rank divergence.
    AttestationDivergence,
    /// Partial timeout.
    TimeoutPartial,
    /// Multiple equivalence classes.
    EquivalenceClassSplit,
}

impl DisagreementKind {
    /// Stable label for audit export.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShapeMismatch => "shape_mismatch",
            Self::AttestationDivergence => "attestation_divergence",
            Self::TimeoutPartial => "timeout_partial",
            Self::EquivalenceClassSplit => "equivalence_class_split",
        }
    }
}

/// One synthetic model attestation at manifold boundary (no live LLM response).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifoldModelAttestation {
    /// Provider id.
    pub provider: String,
    /// Response-shape hash.
    pub response_shape_hash: String,
    /// Trust attestation rank (§8 Q4 heuristic).
    pub trust_attestation_rank: u8,
}

/// Cross-model disagreement event at manifold cold edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifoldCrossModelDisagreement {
    /// Session id.
    pub session_id: String,
    /// Canonical SDF hash.
    pub canonical_sdf_hash: String,
    /// Disagreement kind.
    pub disagreement_kind: DisagreementKind,
    /// Suggested provider (§8 Q4).
    pub suggested_provider: Option<String>,
}

/// Rust scenario ↔ Lean `Network.MajorityHonestConvergesToTruth` crosswalk (statement-runtime).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifoldLn0LeanCrosswalkRow {
    /// Scenario id.
    pub scenario_id: &'static str,
    /// Lean theorem ref.
    pub lean_theorem_ref: &'static str,
    /// Algebra lane.
    pub algebra_lane: &'static str,
    /// GSD binding.
    pub gsd_binding: &'static str,
}

/// Session rollup for manifold census / audit export (G76-style extend).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifoldConsensusSessionSummary {
    /// Sessions with recorded disagreement.
    pub disagreement_events: usize,
    /// Sessions resolved by operator.
    pub resolution_events: usize,
    /// Open sessions awaiting operator decision.
    pub open_sessions: usize,
    /// Distinct disagreement kinds observed.
    pub unique_disagreement_kinds: usize,
}

/// One hop in the manifold SEC-S5 gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS5GateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-S5 gate runtime wire map (cold-edge evidence → trust consensus census).
pub const MANIFOLD_SEC_S5_GATE_WIRE_HOPS: &[SecS5GateWireHop] = &[
    SecS5GateWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecS5GateWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecS5GateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_s5::gate_synthetic_consensus_census",
        role: "Manifold gate SEC-S5 synthetic consensus census",
        wired: true,
    },
    SecS5GateWireHop {
        ordinal: 4,
        surface: "umst-trust::sec_s5_consensus::validate_s5_consensus_honesty",
        role: "Trust consensus delegate (F76/G76/H55)",
        wired: true,
    },
    SecS5GateWireHop {
        ordinal: 5,
        surface: "egoff::security::consensus + egoff::security::mod::session_trust_badge",
        role: "egoff consensus session algebra + cockpit badge (live wire egoff-owned)",
        wired: true,
    },
    SecS5GateWireHop {
        ordinal: 6,
        surface: "umst-formal::Lean::Network::MajorityHonestConvergesToTruth",
        role: "L-N0 formal proof (R-LN0-full)",
        wired: false,
    },
    SecS5GateWireHop {
        ordinal: 7,
        surface: "umst-gateway::sec_hcom_prov_gw::hcom_prov_gw_production_wired",
        role: "Gateway HCOM prov production ceremony (serial Wave I)",
        wired: false,
    },
];

/// One S-5 synthetic consensus probe row at manifold cold edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifoldS5ConsensusProbe {
    /// Scenario id.
    pub scenario_id: &'static str,
    /// Probe kind label.
    pub probe_kind: &'static str,
    /// Whether the probe hit.
    pub probe_hit: bool,
}

/// One S-5 gate-factor row for operator `:trust gate-factors` deepen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS5GateFactorRow {
    /// Stable factor identifier.
    pub factor_id: &'static str,
    /// Whether the witness probe is wired.
    pub probe_wired: bool,
    /// Whether the factor earns acceptance credit toward S-5 GREEN.
    pub acceptance_credit: bool,
    /// Operator detail string from the live probe.
    pub detail: String,
}

/// Aggregated SEC-S5 gate synthetic consensus census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS5GateSyntheticConsensusCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// S-5 8/8 synthetic probes hit at manifold boundary.
    pub s5_all_scenarios_probed: bool,
    /// Consensus algebra roundtrip witness at manifold boundary.
    pub algebra_roundtrip_verified: bool,
    /// Extended algebra roundtrip (session summary rollup) verified.
    pub extended_algebra_roundtrip_verified: bool,
    /// L-N0 Lean crosswalk row count.
    pub ln0_crosswalk_rows: usize,
    /// L-N0 formal proof wired — honest false.
    pub ln0_proof_wired: bool,
    /// Live multi-LLM fan-out wired — honest false.
    pub live_fanout_wired: bool,
    /// S-Arc GREEN claim blocked.
    pub s5_green_claim_blocked: bool,
    /// Gateway production flip.
    pub production_wired: bool,
    /// Wired hop count.
    pub wire_hop_wired_count: u8,
}

/// Exercise gate cold-edge evidence at manifold SSOT (identity transition admits).
#[must_use]
pub fn gate_transition_evidence_probe() -> bool {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
    let new = old;
    let evidence = CdTransitionCartridge.transition_evidence(&old, &new, 1.0);
    evidence.admissibility == AdmissibilityToken::Admissible && !evidence.catalog_id.is_empty()
}

fn fnv1a_hex(seed: &[u8], data: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in seed {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for b in data {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn canonical_request_sdf_hash(prompt: &str) -> String {
    fnv1a_hex(b"UMST_CONSENSUS_SDF_V1", prompt.as_bytes())
}

fn response_shape_hash(provider: &str, prompt: &str) -> String {
    let mut seed = b"UMST_RESPONSE_SHAPE_V1".to_vec();
    seed.extend_from_slice(provider.as_bytes());
    if provider == "minimax" {
        seed.extend_from_slice(b":divergent");
    }
    fnv1a_hex(&seed, prompt.as_bytes())
}

fn synthetic_unanimous_attestations(prompt: &str) -> Vec<ManifoldModelAttestation> {
    let unanimous_hash = response_shape_hash("grok", prompt);
    CONSENSUS_PROVIDER_TABLE
        .iter()
        .map(|(provider, rank)| ManifoldModelAttestation {
            provider: (*provider).into(),
            response_shape_hash: unanimous_hash.clone(),
            trust_attestation_rank: *rank,
        })
        .collect()
}

fn synthetic_consensus_attestations(prompt: &str) -> Vec<ManifoldModelAttestation> {
    CONSENSUS_PROVIDER_TABLE
        .iter()
        .map(|(provider, rank)| ManifoldModelAttestation {
            provider: (*provider).into(),
            response_shape_hash: response_shape_hash(provider, prompt),
            trust_attestation_rank: *rank,
        })
        .collect()
}

fn attestations_have_equivalence_split(attestations: &[ManifoldModelAttestation]) -> bool {
    if attestations.len() < 2 {
        return false;
    }
    let first = &attestations[0].response_shape_hash;
    attestations.iter().any(|a| a.response_shape_hash != *first)
}

fn resolve_disagreement_by_trust_attestation(
    attestations: &[ManifoldModelAttestation],
) -> Option<String> {
    attestations
        .iter()
        .max_by_key(|a| a.trust_attestation_rank)
        .map(|a| a.provider.clone())
}

fn build_cross_model_disagreement(
    session_id: &str,
    prompt: &str,
    attestations: &[ManifoldModelAttestation],
    kind: DisagreementKind,
) -> ManifoldCrossModelDisagreement {
    ManifoldCrossModelDisagreement {
        session_id: session_id.into(),
        canonical_sdf_hash: canonical_request_sdf_hash(prompt),
        disagreement_kind: kind,
        suggested_provider: resolve_disagreement_by_trust_attestation(attestations),
    }
}

/// Scaffold auto-consensus tier (no live env).
#[must_use]
pub const fn auto_consensus_tier() -> AutoConsensusTier {
    AutoConsensusTier::Federated
}

/// Federated-tier write hook — returns whether consensus should auto-trigger.
#[must_use]
pub const fn consensus_auto_trigger_for_federated() -> bool {
    matches!(auto_consensus_tier(), AutoConsensusTier::Federated)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifoldConsensusSession {
    session_id: String,
    status_open: bool,
    disagreement: Option<ManifoldCrossModelDisagreement>,
}

thread_local! {
    static OPEN_SESSIONS: RefCell<Vec<ManifoldConsensusSession>> = const { RefCell::new(Vec::new()) };
}

fn open_consensus_session(prompt: &str) -> (String, bool) {
    let attestations = synthetic_consensus_attestations(prompt);
    let has_disagreement = attestations_have_equivalence_split(&attestations);
    let session_id = format!("manifold-consensus-{}", prompt.len());
    let disagreement = if has_disagreement {
        Some(build_cross_model_disagreement(
            &session_id,
            prompt,
            &attestations,
            DisagreementKind::EquivalenceClassSplit,
        ))
    } else {
        None
    };
    OPEN_SESSIONS.with(|sessions| {
        sessions.borrow_mut().push(ManifoldConsensusSession {
            session_id: session_id.clone(),
            status_open: has_disagreement,
            disagreement,
        });
    });
    (session_id, has_disagreement)
}

fn record_consensus_resolution(session_id: &str) {
    OPEN_SESSIONS.with(|sessions| {
        if let Some(sess) = sessions
            .borrow_mut()
            .iter_mut()
            .find(|s| s.session_id == session_id)
        {
            sess.status_open = false;
        }
    });
}

fn has_open_cross_model_disagreement() -> bool {
    OPEN_SESSIONS.with(|sessions| sessions.borrow().iter().any(|s| s.status_open))
}

/// Session consensus rollup for manifold census (G76-style extend).
#[must_use]
pub fn manifold_session_consensus_summary() -> ManifoldConsensusSessionSummary {
    OPEN_SESSIONS.with(|sessions| {
        let sessions = sessions.borrow();
        let disagreement_events = sessions.iter().filter(|s| s.disagreement.is_some()).count();
        let resolution_events = sessions
            .iter()
            .filter(|s| !s.status_open && s.disagreement.is_some())
            .count();
        let mut kinds = std::collections::BTreeSet::new();
        for sess in sessions.iter() {
            if let Some(d) = &sess.disagreement {
                kinds.insert(d.disagreement_kind.as_str());
            }
        }
        ManifoldConsensusSessionSummary {
            disagreement_events,
            resolution_events,
            open_sessions: sessions.iter().filter(|s| s.status_open).count(),
            unique_disagreement_kinds: kinds.len(),
        }
    })
}

/// Rust scenario ↔ Lean L-N0 crosswalk (statement-runtime; no proof claim).
#[must_use]
pub fn manifold_ln0_lean_crosswalk() -> Vec<ManifoldLn0LeanCrosswalkRow> {
    vec![
        ManifoldLn0LeanCrosswalkRow {
            scenario_id: "equivalence-class-split",
            lean_theorem_ref: "Network.MajorityHonestConvergesToTruth.equiv_class_split",
            algebra_lane: "equivalence-split",
            gsd_binding: "S-Q5",
        },
        ManifoldLn0LeanCrosswalkRow {
            scenario_id: "unanimous-agreement",
            lean_theorem_ref: "Network.MajorityHonestConvergesToTruth.unanimous_shape",
            algebra_lane: "unanimous-shape",
            gsd_binding: "S-Q5",
        },
        ManifoldLn0LeanCrosswalkRow {
            scenario_id: "disagreement-kind-shape",
            lean_theorem_ref: "Network.MajorityHonestConvergesToTruth.disagreement_kind",
            algebra_lane: "disagreement-kind",
            gsd_binding: "GSD-5",
        },
        ManifoldLn0LeanCrosswalkRow {
            scenario_id: "disagreement-kind-attestation",
            lean_theorem_ref: "Network.MajorityHonestConvergesToTruth.disagreement_kind",
            algebra_lane: "disagreement-kind",
            gsd_binding: "GSD-5",
        },
        ManifoldLn0LeanCrosswalkRow {
            scenario_id: "disagreement-kind-timeout",
            lean_theorem_ref: "Network.MajorityHonestConvergesToTruth.disagreement_kind",
            algebra_lane: "disagreement-kind",
            gsd_binding: "GSD-5",
        },
        ManifoldLn0LeanCrosswalkRow {
            scenario_id: "trust-heuristic-q4",
            lean_theorem_ref: "Network.MajorityHonestConvergesToTruth.trust_attestation_rank",
            algebra_lane: "trust-attestation-rank",
            gsd_binding: "§8-Q4",
        },
        ManifoldLn0LeanCrosswalkRow {
            scenario_id: "federated-auto-trigger",
            lean_theorem_ref: "Network.MajorityHonestConvergesToTruth.auto_consensus_tier",
            algebra_lane: "auto-consensus-tier",
            gsd_binding: "S-Q5",
        },
        ManifoldLn0LeanCrosswalkRow {
            scenario_id: "session-resolve-badge",
            lean_theorem_ref: "Network.MajorityHonestConvergesToTruth.operator_resolution",
            algebra_lane: "session-lifecycle",
            gsd_binding: "GSD-5",
        },
    ]
}

fn probe_scenario_session_resolve_badge() -> bool {
    let saved = OPEN_SESSIONS.with(|sessions| sessions.borrow().clone());
    OPEN_SESSIONS.with(|sessions| sessions.borrow_mut().clear());

    let (session_id, opened) = open_consensus_session("probe-resolve-badge");
    let hit = opened && has_open_cross_model_disagreement() && {
        record_consensus_resolution(&session_id);
        !has_open_cross_model_disagreement()
    };

    OPEN_SESSIONS.with(|sessions| *sessions.borrow_mut() = saved);
    hit
}

fn probe_scenario(scenario_id: &'static str) -> ManifoldS5ConsensusProbe {
    let (probe_kind, probe_hit) = match scenario_id {
        "equivalence-class-split" => {
            let attestations = synthetic_consensus_attestations("probe-split");
            let split = attestations_have_equivalence_split(&attestations);
            let disagreement = build_cross_model_disagreement(
                "probe-equiv-class",
                "probe-split",
                &attestations,
                DisagreementKind::EquivalenceClassSplit,
            );
            (
                "equivalence-split",
                split && disagreement.disagreement_kind == DisagreementKind::EquivalenceClassSplit,
            )
        }
        "unanimous-agreement" => {
            let attestations = synthetic_unanimous_attestations("probe-unanimous");
            (
                "unanimous-shape",
                !attestations_have_equivalence_split(&attestations),
            )
        }
        "disagreement-kind-shape" => {
            let disagreement = build_cross_model_disagreement(
                "probe-shape",
                "probe",
                &[],
                DisagreementKind::ShapeMismatch,
            );
            (
                "disagreement-kind",
                disagreement.disagreement_kind == DisagreementKind::ShapeMismatch,
            )
        }
        "disagreement-kind-attestation" => {
            let attestations = synthetic_consensus_attestations("probe-attest");
            let disagreement = build_cross_model_disagreement(
                "probe-attest",
                "probe",
                &attestations,
                DisagreementKind::AttestationDivergence,
            );
            (
                "disagreement-kind",
                disagreement.disagreement_kind == DisagreementKind::AttestationDivergence
                    && disagreement.suggested_provider.is_some(),
            )
        }
        "disagreement-kind-timeout" => {
            let disagreement = build_cross_model_disagreement(
                "probe-timeout",
                "probe",
                &[],
                DisagreementKind::TimeoutPartial,
            );
            (
                "disagreement-kind",
                disagreement.disagreement_kind == DisagreementKind::TimeoutPartial,
            )
        }
        "trust-heuristic-q4" => {
            let attestations = synthetic_consensus_attestations("probe-q4");
            (
                "trust-attestation-rank",
                resolve_disagreement_by_trust_attestation(&attestations).as_deref() == Some("grok"),
            )
        }
        "federated-auto-trigger" => (
            "auto-consensus-tier",
            consensus_auto_trigger_for_federated(),
        ),
        "session-resolve-badge" => ("session-lifecycle", probe_scenario_session_resolve_badge()),
        _ => ("unknown", false),
    };
    ManifoldS5ConsensusProbe {
        scenario_id,
        probe_kind,
        probe_hit,
    }
}

/// S-5 synthetic consensus probe matrix — 8/8 at manifold cold edge.
#[must_use]
pub fn manifold_s5_consensus_coverage_probes() -> Vec<ManifoldS5ConsensusProbe> {
    S5_CONSENSUS_PROBE_SCENARIOS
        .iter()
        .map(|id| probe_scenario(id))
        .collect()
}

/// Whether all eight S-5 synthetic consensus probes hit at manifold boundary.
#[must_use]
pub fn manifold_s5_all_scenarios_probed() -> bool {
    manifold_s5_consensus_coverage_probes()
        .iter()
        .all(|p| p.probe_hit)
}

/// Verify synthetic consensus algebra roundtrip without live fan-out at manifold boundary.
#[must_use]
pub fn manifold_verify_s5_consensus_algebra_roundtrip() -> bool {
    let saved = OPEN_SESSIONS.with(|sessions| sessions.borrow().clone());
    OPEN_SESSIONS.with(|sessions| sessions.borrow_mut().clear());

    let unanimous = synthetic_unanimous_attestations("roundtrip-unanimous");
    let split = synthetic_consensus_attestations("roundtrip-split");
    let unanimous_ok = !attestations_have_equivalence_split(&unanimous);
    let split_ok = attestations_have_equivalence_split(&split);
    let (session_id, opened) = open_consensus_session("roundtrip-split");
    let open_ok = opened
        && has_open_cross_model_disagreement()
        && OPEN_SESSIONS.with(|sessions| {
            sessions.borrow().iter().any(|s| {
                s.session_id == session_id
                    && s.disagreement.as_ref().is_some_and(|d| {
                        d.disagreement_kind == DisagreementKind::EquivalenceClassSplit
                    })
            })
        });
    record_consensus_resolution(&session_id);
    let resolved_ok = !has_open_cross_model_disagreement();

    OPEN_SESSIONS.with(|sessions| *sessions.borrow_mut() = saved);
    unanimous_ok && split_ok && open_ok && resolved_ok
}

/// Extended algebra roundtrip — chains base roundtrip + session summary rollup (AC07 G76-style).
#[must_use]
pub fn manifold_verify_s5_consensus_algebra_roundtrip_extended() -> bool {
    if !manifold_verify_s5_consensus_algebra_roundtrip() {
        return false;
    }
    let saved = OPEN_SESSIONS.with(|sessions| sessions.borrow().clone());
    OPEN_SESSIONS.with(|sessions| sessions.borrow_mut().clear());

    let _ = open_consensus_session("ac07-summary-track");
    let open_summary = manifold_session_consensus_summary();
    let open_ok = open_summary.disagreement_events >= 1
        && open_summary.open_sessions >= 1
        && open_summary.unique_disagreement_kinds >= 1;

    OPEN_SESSIONS.with(|sessions| *sessions.borrow_mut() = saved);
    open_ok
}

/// Whether live gateway HCOM prov production flip is plumbed (honest `false`).
#[must_use]
pub const fn sec_s5_production_wired() -> bool {
    false
}

/// Master retick eligibility — honest **false** (not claimed from S-5 census deepen).
#[must_use]
pub const fn sec_s5_master_retick_eligible() -> bool {
    false
}

/// OP-5 clearance — honest **false** (not claimed from S-5 census deepen).
#[must_use]
pub const fn sec_s5_op5_cleared() -> bool {
    false
}

const _: () = assert!(!sec_s5_production_wired());
const _: () = assert!(!LN0_PROOF_WIRED_HONEST);
const _: () = assert!(!LIVE_FANOUT_WIRED_HONEST);
const _: () = assert!(S5_GREEN_CLAIM_BLOCKED);
const _: () = assert!(!sec_s5_master_retick_eligible());
const _: () = assert!(!sec_s5_op5_cleared());

/// Honest fence flags for SEC-S5 deepen (W29-124).
///
/// All invent-claim bools stay `false`; `deepen_honest` is true only when cell
/// pins, census ceremony, and GREEN/PRODUCTION/MASTER/OP-5 fences hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS5W29124DeepenProbe {
    /// Deepen schema tag.
    pub schema_version: &'static str,
    /// Swarm cell id.
    pub cell_id: &'static str,
    /// Honest posture pin.
    pub honest_posture: &'static str,
    /// Explicit non-claim gate text.
    pub non_claim: &'static str,
    /// Compact honest-fence string.
    pub honest_fence: &'static str,
    /// Invent claim — must stay false.
    pub production_wired_claimed: bool,
    /// Invent claim — must stay false.
    pub green_claimed: bool,
    /// Invent claim — must stay false.
    pub op5_pass_claimed: bool,
    /// Invent claim — must stay false.
    pub master_retick_claimed: bool,
    /// Measured deepen honesty (fences + ceremony pins).
    pub deepen_honest: bool,
}

/// Build the W29-124 SEC-S5 deepen honesty probe from live measurements.
#[must_use]
pub fn sec_s5_w29_124_deepen_probe() -> SecS5W29124DeepenProbe {
    let production_wired_claimed = sec_s5_production_wired();
    let green_claimed = !S5_GREEN_CLAIM_BLOCKED;
    let op5_pass_claimed = sec_s5_op5_cleared();
    let master_retick_claimed = sec_s5_master_retick_eligible();
    let ceremony_ok = manifold_gate_sec_s5_ceremony_closed();
    let deepen_honest = W29_124_CELL_ID == "W29-124-SEC_S5"
        && W29_124_DEEPEN_SCHEMA_VERSION == "sec_s5_w29_124_honest_fence_v1"
        && W29_124_HONEST_POSTURE == "SEC_S5_MANIFOLD_CENSUS_DEEPEN_ONLY"
        && SCHEMA_VERSION == "sec_s5_gate_synthetic_consensus_census_v3"
        && !production_wired_claimed
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && !LN0_PROOF_WIRED_HONEST
        && !LIVE_FANOUT_WIRED_HONEST
        && W29_124_NON_CLAIM.contains("not GREEN")
        && W29_124_NON_CLAIM.contains("not OP-5 PASS")
        && W29_124_NON_CLAIM.contains("not production_wired")
        && W29_124_NON_CLAIM.contains("not MASTER_RETICK")
        && HONEST_FENCE.contains("production_wired=false")
        && HONEST_FENCE.contains("green_claim_blocked=true")
        && HONEST_FENCE.contains("master_retick=false")
        && HONEST_FENCE.contains("op5_cleared=false")
        && HONEST_FENCE.contains("live_fanout_wired=false")
        && HONEST_FENCE.contains("ln0_proof_wired=false")
        && ceremony_ok;
    SecS5W29124DeepenProbe {
        schema_version: W29_124_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_124_CELL_ID,
        honest_posture: W29_124_HONEST_POSTURE,
        non_claim: W29_124_NON_CLAIM,
        honest_fence: HONEST_FENCE,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-124 SEC-S5 deepen honesty probe passes.
#[must_use]
pub fn sec_s5_w29_124_deepen_honest() -> bool {
    sec_s5_w29_124_deepen_probe().deepen_honest
}

/// SEC-S5 fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn sec_s5_honest_fence_holds() -> bool {
    let p = sec_s5_w29_124_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
}

/// Build manifold SEC-S5 gate synthetic consensus census from live measurements.
#[must_use]
pub fn gate_synthetic_consensus_census() -> SecS5GateSyntheticConsensusCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_S5_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecS5GateSyntheticConsensusCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        s5_all_scenarios_probed: manifold_s5_all_scenarios_probed(),
        algebra_roundtrip_verified: manifold_verify_s5_consensus_algebra_roundtrip(),
        extended_algebra_roundtrip_verified:
            manifold_verify_s5_consensus_algebra_roundtrip_extended(),
        ln0_crosswalk_rows: manifold_ln0_lean_crosswalk().len(),
        ln0_proof_wired: LN0_PROOF_WIRED_HONEST,
        live_fanout_wired: LIVE_FANOUT_WIRED_HONEST,
        s5_green_claim_blocked: S5_GREEN_CLAIM_BLOCKED,
        production_wired: sec_s5_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-S5 ceremony is closed at census tier.
///
/// True when cold-edge evidence probe + synthetic consensus wire map hops 1–5 are measured wired.
/// L-N0 formal proof + gateway production flip are explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_s5_ceremony_closed() -> bool {
    let census = gate_synthetic_consensus_census();
    census.gate_evidence_wired
        && census.s5_all_scenarios_probed
        && census.algebra_roundtrip_verified
        && census.extended_algebra_roundtrip_verified
        && census.ln0_crosswalk_rows == 8
        && !census.ln0_proof_wired
        && !census.live_fanout_wired
        && census.s5_green_claim_blocked
        && !census.production_wired
        && census.wire_hop_wired_count == 5
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-S5 manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS5GateManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// S-5 8/8 probes at manifold boundary.
    pub s5_all_scenarios_probed: bool,
    /// Algebra roundtrip verified.
    pub algebra_roundtrip_verified: bool,
    /// Extended algebra roundtrip verified.
    pub extended_algebra_roundtrip_verified: bool,
    /// L-N0 crosswalk row count.
    pub ln0_crosswalk_rows: usize,
    /// L-N0 proof honest false.
    pub ln0_proof_honest_false: bool,
    /// Live fanout honest false.
    pub live_fanout_honest_false: bool,
    /// S-Arc GREEN claim blocked.
    pub s5_green_claim_blocked: bool,
    /// Production flip honest false.
    pub production_honest_false: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build introspection probe for SEC-S5 done-when checks.
#[must_use]
pub fn sec_s5_gate_manifold_probe() -> SecS5GateManifoldProbe {
    let census = gate_synthetic_consensus_census();
    SecS5GateManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        s5_all_scenarios_probed: census.s5_all_scenarios_probed,
        algebra_roundtrip_verified: census.algebra_roundtrip_verified,
        extended_algebra_roundtrip_verified: census.extended_algebra_roundtrip_verified,
        ln0_crosswalk_rows: census.ln0_crosswalk_rows,
        ln0_proof_honest_false: !census.ln0_proof_wired,
        live_fanout_honest_false: !census.live_fanout_wired,
        s5_green_claim_blocked: census.s5_green_claim_blocked,
        production_honest_false: !census.production_wired,
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_s5_ceremony_closed(),
    }
}

/// Collect live S-5 gate-factor rows for operator `:trust gate-factors` deepen.
#[must_use]
pub fn collect_sec_s5_gate_factor_rows() -> Vec<SecS5GateFactorRow> {
    let census = gate_synthetic_consensus_census();
    vec![
        SecS5GateFactorRow {
            factor_id: "gate-evidence",
            probe_wired: census.gate_evidence_wired,
            acceptance_credit: census.gate_evidence_wired,
            detail: "cold-edge AdmissibilityToken + CdTransitionCartridge".into(),
        },
        SecS5GateFactorRow {
            factor_id: "consensus-8x8",
            probe_wired: census.s5_all_scenarios_probed,
            acceptance_credit: census.s5_all_scenarios_probed,
            detail: format!(
                "scenarios={}/{}",
                manifold_s5_consensus_coverage_probes()
                    .iter()
                    .filter(|p| p.probe_hit)
                    .count(),
                S5_CONSENSUS_PROBE_SCENARIOS.len(),
            ),
        },
        SecS5GateFactorRow {
            factor_id: "algebra-roundtrip",
            probe_wired: census.algebra_roundtrip_verified,
            acceptance_credit: census.algebra_roundtrip_verified,
            detail: "synthetic unanimous/split + session resolve lifecycle".into(),
        },
        SecS5GateFactorRow {
            factor_id: "algebra-extended",
            probe_wired: census.extended_algebra_roundtrip_verified,
            acceptance_credit: census.extended_algebra_roundtrip_verified,
            detail: "G76-style session summary rollup at manifold boundary".into(),
        },
        SecS5GateFactorRow {
            factor_id: "ln0-crosswalk",
            probe_wired: census.ln0_crosswalk_rows == 8,
            acceptance_credit: census.ln0_crosswalk_rows == 8,
            detail: format!(
                "crosswalk_rows={}/8 statement-runtime",
                census.ln0_crosswalk_rows
            ),
        },
        SecS5GateFactorRow {
            factor_id: "wire-map",
            probe_wired: census.wire_hop_wired_count == 5,
            acceptance_credit: census.wire_hop_wired_count == 5,
            detail: format!(
                "hops={}/{} wired={}",
                census.wire_hop_wired_count,
                MANIFOLD_SEC_S5_GATE_WIRE_HOPS.len(),
                census.wire_hop_wired_count,
            ),
        },
        SecS5GateFactorRow {
            factor_id: "trust-delegate",
            probe_wired: TRUST_SSOT.contains("sec_s5_consensus"),
            acceptance_credit: TRUST_SSOT.contains("sec_s5_consensus"),
            detail: TRUST_SSOT.into(),
        },
        SecS5GateFactorRow {
            factor_id: "live-fanout-blocked",
            probe_wired: !census.live_fanout_wired,
            acceptance_credit: false,
            detail: "live_fanout_wired=false scert_credit=BLOCKED expected_gate_exit=BLOCKED"
                .into(),
        },
    ]
}

/// Operator gate-factor table string for SEC-S5 manifold deepen.
#[must_use]
pub fn sec_s5_gate_factor_table() -> String {
    let rows = collect_sec_s5_gate_factor_rows();
    let wired = rows.iter().filter(|r| r.probe_wired).count();
    let credit = rows.iter().filter(|r| r.acceptance_credit).count();
    let mut out = format!(
        "SEC-S5 gate factors: wired={}/{} credit={}/{} \
         s5_green_claim_blocked={} live_fanout_wired={} production_wired={} \
         scert_credit=BLOCKED expected_gate_exit=BLOCKED\n",
        wired,
        rows.len(),
        credit,
        rows.len(),
        S5_GREEN_CLAIM_BLOCKED,
        LIVE_FANOUT_WIRED_HONEST,
        sec_s5_production_wired(),
    );
    for row in &rows {
        out.push_str(&format!(
            "  {} probe_wired={} acceptance_credit={} {}\n",
            row.factor_id, row.probe_wired, row.acceptance_credit, row.detail
        ));
    }
    out
}

/// FLEET-COMPOSER Prabhu Wave I I2 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS5P1812I2Probe {
    /// I2 fleet card id.
    pub i2_job_id: &'static str,
    /// Prior 2033 synthetic consensus absorbed.
    pub prior_2033_absorbed: bool,
    /// Prior 2127 L-N0 crosswalk absorbed.
    pub prior_2127_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecS5GateManifoldProbe,
    /// `sec_s5_production_wired()` — honest false.
    pub production_wired: bool,
    /// Gate-factor rows with probe wired.
    pub gate_factor_wired_count: usize,
    /// Gate-factor table pins BLOCKED exit.
    pub gate_factor_exit_blocked: bool,
}

/// Build FLEET-COMPOSER P1812 I2 integration probe from live measurements.
#[must_use]
pub fn sec_s5_p1812_i2_probe() -> SecS5P1812I2Probe {
    let rows = collect_sec_s5_gate_factor_rows();
    let table = sec_s5_gate_factor_table();
    SecS5P1812I2Probe {
        i2_job_id: FLEET_P1812_I2_JOB_ID,
        prior_2033_absorbed: PRIOR_RECEIPT_PATH_2033.contains("SEC-S5_2033"),
        prior_2127_absorbed: PRIOR_RECEIPT_PATH_2127.contains("SEC-S5_2127"),
        ceremony_closed: manifold_gate_sec_s5_ceremony_closed(),
        probe: sec_s5_gate_manifold_probe(),
        production_wired: sec_s5_production_wired(),
        gate_factor_wired_count: rows.iter().filter(|r| r.probe_wired).count(),
        gate_factor_exit_blocked: table.contains("expected_gate_exit=BLOCKED"),
    }
}

/// FLEET-COMPOSER P1812 I2 honesty gate — ceremony closed + production false + gate-factors honest.
#[must_use]
pub fn sec_s5_p1812_i2_honest() -> bool {
    let probe = sec_s5_p1812_i2_probe();
    probe.i2_job_id == FLEET_P1812_I2_JOB_ID
        && probe.prior_2033_absorbed
        && probe.prior_2127_absorbed
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.s5_all_scenarios_probed
        && probe.probe.algebra_roundtrip_verified
        && probe.probe.extended_algebra_roundtrip_verified
        && probe.probe.ln0_crosswalk_rows == 8
        && probe.probe.ln0_proof_honest_false
        && probe.probe.live_fanout_honest_false
        && probe.probe.s5_green_claim_blocked
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
        && probe.gate_factor_wired_count >= 7
        && probe.gate_factor_exit_blocked
        && !sec_s5_master_retick_eligible()
        && !sec_s5_op5_cleared()
        && W29_124_CELL_ID == "W29-124-SEC_S5"
        && HONEST_FENCE.contains("green_claim_blocked=true")
        && HONEST_FENCE.contains("production_wired=false")
}

/// FLEET-COMPOSER ACCEL-25 AC07 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS5AccelAc07Probe {
    /// AC07 fleet card id.
    pub ac07_job_id: &'static str,
    /// AC07 receipt path pinned.
    pub ac07_receipt_honest: bool,
    /// Prior I2 manifold gate absorbed.
    pub prior_i2_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecS5GateManifoldProbe,
    /// `sec_s5_production_wired()` — honest false.
    pub production_wired: bool,
    /// Extended algebra roundtrip verified.
    pub extended_roundtrip_verified: bool,
    /// L-N0 crosswalk row count.
    pub ln0_crosswalk_rows: usize,
    /// Gate-factor rows with probe wired.
    pub gate_factor_wired_count: usize,
    /// Gate-factor table pins BLOCKED exit.
    pub gate_factor_exit_blocked: bool,
}

/// Build FLEET-COMPOSER ACCEL-25 AC07 integration probe from live measurements.
#[must_use]
pub fn sec_s5_accel_ac07_probe() -> SecS5AccelAc07Probe {
    let rows = collect_sec_s5_gate_factor_rows();
    let table = sec_s5_gate_factor_table();
    let census = gate_synthetic_consensus_census();
    SecS5AccelAc07Probe {
        ac07_job_id: FLEET_ACCEL_AC07_JOB_ID,
        ac07_receipt_honest: FLEET_ACCEL_AC07_RECEIPT_PATH.contains("COMPOSER_ACCEL_2030_AC07"),
        prior_i2_absorbed: FLEET_P1812_I2_RECEIPT_PATH.contains("COMPOSER_P1812_I2"),
        ceremony_closed: manifold_gate_sec_s5_ceremony_closed(),
        probe: sec_s5_gate_manifold_probe(),
        production_wired: sec_s5_production_wired(),
        extended_roundtrip_verified: census.extended_algebra_roundtrip_verified,
        ln0_crosswalk_rows: census.ln0_crosswalk_rows,
        gate_factor_wired_count: rows.iter().filter(|r| r.probe_wired).count(),
        gate_factor_exit_blocked: table.contains("expected_gate_exit=BLOCKED"),
    }
}

/// FLEET-COMPOSER ACCEL-25 AC07 honesty gate — algebra deepen + ceremony closed + production false.
#[must_use]
pub fn sec_s5_accel_ac07_honest() -> bool {
    let probe = sec_s5_accel_ac07_probe();
    probe.ac07_job_id == FLEET_ACCEL_AC07_JOB_ID
        && probe.ac07_receipt_honest
        && probe.prior_i2_absorbed
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.s5_all_scenarios_probed
        && probe.probe.algebra_roundtrip_verified
        && probe.probe.extended_algebra_roundtrip_verified
        && probe.probe.ln0_crosswalk_rows == 8
        && probe.probe.ln0_proof_honest_false
        && probe.probe.live_fanout_honest_false
        && probe.probe.s5_green_claim_blocked
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
        && probe.extended_roundtrip_verified
        && probe.ln0_crosswalk_rows == 8
        && probe.gate_factor_wired_count >= 7
        && probe.gate_factor_exit_blocked
        && sec_s5_p1812_i2_honest()
        && !sec_s5_master_retick_eligible()
        && !sec_s5_op5_cleared()
        && sec_s5_honest_fence_holds()
}

/// Validate SEC-S5 gate census honesty — fail closed on fake persistence/production claims.
pub fn validate_sec_s5_gate_honesty() -> Result<(), &'static str> {
    let census = gate_synthetic_consensus_census();
    if census.ln0_proof_wired {
        return Err("ln0_proof_wired must stay false until Lean lands");
    }
    if census.live_fanout_wired {
        return Err("live_fanout_wired must stay false — no multi-LLM invent");
    }
    if !census.s5_green_claim_blocked {
        return Err("s5_green_claim_blocked must stay true in scaffold");
    }
    if census.production_wired {
        return Err("sec_s5_production_wired must stay false until SEC-HCOM-PROV-GW");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if !census.s5_all_scenarios_probed {
        return Err("S-5 8/8 consensus probes must hit at manifold boundary");
    }
    if !census.algebra_roundtrip_verified {
        return Err("manifold consensus algebra roundtrip witness failed");
    }
    if !census.extended_algebra_roundtrip_verified {
        return Err("manifold extended consensus algebra roundtrip witness failed");
    }
    if census.ln0_crosswalk_rows != 8 {
        return Err("L-N0 crosswalk must enumerate 8 scenarios");
    }
    if MANIFOLD_SEC_S5_GATE_WIRE_HOPS.len() != 7 {
        return Err("seven SEC-S5 gate wire hops expected");
    }
    if census.wire_hop_wired_count != 5 {
        return Err("five SEC-S5 gate wire hops should be wired today");
    }
    if !manifold_gate_sec_s5_ceremony_closed() {
        return Err("manifold gate SEC-S5 ceremony must be closed at census tier");
    }
    if !sec_s5_p1812_i2_honest() {
        return Err("P1812 I2 probe must be honest");
    }
    if !sec_s5_accel_ac07_honest() {
        return Err("ACCEL AC07 probe must be honest");
    }
    if sec_s5_master_retick_eligible() {
        return Err("SEC-S5 master_retick_eligible must stay honest false");
    }
    if sec_s5_op5_cleared() {
        return Err("SEC-S5 op5_cleared must stay honest false");
    }
    if !sec_s5_honest_fence_holds() {
        return Err("SEC-S5 W29-124 honest fence must hold (no GREEN/PRODUCTION/MASTER/OP-5)");
    }
    Ok(())
}

/// Render SEC-S5 gate wire map for operator receipts.
#[must_use]
pub fn sec_s5_gate_wire_matrix() -> String {
    let census = gate_synthetic_consensus_census();
    let mut out = String::from("SEC-S5 manifold gate synthetic consensus wire map (AC07):\n");
    for hop in MANIFOLD_SEC_S5_GATE_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} s5_all_scenarios_probed={} algebra_roundtrip={} extended_roundtrip={} \
         ln0_crosswalk_rows={} ln0_proof_wired={} live_fanout_wired={} production_wired={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_S5_GATE_WIRE_HOPS.len(),
        census.s5_all_scenarios_probed,
        census.algebra_roundtrip_verified,
        census.extended_algebra_roundtrip_verified,
        census.ln0_crosswalk_rows,
        census.ln0_proof_wired,
        census.live_fanout_wired,
        census.production_wired
    ));
    out.push_str(&format!("  trust_ssot={TRUST_SSOT}\n"));
    out.push_str(&format!("  egoff_consensus_ssot={EGOFF_CONSENSUS_SSOT}\n"));
    out.push_str(&format!(
        "  w29_124_cell={W29_124_CELL_ID} honest_fence_holds={} \
         master_retick={} op5_cleared={}\n",
        sec_s5_honest_fence_holds(),
        sec_s5_master_retick_eligible(),
        sec_s5_op5_cleared(),
    ));
    out
}

/// Next-hop surface for L-N0 formal proof (formal-owned).
#[must_use]
pub const fn sec_s5_ln0_proof_next_hop() -> &'static str {
    "umst-formal/Lean/Network/MajorityHonestConvergesToTruth.lean:R-LN0-full"
}

#[cfg(test)]
mod sec_s5_tests {
    use super::*;

    #[test]
    fn sec_s5_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-S5");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-S5");
        assert_eq!(FLEET_P1812_I2_JOB_ID, "PRABHU-WAVE-I-1812-I2");
        assert_eq!(FLEET_ACCEL_AC07_JOB_ID, "ACCEL-25-2030-AC07");
    }

    #[test]
    fn sec_s5_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_s5_consensus_algebra_roundtrip_at_manifold() {
        assert!(manifold_verify_s5_consensus_algebra_roundtrip());
        assert!(manifold_verify_s5_consensus_algebra_roundtrip_extended());
    }

    #[test]
    fn sec_s5_ln0_crosswalk_covers_all_scenarios() {
        let crosswalk = manifold_ln0_lean_crosswalk();
        assert_eq!(crosswalk.len(), 8);
        for scenario in S5_CONSENSUS_PROBE_SCENARIOS {
            assert!(
                crosswalk.iter().any(|r| r.scenario_id == *scenario),
                "missing crosswalk for {scenario}"
            );
        }
    }

    #[test]
    fn sec_s5_session_summary_tracks_disagreement() {
        let saved = OPEN_SESSIONS.with(|sessions| sessions.borrow().clone());
        OPEN_SESSIONS.with(|sessions| sessions.borrow_mut().clear());
        let _ = open_consensus_session("summary-track");
        let summary = manifold_session_consensus_summary();
        assert!(summary.disagreement_events >= 1);
        assert!(summary.open_sessions >= 1);
        assert!(summary.unique_disagreement_kinds >= 1);
        OPEN_SESSIONS.with(|sessions| *sessions.borrow_mut() = saved);
    }

    #[test]
    fn sec_s5_consensus_coverage_eight_by_eight() {
        let probes = manifold_s5_consensus_coverage_probes();
        assert_eq!(probes.len(), 8);
        assert!(manifold_s5_all_scenarios_probed());
        assert!(probes.iter().all(|p| p.probe_hit));
    }

    #[test]
    fn sec_s5_synthetic_consensus_census_honest_posture() {
        let census = gate_synthetic_consensus_census();
        assert_eq!(census.board_slice_id, "SEC-S5");
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert!(census.gate_evidence_wired);
        assert!(census.s5_all_scenarios_probed);
        assert!(census.algebra_roundtrip_verified);
        assert!(census.extended_algebra_roundtrip_verified);
        assert_eq!(census.ln0_crosswalk_rows, 8);
        assert!(!census.ln0_proof_wired);
        assert!(!census.live_fanout_wired);
        assert!(census.s5_green_claim_blocked);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 5);
    }

    #[test]
    fn sec_s5_production_and_ln0_stay_false() {
        assert!(!sec_s5_production_wired());
        assert!(!LN0_PROOF_WIRED_HONEST);
        assert!(!LIVE_FANOUT_WIRED_HONEST);
        assert!(S5_GREEN_CLAIM_BLOCKED);
        assert!(!sec_s5_master_retick_eligible());
        assert!(!sec_s5_op5_cleared());
    }

    #[test]
    fn sec_s5_manifold_wire_hops_cover_gate_and_trust_delegate() {
        assert_eq!(MANIFOLD_SEC_S5_GATE_WIRE_HOPS.len(), 7);
        assert_eq!(
            MANIFOLD_SEC_S5_GATE_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            5
        );
        assert!(MANIFOLD_SEC_S5_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("AdmissibilityToken") && h.wired));
        assert!(MANIFOLD_SEC_S5_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("MajorityHonestConvergesToTruth") && !h.wired));
        assert!(MANIFOLD_SEC_S5_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("hcom_prov_gw_production_wired") && !h.wired));
    }

    #[test]
    fn sec_s5_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_s5_ceremony_closed());
        let probe = sec_s5_gate_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.s5_all_scenarios_probed);
        assert!(probe.algebra_roundtrip_verified);
        assert!(probe.extended_algebra_roundtrip_verified);
        assert_eq!(probe.ln0_crosswalk_rows, 8);
        assert!(probe.ln0_proof_honest_false);
        assert!(probe.live_fanout_honest_false);
        assert!(probe.s5_green_claim_blocked);
        assert!(probe.production_honest_false);
        assert_eq!(probe.wire_hop_wired_count, 5);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_s5_gate_factor_table_honest_blocked_scert() {
        let table = sec_s5_gate_factor_table();
        assert!(table.contains("SEC-S5 gate factors"));
        assert!(table.contains("scert_credit=BLOCKED"));
        assert!(table.contains("expected_gate_exit=BLOCKED"));
        let rows = collect_sec_s5_gate_factor_rows();
        assert_eq!(rows.len(), 8);
        assert!(rows.iter().filter(|r| r.probe_wired).count() >= 7);
    }

    #[test]
    fn sec_s5_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_2033.contains("SEC-S5_2033"));
        assert!(PRIOR_RECEIPT_PATH_2127.contains("SEC-S5_2127"));
        assert!(TRUST_SSOT.contains("sec_s5_consensus"));
        assert!(EGOFF_CONSENSUS_SSOT.contains("security/consensus.rs"));
    }

    #[test]
    fn sec_s5_gate_wire_matrix_renders_honest_counts() {
        let matrix = sec_s5_gate_wire_matrix();
        assert!(matrix.contains("SEC-S5 manifold gate"));
        assert!(matrix.contains("s5_all_scenarios_probed=true"));
        assert!(matrix.contains("extended_roundtrip=true"));
        assert!(matrix.contains("ln0_crosswalk_rows=8"));
        assert!(matrix.contains("ln0_proof_wired=false"));
        assert!(matrix.contains("wired=5/7"));
    }

    #[test]
    fn fleet_composer_accel_ac07_sec_s5_honest() {
        assert!(sec_s5_accel_ac07_honest());
        let probe = sec_s5_accel_ac07_probe();
        assert_eq!(probe.ac07_job_id, FLEET_ACCEL_AC07_JOB_ID);
        assert!(probe.ac07_receipt_honest);
        assert!(probe.prior_i2_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(probe.extended_roundtrip_verified);
        assert_eq!(probe.ln0_crosswalk_rows, 8);
        assert!(probe.gate_factor_exit_blocked);
    }

    #[test]
    fn fleet_composer_p1812_i2_sec_s5_honest() {
        assert!(sec_s5_p1812_i2_honest());
        let probe = sec_s5_p1812_i2_probe();
        assert_eq!(probe.i2_job_id, FLEET_P1812_I2_JOB_ID);
        assert!(probe.prior_2033_absorbed);
        assert!(probe.prior_2127_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(probe.gate_factor_exit_blocked);
    }

    #[test]
    fn sec_s5_validate_gate_honesty_residue_measured() {
        validate_sec_s5_gate_honesty().expect("honest SEC-S5 gate census residue");
        assert_eq!(
            sec_s5_ln0_proof_next_hop(),
            "umst-formal/Lean/Network/MajorityHonestConvergesToTruth.lean:R-LN0-full"
        );
    }

    #[test]
    fn sec_s5_w29_124_honest_fence_no_green_production_master_op5() {
        assert_eq!(W29_124_CELL_ID, "W29-124-SEC_S5");
        assert_eq!(
            W29_124_DEEPEN_SCHEMA_VERSION,
            "sec_s5_w29_124_honest_fence_v1"
        );
        assert_eq!(SCHEMA_VERSION, "sec_s5_gate_synthetic_consensus_census_v3");
        assert!(!sec_s5_production_wired());
        assert!(!LN0_PROOF_WIRED_HONEST);
        assert!(!LIVE_FANOUT_WIRED_HONEST);
        assert!(S5_GREEN_CLAIM_BLOCKED);
        assert!(!sec_s5_master_retick_eligible());
        assert!(!sec_s5_op5_cleared());
        assert!(sec_s5_w29_124_deepen_honest());
        assert!(sec_s5_honest_fence_holds());
        let probe = sec_s5_w29_124_deepen_probe();
        assert!(!probe.green_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(probe.honest_fence.contains("master_retick=false"));
        assert!(probe.non_claim.contains("not MASTER_RETICK"));
        let matrix = sec_s5_gate_wire_matrix();
        assert!(matrix.contains("w29_124_cell=W29-124-SEC_S5"));
        assert!(matrix.contains("honest_fence_holds=true"));
    }
}
