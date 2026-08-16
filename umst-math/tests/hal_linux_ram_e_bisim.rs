// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! H-9 LinuxRam: MemTotal + honest DRAM RAPL posture (no STREAM in this slice)
#![cfg(target_os = "linux")]

use std::path::PathBuf;

use umst_math::hal::backends::linux::LinuxRam;
use umst_math::hal::traits::{
    AllocationSpec, HardwareUnit, InferenceBatch, WorkloadClass, SMOKE_INFER_OP,
};

const RAPL: &str = "/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj";

fn rapl_usable() -> bool {
    PathBuf::from(RAPL).is_file() && std::fs::read(RAPL).is_ok()
}

#[test]
fn aa_hal_linux_ram_enumerates_memtotal() {
    let r = LinuxRam::new();
    let s = r.enumerate_models();
    assert_eq!(s.len(), 1);
    assert!(s[0].contains("MemTotal_kb="));
    let kb: u64 = s[0]
        .split("MemTotal_kb=")
        .nth(1)
        .and_then(|t: &str| t.parse::<u64>().ok())
        .expect("memtotal parse");
    assert!(kb > 0, "host must report MemTotal on a normal Linux");
}

#[test]
fn aa_hal_linux_ram_rapl_posture_matches_m0_12() {
    let r = LinuxRam::new();
    let p = r.power_state().expect("p");
    if !rapl_usable() {
        let rs = p.reason.as_deref().unwrap_or("");
        let ok = p.headroom < 0.1 || rs.contains("rapl") || rs.contains("unread");
        assert!(ok, "expected pessimistic or explicit note, got {p:?}");
    }
}

#[test]
fn aa_hal_linux_ram_smoke_workload() {
    let r = LinuxRam::new();
    let id = r
        .allocate(&AllocationSpec {
            bytes: 1024,
            class: WorkloadClass::Inference as u32,
        })
        .expect("a");
    r.infer(
        id,
        &InferenceBatch {
            batch: 0,
            op: SMOKE_INFER_OP,
        },
    )
    .expect("i");
    r.deallocate(id).ok();
}

#[test]
fn aa_hal_linux_ram_drift_window_positive() {
    assert!(LinuxRam::new().drift_window() > 0);
}

#[test]
fn aa_hal_linux_ram_e_bisim_twin() {
    let a = {
        let r = LinuxRam::new();
        r.enumerate_models()
    };
    let b = {
        let r = LinuxRam::new();
        r.enumerate_models()
    };
    assert_eq!(a, b);
}
