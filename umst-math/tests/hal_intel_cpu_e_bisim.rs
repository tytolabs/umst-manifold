// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! H-9 IntelCpu ε-bisim (Linux/Intel; `aa_hal_` for `cargo test hal_` substring filter)
#![cfg(target_os = "linux")]

use std::path::PathBuf;

use umst_math::hal::backends::linux::{IntelCpu, PermissionState};
use umst_math::hal::traits::{
    AllocationSpec, HardwareUnit, InferenceBatch, PowerStateKind, WorkloadClass, SMOKE_INFER_OP,
};

fn m0_12_rapl_path_readable() -> bool {
    let p = PathBuf::from("/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj");
    p.is_file() && std::fs::read(p).is_ok()
}

fn probe_snapshot() -> (Vec<String>, PermissionState, bool) {
    let a = IntelCpu::new();
    (
        a.enumerate_models().clone(),
        a.permission.clone(),
        a.power_readable,
    )
}

#[test]
fn aa_hal_intel_cpu_enumerates_models_total() {
    let c = IntelCpu::new();
    let m = c.enumerate_models();
    assert!(!m.is_empty());
    assert!(m.iter().any(|s: &String| s.contains("intel-cpu")));
    assert!(m
        .iter()
        .any(|s: &String| s.contains("reg#cores=") && s.contains("l3_kb=")));
}

#[test]
fn aa_hal_intel_cpu_supported_precisions_nonempty() {
    let c = IntelCpu::new();
    let p = c.supported_precisions();
    assert_eq!(p.len(), 4, "{p:?}");
}

#[test]
fn aa_hal_intel_cpu_smoke_workload_round_trips() {
    let c = IntelCpu::new();
    let id = c
        .allocate(&AllocationSpec {
            bytes: 1024,
            class: WorkloadClass::Inference as u32,
        })
        .expect("alloc");
    let b = InferenceBatch {
        batch: 0,
        op: SMOKE_INFER_OP,
    };
    c.infer(id, &b).expect("smoke infer");
    c.deallocate(id).expect("dealloc");
}

#[test]
fn aa_hal_intel_cpu_power_state_honors_permission_state() {
    let _m0_12_rapl_readable = m0_12_rapl_path_readable();
    let c = IntelCpu::new();
    let p = c.power_state().expect("ps");
    // NED: denied permission must not be reported as a confident P0 with a fake RAPL value
    match &c.permission {
        PermissionState::Denied { .. } => {
            assert_eq!(p.kind, PowerStateKind::P3, "{p:?}");
            let r = p.reason.as_deref().unwrap_or("");
            assert!(r.contains("permission_denied"), "reason: {p:?}");
        }
        _ if c.power_readable => {
            let _ = p;
            // Headroom is allowed when RAPL is readable; see implementation
        }
        _ => {
            assert!(!c.power_readable);
            // honest pessimistic: P2 when path exists but not read as RAPL
            if matches!(
                &c.permission,
                PermissionState::NotApplicable | PermissionState::Untested
            ) && !c.power_readable
            {
                assert!(
                    !matches!(p.kind, PowerStateKind::P0),
                    "{p:?} perm={:?}",
                    c.permission
                );
            }
        }
    }
}

#[test]
fn aa_hal_intel_cpu_drift_window_is_positive() {
    assert!(IntelCpu::new().drift_window() > 0);
}

#[test]
fn aa_hal_intel_cpu_e_bisim_two_runs_byte_equal_modulo_timestamps() {
    let (m1, perm1, r1) = probe_snapshot();
    let (m2, perm2, r2) = probe_snapshot();
    assert_eq!(m1, m2, "model strings");
    assert_eq!(perm1, perm2, "PermissionState");
    assert_eq!(r1, r2, "power_readable");
}
