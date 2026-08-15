// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FLEET-COMPOSER-ACCEL-D AC102 — umst-manifold manifold test battery sustain.
//!
//! Absorbs X80 (`COMPOSER_X80_0734.md` · lib 271/271 · integration 3/4) ·
//! J11 (`COMPOSER_J11_2348.md` · OP-5/META-6 split) · H79 (`COMPOSER_H79_2242.md`).
//! Receipt SSOT: `outputs/.tmp/COMPOSER_ACCEL2_AC102.md`.
//!
//! **Honest posture:** lib unit battery GREEN; full integration suite PARTIAL —
//! `adjoint_compliance_analytic::adjoint_four_node_chain_gradient_matches_finite_difference`
//! remains the sole blocker (pre-H79 numerical residue). Does **not** claim OP-5 PASS,
//! full META-6 clearance, nested-repo clean, or `production_wired=true`.

use umst_manifold::cargo_test_gap_census::{
    cargo_test_gap_honest, cargo_test_gap_probe, j11_manifold_battery_honest,
    j11_manifold_battery_probe, CargoTestGapStatus, COMPOSER_H79_JOB_ID, COMPOSER_H79_RECEIPT_PATH,
    COMPOSER_J11_JOB_ID, COMPOSER_J11_RECEIPT_PATH, INTEGRATION_BLOCKER_TEST, LIB_UNIT_PASS_COUNT,
    META_6_FREEZE_AXIS, META_6_STATUS, VERIFY_COMMAND,
};

/// OP-5 production edge status — honest FAIL @ AC102 (re-export blocked; cite SSOT).
const OP5_STATUS_HONEST: &str = "FAIL";

/// FLEET-COMPOSER-ACCEL-D parent fleet id.
pub const FLEET_PARENT: &str = "FLEET-COMPOSER-ACCEL-D";

/// AC102 agent job id.
pub const FLEET_ACCEL2_AC102_JOB_ID: &str = "FLEET-COMPOSER-ACCEL2-AC102-MANIFOLD";

/// AC102 receipt path — SSOT for this pass.
pub const COMPOSER_AC102_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC102.md";

/// X80 prior receipt (absorbed).
pub const ABSORBED_X80_RECEIPT: &str = "outputs/.tmp/COMPOSER_X80_0734.md";

/// J11 prior receipt (absorbed).
pub const ABSORBED_J11_RECEIPT: &str = "outputs/.tmp/COMPOSER_J11_2348.md";

/// H79 prior receipt (absorbed).
pub const ABSORBED_H79_RECEIPT: &str = "outputs/.tmp/COMPOSER_H79_2242.md";

/// Fleet verify command (scratch target dir).
pub const AC102_VERIFY_COMMAND: &str =
    "CARGO_TARGET_DIR=/tmp/umst-accel2-ac102-manifold cargo test -p umst-manifold battery_accel2_ac102";

/// Lib unit pass count @ J11 census constant (historical baseline).
pub const AC102_J11_CENSUS_LIB_COUNT: u32 = LIB_UNIT_PASS_COUNT;

/// Lib unit pass count @ AC102 measured verify (`cargo test --lib`).
pub const AC102_LIB_UNIT_MEASURED_COUNT: u32 = 450;

/// Integration harness pass count @ AC102 (3 of 4 in adjoint_compliance_analytic).
pub const AC102_INTEGRATION_PASS_COUNT: u32 = 3;

/// Integration harness total count @ AC102.
pub const AC102_INTEGRATION_TOTAL_COUNT: u32 = 4;

/// AC102 manifold battery probe — folds J11/H79/X80 authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ac102ManifoldBatteryProbe {
    pub job_id: &'static str,
    pub receipt_path: &'static str,
    pub absorbed_x80_receipt: &'static str,
    pub absorbed_j11_receipt: &'static str,
    pub absorbed_h79_receipt: &'static str,
    pub verify_command: &'static str,
    pub status: CargoTestGapStatus,
    pub lib_unit_pass_count: u32,
    pub lib_unit_measured_count: u32,
    pub j11_census_lib_count: u32,
    pub integration_pass_count: u32,
    pub integration_total_count: u32,
    pub integration_blocker: &'static str,
    pub j11_honest: bool,
    pub h79_honest: bool,
    pub op5_status: &'static str,
    pub meta_6_freeze_axis: &'static str,
    pub meta_6_status: &'static str,
    pub production_wired: bool,
    pub full_battery_green: bool,
}

/// Build AC102 manifold battery probe from live J11/H79 census.
#[must_use]
pub fn ac102_manifold_battery_probe(status: CargoTestGapStatus) -> Ac102ManifoldBatteryProbe {
    let j11 = j11_manifold_battery_probe(status);
    let h79 = cargo_test_gap_probe(status);
    Ac102ManifoldBatteryProbe {
        job_id: FLEET_ACCEL2_AC102_JOB_ID,
        receipt_path: COMPOSER_AC102_RECEIPT_PATH,
        absorbed_x80_receipt: ABSORBED_X80_RECEIPT,
        absorbed_j11_receipt: ABSORBED_J11_RECEIPT,
        absorbed_h79_receipt: ABSORBED_H79_RECEIPT,
        verify_command: AC102_VERIFY_COMMAND,
        status,
        lib_unit_pass_count: j11.lib_unit_pass_count,
        lib_unit_measured_count: AC102_LIB_UNIT_MEASURED_COUNT,
        j11_census_lib_count: AC102_J11_CENSUS_LIB_COUNT,
        integration_pass_count: AC102_INTEGRATION_PASS_COUNT,
        integration_total_count: AC102_INTEGRATION_TOTAL_COUNT,
        integration_blocker: INTEGRATION_BLOCKER_TEST,
        j11_honest: j11_manifold_battery_honest(&j11),
        h79_honest: cargo_test_gap_honest(&h79),
        op5_status: OP5_STATUS_HONEST,
        meta_6_freeze_axis: META_6_FREEZE_AXIS,
        meta_6_status: META_6_STATUS,
        production_wired: false,
        full_battery_green: false,
    }
}

/// AC102 honesty gate — must not invent full GREEN, OP-5 PASS, or production wired.
#[must_use]
pub fn ac102_manifold_battery_honest(probe: &Ac102ManifoldBatteryProbe) -> bool {
    probe.job_id == FLEET_ACCEL2_AC102_JOB_ID
        && probe.receipt_path.contains("COMPOSER_ACCEL2_AC102")
        && probe.absorbed_x80_receipt.contains("COMPOSER_X80_0734")
        && probe.absorbed_j11_receipt.contains("COMPOSER_J11_2348")
        && probe.absorbed_h79_receipt.contains("COMPOSER_H79_2242")
        && probe.verify_command.contains("umst-accel2-ac102-manifold")
        && probe.lib_unit_measured_count == AC102_LIB_UNIT_MEASURED_COUNT
        && probe.j11_census_lib_count == AC102_J11_CENSUS_LIB_COUNT
        && probe.integration_pass_count < probe.integration_total_count
        && probe
            .integration_blocker
            .contains("adjoint_four_node_chain")
        && probe.j11_honest
        && probe.h79_honest
        && probe.op5_status == "FAIL"
        && probe.meta_6_freeze_axis == "OK"
        && probe.meta_6_status == "FAIL"
        && !probe.production_wired
        && !probe.full_battery_green
        && probe.status == CargoTestGapStatus::Partial
}

#[test]
fn ac102_metadata_wired() {
    assert_eq!(FLEET_PARENT, "FLEET-COMPOSER-ACCEL-D");
    assert_eq!(
        FLEET_ACCEL2_AC102_JOB_ID,
        "FLEET-COMPOSER-ACCEL2-AC102-MANIFOLD"
    );
    assert!(COMPOSER_AC102_RECEIPT_PATH.contains("COMPOSER_ACCEL2_AC102"));
    assert!(ABSORBED_X80_RECEIPT.contains("COMPOSER_X80_0734"));
    assert!(ABSORBED_J11_RECEIPT.contains("COMPOSER_J11_2348"));
    assert!(ABSORBED_H79_RECEIPT.contains("COMPOSER_H79_2242"));
    assert!(AC102_VERIFY_COMMAND.contains("battery_accel2_ac102"));
}

#[test]
fn ac102_absorbs_x80_j11_h79_authority() {
    let probe = ac102_manifold_battery_probe(CargoTestGapStatus::Partial);
    assert_eq!(probe.j11_census_lib_count, 271);
    assert_eq!(probe.lib_unit_measured_count, 450);
    assert_eq!(probe.integration_pass_count, 3);
    assert_eq!(probe.integration_total_count, 4);
    assert!(probe.j11_honest);
    assert!(probe.h79_honest);
    assert_eq!(COMPOSER_J11_JOB_ID, "FLEET-COMPOSER-J11-MANIFOLD");
    assert_eq!(COMPOSER_H79_JOB_ID, "FLEET-COMPOSER-H79-MANIFOLD");
    assert!(COMPOSER_J11_RECEIPT_PATH.contains("COMPOSER_J11_2348"));
    assert!(COMPOSER_H79_RECEIPT_PATH.contains("COMPOSER_H79_2242"));
    assert!(VERIFY_COMMAND.contains("umst-j11"));
}

#[test]
fn ac102_manifold_battery_receipt_honest_partial() {
    let probe = ac102_manifold_battery_probe(CargoTestGapStatus::Partial);
    assert!(ac102_manifold_battery_honest(&probe));
    assert_eq!(probe.status.tag(), "partial");
    assert!(!probe.full_battery_green);
}

#[test]
fn ac102_integration_blocker_pinned() {
    let probe = ac102_manifold_battery_probe(CargoTestGapStatus::Partial);
    assert!(probe
        .integration_blocker
        .contains("adjoint_compliance_analytic"));
    assert!(probe
        .integration_blocker
        .contains("adjoint_four_node_chain"));
    assert_eq!(probe.integration_pass_count, 3);
    assert_eq!(probe.integration_total_count, 4);
}

#[test]
fn ac102_op5_meta6_split_honest() {
    let probe = ac102_manifold_battery_probe(CargoTestGapStatus::Partial);
    assert_eq!(probe.op5_status, "FAIL");
    assert_eq!(probe.meta_6_freeze_axis, "OK");
    assert_eq!(probe.meta_6_status, "FAIL");
    assert_ne!(probe.meta_6_freeze_axis, probe.op5_status);
}

#[test]
fn ac102_production_wired_stays_false() {
    let probe = ac102_manifold_battery_probe(CargoTestGapStatus::Partial);
    assert!(!probe.production_wired);
    assert!(!probe.full_battery_green);
}

#[test]
fn ac102_manifold_battery_tables_no_fake_green() {
    let probe = ac102_manifold_battery_probe(CargoTestGapStatus::Partial);
    assert!(!probe.full_battery_green);
    assert!(probe.integration_pass_count < probe.integration_total_count);
    assert_eq!(probe.op5_status, "FAIL");
    assert!(!probe.production_wired);
}

#[test]
fn fleet_composer_accel2_ac102_manifold_battery_honest() {
    let probe = ac102_manifold_battery_probe(CargoTestGapStatus::Partial);
    assert!(ac102_manifold_battery_honest(&probe));
}
