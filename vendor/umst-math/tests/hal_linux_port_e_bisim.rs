//! H-9 LinuxPort: net + USB rough enumeration
#![cfg(target_os = "linux")]

use umst_math::hal::backends::linux::build_linux_inventory;
use umst_math::hal::backends::linux::LinuxPort;
use umst_math::hal::presence::UnitPresence;
use umst_math::hal::traits::{
    AllocationSpec, HardwareUnit, InferenceBatch, WorkloadClass, SMOKE_INFER_OP,
};

#[test]
fn aa_hal_linux_port_enumeration_non_panicking() {
    let p = LinuxPort::new();
    let _ = p.enumerate_models();
    assert!(p.drift_window() > 0);
}

#[test]
fn aa_hal_linux_port_smoke_workload() {
    let p = LinuxPort::new();
    let id = p
        .allocate(&AllocationSpec {
            bytes: 1024,
            class: WorkloadClass::Inference as u32,
        })
        .expect("a");
    p.infer(
        id,
        &InferenceBatch {
            batch: 0,
            op: SMOKE_INFER_OP,
        },
    )
    .expect("i");
    p.deallocate(id).ok();
}

#[test]
fn aa_hal_linux_port_inventory_slot_present() {
    let (inv, _, _) = build_linux_inventory();
    assert!(
        matches!(&inv.port, UnitPresence::Present(_)),
        "port cluster lane is always exposed on Linux H-9"
    );
}

#[test]
fn aa_hal_linux_port_e_bisim_twin() {
    let a = {
        let p = LinuxPort::new();
        p.enumerate_models()
    };
    let b = {
        let p = LinuxPort::new();
        p.enumerate_models()
    };
    assert_eq!(a, b);
}

#[test]
fn aa_hal_linux_port_count_nonnegative() {
    let p = LinuxPort::new();
    // Public via total from backend: we only have trait; re-probe by two constructs same models
    let m = p.enumerate_models();
    if m.len() == 1 && m[0].contains("port#empty") {
        // Honest empty host
        return;
    }
    assert!(m[0].contains("linux-port#") || m[0].contains("port"));
}
