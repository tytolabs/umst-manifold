// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! H-9 `PermissionState` ADT totality (Linux sysfs probes)
#![cfg(target_os = "linux")]

use std::path::Path;

use umst_math::hal::backends::linux::{default_rapl_package_energy, read_probe, PermissionState};

const RAPL_LABEL: &str = "/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj";

#[test]
fn aa_hal_permission_state_nonexistent_is_notapplicable() {
    let s = read_probe(Path::new("/__nonexistent_umst_h9__"), "n/a");
    assert_eq!(s, PermissionState::NotApplicable);
}

#[test]
fn aa_hal_permission_state_rapl_m0_12_liveness() {
    let path = default_rapl_package_energy();
    let s = read_probe(&path, RAPL_LABEL);
    assert!(
        matches!(
            s,
            PermissionState::Granted | PermissionState::Denied { .. } | PermissionState::Untested
        ),
        "{s:?} for {path:?}"
    );
    if !path.exists() {
        assert!(
            matches!(
                s,
                PermissionState::NotApplicable | PermissionState::Untested
            ),
            "{s:?}"
        );
    }
}

#[test]
fn aa_hal_permission_state_untested_vs_not_applicable() {
    assert_ne!(PermissionState::Untested, PermissionState::NotApplicable);
}

#[test]
fn aa_hal_permission_state_denied_eacces_round_trip() {
    let d = PermissionState::Denied {
        sysfs_path: "/test/path",
        errno: 13,
    };
    match d {
        PermissionState::Denied { sysfs_path, errno } => {
            assert_eq!(errno, 13);
            assert_eq!(sysfs_path, "/test/path");
        }
        _ => panic!(),
    }
}
