// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//
// ACCEL-D AC102 — umst-manifold test battery sustain deepen witness.
// Absorbs J11 (`COMPOSER_J11_2348.md`) · X80 (`COMPOSER_X80_0734.md`) · H79.
// `production_wired` stays honest false. No OP-5 PASS. No INV4 4/4 invent.

use umst_manifold::cargo_test_gap_census::{
    cargo_test_gap_honest, cargo_test_gap_probe, j11_manifold_battery_honest,
    j11_manifold_battery_probe, CargoTestGapStatus, COMPOSER_H79_RECEIPT_PATH,
    COMPOSER_J11_RECEIPT_PATH, INTEGRATION_BLOCKER_TEST, LIB_UNIT_PASS_COUNT, META_6_STATUS,
    NESTED_REPO_CLEAN, OP5_EXCEPTION_DOC, VERIFY_COMMAND,
};
use umst_manifold::nested_drift_census::{
    nested_drift_census_honest, nested_drift_census_probe, OP5_STATUS,
};

/// FLEET-COMPOSER-ACCEL-D slot id.
pub const AC102_JOB_ID: &str = "FLEET-COMPOSER-ACCEL-D-AC102-MANIFOLD";

/// AC102 receipt SSOT.
pub const AC102_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC102.md";

/// Scratch target dir for AC102 verify.
pub const AC102_SCRATCH_TARGET: &str = "/tmp/umst-accel2-ac102-manifold";

/// Absorbed X80 manifold battery receipt.
pub const AC102_ABSORBED_X80: &str = "outputs/.tmp/COMPOSER_X80_0734.md";

/// Absorbed J11 full battery + OP-5 measure receipt.
pub const AC102_ABSORBED_J11: &str = "outputs/.tmp/COMPOSER_J11_2348.md";

/// Absorbed H79 cargo-test gap receipt.
pub const AC102_ABSORBED_H79: &str = "outputs/.tmp/COMPOSER_H79_2242.md";

/// Honest posture tag — sustain deepen only.
pub const AC102_POSTURE_TAG: &str = "manifold-battery-sustain-honest-partial";

/// AC102 manifold battery sustain probe — folds J11 census + integration residue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ac102BatterySustainProbe {
    pub job_id: &'static str,
    pub receipt_path: &'static str,
    pub scratch_target: &'static str,
    pub absorbed_x80: &'static str,
    pub absorbed_j11: &'static str,
    pub absorbed_h79: &'static str,
    pub posture_tag: &'static str,
    pub lib_unit_pass_count: u32,
    pub integration_blocker: &'static str,
    pub cargo_status: CargoTestGapStatus,
    pub j11_honest: bool,
    pub h79_honest: bool,
    pub g15_drift_honest: bool,
    pub op5_status: &'static str,
    pub meta_6_status: &'static str,
    pub nested_repo_clean: bool,
    pub production_wired: bool,
    pub master_retick_eligible: bool,
}

/// Honest AC102 battery sustain probe @ partial posture (lib GREEN · integration 3/4).
#[must_use]
pub fn ac102_battery_sustain_probe() -> Ac102BatterySustainProbe {
    let j11 = j11_manifold_battery_probe(CargoTestGapStatus::Partial);
    let h79 = cargo_test_gap_probe(CargoTestGapStatus::Partial);
    let g15 = nested_drift_census_probe();
    Ac102BatterySustainProbe {
        job_id: AC102_JOB_ID,
        receipt_path: AC102_RECEIPT_PATH,
        scratch_target: AC102_SCRATCH_TARGET,
        absorbed_x80: AC102_ABSORBED_X80,
        absorbed_j11: AC102_ABSORBED_J11,
        absorbed_h79: AC102_ABSORBED_H79,
        posture_tag: AC102_POSTURE_TAG,
        lib_unit_pass_count: LIB_UNIT_PASS_COUNT,
        integration_blocker: INTEGRATION_BLOCKER_TEST,
        cargo_status: CargoTestGapStatus::Partial,
        j11_honest: j11_manifold_battery_honest(&j11),
        h79_honest: cargo_test_gap_honest(&h79),
        g15_drift_honest: nested_drift_census_honest(&g15),
        op5_status: OP5_STATUS,
        meta_6_status: META_6_STATUS,
        nested_repo_clean: NESTED_REPO_CLEAN,
        production_wired: false,
        master_retick_eligible: false,
    }
}

/// Honesty gate — must not invent OP-5 PASS, full GREEN, or production wired.
#[must_use]
pub fn ac102_battery_sustain_honest(probe: &Ac102BatterySustainProbe) -> bool {
    probe.job_id == AC102_JOB_ID
        && probe.receipt_path.contains("COMPOSER_ACCEL2_AC102")
        && probe.scratch_target.contains("umst-accel2-ac102-manifold")
        && probe.absorbed_x80.contains("COMPOSER_X80_0734")
        && probe.absorbed_j11.contains("COMPOSER_J11_2348")
        && probe.absorbed_h79.contains("COMPOSER_H79_2242")
        && probe.posture_tag == AC102_POSTURE_TAG
        && probe.lib_unit_pass_count == LIB_UNIT_PASS_COUNT
        && probe
            .integration_blocker
            .contains("adjoint_four_node_chain")
        && probe.cargo_status == CargoTestGapStatus::Partial
        && probe.j11_honest
        && probe.h79_honest
        && probe.g15_drift_honest
        && probe.op5_status == "FAIL"
        && probe.meta_6_status == "FAIL"
        && !probe.nested_repo_clean
        && !probe.production_wired
        && !probe.master_retick_eligible
}

#[test]
fn ac102_job_metadata_pins() {
    assert_eq!(AC102_JOB_ID, "FLEET-COMPOSER-ACCEL-D-AC102-MANIFOLD");
    assert!(AC102_RECEIPT_PATH.contains("COMPOSER_ACCEL2_AC102"));
    assert_eq!(AC102_SCRATCH_TARGET, "/tmp/umst-accel2-ac102-manifold");
    assert!(VERIFY_COMMAND.contains("cargo test -p umst-manifold"));
    assert!(OP5_EXCEPTION_DOC.contains("OP5_EXCEPTION_UMST_ALGEBRA"));
}

#[test]
fn ac102_absorbs_j11_x80_h79_receipts() {
    assert!(COMPOSER_J11_RECEIPT_PATH.contains("COMPOSER_J11_2348"));
    assert!(COMPOSER_H79_RECEIPT_PATH.contains("COMPOSER_H79_2242"));
    assert!(AC102_ABSORBED_X80.contains("COMPOSER_X80_0734"));
    let probe = ac102_battery_sustain_probe();
    assert_eq!(probe.absorbed_j11, AC102_ABSORBED_J11);
    assert_eq!(probe.absorbed_h79, AC102_ABSORBED_H79);
}

#[test]
fn ac102_j11_battery_census_honest_partial() {
    let j11 = j11_manifold_battery_probe(CargoTestGapStatus::Partial);
    assert!(j11_manifold_battery_honest(&j11));
    assert_eq!(j11.lib_unit_pass_count, LIB_UNIT_PASS_COUNT);
    assert_eq!(j11.status, CargoTestGapStatus::Partial);
    assert!(!j11.production_wired);
    assert_eq!(j11.op5_meta6.op5_production, "FAIL");
}

#[test]
fn ac102_integration_blocker_pinned_adjoint_fd() {
    let probe = ac102_battery_sustain_probe();
    assert!(probe
        .integration_blocker
        .contains("adjoint_compliance_analytic"));
    assert!(probe
        .integration_blocker
        .contains("adjoint_four_node_chain"));
    assert_eq!(probe.cargo_status.tag(), "partial");
}

#[test]
fn ac102_production_wired_and_op5_meta6_honest_false() {
    let probe = ac102_battery_sustain_probe();
    assert!(!probe.production_wired);
    assert_eq!(probe.op5_status, "FAIL");
    assert_eq!(probe.meta_6_status, "FAIL");
    assert!(!probe.nested_repo_clean);
}

#[test]
fn ac102_fleet_composer_accel2_battery_sustain_honest() {
    let probe = ac102_battery_sustain_probe();
    assert!(ac102_battery_sustain_honest(&probe));
    assert!(probe.j11_honest);
    assert!(probe.h79_honest);
    assert!(probe.g15_drift_honest);
}

#[test]
fn ac102_master_retick_residue_honest_no() {
    let probe = ac102_battery_sustain_probe();
    assert!(!probe.master_retick_eligible);
    assert_eq!(probe.cargo_status, CargoTestGapStatus::Partial);
    assert!(!probe.production_wired);
}
