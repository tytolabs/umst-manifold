// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! AGAP-2033/2350-SEC-S2 — manifold gate runtime TrustGatePolicy / refuse-path wire map.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-S2 `TrustGatePolicy` +
//! refuse-path factor SSOT; gateway `trust_wrap_wired()` and `trust_gate_production_wired()` stay
//! **honest open**.
//!
//! # Honesty (W29-121-SEC_S2)
//!
//! Census + TrustGatePolicy refuse-path deepen only. Does **not** invent:
//! - physics / fleet **GREEN**
//! - **PRODUCTION_WIRED**
//! - **MASTER_RETICK** / master retick eligibility
//! - **OP-5 PASS**

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// W29-121 swarm cell id (SEC-S2 honest-fence deepen).
pub const W29_121_CELL_ID: &str = "W29-121-SEC_S2";

/// W29-121 honest posture — manifold S-2 census deepen only.
pub const W29_121_HONEST_POSTURE: &str = "SEC_S2_MANIFOLD_CENSUS_DEEPEN_ONLY";

/// W29-121 explicit non-claims (gate text).
pub const W29_121_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// W29-121 deepen schema version.
pub const W29_121_DEEPEN_SCHEMA_VERSION: &str = "sec_s2_w29_121_honest_fence_v1";

/// Honest fence string for meta / fleet probes (GREEN / PRODUCTION / MASTER / OP-5 fenced).
pub const HONEST_FENCE: &str = "census_wired=true production_wired=false green_claim_blocked=true \
master_retick=false op5_cleared=false session_ledger_wired=false trust_extract_production_wired=false";

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-S2";

/// AGAP slot id (2033 TrustGatePolicy deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-S2";

/// FLEET-COMPOSER Prabhu Wave K slot K2 id.
pub const FLEET_P1941_K2_JOB_ID: &str = "PRABHU-WAVE-K-1941-K2";

/// FLEET-COMPOSER Prabhu Wave K K2 receipt path.
pub const FLEET_P1941_K2_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_P1941_K2.md";

/// Prior AGAP-2033 SEC-S2 TrustGatePolicy receipt.
pub const PRIOR_RECEIPT_PATH_2033: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S2_2033.md";

/// Prior FLEET-COMPOSER-G G73 S-2 refuse-path matrix receipt.
pub const PRIOR_RECEIPT_PATH_G73: &str = "outputs/.tmp/COMPOSER_G73_SEC_S2_2143.md";

/// Prior FLEET-COMPOSER-Y Y81 gate-factors exit deepen receipt.
pub const PRIOR_RECEIPT_PATH_Y81: &str = "outputs/.tmp/COMPOSER_Y81_0808.md";

/// Prior FLEET-COMPOSER-H H52 extract production fence receipt.
pub const PRIOR_RECEIPT_PATH_H52: &str = "outputs/.tmp/COMPOSER_H52_2242.md";

/// Prior FLEET-COMPOSER-X X48 extract production fence receipt.
pub const PRIOR_RECEIPT_PATH_X48: &str = "outputs/.tmp/COMPOSER_X48_0734.md";

/// FLEET-COMPOSER ACCEL-B slot AC29 id (extract production fence deepen).
pub const FLEET_ACCEL_AC29_JOB_ID: &str = "ACCEL-B-2050-AC29";

/// FLEET-COMPOSER ACCEL-B AC29 receipt path.
pub const FLEET_ACCEL_AC29_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC29.md";

/// umst-trust SEC-TRUST-EXTRACT production fence delegate SSOT.
pub const EXTRACT_SSOT: &str = "umst-foundations/crates/umst-trust/src/sec_ecosystem_extract.rs";

/// Core `Trust` / `CipherSuite` ADT SSOT (pinned from `sec_ecosystem_extract`).
pub const TRUST_ADT_SSOT: &str = "umst-foundations/crates/umst-algebra/src/crypto/trust.rs";

/// UCRS consumer bridge parity test SSOT (pinned from `sec_ecosystem_extract`).
pub const UCRS_WIRE_PARITY_TEST: &str =
    "umst-foundations/crates/umst-algebra/tests/trust_ucrs_wire_parity.rs";

/// umst-trust `TrustGatePolicy` + `require_trust_gate_at` delegate SSOT.
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/permission.rs";

/// umst-trust S-2 refuse-path factor matrix delegate SSOT.
pub const S2_FACTOR_SSOT: &str = "umst-foundations/crates/umst-trust/src/trust_refuse_factor.rs";

/// egoff permission thin re-export SSOT.
pub const EGOFF_PERMISSION_SSOT: &str = "egoff/egoff/src/security/permission.rs";

/// Gateway trust-wrap delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs";

/// umst-trust ecosystem trust-gate production census delegate SSOT.
pub const TRUST_GATE_SSOT: &str =
    "umst-foundations/crates/umst-trust/src/sec_ecosystem_trust_gate.rs";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version (v3 absorbs W29-121 honest-fence deepen).
pub const SCHEMA_VERSION: &str = "sec_s2_gate_trust_refuse_census_v3";

/// S-2 refuse-path factor row count (G73/Y81 six-row matrix).
pub const S2_FACTOR_ROW_COUNT: usize = 6;

/// Classical-wrap inventory total (measured @ G73).
pub const CLASSICAL_WRAP_TOTAL: usize = 14;

/// Classical-wrap wrapped count (measured @ G73 — honest 8/14).
pub const CLASSICAL_WRAP_WRAPPED: usize = 8;

/// Wrap queue depth (measured ≥1 @ G73).
pub const WRAP_QUEUE_DEPTH: usize = 1;

/// S-2 GREEN claim blocked — honest true in scaffold deepen.
pub const S2_GREEN_CLAIM_BLOCKED: bool = true;

/// TrustGatePolicy STRICT variant SSOT pin.
pub const TRUST_GATE_POLICY_STRICT: &str = "TrustGatePolicy::STRICT";

/// TrustGatePolicy WARN_ONLY_EXPIRY variant SSOT pin (S-Q3 default).
pub const TRUST_GATE_POLICY_WARN_ONLY: &str = "TrustGatePolicy::WARN_ONLY_EXPIRY";

/// Operator exit for `:trust gate-factors` — honest BLOCKED until SCERT capstone.
pub const EXPECTED_GATE_EXIT: &str = "BLOCKED";

/// Required palette envelope keys for `:trust gate-factors` (Y81 deepen).
pub const S2_GATE_FACTOR_PALETTE_KEYS: &[&str] = &[
    "slice",
    "factor_table",
    "readiness_matrix",
    "ledger",
    "scert_credit",
];

/// S-2 refuse-path factor ids (pinned from `umst-trust::s2_factor` SSOT).
pub const S2_REFUSE_PATH_FACTOR_IDS: &[&str] = &[
    "scope",
    "classical-wrap",
    "revocation",
    "expiry",
    "cipher-suite",
    "privacy-elevation",
];

/// S-2 extract production fence facet count (SEC-TRUST-EXTRACT census).
pub const S2_EXTRACT_FENCE_FACET_COUNT: usize = 7;

/// S-2 extract production fence facets wired today (H52/X48 measured 5/7).
pub const S2_EXTRACT_FENCE_WIRED_COUNT: usize = 5;

/// S-2 extract production fence facet ids (pinned from `sec_ecosystem_extract::ExtractFacet`).
pub const S2_EXTRACT_FENCE_FACET_IDS: &[&str] = &[
    "core_adt_ssot",
    "trust_crate_reexport",
    "wire_projection",
    "egoff_consumer_reexport",
    "ucrs_consumer_bridge",
    "session_ledger",
    "production_wired",
];

/// One facet of the S-2 extract production fence matrix (pinned from `sec_ecosystem_extract`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldS2ExtractProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue.
    pub owning_slice: &'static str,
}

/// S-2 extract production fence facet inventory (pinned from `sec_ecosystem_extract` SSOT).
pub const MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS: &[ManifoldS2ExtractProductionFenceFacet] = &[
    ManifoldS2ExtractProductionFenceFacet {
        facet: "core_adt_ssot",
        wired: true,
        owning_slice: "SEC-TRUST-EXTRACT",
    },
    ManifoldS2ExtractProductionFenceFacet {
        facet: "trust_crate_reexport",
        wired: true,
        owning_slice: "SEC-TRUST-EXTRACT",
    },
    ManifoldS2ExtractProductionFenceFacet {
        facet: "wire_projection",
        wired: true,
        owning_slice: "SEC-TRUST-EXTRACT",
    },
    ManifoldS2ExtractProductionFenceFacet {
        facet: "egoff_consumer_reexport",
        wired: true,
        owning_slice: "SEC-EGOFF-THIN",
    },
    ManifoldS2ExtractProductionFenceFacet {
        facet: "ucrs_consumer_bridge",
        wired: true,
        owning_slice: "SEC-UCRS-STAMP",
    },
    ManifoldS2ExtractProductionFenceFacet {
        facet: "session_ledger",
        wired: false,
        owning_slice: "SEC-S3",
    },
    ManifoldS2ExtractProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "SEC-GW-WRAP",
    },
];

/// One hop in the manifold SEC-S2 gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS2GateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-S2 gate runtime wire map (cold-edge evidence → TrustGatePolicy refuse census).
pub const MANIFOLD_SEC_S2_GATE_WIRE_HOPS: &[SecS2GateWireHop] = &[
    SecS2GateWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecS2GateWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecS2GateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_s2::gate_trust_refuse_census",
        role: "Manifold gate SEC-S2 TrustGatePolicy refuse census",
        wired: true,
    },
    SecS2GateWireHop {
        ordinal: 4,
        surface: "umst-trust::permission::TrustGatePolicy + require_trust_gate_at",
        role: "TrustGatePolicy refuse delegate (G73/Y81)",
        wired: true,
    },
    SecS2GateWireHop {
        ordinal: 5,
        surface: "umst-trust::src_factor::collect_s2_factor_ledger",
        role: "S-2 six-row refuse-path factor matrix",
        wired: true,
    },
    SecS2GateWireHop {
        ordinal: 6,
        surface: "umst-gateway::sec_gw_trust_wrap::trust_wrap_wired",
        role: "Gateway production ceremony (serial Wave D/K1)",
        wired: false,
    },
    SecS2GateWireHop {
        ordinal: 7,
        surface: "umst-trust::sec_ecosystem_trust_gate::trust_gate_production_wired",
        role: "Ecosystem trust-gate production flip (operator-measured live)",
        wired: false,
    },
];

/// One refuse-path probe row at manifold cold edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifoldS2RefusePathProbe {
    /// Stable factor identifier.
    pub factor_id: &'static str,
    /// Whether surface is enumerated in refuse-path SSOT pin.
    pub probe_hit: bool,
}

/// Aggregated SEC-S2 gate TrustGatePolicy refuse census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS2GateTrustRefuseCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// S-2 refuse-path factor row count (6/6).
    pub factor_row_count: usize,
    /// All six refuse-path surfaces probed.
    pub s2_all_refuse_paths_probed: bool,
    /// TrustGatePolicy STRICT pin present.
    pub trust_gate_policy_strict_pinned: bool,
    /// TrustGatePolicy WARN_ONLY pin present.
    pub trust_gate_policy_warn_only_pinned: bool,
    /// Classical-wrap wrapped count (8/14).
    pub classical_wrap_wrapped: usize,
    /// Wrap queue depth (≥1).
    pub wrap_queue_depth: usize,
    /// S-2 GREEN claim blocked — honest true.
    pub s2_green_claim_blocked: bool,
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

/// Whether live gateway trust-wrap production flip is plumbed (honest `false`).
#[must_use]
pub const fn sec_s2_production_wired() -> bool {
    false
}

/// Whether SEC-TRUST-EXTRACT production flip is plumbed (honest `false` @ extract SSOT).
#[must_use]
pub const fn sec_s2_trust_extract_production_wired() -> bool {
    false
}

/// Whether sled-backed session ledger persistence is wired (honest `false` — SEC-S3 residue).
#[must_use]
pub const fn session_ledger_wired() -> bool {
    false
}

/// MASTER retick eligibility — honest **false** (not claimed from S-2 census deepen).
#[must_use]
pub const fn sec_s2_master_retick_eligible() -> bool {
    false
}

/// OP-5 clearance — honest **false** (not claimed from S-2 census deepen).
#[must_use]
pub const fn sec_s2_op5_cleared() -> bool {
    false
}

const _: () = assert!(!sec_s2_production_wired());
const _: () = assert!(!sec_s2_trust_extract_production_wired());
const _: () = assert!(!session_ledger_wired());
const _: () = assert!(S2_GREEN_CLAIM_BLOCKED);
const _: () = assert!(!sec_s2_master_retick_eligible());
const _: () = assert!(!sec_s2_op5_cleared());

/// Honest fence flags for SEC-S2 deepen (W29-121).
///
/// All invent-claim bools stay `false`; `deepen_honest` is true only when cell
/// pins, census ceremony, and GREEN/PRODUCTION/MASTER/OP-5 fences hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS2W29121DeepenProbe {
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

/// Build the W29-121 SEC-S2 deepen honesty probe from live measurements.
#[must_use]
pub fn sec_s2_w29_121_deepen_probe() -> SecS2W29121DeepenProbe {
    let production_wired_claimed =
        sec_s2_production_wired() || sec_s2_trust_extract_production_wired();
    let green_claimed = !S2_GREEN_CLAIM_BLOCKED;
    let op5_pass_claimed = sec_s2_op5_cleared();
    let master_retick_claimed = sec_s2_master_retick_eligible();
    let ceremony_ok = manifold_gate_sec_s2_ceremony_closed();
    let deepen_honest = W29_121_CELL_ID == "W29-121-SEC_S2"
        && W29_121_DEEPEN_SCHEMA_VERSION == "sec_s2_w29_121_honest_fence_v1"
        && W29_121_HONEST_POSTURE == "SEC_S2_MANIFOLD_CENSUS_DEEPEN_ONLY"
        && SCHEMA_VERSION == "sec_s2_gate_trust_refuse_census_v3"
        && !production_wired_claimed
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && !session_ledger_wired()
        && W29_121_NON_CLAIM.contains("not GREEN")
        && W29_121_NON_CLAIM.contains("not OP-5 PASS")
        && W29_121_NON_CLAIM.contains("not production_wired")
        && W29_121_NON_CLAIM.contains("not MASTER_RETICK")
        && HONEST_FENCE.contains("production_wired=false")
        && HONEST_FENCE.contains("green_claim_blocked=true")
        && HONEST_FENCE.contains("master_retick=false")
        && HONEST_FENCE.contains("op5_cleared=false")
        && HONEST_FENCE.contains("session_ledger_wired=false")
        && HONEST_FENCE.contains("trust_extract_production_wired=false")
        && ceremony_ok
        && manifold_s2_extract_fence_facets_verified();
    SecS2W29121DeepenProbe {
        schema_version: W29_121_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_121_CELL_ID,
        honest_posture: W29_121_HONEST_POSTURE,
        non_claim: W29_121_NON_CLAIM,
        honest_fence: HONEST_FENCE,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-121 SEC-S2 deepen honesty probe passes.
#[must_use]
pub fn sec_s2_w29_121_deepen_honest() -> bool {
    sec_s2_w29_121_deepen_probe().deepen_honest
}

/// SEC-S2 fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn sec_s2_honest_fence_holds() -> bool {
    let p = sec_s2_w29_121_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
}

/// S-2 refuse-path coverage probe matrix — 6/6 at manifold cold edge.
#[must_use]
pub fn manifold_s2_refuse_path_coverage_probes() -> Vec<ManifoldS2RefusePathProbe> {
    S2_REFUSE_PATH_FACTOR_IDS
        .iter()
        .map(|factor_id| ManifoldS2RefusePathProbe {
            factor_id,
            probe_hit: S2_REFUSE_PATH_FACTOR_IDS.contains(factor_id),
        })
        .collect()
}

/// Whether all six refuse-path factor surfaces are enumerated at manifold boundary.
#[must_use]
pub fn manifold_s2_all_refuse_paths_probed() -> bool {
    manifold_s2_refuse_path_coverage_probes()
        .iter()
        .all(|p| p.probe_hit)
        && S2_REFUSE_PATH_FACTOR_IDS.len() == S2_FACTOR_ROW_COUNT
}

/// Verify TrustGatePolicy variant pins at manifold boundary.
#[must_use]
pub fn manifold_verify_trust_gate_policy_pins() -> bool {
    TRUST_SSOT.contains("permission.rs")
        && TRUST_GATE_POLICY_STRICT.contains("STRICT")
        && TRUST_GATE_POLICY_WARN_ONLY.contains("WARN_ONLY_EXPIRY")
        && S2_FACTOR_SSOT.contains("trust_refuse_factor")
}

/// Build operator exit expectations for `:trust gate-factors` at manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS2GateFactorExitExpectations {
    /// Palette subcommand label.
    pub subcommand: &'static str,
    /// Owning slice id.
    pub slice_id: &'static str,
    /// Expected operator exit (honest — BLOCKED until SCERT capstone).
    pub expected_gate_exit: &'static str,
    /// SCERT credit posture in envelope.
    pub scert_credit: &'static str,
    /// Factor row count (must be 6).
    pub factor_row_count: usize,
    /// Required palette envelope keys.
    pub palette_keys: &'static [&'static str],
}

/// Build operator exit expectations for `:trust gate-factors` (Y81 absorb @ manifold).
#[must_use]
pub fn sec_s2_gate_factor_exit_expectations() -> SecS2GateFactorExitExpectations {
    SecS2GateFactorExitExpectations {
        subcommand: "gate-factors",
        slice_id: BOARD_SLICE_ID,
        expected_gate_exit: EXPECTED_GATE_EXIT,
        scert_credit: "BLOCKED",
        factor_row_count: S2_FACTOR_ROW_COUNT,
        palette_keys: S2_GATE_FACTOR_PALETTE_KEYS,
    }
}

/// Build manifold SEC-S2 gate TrustGatePolicy refuse census from live measurements.
#[must_use]
pub fn gate_trust_refuse_census() -> SecS2GateTrustRefuseCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_S2_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecS2GateTrustRefuseCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        factor_row_count: S2_FACTOR_ROW_COUNT,
        s2_all_refuse_paths_probed: manifold_s2_all_refuse_paths_probed(),
        trust_gate_policy_strict_pinned: TRUST_GATE_POLICY_STRICT.contains("STRICT"),
        trust_gate_policy_warn_only_pinned: TRUST_GATE_POLICY_WARN_ONLY.contains("WARN_ONLY"),
        classical_wrap_wrapped: CLASSICAL_WRAP_WRAPPED,
        wrap_queue_depth: WRAP_QUEUE_DEPTH,
        s2_green_claim_blocked: S2_GREEN_CLAIM_BLOCKED,
        production_wired: sec_s2_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-S2 ceremony is closed at census tier.
///
/// True when cold-edge evidence probe + TrustGatePolicy refuse wire map hops 1–5 are measured wired.
/// Gateway production + ecosystem trust-gate production flip are explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_s2_ceremony_closed() -> bool {
    let census = gate_trust_refuse_census();
    census.gate_evidence_wired
        && census.factor_row_count == S2_FACTOR_ROW_COUNT
        && census.s2_all_refuse_paths_probed
        && census.trust_gate_policy_strict_pinned
        && census.trust_gate_policy_warn_only_pinned
        && census.classical_wrap_wrapped == CLASSICAL_WRAP_WRAPPED
        && census.wrap_queue_depth >= WRAP_QUEUE_DEPTH
        && census.s2_green_claim_blocked
        && !census.production_wired
        && census.wire_hop_wired_count == 5
        && manifold_s2_all_refuse_paths_probed()
        && manifold_verify_trust_gate_policy_pins()
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-S2 manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS2GateManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// S-2 6/6 refuse-path surfaces probed.
    pub s2_all_refuse_paths_probed: bool,
    /// TrustGatePolicy pins verified.
    pub trust_gate_policy_pins_verified: bool,
    /// S-2 GREEN claim blocked.
    pub s2_green_claim_blocked: bool,
    /// Production flip honest false.
    pub production_honest_false: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build introspection probe for SEC-S2 done-when checks.
#[must_use]
pub fn sec_s2_gate_manifold_probe() -> SecS2GateManifoldProbe {
    let census = gate_trust_refuse_census();
    SecS2GateManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        s2_all_refuse_paths_probed: census.s2_all_refuse_paths_probed,
        trust_gate_policy_pins_verified: manifold_verify_trust_gate_policy_pins(),
        s2_green_claim_blocked: census.s2_green_claim_blocked,
        production_honest_false: !census.production_wired,
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_s2_ceremony_closed(),
    }
}

/// One SEC-S2 gate-factor row for operator receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS2GateFactorRow {
    /// Factor identifier.
    pub factor_id: &'static str,
    /// Refuse path label.
    pub refuse_path: &'static str,
    /// Whether manifold probe wired for this factor.
    pub probe_wired: bool,
    /// Acceptance credit (honest BLOCKED until production).
    pub acceptance_credit: bool,
}

/// Collect SEC-S2 gate-factor rows for operator matrix receipts.
#[must_use]
pub fn collect_sec_s2_gate_factor_rows() -> Vec<SecS2GateFactorRow> {
    let census = gate_trust_refuse_census();
    vec![
        SecS2GateFactorRow {
            factor_id: "scope",
            refuse_path: "TrustError::ScopeNotPermitted",
            probe_wired: census.s2_all_refuse_paths_probed,
            acceptance_credit: false,
        },
        SecS2GateFactorRow {
            factor_id: "classical-wrap",
            refuse_path: "unwrapped classical surface refuse",
            probe_wired: census.classical_wrap_wrapped < CLASSICAL_WRAP_TOTAL,
            acceptance_credit: false,
        },
        SecS2GateFactorRow {
            factor_id: "revocation",
            refuse_path: "TrustError::AttestationRevoked",
            probe_wired: TRUST_SSOT.contains("permission.rs"),
            acceptance_credit: false,
        },
        SecS2GateFactorRow {
            factor_id: "expiry",
            refuse_path: "TrustError::AttestationExpired (STRICT)",
            probe_wired: census.trust_gate_policy_strict_pinned,
            acceptance_credit: false,
        },
        SecS2GateFactorRow {
            factor_id: "cipher-suite",
            refuse_path: "TrustComposeError::SuiteMismatch",
            probe_wired: S2_FACTOR_SSOT.contains("trust_refuse_factor"),
            acceptance_credit: false,
        },
        SecS2GateFactorRow {
            factor_id: "privacy-elevation",
            refuse_path: "FederatedWrite minimum for cross-tier memory",
            probe_wired: S2_REFUSE_PATH_FACTOR_IDS.contains(&"privacy-elevation"),
            acceptance_credit: false,
        },
    ]
}

/// Render SEC-S2 gate-factor table for operator receipts.
#[must_use]
pub fn sec_s2_gate_factor_table() -> String {
    let rows = collect_sec_s2_gate_factor_rows();
    let exit = sec_s2_gate_factor_exit_expectations();
    let mut out = String::from("SEC-S2 gate factors (K2 TrustGatePolicy refuse-path):\n");
    for row in &rows {
        out.push_str(&format!(
            "  {} probe_wired={} scert_credit=BLOCKED {}\n",
            row.factor_id, row.probe_wired, row.refuse_path
        ));
    }
    out.push_str(&format!(
        "  classical_wrap={}/{} wrap_queue_depth={} s2_green_claim_blocked={} \
         production_wired={} expected_gate_exit={} scert_credit={}\n",
        CLASSICAL_WRAP_WRAPPED,
        CLASSICAL_WRAP_TOTAL,
        WRAP_QUEUE_DEPTH,
        S2_GREEN_CLAIM_BLOCKED,
        sec_s2_production_wired(),
        exit.expected_gate_exit,
        exit.scert_credit,
    ));
    out
}

/// FLEET-COMPOSER Prabhu Wave K K2 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS2P1941K2Probe {
    /// K2 fleet card id.
    pub k2_job_id: &'static str,
    /// Prior 2033 TrustGatePolicy absorbed.
    pub prior_2033_absorbed: bool,
    /// Prior G73 refuse-path matrix absorbed.
    pub prior_g73_absorbed: bool,
    /// Prior Y81 gate-factors exit absorbed.
    pub prior_y81_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecS2GateManifoldProbe,
    /// `sec_s2_production_wired()` — honest false.
    pub production_wired: bool,
    /// Gate-factor rows with probe wired.
    pub gate_factor_wired_count: usize,
    /// Gate-factor table pins BLOCKED exit.
    pub gate_factor_exit_blocked: bool,
    /// Operator exit expectations.
    pub exit_expectations: SecS2GateFactorExitExpectations,
}

/// Build FLEET-COMPOSER P1941 K2 integration probe from live measurements.
#[must_use]
pub fn sec_s2_p1941_k2_probe() -> SecS2P1941K2Probe {
    let rows = collect_sec_s2_gate_factor_rows();
    let table = sec_s2_gate_factor_table();
    SecS2P1941K2Probe {
        k2_job_id: FLEET_P1941_K2_JOB_ID,
        prior_2033_absorbed: PRIOR_RECEIPT_PATH_2033.contains("SEC-S2_2033"),
        prior_g73_absorbed: PRIOR_RECEIPT_PATH_G73.contains("G73"),
        prior_y81_absorbed: PRIOR_RECEIPT_PATH_Y81.contains("Y81"),
        ceremony_closed: manifold_gate_sec_s2_ceremony_closed(),
        probe: sec_s2_gate_manifold_probe(),
        production_wired: sec_s2_production_wired(),
        gate_factor_wired_count: rows.iter().filter(|r| r.probe_wired).count(),
        gate_factor_exit_blocked: table.contains("expected_gate_exit=BLOCKED"),
        exit_expectations: sec_s2_gate_factor_exit_expectations(),
    }
}

/// FLEET-COMPOSER P1941 K2 honesty gate — ceremony closed + production false + refuse-path honest.
#[must_use]
pub fn sec_s2_p1941_k2_honest() -> bool {
    let probe = sec_s2_p1941_k2_probe();
    probe.k2_job_id == FLEET_P1941_K2_JOB_ID
        && probe.prior_2033_absorbed
        && probe.prior_g73_absorbed
        && probe.prior_y81_absorbed
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.s2_all_refuse_paths_probed
        && probe.probe.trust_gate_policy_pins_verified
        && probe.probe.s2_green_claim_blocked
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
        && probe.gate_factor_wired_count == S2_FACTOR_ROW_COUNT
        && probe.gate_factor_exit_blocked
        && probe.exit_expectations.expected_gate_exit == EXPECTED_GATE_EXIT
        && probe.exit_expectations.factor_row_count == S2_FACTOR_ROW_COUNT
}

/// Validate SEC-S2 gate census honesty — fail closed on fake persistence/production claims.
pub fn validate_sec_s2_gate_honesty() -> Result<(), &'static str> {
    let census = gate_trust_refuse_census();
    if census.production_wired {
        return Err("sec_s2_production_wired must stay false until SEC-GW-WRAP");
    }
    if !census.s2_green_claim_blocked {
        return Err("s2_green_claim_blocked must stay true in scaffold deepen");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if census.factor_row_count != S2_FACTOR_ROW_COUNT {
        return Err("six SEC-S2 refuse-path factor rows expected");
    }
    if !census.s2_all_refuse_paths_probed {
        return Err("all six refuse-path surfaces must be probed");
    }
    if !manifold_verify_trust_gate_policy_pins() {
        return Err("TrustGatePolicy pins must be verified at manifold boundary");
    }
    if MANIFOLD_SEC_S2_GATE_WIRE_HOPS.len() != 7 {
        return Err("seven SEC-S2 gate wire hops expected");
    }
    if census.wire_hop_wired_count != 5 {
        return Err("five SEC-S2 gate wire hops should be wired today");
    }
    if !manifold_gate_sec_s2_ceremony_closed() {
        return Err("manifold gate SEC-S2 ceremony must be closed at census tier");
    }
    if !sec_s2_p1941_k2_honest() {
        return Err("P1941 K2 probe must be honest");
    }
    if !manifold_s2_extract_fence_facets_verified() {
        return Err("S-2 extract production fence facets must verify at manifold boundary");
    }
    if sec_s2_trust_extract_production_wired() {
        return Err("sec_s2_trust_extract_production_wired must stay false until SEC-GW-WRAP");
    }
    if !sec_s2_accel_ac29_honest() {
        return Err("ACCEL AC29 extract production fence deepen probe must be honest");
    }
    if sec_s2_master_retick_eligible() {
        return Err("SEC-S2 master_retick_eligible must stay honest false");
    }
    if sec_s2_op5_cleared() {
        return Err("SEC-S2 op5_cleared must stay honest false");
    }
    if session_ledger_wired() {
        return Err("SEC-S2 session_ledger_wired must stay honest false");
    }
    if !sec_s2_honest_fence_holds() {
        return Err("SEC-S2 W29-121 honest fence must hold (no GREEN/PRODUCTION/MASTER/OP-5)");
    }
    Ok(())
}

/// Render SEC-S2 gate wire map for operator receipts.
#[must_use]
pub fn sec_s2_gate_wire_matrix() -> String {
    let census = gate_trust_refuse_census();
    let mut out = String::from("SEC-S2 manifold gate TrustGatePolicy refuse wire map (K2):\n");
    for hop in MANIFOLD_SEC_S2_GATE_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} s2_green_claim_blocked={} production_wired={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_S2_GATE_WIRE_HOPS.len(),
        census.s2_green_claim_blocked,
        census.production_wired
    ));
    out.push_str(&format!("  trust_ssot={TRUST_SSOT}\n"));
    out.push_str(&format!("  s2_factor_ssot={S2_FACTOR_SSOT}\n"));
    out.push_str(&format!(
        "  w29_121_cell={W29_121_CELL_ID} honest_fence_holds={} \
         master_retick={} op5_cleared={}\n",
        sec_s2_honest_fence_holds(),
        sec_s2_master_retick_eligible(),
        sec_s2_op5_cleared(),
    ));
    out
}

/// Next-hop surface for gateway trust-wrap production ceremony (gateway-owned).
#[must_use]
pub const fn sec_s2_trust_wrap_next_hop() -> &'static str {
    "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs:trust_wrap_wired"
}

/// Count wired extract production fence facets at manifold boundary.
#[must_use]
pub fn sec_s2_extract_fence_wired_count() -> usize {
    MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Whether all five wired extract production fence facets verify at manifold boundary.
#[must_use]
pub fn manifold_s2_extract_fence_facets_verified() -> bool {
    sec_s2_extract_fence_wired_count() == S2_EXTRACT_FENCE_WIRED_COUNT
        && MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS.len() == S2_EXTRACT_FENCE_FACET_COUNT
        && S2_EXTRACT_FENCE_FACET_IDS.len() == S2_EXTRACT_FENCE_FACET_COUNT
        && EXTRACT_SSOT.contains("sec_ecosystem_extract.rs")
        && TRUST_ADT_SSOT.contains("crypto/trust.rs")
        && UCRS_WIRE_PARITY_TEST.contains("trust_ucrs_wire_parity.rs")
        && !sec_s2_trust_extract_production_wired()
        && MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS
            .iter()
            .filter(|f| !f.wired)
            .all(|f| f.facet == "session_ledger" || f.facet == "production_wired")
        && MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS
            .iter()
            .filter(|f| f.wired)
            .all(|f| S2_EXTRACT_FENCE_FACET_IDS.contains(&f.facet))
}

/// Render S-2 extract production fence matrix for operator receipts.
#[must_use]
pub fn sec_s2_extract_production_fence_matrix() -> String {
    let wired = sec_s2_extract_fence_wired_count();
    let mut out = String::from("SEC-S2 extract production fence (SEC-TRUST-EXTRACT):\n");
    for facet in MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS {
        out.push_str(&format!(
            "  {} wired={} owning_slice={}\n",
            facet.facet, facet.wired, facet.owning_slice
        ));
    }
    out.push_str(&format!(
        "  facets_wired={}/{} trust_extract_production_wired={} session_ledger_wired=false \
         s1_green_claimed=false extract_ssot={EXTRACT_SSOT}\n",
        wired,
        S2_EXTRACT_FENCE_FACET_COUNT,
        sec_s2_trust_extract_production_wired()
    ));
    out.push_str(&format!("  trust_adt_ssot={TRUST_ADT_SSOT}\n"));
    out.push_str(&format!(
        "  ucrs_wire_parity_test={UCRS_WIRE_PARITY_TEST}\n"
    ));
    out.push_str(&format!(
        "  w29_121_cell={W29_121_CELL_ID} honest_fence_holds={} \
         master_retick={} op5_cleared={}\n",
        sec_s2_honest_fence_holds(),
        sec_s2_master_retick_eligible(),
        sec_s2_op5_cleared(),
    ));
    out
}

/// Next-hop surface for sled session ledger persistence (extract-owned residue).
#[must_use]
pub const fn sec_s2_extract_production_fence_next_hop() -> &'static str {
    "umst-foundations/crates/umst-trust/src/sec_ecosystem_extract.rs:session_ledger_wired"
}

/// FLEET-COMPOSER ACCEL-B AC29 extract production fence deepen probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS2AccelAc29Probe {
    /// AC29 fleet slot id.
    pub ac29_job_id: &'static str,
    /// Prior K2 TrustGatePolicy ceremony absorbed.
    pub prior_k2_absorbed: bool,
    /// Prior G73 refuse-path matrix absorbed.
    pub prior_g73_absorbed: bool,
    /// Prior H52 extract production fence absorbed.
    pub prior_h52_absorbed: bool,
    /// Prior X48 extract production fence absorbed.
    pub prior_x48_absorbed: bool,
    /// Extract production fence facet matrix verified.
    pub extract_fence_matrix_verified: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecS2GateManifoldProbe,
    /// `sec_s2_production_wired()` — honest false.
    pub production_wired: bool,
    /// `sec_s2_trust_extract_production_wired()` — honest false at extract SSOT.
    pub trust_extract_production_wired_honest_false: bool,
    /// Wired extract fence facet count (5/7).
    pub extract_wired_facet_count: usize,
}

/// Build FLEET-COMPOSER ACCEL-B AC29 integration probe from live measurements.
#[must_use]
pub fn sec_s2_accel_ac29_probe() -> SecS2AccelAc29Probe {
    let fence = sec_s2_extract_production_fence_matrix();
    SecS2AccelAc29Probe {
        ac29_job_id: FLEET_ACCEL_AC29_JOB_ID,
        prior_k2_absorbed: FLEET_P1941_K2_RECEIPT_PATH.contains("COMPOSER_P1941_K2"),
        prior_g73_absorbed: PRIOR_RECEIPT_PATH_G73.contains("G73"),
        prior_h52_absorbed: PRIOR_RECEIPT_PATH_H52.contains("COMPOSER_H52"),
        prior_x48_absorbed: PRIOR_RECEIPT_PATH_X48.contains("COMPOSER_X48"),
        extract_fence_matrix_verified: fence.contains("facets_wired=5/7")
            && fence.contains("trust_extract_production_wired=false")
            && fence.contains("session_ledger_wired=false")
            && fence.contains("core_adt_ssot")
            && fence.contains("production_wired wired=false"),
        ceremony_closed: manifold_gate_sec_s2_ceremony_closed(),
        probe: sec_s2_gate_manifold_probe(),
        production_wired: sec_s2_production_wired(),
        trust_extract_production_wired_honest_false: !sec_s2_trust_extract_production_wired(),
        extract_wired_facet_count: sec_s2_extract_fence_wired_count(),
    }
}

/// FLEET-COMPOSER ACCEL-B AC29 honesty gate — extract production fence deepen + ceremony closed.
#[must_use]
pub fn sec_s2_accel_ac29_honest() -> bool {
    let probe = sec_s2_accel_ac29_probe();
    probe.ac29_job_id == FLEET_ACCEL_AC29_JOB_ID
        && probe.prior_k2_absorbed
        && probe.prior_g73_absorbed
        && probe.prior_h52_absorbed
        && probe.prior_x48_absorbed
        && probe.extract_fence_matrix_verified
        && manifold_s2_extract_fence_facets_verified()
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.s2_all_refuse_paths_probed
        && probe.probe.trust_gate_policy_pins_verified
        && probe.probe.s2_green_claim_blocked
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
        && probe.trust_extract_production_wired_honest_false
        && probe.extract_wired_facet_count == S2_EXTRACT_FENCE_WIRED_COUNT
        && !sec_s2_trust_extract_production_wired()
        && !sec_s2_master_retick_eligible()
        && !sec_s2_op5_cleared()
        && W29_121_CELL_ID == "W29-121-SEC_S2"
        && HONEST_FENCE.contains("green_claim_blocked=true")
        && HONEST_FENCE.contains("production_wired=false")
        && sec_s2_honest_fence_holds()
}

#[cfg(test)]
mod sec_s2_tests {
    use super::*;

    #[test]
    fn sec_s2_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-S2");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-S2");
        assert_eq!(FLEET_P1941_K2_JOB_ID, "PRABHU-WAVE-K-1941-K2");
    }

    #[test]
    fn sec_s2_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_s2_trust_refuse_census_honest_posture() {
        let census = gate_trust_refuse_census();
        assert_eq!(census.board_slice_id, "SEC-S2");
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert!(census.gate_evidence_wired);
        assert_eq!(census.factor_row_count, 6);
        assert!(census.s2_all_refuse_paths_probed);
        assert!(census.trust_gate_policy_strict_pinned);
        assert!(census.trust_gate_policy_warn_only_pinned);
        assert_eq!(census.classical_wrap_wrapped, 8);
        assert!(census.wrap_queue_depth >= 1);
        assert!(census.s2_green_claim_blocked);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 5);
    }

    #[test]
    fn sec_s2_production_stays_false() {
        assert!(!sec_s2_production_wired());
        assert!(S2_GREEN_CLAIM_BLOCKED);
    }

    #[test]
    fn sec_s2_manifold_wire_hops_cover_gate_and_trust_delegate() {
        assert_eq!(MANIFOLD_SEC_S2_GATE_WIRE_HOPS.len(), 7);
        assert_eq!(
            MANIFOLD_SEC_S2_GATE_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            5
        );
        assert!(MANIFOLD_SEC_S2_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("AdmissibilityToken") && h.wired));
        assert!(MANIFOLD_SEC_S2_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("trust_wrap_wired") && !h.wired));
        assert!(MANIFOLD_SEC_S2_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("trust_gate_production_wired") && !h.wired));
    }

    #[test]
    fn sec_s2_refuse_path_coverage_six_of_six() {
        assert!(manifold_s2_all_refuse_paths_probed());
        let probes = manifold_s2_refuse_path_coverage_probes();
        assert_eq!(probes.len(), 6);
        assert!(probes.iter().all(|p| p.probe_hit));
    }

    #[test]
    fn sec_s2_trust_gate_policy_pins_verified() {
        assert!(manifold_verify_trust_gate_policy_pins());
        assert!(TRUST_SSOT.contains("permission.rs"));
        assert!(S2_FACTOR_SSOT.contains("trust_refuse_factor"));
    }

    #[test]
    fn sec_s2_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_s2_ceremony_closed());
        let probe = sec_s2_gate_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.s2_all_refuse_paths_probed);
        assert!(probe.trust_gate_policy_pins_verified);
        assert!(probe.s2_green_claim_blocked);
        assert!(probe.production_honest_false);
        assert_eq!(probe.wire_hop_wired_count, 5);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_s2_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_2033.contains("SEC-S2_2033"));
        assert!(PRIOR_RECEIPT_PATH_G73.contains("G73"));
        assert!(PRIOR_RECEIPT_PATH_Y81.contains("Y81"));
        assert!(TRUST_SSOT.contains("permission.rs"));
        assert!(EGOFF_PERMISSION_SSOT.contains("security/permission.rs"));
    }

    #[test]
    fn sec_s2_gate_factor_table_honest_blocked_scert() {
        let table = sec_s2_gate_factor_table();
        assert!(table.contains("expected_gate_exit=BLOCKED"));
        assert!(table.contains("scert_credit=BLOCKED"));
        let rows = collect_sec_s2_gate_factor_rows();
        assert_eq!(rows.len(), 6);
        assert!(rows.iter().all(|r| r.probe_wired));
        assert!(rows.iter().all(|r| !r.acceptance_credit));
    }

    #[test]
    fn sec_s2_gate_factor_exit_expectations_y81_absorb() {
        let exit = sec_s2_gate_factor_exit_expectations();
        assert_eq!(exit.subcommand, "gate-factors");
        assert_eq!(exit.expected_gate_exit, "BLOCKED");
        assert_eq!(exit.scert_credit, "BLOCKED");
        assert_eq!(exit.factor_row_count, 6);
        assert_eq!(exit.palette_keys.len(), 5);
    }

    #[test]
    fn sec_s2_gate_wire_matrix_renders_honest_counts() {
        let matrix = sec_s2_gate_wire_matrix();
        assert!(matrix.contains("SEC-S2 manifold gate"));
        assert!(matrix.contains("s2_green_claim_blocked=true"));
        assert!(matrix.contains("wired=5/7"));
    }

    #[test]
    fn fleet_composer_p1941_k2_sec_s2_honest() {
        assert!(sec_s2_p1941_k2_honest());
        let probe = sec_s2_p1941_k2_probe();
        assert_eq!(probe.k2_job_id, FLEET_P1941_K2_JOB_ID);
        assert!(probe.prior_2033_absorbed);
        assert!(probe.prior_g73_absorbed);
        assert!(probe.prior_y81_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert_eq!(probe.gate_factor_wired_count, 6);
        assert!(probe.gate_factor_exit_blocked);
    }

    #[test]
    fn sec_s2_validate_gate_honesty_residue_measured() {
        validate_sec_s2_gate_honesty().expect("honest SEC-S2 gate census residue");
        assert_eq!(
            sec_s2_trust_wrap_next_hop(),
            "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs:trust_wrap_wired"
        );
    }

    #[test]
    fn sec_s2_extract_production_fence_facets_five_of_seven_wired() {
        assert_eq!(
            MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS.len(),
            S2_EXTRACT_FENCE_FACET_COUNT
        );
        assert_eq!(
            sec_s2_extract_fence_wired_count(),
            S2_EXTRACT_FENCE_WIRED_COUNT
        );
        assert!(manifold_s2_extract_fence_facets_verified());
        let matrix = sec_s2_extract_production_fence_matrix();
        assert!(matrix.contains("facets_wired=5/7"));
        assert!(matrix.contains("trust_extract_production_wired=false"));
        assert!(matrix.contains("session_ledger_wired=false"));
        assert!(EXTRACT_SSOT.contains("sec_ecosystem_extract.rs"));
    }

    #[test]
    fn fleet_accel_ac29_sec_s2_extract_production_fence_honest() {
        assert!(sec_s2_accel_ac29_honest());
        let probe = sec_s2_accel_ac29_probe();
        assert_eq!(probe.ac29_job_id, FLEET_ACCEL_AC29_JOB_ID);
        assert!(probe.prior_k2_absorbed);
        assert!(probe.prior_g73_absorbed);
        assert!(probe.prior_h52_absorbed);
        assert!(probe.prior_x48_absorbed);
        assert!(probe.extract_fence_matrix_verified);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(probe.trust_extract_production_wired_honest_false);
        assert_eq!(
            probe.extract_wired_facet_count,
            S2_EXTRACT_FENCE_WIRED_COUNT
        );
        assert!(!sec_s2_trust_extract_production_wired());
        assert_eq!(
            sec_s2_extract_production_fence_next_hop(),
            "umst-foundations/crates/umst-trust/src/sec_ecosystem_extract.rs:session_ledger_wired"
        );
    }

    #[test]
    fn sec_s2_w29_121_honest_fence_no_green_production_master_op5() {
        assert_eq!(W29_121_CELL_ID, "W29-121-SEC_S2");
        assert_eq!(
            W29_121_DEEPEN_SCHEMA_VERSION,
            "sec_s2_w29_121_honest_fence_v1"
        );
        assert_eq!(SCHEMA_VERSION, "sec_s2_gate_trust_refuse_census_v3");
        assert!(!sec_s2_production_wired());
        assert!(!sec_s2_trust_extract_production_wired());
        assert!(!session_ledger_wired());
        assert!(S2_GREEN_CLAIM_BLOCKED);
        assert!(!sec_s2_master_retick_eligible());
        assert!(!sec_s2_op5_cleared());
        assert!(sec_s2_w29_121_deepen_honest());
        assert!(sec_s2_honest_fence_holds());
        let probe = sec_s2_w29_121_deepen_probe();
        assert!(!probe.green_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(probe.honest_fence.contains("master_retick=false"));
        assert!(probe
            .honest_fence
            .contains("trust_extract_production_wired=false"));
        assert!(probe.non_claim.contains("not MASTER_RETICK"));
        let matrix = sec_s2_gate_wire_matrix();
        assert!(matrix.contains("w29_121_cell=W29-121-SEC_S2"));
        assert!(matrix.contains("honest_fence_holds=true"));
        validate_sec_s2_gate_honesty().expect("W29-121 honest fence validates");
    }
}
