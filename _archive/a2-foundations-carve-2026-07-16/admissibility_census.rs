// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//
// QUARANTINE — dual writer superseded by `umst-gate::admissibility_census` (P2.0).
// Do not import; kept for carve archaeology only.

//! Phase 0a admissibility census — compute/consume site registry and open reconciliation deltas.
//!
//! Blueprint §7 0a · steelman I.1 · `NEW_REPOS_BUILD_SPEC` §E.4.
//! Phase 0b adds [`core_gate`] (Mass + CD only) and [`material_gate`] (cartridge conjuncts);
//! legacy [`transition_outcome`] composes both for parity. Phase 0f cleared
//! [`OPEN_RECONCILIATION_DELTAS`] — matrix test GREEN.

/// Parity anchor fixture (repo wins over stale doc shorthand `a389b838…`).
pub const GATE_PARITY_V0_FIXTURE_REL: &str =
    "umst-concrete-cartridge/crates/umst-mcp/tests/fixtures/gate_parity_v0.json";

/// Full SHA256 of `gate_parity_v0.json` (pinned acceptance receipt).
pub const GATE_PARITY_V0_SHA256: &str =
    "149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3";

/// Doc shorthand prefix for receipts and dashboards.
pub const GATE_PARITY_V0_SHA256_PREFIX: &str = "149081fa81a6525f";

/// Whether a registry row is a compute or consume site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiteRole {
    Compute,
    Consume,
}

/// Conjunct families tracked in the reconciliation matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConjunctFamily {
    Mass,
    ClausiusDuhem,
    StrengthMonotonic,
    StrengthUpperBound,
    ReactionExtent,
    RegimeEnvelope,
    LandauerDebit,
    PowerInput,
    CompositeVerdict,
}

/// One row in the census table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissibilitySite {
    pub symbol: &'static str,
    pub repo: &'static str,
    pub path: &'static str,
    pub role: SiteRole,
    pub conjuncts: &'static [ConjunctFamily],
    /// When true, only a subset of conjuncts is evaluated at this site today.
    pub partial: bool,
}

/// One documented drift between a compute site and the canonical reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReconciliationDelta {
    pub id: &'static str,
    pub compute_symbol: &'static str,
    pub reference_symbol: &'static str,
    pub conjunct: ConjunctFamily,
    pub fixture_family: &'static str,
    pub detail: &'static str,
    pub clear_in_phase: &'static str,
}

/// Canonical SSOT cluster and parallel compute paths (blueprint §7 0a anchors).
pub static ADMISSIBILITY_COMPUTE_SITES: &[AdmissibilitySite] = &[
    AdmissibilitySite {
        symbol: "core_gate",
        repo: "umst-manifold",
        path: "src/gate/core_gate.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::PowerInput,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "material_gate",
        repo: "umst-manifold",
        path: "src/gate/material_gate.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "transition_outcome",
        repo: "umst-manifold",
        path: "src/gate/transition_proposal.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "thermodynamic_transition_admissible_tol",
        repo: "umst-manifold",
        path: "src/gate/transition_proposal.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::StrengthUpperBound,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "thermodynamic_transition_admissible",
        repo: "umst-manifold",
        path: "src/gate/transition_proposal.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::StrengthUpperBound,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "evaluate_transition_pure_with_params",
        repo: "umst-manifold",
        path: "src/gate/transition_proposal.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "evaluate_http_mix_manifest",
        repo: "umst-manifold",
        path: "src/gate/http_manifest.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "clausius_duhem_margin",
        repo: "umst-manifold",
        path: "src/ai/constraint_loss.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::Mass,
        ],
        partial: true,
    },
    AdmissibilitySite {
        symbol: "ThermodynamicCBF",
        repo: "umst-manifold",
        path: "src/ai/cbf.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::LandauerDebit,
            ConjunctFamily::PowerInput,
        ],
        partial: true,
    },
    AdmissibilitySite {
        symbol: "open_system_core_gate",
        repo: "umst-manifold",
        path: "src/gate/open_system.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::PowerInput,
            ConjunctFamily::LandauerDebit,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "gate_check_mix",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-concrete-cartridge/src/research/contribution/gate.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
            ConjunctFamily::RegimeEnvelope,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "thermo_gate_transition_outcome",
        repo: "umst-manifold",
        path: "src/gate/thermo_transition.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "kleisli_admissible",
        repo: "umst-manifold",
        path: "src/gate/kleisli.rs",
        role: SiteRole::Compute,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "runtime_cold_wire_admissible",
        repo: "umst-manifold",
        path: "src/runtime/gate/cold_wire.rs",
        role: SiteRole::Compute,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: true,
    },
    AdmissibilitySite {
        symbol: "physics_orchestration_admissible",
        repo: "umst-manifold",
        path: "src/physics/orchestration.rs",
        role: SiteRole::Compute,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: true,
    },
];

/// Every caller / golden harness that consumes an admissibility verdict.
pub static ADMISSIBILITY_CONSUME_SITES: &[AdmissibilitySite] = &[
    AdmissibilitySite {
        symbol: "gate_check_mix_result",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-concrete-cartridge/src/research/contribution/infra.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "gate_recheck",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-concrete-cartridge/src/research/contribution/gate.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "gate_check_mix_result_parity_fixture",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-mcp/tests/gate_parity.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "gate_dual_run_fixtures",
        repo: "umst-manifold",
        path: "tests/gate_dual_run_parity.rs",
        role: SiteRole::Consume,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "golden_adversarial_gate_check_verdicts",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-concrete-cartridge/tests/golden_gate_check.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "gate_bridge_thermodynamic_transition_admissible",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-gate-ffi/src/gate_bridge.rs",
        role: SiteRole::Consume,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "soft_gate",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-agent-mcp-core/src/soft_gate.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "gate_server_http",
        repo: "umst-manifold",
        path: "src/bin/gate_server.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::StrengthMonotonic],
        partial: true,
    },
    AdmissibilitySite {
        symbol: "ffi_bridge_thermodynamic_transition_admissible",
        repo: "umst-manifold",
        path: "egoff/umst-formal/ffi-bridge/src/lib.rs",
        role: SiteRole::Consume,
        conjuncts: &[
            ConjunctFamily::Mass,
            ConjunctFamily::ClausiusDuhem,
            ConjunctFamily::StrengthMonotonic,
            ConjunctFamily::ReactionExtent,
        ],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "inprocess_gate_batch",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-concrete-cartridge/tests/inprocess_gate_batch.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "manifest_parity",
        repo: "umst-concrete-cartridge",
        path: "crates/umst-mcp/tests/manifest_parity.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::CompositeVerdict],
        partial: false,
    },
    AdmissibilitySite {
        symbol: "admissibility_margin_soft_tracks_hard",
        repo: "umst-manifold",
        path: "tests/admissibility_margin_soft_tracks_hard.rs",
        role: SiteRole::Consume,
        conjuncts: &[ConjunctFamily::ClausiusDuhem],
        partial: false,
    },
];

/// Known drifts — cleared in phases 0b–0f (matrix test RED until empty).
///
/// Phase 0e cleared `cbf_open_system_extension`: Landauer debit now bridges C10 → Core
/// `P_input` via [`super::open_system`].
pub static OPEN_RECONCILIATION_DELTAS: &[ReconciliationDelta] = &[];

/// Human-readable dump for failing matrix tests and operator receipts.
#[must_use]
pub fn format_open_deltas() -> String {
    let mut lines = Vec::with_capacity(OPEN_RECONCILIATION_DELTAS.len());
    for d in OPEN_RECONCILIATION_DELTAS {
        lines.push(format!(
            "- {}: {} vs {} [{:?}] @ {} — {} (clear in {})",
            d.id,
            d.compute_symbol,
            d.reference_symbol,
            d.conjunct,
            d.fixture_family,
            d.detail,
            d.clear_in_phase,
        ));
    }
    lines.join("\n")
}
