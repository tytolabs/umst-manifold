// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! PRABHU-WAVE-E-1700 · Slot E2 — `umst-runtime` alias surfaces PBM-010 F1 deepen rollup.

use umst_runtime::runtime::atoms_f1_deepen::{
    atoms_f1_p1700_e2_deepen_honest, atoms_f1_p1700_e2_deepen_probe, pbm010_f1_fully_closed,
    pbm010_production_wired, BLOCKING_ROW_COUNT, FLEET_ID, JOB_ID, OPEN_ROW_COUNT, PBM_ID,
    SLICE_RESIDUAL_ROW_COUNT, WAVE_SLOT,
};

#[test]
fn fleet_composer_p1700_e2_atoms_f1_deepen() {
    let probe = atoms_f1_p1700_e2_deepen_probe();
    assert_eq!(FLEET_ID, "PRABHU-WAVE-E-1700");
    assert_eq!(WAVE_SLOT, "E2");
    assert_eq!(probe.job_id, JOB_ID);
    assert_eq!(PBM_ID, "PBM-010");
    assert_eq!(probe.slice_residual_row_count, SLICE_RESIDUAL_ROW_COUNT);
    assert_eq!(probe.blocking_row_count, BLOCKING_ROW_COUNT);
    assert_eq!(probe.open_row_count, OPEN_ROW_COUNT);
    assert!(atoms_f1_p1700_e2_deepen_honest(&probe));
    assert!(!pbm010_f1_fully_closed());
    assert!(!pbm010_production_wired());
}
