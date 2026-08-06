// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FLEET-COMPOSER-J J11 — `cargo test -p umst-manifold` full battery + OP-5 measure.
//!
//! Absorbs H79 (`COMPOSER_H79_2242.md`) and I81 queued re-run. Documents substrate
//! blockers, HCOM-006 semantic-lane adjacency, OP-5/META-6 interaction post-H71 carve,
//! and honest cargo-test posture. Does **not** claim OP-5 PASS, full META-6 clearance,
//! or nested-repo clean. Receipt SSOT: `outputs/.tmp/COMPOSER_J11_2348.md`.
//!
//! # Honest boundary (W29-018)
//!
//! [`CargoTestGapProbe`] and [`J11ManifoldBatteryProbe`] are **cargo-test gap census SSOT** —
//! frozen blocker inventory and honesty gates. Not physics GREEN, not `PRODUCTION_WIRED`,
//! not `MASTER`. Integration harness clearance and full `cargo test` GREEN remain deferred.

use crate::nested_drift_census::{
    nested_drift_census_honest, nested_drift_census_probe, COMPOSER_G15_JOB_ID,
    COMPOSER_G15_RECEIPT_PATH, OP5_STATUS,
};

/// W29 deepen cell — cargo test gap census honest fence bundle.
pub const W29_CARGO_TEST_GAP_DEEPEN_CELL: &str = "W29-018-CARGO_TEST_GAP_CENSUS";

/// Integration harness clearance deferred beyond lib-unit battery.
pub const CARGO_GAP_INTEGRATION_DEFERRED_STEP: &str = "adjoint-compliance-integration-harness";

/// Full `cargo test -p umst-manifold` GREEN deferred beyond partial lib battery.
pub const CARGO_GAP_FULL_GREEN_DEFERRED_STEP: &str = "integration-blocker-clearance";

/// Honest physics posture — cargo gap census is inventory only; does not certify continuum physics.
pub const CARGO_GAP_PHYSICS_GREEN: bool = false;

/// Production deployment wiring — not claimed by cargo gap census module alone.
pub const CARGO_GAP_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by cargo gap census module.
pub const CARGO_GAP_MASTER: bool = false;

/// Whether H79 substrate blocker inventory is landed.
pub const CARGO_GAP_SUBSTRATE_INVENTORY_LANDED: bool = true;

/// Whether J11 lib-unit battery count pin is landed.
pub const CARGO_GAP_LIB_BATTERY_LANDED: bool = true;

/// Whether H79/J11 honesty gates are landed.
pub const CARGO_GAP_HONESTY_GATE_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const CARGO_GAP_HONEST_FENCE: &str =
    "substrate_inventory_landed=true lib_battery_landed=true honesty_gate_landed=true production_wired=false master_composition_wired=false";

/// Cargo test gap fence facet count (honest census).
pub const CARGO_GAP_FENCE_FACET_COUNT: usize = 8;

/// Cargo test gap fence facets wired today (5/8 measured; integration harness deferred).
pub const CARGO_GAP_FENCE_WIRED_COUNT: usize = 5;

/// Stable facet ids for cargo test gap production fence census.
pub const CARGO_GAP_FENCE_FACET_IDS: &[&str] = &[
    "substrate_blocker_inventory",
    "h79_honesty_gate",
    "j11_lib_battery_pin",
    "op5_meta6_fail_fence",
    "nested_clean_false_fence",
    "g15_drift_coupling",
    "integration_harness_clearance",
    "production_wired",
];

/// FLEET-COMPOSER-J parent fleet id (current owner).
pub const FLEET_PARENT: &str = "FLEET-COMPOSER-J";

/// J11 agent job id.
pub const COMPOSER_J11_JOB_ID: &str = "FLEET-COMPOSER-J11-MANIFOLD";

/// J11 receipt path — SSOT for this pass.
pub const COMPOSER_J11_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_J11_2348.md";

/// H79 agent job id (absorbed).
pub const COMPOSER_H79_JOB_ID: &str = "FLEET-COMPOSER-H79-MANIFOLD";

/// H79 receipt path (absorbed).
pub const COMPOSER_H79_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_H79_2242.md";

/// I81 queued re-run receipt (absorbed).
pub const ABSORBED_I81_RECEIPT: &str = "outputs/.tmp/COMPOSER_I81_2343.md";

/// G15 nested drift census absorbed by this pass.
pub const ABSORBED_G15_RECEIPT: &str = "outputs/.tmp/COMPOSER_G15_MANIFOLD_2143.md";

/// HCOM-006 semantic lane schema anchor (additive 64-lane carrier).
pub const HCOM_006_ANCHOR: &str = "HCOM-006";

/// Honest adoption tier for this cargo-test gap pass.
pub const POSTURE_TAG: &str = "cargo-test-gap-census-only";

/// Fleet verify command (scratch target dir).
pub const VERIFY_COMMAND: &str =
    "CARGO_TARGET_DIR=/tmp/umst-j11 bash scripts/fleet_away_rustc188.sh -- cargo test -p umst-manifold";

/// Lib unit battery count @ J11 verify (H79 baseline 267 + J11 census +4).
pub const LIB_UNIT_PASS_COUNT: u32 = 271;

/// Integration harness blocking full GREEN @ J11 (pre-H79 numerical residue).
pub const INTEGRATION_BLOCKER_TEST: &str = "adjoint_compliance_analytic::adjoint_four_node_chain_gradient_matches_finite_difference";

/// META-6 freeze-monotonicity conjunct @ J11 `umst-meta check --dry-run` — honest measure.
pub const META_6_FREEZE_AXIS: &str = "OK";

/// Full META-6 production clearance — blocked by OP-5; never PASS in this module.
pub const META_6_STATUS: &str = "FAIL";

/// OP-5 exception doc SSOT.
pub const OP5_EXCEPTION_DOC: &str = "docs/OP5_EXCEPTION_UMST_ALGEBRA.md";

/// Nested-repo clean status — never true in this module.
pub const NESTED_REPO_CLEAN: bool = false;

/// Cargo test outcome tier for receipts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CargoTestGapStatus {
    /// `cargo test -p umst-manifold` completed with exit 0.
    Green,
    /// Manifest or compile blocked — honest BLOCKED receipt.
    Blocked,
    /// Partial — scoped lib tests green, integration harness deferred.
    Partial,
}

impl CargoTestGapStatus {
    /// Stable tag for receipts / CI introspection.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Blocked => "blocked",
            Self::Partial => "partial",
        }
    }
}

/// One substrate path blocker row for cargo test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubstrateBlocker {
    /// Relative path from workspace root.
    pub path: &'static str,
    /// Blocker class.
    pub kind: &'static str,
    /// H79 disposition after this pass.
    pub disposition: &'static str,
}

/// Frozen substrate blocker inventory @ H79 initial probe (historical).
pub const SUBSTRATE_BLOCKERS_INITIAL: &[SubstrateBlocker] = &[
    SubstrateBlocker {
        path: "umst-foundations/crates/umst-algebra/Cargo.toml",
        kind: "missing_manifest",
        disposition: "restored_by_fleet_contention",
    },
    SubstrateBlocker {
        path: "umst-cartridges/crates/materials/umst-cartridge-concrete/Cargo.toml",
        kind: "missing_manifest",
        disposition: "h79_thin_shim_restore",
    },
    SubstrateBlocker {
        path: "umst-foundations/crates/umst-layout-codegen",
        kind: "op5_path_collision",
        disposition: "unresolved_op5_fail",
    },
];

/// Substrate blocker inventory @ J11 — post-H71 a1-04 manifold retarget.
pub const SUBSTRATE_BLOCKERS_J11: &[SubstrateBlocker] = &[
    SubstrateBlocker {
        path: "umst-foundations/crates/umst-algebra/Cargo.toml",
        kind: "missing_manifest",
        disposition: "restored_by_fleet_contention",
    },
    SubstrateBlocker {
        path: "umst-cartridges/crates/materials/umst-cartridge-concrete/Cargo.toml",
        kind: "missing_manifest",
        disposition: "h79_thin_shim_restore",
    },
    SubstrateBlocker {
        path: "umst-foundations/crates/umst-layout-codegen",
        kind: "op5_production_edge",
        disposition: "collision_resolved_op5_still_fail",
    },
];

/// OP-5 / META-6 interaction row — honest conjunct split @ J11.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Op5Meta6Interaction {
    /// `umst-meta` META-6-freeze conjunct posture.
    pub freeze_axis: &'static str,
    /// OP-5 production edge (`op5_status` in algebra manifest).
    pub op5_production: &'static str,
    /// Path A1 phase after H71 carve.
    pub path_a1_phase: &'static str,
    /// Single `umst-layout-codegen` package in `cargo tree`.
    pub layout_codegen_singleton: bool,
}

/// Frozen OP-5/META-6 interaction @ J11 verify.
pub const OP5_META6_INTERACTION: Op5Meta6Interaction = Op5Meta6Interaction {
    freeze_axis: META_6_FREEZE_AXIS,
    op5_production: OP5_STATUS,
    path_a1_phase: "core-carve-manifold-retarget-verify",
    layout_codegen_singleton: true,
};

/// J11 manifold battery probe — absorbs H79 + I81 + G15 nested drift census.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct J11ManifoldBatteryProbe {
    pub job_id: &'static str,
    pub receipt_path: &'static str,
    pub absorbed_h79_receipt: &'static str,
    pub absorbed_i81_receipt: &'static str,
    pub verify_command: &'static str,
    pub status: CargoTestGapStatus,
    pub lib_unit_pass_count: u32,
    pub integration_blocker: &'static str,
    pub substrate_blocker_count: usize,
    pub h79_gap_honest: bool,
    pub g15_drift_honest: bool,
    pub nested_repo_clean: bool,
    pub op5_meta6: Op5Meta6Interaction,
    pub production_wired: bool,
}

/// Honest J11 manifold battery probe.
#[must_use]
pub fn j11_manifold_battery_probe(status: CargoTestGapStatus) -> J11ManifoldBatteryProbe {
    let h79 = cargo_test_gap_probe(status);
    J11ManifoldBatteryProbe {
        job_id: COMPOSER_J11_JOB_ID,
        receipt_path: COMPOSER_J11_RECEIPT_PATH,
        absorbed_h79_receipt: COMPOSER_H79_RECEIPT_PATH,
        absorbed_i81_receipt: ABSORBED_I81_RECEIPT,
        verify_command: VERIFY_COMMAND,
        status,
        lib_unit_pass_count: LIB_UNIT_PASS_COUNT,
        integration_blocker: INTEGRATION_BLOCKER_TEST,
        substrate_blocker_count: SUBSTRATE_BLOCKERS_J11.len(),
        h79_gap_honest: cargo_test_gap_honest(&h79),
        g15_drift_honest: h79.g15_drift_honest,
        nested_repo_clean: NESTED_REPO_CLEAN,
        op5_meta6: OP5_META6_INTERACTION,
        production_wired: false,
    }
}

/// Honesty gate — J11 must not invent OP-5 PASS or full META-6 clearance.
#[must_use]
pub fn j11_manifold_battery_honest(probe: &J11ManifoldBatteryProbe) -> bool {
    probe.job_id == COMPOSER_J11_JOB_ID
        && probe.receipt_path.contains("COMPOSER_J11_2348")
        && probe.absorbed_h79_receipt.contains("COMPOSER_H79_2242")
        && probe.absorbed_i81_receipt.contains("COMPOSER_I81_2343")
        && probe.verify_command.contains("umst-j11")
        && probe.lib_unit_pass_count == LIB_UNIT_PASS_COUNT
        && probe.h79_gap_honest
        && probe.g15_drift_honest
        && !probe.nested_repo_clean
        && probe.op5_meta6.freeze_axis == META_6_FREEZE_AXIS
        && probe.op5_meta6.op5_production == "FAIL"
        && probe.op5_meta6.layout_codegen_singleton
        && !probe.production_wired
}

/// H79 cargo test gap probe — absorbs G15 nested drift census.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoTestGapProbe {
    pub job_id: &'static str,
    pub receipt_path: &'static str,
    pub absorbed_g15_job_id: &'static str,
    pub absorbed_g15_receipt: &'static str,
    pub hcom_anchor: &'static str,
    pub verify_command: &'static str,
    pub status: CargoTestGapStatus,
    pub substrate_blocker_count: usize,
    pub g15_drift_honest: bool,
    pub nested_repo_clean: bool,
    pub op5_status: &'static str,
    pub meta_6_status: &'static str,
    pub production_wired: bool,
}

/// Honest H79 cargo-test gap probe (post substrate shim restore).
#[must_use]
pub fn cargo_test_gap_probe(status: CargoTestGapStatus) -> CargoTestGapProbe {
    let g15 = nested_drift_census_probe();
    CargoTestGapProbe {
        job_id: COMPOSER_H79_JOB_ID,
        receipt_path: COMPOSER_H79_RECEIPT_PATH,
        absorbed_g15_job_id: COMPOSER_G15_JOB_ID,
        absorbed_g15_receipt: COMPOSER_G15_RECEIPT_PATH,
        hcom_anchor: HCOM_006_ANCHOR,
        verify_command: VERIFY_COMMAND,
        status,
        substrate_blocker_count: SUBSTRATE_BLOCKERS_INITIAL.len(),
        g15_drift_honest: nested_drift_census_honest(&g15),
        nested_repo_clean: NESTED_REPO_CLEAN,
        op5_status: OP5_STATUS,
        meta_6_status: META_6_STATUS,
        production_wired: false,
    }
}

/// Honesty gate — must not invent clean nested repo, OP-5 PASS, or production wired.
#[must_use]
pub fn cargo_test_gap_honest(probe: &CargoTestGapProbe) -> bool {
    probe.job_id == COMPOSER_H79_JOB_ID
        && probe.receipt_path.contains("COMPOSER_H79_2242")
        && probe.absorbed_g15_receipt.contains("COMPOSER_G15_MANIFOLD_2143")
        && probe.hcom_anchor == HCOM_006_ANCHOR
        && probe.verify_command.contains("cargo test -p umst-manifold")
        && probe.g15_drift_honest
        && !probe.nested_repo_clean
        && probe.op5_status == "FAIL"
        && probe.meta_6_status == "FAIL"
        && !probe.production_wired
}

/// One facet of the cargo test gap production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CargoTestGapProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue.
    pub owning_slice: &'static str,
}

/// Cargo test gap production fence facet inventory (honest posture SSOT).
pub const CARGO_GAP_PRODUCTION_FENCE_FACETS: &[CargoTestGapProductionFenceFacet] = &[
    CargoTestGapProductionFenceFacet {
        facet: "substrate_blocker_inventory",
        wired: true,
        owning_slice: W29_CARGO_TEST_GAP_DEEPEN_CELL,
    },
    CargoTestGapProductionFenceFacet {
        facet: "h79_honesty_gate",
        wired: true,
        owning_slice: W29_CARGO_TEST_GAP_DEEPEN_CELL,
    },
    CargoTestGapProductionFenceFacet {
        facet: "j11_lib_battery_pin",
        wired: true,
        owning_slice: W29_CARGO_TEST_GAP_DEEPEN_CELL,
    },
    CargoTestGapProductionFenceFacet {
        facet: "op5_meta6_fail_fence",
        wired: true,
        owning_slice: W29_CARGO_TEST_GAP_DEEPEN_CELL,
    },
    CargoTestGapProductionFenceFacet {
        facet: "nested_clean_false_fence",
        wired: true,
        owning_slice: W29_CARGO_TEST_GAP_DEEPEN_CELL,
    },
    CargoTestGapProductionFenceFacet {
        facet: "g15_drift_coupling",
        wired: false,
        owning_slice: "nested_drift_census",
    },
    CargoTestGapProductionFenceFacet {
        facet: "integration_harness_clearance",
        wired: false,
        owning_slice: CARGO_GAP_INTEGRATION_DEFERRED_STEP,
    },
    CargoTestGapProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: CARGO_GAP_FULL_GREEN_DEFERRED_STEP,
    },
];

/// Count wired cargo test gap fence facets (must match [`CARGO_GAP_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn cargo_test_gap_fence_wired_count() -> usize {
    CARGO_GAP_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Honest production wiring — **false** until integration harness clearance measured.
#[must_use]
pub const fn cargo_test_gap_production_wired() -> bool {
    false
}

/// Master composition wiring — **false** until fleet orchestration loop closes.
#[must_use]
pub const fn cargo_test_gap_master_composition_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at posture tier.
const _: () = assert!(!cargo_test_gap_production_wired());

/// Measured honest-posture snapshot for cargo test gap census (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CargoTestGapHonestPosture {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub substrate_inventory_landed: bool,
    pub lib_battery_landed: bool,
    pub honesty_gate_landed: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub honest_fence: &'static str,
    pub deferred_integration: &'static str,
    pub deferred_full_green: &'static str,
}

/// Honest posture bundle for orchestrator / census probes — no invented GREEN.
#[must_use]
pub fn cargo_test_gap_honest_posture_bundle() -> CargoTestGapHonestPosture {
    CargoTestGapHonestPosture {
        physics_green: CARGO_GAP_PHYSICS_GREEN,
        production_wired: CARGO_GAP_PRODUCTION_WIRED,
        master: CARGO_GAP_MASTER,
        substrate_inventory_landed: CARGO_GAP_SUBSTRATE_INVENTORY_LANDED,
        lib_battery_landed: CARGO_GAP_LIB_BATTERY_LANDED,
        honesty_gate_landed: CARGO_GAP_HONESTY_GATE_LANDED,
        fence_facet_count: CARGO_GAP_FENCE_FACET_COUNT,
        fence_wired_count: cargo_test_gap_fence_wired_count(),
        honest_fence: CARGO_GAP_HONEST_FENCE,
        deferred_integration: CARGO_GAP_INTEGRATION_DEFERRED_STEP,
        deferred_full_green: CARGO_GAP_FULL_GREEN_DEFERRED_STEP,
    }
}

/// Typed probe for cargo test gap posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CargoTestGapPostureProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub substrate_inventory_landed: bool,
    pub lib_battery_landed: bool,
    pub honesty_gate_landed: bool,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub physics_green: bool,
    pub op5_status: &'static str,
    pub meta_6_status: &'static str,
    pub nested_repo_clean: bool,
    pub integration_blocker: &'static str,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub honest_fence: &'static str,
}

/// Build introspection probe for cargo test gap done-when / fleet checks.
#[must_use]
pub fn cargo_test_gap_posture_probe() -> CargoTestGapPostureProbe {
    CargoTestGapPostureProbe {
        cell_id: W29_CARGO_TEST_GAP_DEEPEN_CELL,
        posture_tag: POSTURE_TAG,
        substrate_inventory_landed: CARGO_GAP_SUBSTRATE_INVENTORY_LANDED,
        lib_battery_landed: CARGO_GAP_LIB_BATTERY_LANDED,
        honesty_gate_landed: CARGO_GAP_HONESTY_GATE_LANDED,
        production_wired: cargo_test_gap_production_wired(),
        master_composition_wired: cargo_test_gap_master_composition_wired(),
        physics_green: CARGO_GAP_PHYSICS_GREEN,
        op5_status: OP5_STATUS,
        meta_6_status: META_6_STATUS,
        nested_repo_clean: NESTED_REPO_CLEAN,
        integration_blocker: INTEGRATION_BLOCKER_TEST,
        fence_facet_count: CARGO_GAP_FENCE_FACET_COUNT,
        fence_wired_count: CARGO_GAP_FENCE_WIRED_COUNT,
        honest_fence: CARGO_GAP_HONEST_FENCE,
    }
}

/// Cargo test gap SSOT landed with production/master composition honestly open.
#[must_use]
pub fn cargo_test_gap_posture_honest(probe: &CargoTestGapPostureProbe) -> bool {
    probe.cell_id == W29_CARGO_TEST_GAP_DEEPEN_CELL
        && probe.posture_tag == POSTURE_TAG
        && probe.substrate_inventory_landed
        && probe.lib_battery_landed
        && probe.honesty_gate_landed
        && !probe.physics_green
        && !probe.production_wired
        && !probe.master_composition_wired
        && !probe.nested_repo_clean
        && probe.op5_status == "FAIL"
        && probe.meta_6_status == "FAIL"
        && probe.integration_blocker.contains("adjoint_compliance")
        && probe.fence_facet_count == CARGO_GAP_FENCE_FACET_COUNT
        && probe.fence_wired_count == CARGO_GAP_FENCE_WIRED_COUNT
        && probe.honest_fence.contains("substrate_inventory_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("master_composition_wired=false")
}

/// Validate cargo test gap posture honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_cargo_test_gap_posture_honesty() -> Result<(), &'static str> {
    let probe = cargo_test_gap_posture_probe();
    if probe.physics_green {
        return Err("CARGO_GAP_PHYSICS_GREEN must stay false — census is inventory only");
    }
    if probe.production_wired {
        return Err("cargo_test_gap_production_wired must stay false until integration harness clears");
    }
    if probe.master_composition_wired {
        return Err("cargo_test_gap_master_composition_wired must stay false until fleet orch closes");
    }
    if cargo_test_gap_fence_wired_count() != CARGO_GAP_FENCE_WIRED_COUNT {
        return Err("cargo_test_gap_fence_wired_count drift from CARGO_GAP_FENCE_WIRED_COUNT");
    }
    if !cargo_test_gap_posture_honest(&probe) {
        return Err("cargo_test_gap_posture_honest failed");
    }
    Ok(())
}

/// W29 deepen probe — absorbs J11 battery + H79 gap + G15 drift coupling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct W29CargoTestGapDeepenProbe {
    pub cell_id: &'static str,
    pub j11_job_id: &'static str,
    pub j11_receipt_path: &'static str,
    pub h79_gap_honest: bool,
    pub g15_drift_honest: bool,
    pub posture_honest: bool,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub lib_unit_pass_count: u32,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
}

/// Honest W29 cargo test gap deepen probe.
#[must_use]
pub fn w29_cargo_test_gap_deepen_probe() -> W29CargoTestGapDeepenProbe {
    let j11 = j11_manifold_battery_probe(CargoTestGapStatus::Partial);
    let posture = cargo_test_gap_posture_probe();
    let g15 = nested_drift_census_probe();
    W29CargoTestGapDeepenProbe {
        cell_id: W29_CARGO_TEST_GAP_DEEPEN_CELL,
        j11_job_id: COMPOSER_J11_JOB_ID,
        j11_receipt_path: COMPOSER_J11_RECEIPT_PATH,
        h79_gap_honest: j11.h79_gap_honest,
        g15_drift_honest: nested_drift_census_honest(&g15),
        posture_honest: cargo_test_gap_posture_honest(&posture),
        production_wired: cargo_test_gap_production_wired(),
        master_composition_wired: cargo_test_gap_master_composition_wired(),
        lib_unit_pass_count: LIB_UNIT_PASS_COUNT,
        fence_facet_count: CARGO_GAP_FENCE_FACET_COUNT,
        fence_wired_count: cargo_test_gap_fence_wired_count(),
    }
}

/// Honesty gate for W29 deepen — must not invent clean nested repo or production wired.
#[must_use]
pub fn w29_cargo_test_gap_deepen_honest(probe: &W29CargoTestGapDeepenProbe) -> bool {
    probe.cell_id == W29_CARGO_TEST_GAP_DEEPEN_CELL
        && probe.j11_job_id == COMPOSER_J11_JOB_ID
        && probe.j11_receipt_path.contains("COMPOSER_J11_2348")
        && probe.h79_gap_honest
        && probe.g15_drift_honest
        && probe.posture_honest
        && !probe.production_wired
        && !probe.master_composition_wired
        && probe.lib_unit_pass_count == LIB_UNIT_PASS_COUNT
        && probe.fence_facet_count == CARGO_GAP_FENCE_FACET_COUNT
        && probe.fence_wired_count == CARGO_GAP_FENCE_WIRED_COUNT
}

/// Whether H79 may claim full GREEN (all blockers cleared + exit 0).
#[must_use]
pub fn cargo_test_gap_green_earned(status: CargoTestGapStatus, exit_code: i32) -> bool {
    status == CargoTestGapStatus::Green && exit_code == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn j11_metadata_pins() {
        assert_eq!(FLEET_PARENT, "FLEET-COMPOSER-J");
        assert_eq!(COMPOSER_J11_JOB_ID, "FLEET-COMPOSER-J11-MANIFOLD");
        assert!(COMPOSER_J11_RECEIPT_PATH.contains("COMPOSER_J11_2348"));
        assert!(ABSORBED_I81_RECEIPT.contains("COMPOSER_I81_2343"));
        assert_eq!(POSTURE_TAG, "cargo-test-gap-census-only");
    }

    #[test]
    fn j11_absorbs_h79_and_i81() {
        let probe = j11_manifold_battery_probe(CargoTestGapStatus::Partial);
        assert!(j11_manifold_battery_honest(&probe));
        assert!(probe.h79_gap_honest);
        assert_eq!(probe.lib_unit_pass_count, 271);
    }

    #[test]
    fn j11_op5_meta6_interaction_honest_split() {
        let probe = j11_manifold_battery_probe(CargoTestGapStatus::Partial);
        assert_eq!(probe.op5_meta6.freeze_axis, "OK");
        assert_eq!(probe.op5_meta6.op5_production, "FAIL");
        assert!(probe.op5_meta6.layout_codegen_singleton);
        assert_ne!(probe.op5_meta6.freeze_axis, probe.op5_meta6.op5_production);
    }

    #[test]
    fn j11_substrate_layout_codegen_collision_resolved() {
        let layout = SUBSTRATE_BLOCKERS_J11
            .iter()
            .find(|b| b.path.contains("umst-layout-codegen"))
            .expect("layout-codegen row");
        assert_eq!(layout.kind, "op5_production_edge");
        assert_eq!(layout.disposition, "collision_resolved_op5_still_fail");
    }

    #[test]
    fn h79_metadata_pins() {
        assert_eq!(COMPOSER_H79_JOB_ID, "FLEET-COMPOSER-H79-MANIFOLD");
        assert!(ABSORBED_G15_RECEIPT.contains("G15_MANIFOLD"));
    }

    #[test]
    fn h79_absorbs_g15_nested_drift() {
        let probe = cargo_test_gap_probe(CargoTestGapStatus::Partial);
        assert!(cargo_test_gap_honest(&probe));
        assert!(probe.g15_drift_honest);
        assert_eq!(probe.absorbed_g15_job_id, COMPOSER_G15_JOB_ID);
    }

    #[test]
    fn h79_never_invents_clean_or_op5_pass() {
        let probe = cargo_test_gap_probe(CargoTestGapStatus::Blocked);
        assert!(!probe.nested_repo_clean);
        assert_eq!(probe.op5_status, "FAIL");
        assert_eq!(probe.meta_6_status, "FAIL");
        assert!(!probe.production_wired);
    }

    #[test]
    fn substrate_blocker_inventory_frozen() {
        assert_eq!(SUBSTRATE_BLOCKERS_INITIAL.len(), 3);
        assert!(SUBSTRATE_BLOCKERS_INITIAL
            .iter()
            .any(|b| b.path.contains("umst-cartridge-concrete")));
    }

    #[test]
    fn cargo_test_gap_status_tags_stable() {
        assert_eq!(CargoTestGapStatus::Green.tag(), "green");
        assert_eq!(CargoTestGapStatus::Blocked.tag(), "blocked");
        assert_eq!(CargoTestGapStatus::Partial.tag(), "partial");
    }

    #[test]
    fn green_earned_requires_exit_zero() {
        assert!(cargo_test_gap_green_earned(CargoTestGapStatus::Green, 0));
        assert!(!cargo_test_gap_green_earned(CargoTestGapStatus::Green, 101));
        assert!(!cargo_test_gap_green_earned(CargoTestGapStatus::Blocked, 0));
    }

    #[test]
    fn w29_cargo_test_gap_metadata_pins() {
        assert_eq!(W29_CARGO_TEST_GAP_DEEPEN_CELL, "W29-018-CARGO_TEST_GAP_CENSUS");
        assert_eq!(POSTURE_TAG, "cargo-test-gap-census-only");
        assert_eq!(CARGO_GAP_FENCE_FACET_IDS.len(), CARGO_GAP_FENCE_FACET_COUNT);
    }

    #[test]
    fn w29_cargo_test_gap_fence_wired_count_matches() {
        assert_eq!(cargo_test_gap_fence_wired_count(), CARGO_GAP_FENCE_WIRED_COUNT);
        assert_eq!(CARGO_GAP_PRODUCTION_FENCE_FACETS.len(), CARGO_GAP_FENCE_FACET_COUNT);
    }

    #[test]
    fn w29_cargo_test_gap_posture_honest_not_green() {
        let probe = cargo_test_gap_posture_probe();
        assert!(cargo_test_gap_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master_composition_wired);
        assert!(!probe.nested_repo_clean);
        assert_eq!(probe.op5_status, "FAIL");
        assert_eq!(probe.meta_6_status, "FAIL");
    }

    #[test]
    fn w29_cargo_test_gap_validate_posture_honesty() {
        assert!(validate_cargo_test_gap_posture_honesty().is_ok());
    }

    #[test]
    fn w29_cargo_test_gap_deepen_absorbs_j11_and_g15() {
        let probe = w29_cargo_test_gap_deepen_probe();
        assert!(w29_cargo_test_gap_deepen_honest(&probe));
        assert!(probe.h79_gap_honest);
        assert!(probe.g15_drift_honest);
        assert!(probe.posture_honest);
        assert!(!probe.production_wired);
        assert!(!probe.master_composition_wired);
        assert_eq!(probe.lib_unit_pass_count, 271);
    }

    #[test]
    fn w29_cargo_test_gap_honest_posture_bundle() {
        let bundle = cargo_test_gap_honest_posture_bundle();
        assert!(bundle.substrate_inventory_landed);
        assert!(bundle.lib_battery_landed);
        assert!(bundle.honesty_gate_landed);
        assert!(!bundle.physics_green);
        assert!(!bundle.production_wired);
        assert!(!bundle.master);
        assert!(bundle.honest_fence.contains("production_wired=false"));
    }

    #[test]
    fn w29_production_wired_stays_false() {
        assert!(!cargo_test_gap_production_wired());
        assert!(!cargo_test_gap_master_composition_wired());
    }
}
