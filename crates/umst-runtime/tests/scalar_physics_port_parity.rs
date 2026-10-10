// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MPL-2.0
//! Scalar physics port parity — external legacy FFI oracle + absent-boundary guards (no self-copy).

use std::fs;
use std::path::PathBuf;

use umst_runtime::scalar_physics_port::{
    c_state_from_mix, hydration_degree, legacy_ungrounded_constant_count, strength_powers,
    thermo_snapshot_from_mix, ScalarPhysicsPortRefuse, CEMENT_SCALAR_PHYSICS_HOME,
    LEGACY_SCALAR_PHYSICS_ORACLE, LEGACY_UNGROUNDED_CONSTANTS, SCALAR_PHYSICS_PORT_MARKER,
};

fn workspace_root_from_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

fn legacy_oracle_source() -> String {
    let path = workspace_root_from_manifest().join(LEGACY_SCALAR_PHYSICS_ORACLE);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("legacy oracle missing at {}: {e}", path.display()))
}

/// Distinctive legacy hydration body line — must live only in the FFI oracle, not the port.
const LEGACY_HYDRATION_SIGNATURE: &str = "let alpha_max = 0.95 - scm_ratio * 0.15";

#[test]
fn port_source_is_not_verbatim_legacy_hydration() {
    let port_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/scalar_physics_port.rs");
    let port_src = fs::read_to_string(&port_path).expect("read port source");
    assert!(
        !port_src.contains(LEGACY_HYDRATION_SIGNATURE),
        "port must not duplicate legacy hydration body from oracle"
    );
    let oracle = legacy_oracle_source();
    assert!(
        oracle.contains(LEGACY_HYDRATION_SIGNATURE),
        "oracle file must still carry hydration witness"
    );
}

#[test]
fn parity_test_has_no_inline_legacy_duplicate_fns() {
    let test_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/scalar_physics_port_parity.rs");
    let test_src = fs::read_to_string(&test_path).expect("read test source");
    for stem in [
        "hydration_degree",
        "strength_powers",
        "thermo_snapshot_from_mix",
    ] {
        let legacy_fn = ["fn ", "legacy_", stem].concat();
        assert!(
            !test_src.contains(&legacy_fn),
            "self-comparison trap: test must not define {legacy_fn}"
        );
    }
    let legacy_closure = ["struct ", "Legacy", "Cement", "Closure"].concat();
    assert!(
        !test_src.contains(&legacy_closure),
        "self-comparison trap: test must not define LegacyCementClosure"
    );
}

#[test]
fn legacy_oracle_exports_match_port_surface_names() {
    let oracle = legacy_oracle_source();
    for sym in [
        "pub fn hydration_degree",
        "pub fn strength_powers",
        "pub fn thermo_snapshot_from_mix",
        "pub fn c_state_from_mix",
    ] {
        assert!(oracle.contains(sym), "oracle missing {sym}");
    }
    assert!(oracle.contains("CEMENT_DEFAULT_S_INTRINSIC_MPA"));
}

#[test]
fn executor_port_refuses_material_physics_until_cartridge_wire() {
    assert_eq!(
        SCALAR_PHYSICS_PORT_MARKER,
        "scalar_physics_port_v2_absent_boundary"
    );
    assert!(legacy_ungrounded_constant_count() >= 7);
    assert!(CEMENT_SCALAR_PHYSICS_HOME.contains("umst-cartridge-concrete"));

    assert!(matches!(
        hydration_degree(7.0, 20.0, 0.0),
        Err(ScalarPhysicsPortRefuse::MaterialPhysicsHomedInCartridges { .. })
    ));
    assert!(matches!(
        strength_powers(0.45, 0.4, 0.02, 234.0),
        Err(ScalarPhysicsPortRefuse::MaterialPhysicsHomedInCartridges { .. })
    ));
    assert!(matches!(
        thermo_snapshot_from_mix(0.45, 0.5, 293.0),
        Err(ScalarPhysicsPortRefuse::UngroundedConstant(_))
    ));
    assert!(matches!(
        c_state_from_mix(0.45, 0.5, 293.0),
        Err(ScalarPhysicsPortRefuse::MaterialPhysicsHomedInCartridges { .. })
    ));
}

#[test]
fn ungrounded_legacy_constants_are_typed_absent_rows() {
    assert!(!LEGACY_UNGROUNDED_CONSTANTS.is_empty());
    for row in LEGACY_UNGROUNDED_CONSTANTS {
        assert!(!row.quantity.is_empty());
        assert!(!row.unit.is_empty());
        assert!(
            !row.follow_up_cell.is_empty(),
            "follow-up cell id required for Absent row"
        );
    }
}
