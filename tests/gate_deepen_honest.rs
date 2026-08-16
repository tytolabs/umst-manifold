// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! FLEET-COMPOSER ACCEL-G AC157 — MANIFOLD gate deepen honest witness.
//!
//! Consolidates all SEC gate runtime deepen probes on the manifold cold-edge census.
//! Receipt SSOT: `outputs/.tmp/COMPOSER_ACCEL2_AC157.md`.
//!
//! **Honest posture:** all eleven SEC arcs ceremony-closed with deepen probes honest;
//! `production_wired=false` across arcs; GREEN claims blocked. Does **not** claim
//! full production wiring, master retick, or INV4 4/4 clearance.

use umst_manifold::runtime::gate::sec_bridge_arcs::{
    sec_bridge_arcs_accel_ac35_honest, sec_bridge_arcs_production_wired,
    validate_sec_bridge_arcs_gate_honesty, BRIDGE_ARCS_GREEN_CLAIM_BLOCKED,
};
use umst_manifold::runtime::gate::sec_s2::sec_s2_accel_ac29_honest;
use umst_manifold::runtime::gate::sec_s3::{sec_s3_accel_ac05_honest, S3_GREEN_CLAIM_BLOCKED};
use umst_manifold::runtime::gate::sec_s4::{sec_s4_accel_ac06_honest, L_S5_PROOF_WIRED_HONEST};
use umst_manifold::runtime::gate::sec_s5::sec_s5_accel_ac07_honest;
use umst_manifold::runtime::gate::sec_s7::sec_s7_accel_ac08_honest;
use umst_manifold::runtime::gate::{
    sec_gw_audit_accel2_ac31_honest, sec_gw_audit_production_wired, sec_gw_wrap_accel2_ac32_honest,
    sec_gw_wrap_production_wired, sec_mcp_wrap_accel_ac34_honest, sec_mcp_wrap_production_wired,
    sec_s1_accel_ac28_honest, sec_s1_production_wired, sec_s2_production_wired,
    sec_s3_production_wired, sec_s4_production_wired, sec_s5_production_wired,
    sec_s6_accel_ac33_honest, sec_s6_production_wired, sec_s7_production_wired,
    validate_sec_gw_audit_honesty, validate_sec_gw_wrap_honesty,
    validate_sec_mcp_wrap_gate_honesty, validate_sec_s1_gate_honesty, validate_sec_s2_gate_honesty,
    validate_sec_s3_gate_honesty, validate_sec_s4_gate_honesty, validate_sec_s5_gate_honesty,
    validate_sec_s6_gate_honesty, validate_sec_s7_gate_honesty, GW_AUDIT_GREEN_CLAIM_BLOCKED,
    GW_WRAP_GREEN_CLAIM_BLOCKED, MCP_WRAP_GREEN_CLAIM_BLOCKED, S1_GREEN_CLAIM_BLOCKED,
    S2_GREEN_CLAIM_BLOCKED, S5_GREEN_CLAIM_BLOCKED, S6_GREEN_CLAIM_BLOCKED, S7_GREEN_CLAIM_BLOCKED,
};

/// FLEET-COMPOSER ACCEL-G parent fleet id.
pub const FLEET_PARENT: &str = "FLEET-COMPOSER-ACCEL-G";

/// AC157 agent job id.
pub const FLEET_ACCEL2_AC157_JOB_ID: &str = "FLEET-COMPOSER-ACCEL2-AC157-MANIFOLD-GATE";

/// AC157 receipt path — SSOT for this pass.
pub const COMPOSER_AC157_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC157.md";

/// Fleet verify command (scratch target dir).
pub const AC157_VERIFY_COMMAND: &str =
    "CARGO_TARGET_DIR=/tmp/umst-accel2-ac157-mgate cargo test -p umst-manifold gate_deepen";

/// Number of SEC gate arcs covered by the deepen rollup.
pub const GATE_DEEPEN_ARC_COUNT: usize = 11;

/// AC157 manifold gate deepen probe — folds all SEC arc deepen honesty predicates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateDeepenHonestProbe {
    pub job_id: &'static str,
    pub receipt_path: &'static str,
    pub verify_command: &'static str,
    pub arc_count: usize,
    pub s1_accel_ac28_honest: bool,
    pub s2_accel_ac29_honest: bool,
    pub s3_accel_ac05_honest: bool,
    pub s4_accel_ac06_honest: bool,
    pub s5_accel_ac07_honest: bool,
    pub s6_accel_ac33_honest: bool,
    pub s7_accel_ac08_honest: bool,
    pub mcp_wrap_accel_ac34_honest: bool,
    pub gw_wrap_accel_ac32_honest: bool,
    pub gw_audit_accel_ac31_honest: bool,
    pub bridge_arcs_accel_ac35_honest: bool,
    pub all_validate_honest: bool,
    pub production_wired: bool,
    pub green_claim_blocked: bool,
}

/// Build AC157 gate deepen probe from live SEC arc measurements.
#[must_use]
pub fn gate_deepen_probe() -> GateDeepenHonestProbe {
    let all_validate_honest = validate_sec_s1_gate_honesty().is_ok()
        && validate_sec_s2_gate_honesty().is_ok()
        && validate_sec_s3_gate_honesty().is_ok()
        && validate_sec_s4_gate_honesty().is_ok()
        && validate_sec_s5_gate_honesty().is_ok()
        && validate_sec_s6_gate_honesty().is_ok()
        && validate_sec_s7_gate_honesty().is_ok()
        && validate_sec_mcp_wrap_gate_honesty().is_ok()
        && validate_sec_gw_wrap_honesty().is_ok()
        && validate_sec_gw_audit_honesty().is_ok()
        && validate_sec_bridge_arcs_gate_honesty().is_ok();

    GateDeepenHonestProbe {
        job_id: FLEET_ACCEL2_AC157_JOB_ID,
        receipt_path: COMPOSER_AC157_RECEIPT_PATH,
        verify_command: AC157_VERIFY_COMMAND,
        arc_count: GATE_DEEPEN_ARC_COUNT,
        s1_accel_ac28_honest: sec_s1_accel_ac28_honest(),
        s2_accel_ac29_honest: sec_s2_accel_ac29_honest(),
        s3_accel_ac05_honest: sec_s3_accel_ac05_honest(),
        s4_accel_ac06_honest: sec_s4_accel_ac06_honest(),
        s5_accel_ac07_honest: sec_s5_accel_ac07_honest(),
        s6_accel_ac33_honest: sec_s6_accel_ac33_honest(),
        s7_accel_ac08_honest: sec_s7_accel_ac08_honest(),
        mcp_wrap_accel_ac34_honest: sec_mcp_wrap_accel_ac34_honest(),
        gw_wrap_accel_ac32_honest: sec_gw_wrap_accel2_ac32_honest(),
        gw_audit_accel_ac31_honest: sec_gw_audit_accel2_ac31_honest(),
        bridge_arcs_accel_ac35_honest: sec_bridge_arcs_accel_ac35_honest(),
        all_validate_honest,
        production_wired: false,
        green_claim_blocked: true,
    }
}

/// AC157 honesty gate — must not invent production wired or unblocked GREEN claims.
#[must_use]
pub fn gate_deepen_honest(probe: &GateDeepenHonestProbe) -> bool {
    probe.job_id == FLEET_ACCEL2_AC157_JOB_ID
        && probe.receipt_path.contains("COMPOSER_ACCEL2_AC157")
        && probe.verify_command.contains("umst-accel2-ac157-mgate")
        && probe.arc_count == GATE_DEEPEN_ARC_COUNT
        && probe.s1_accel_ac28_honest
        && probe.s2_accel_ac29_honest
        && probe.s3_accel_ac05_honest
        && probe.s4_accel_ac06_honest
        && probe.s5_accel_ac07_honest
        && probe.s6_accel_ac33_honest
        && probe.s7_accel_ac08_honest
        && probe.mcp_wrap_accel_ac34_honest
        && probe.gw_wrap_accel_ac32_honest
        && probe.gw_audit_accel_ac31_honest
        && probe.bridge_arcs_accel_ac35_honest
        && probe.all_validate_honest
        && !probe.production_wired
        && probe.green_claim_blocked
}

#[test]
fn ac157_metadata_wired() {
    assert_eq!(FLEET_PARENT, "FLEET-COMPOSER-ACCEL-G");
    assert_eq!(
        FLEET_ACCEL2_AC157_JOB_ID,
        "FLEET-COMPOSER-ACCEL2-AC157-MANIFOLD-GATE"
    );
    assert!(COMPOSER_AC157_RECEIPT_PATH.contains("COMPOSER_ACCEL2_AC157"));
    assert!(AC157_VERIFY_COMMAND.contains("gate_deepen"));
    assert_eq!(GATE_DEEPEN_ARC_COUNT, 11);
}

#[test]
fn ac157_all_sec_validate_honesty_pass() {
    validate_sec_s1_gate_honesty().expect("SEC-S1");
    validate_sec_s2_gate_honesty().expect("SEC-S2");
    validate_sec_s3_gate_honesty().expect("SEC-S3");
    validate_sec_s4_gate_honesty().expect("SEC-S4");
    validate_sec_s5_gate_honesty().expect("SEC-S5");
    validate_sec_s6_gate_honesty().expect("SEC-S6");
    validate_sec_s7_gate_honesty().expect("SEC-S7");
    validate_sec_mcp_wrap_gate_honesty().expect("SEC-MCP-WRAP");
    validate_sec_gw_wrap_honesty().expect("SEC-GW-WRAP");
    validate_sec_gw_audit_honesty().expect("SEC-GW-AUDIT");
    validate_sec_bridge_arcs_gate_honesty().expect("SEC-BRIDGE-ARCS");
}

#[test]
fn ac157_all_accel_deepen_probes_honest() {
    assert!(sec_s1_accel_ac28_honest());
    assert!(sec_s2_accel_ac29_honest());
    assert!(sec_s3_accel_ac05_honest());
    assert!(sec_s4_accel_ac06_honest());
    assert!(sec_s5_accel_ac07_honest());
    assert!(sec_s6_accel_ac33_honest());
    assert!(sec_s7_accel_ac08_honest());
    assert!(sec_mcp_wrap_accel_ac34_honest());
    assert!(sec_gw_wrap_accel2_ac32_honest());
    assert!(sec_gw_audit_accel2_ac31_honest());
    assert!(sec_bridge_arcs_accel_ac35_honest());
}

#[test]
fn ac157_production_wired_stays_false_all_arcs() {
    assert!(!sec_s1_production_wired());
    assert!(!sec_s2_production_wired());
    assert!(!sec_s3_production_wired());
    assert!(!sec_s4_production_wired());
    assert!(!sec_s5_production_wired());
    assert!(!sec_s6_production_wired());
    assert!(!sec_s7_production_wired());
    assert!(!sec_mcp_wrap_production_wired());
    assert!(!sec_gw_wrap_production_wired());
    assert!(!sec_gw_audit_production_wired());
    assert!(!sec_bridge_arcs_production_wired());
}

#[test]
fn ac157_green_claim_blocked_all_arcs() {
    assert!(S1_GREEN_CLAIM_BLOCKED);
    assert!(S2_GREEN_CLAIM_BLOCKED);
    assert!(S3_GREEN_CLAIM_BLOCKED);
    assert!(!L_S5_PROOF_WIRED_HONEST);
    assert!(S5_GREEN_CLAIM_BLOCKED);
    assert!(S6_GREEN_CLAIM_BLOCKED);
    assert!(S7_GREEN_CLAIM_BLOCKED);
    assert!(MCP_WRAP_GREEN_CLAIM_BLOCKED);
    assert!(GW_WRAP_GREEN_CLAIM_BLOCKED);
    assert!(GW_AUDIT_GREEN_CLAIM_BLOCKED);
    assert!(BRIDGE_ARCS_GREEN_CLAIM_BLOCKED);
}

#[test]
fn ac157_gate_deepen_probe_rollup_honest() {
    let probe = gate_deepen_probe();
    assert_eq!(probe.arc_count, 11);
    assert!(probe.all_validate_honest);
    assert!(!probe.production_wired);
    assert!(probe.green_claim_blocked);
    assert!(gate_deepen_honest(&probe));
}

#[test]
fn fleet_composer_accel2_ac157_manifold_gate_deepen_honest() {
    let probe = gate_deepen_probe();
    assert!(gate_deepen_honest(&probe));
}
