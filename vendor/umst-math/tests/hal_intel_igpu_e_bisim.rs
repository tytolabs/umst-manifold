//! H-9 IntelIgpu (Linux; i915 / DRM) — no hard-coded Present: respect host probes.
#![cfg(target_os = "linux")]

use umst_math::hal::backends::linux::IntelIgpu;
use umst_math::hal::build_linux_inventory;
use umst_math::hal::presence::UnitPresence;
use umst_math::hal::traits::{
    AllocationSpec, HalError, HardwareUnit, InferenceBatch, WorkloadClass, SMOKE_INFER_OP,
};

fn two_igpu_snapshots() -> (Vec<String>, bool) {
    let a = IntelIgpu::new();
    (a.enumerate_models(), a.is_present())
}

#[test]
fn aa_hal_intel_igpu_enumeration_matches_presence() {
    let g = IntelIgpu::new();
    if g.is_present() {
        let m = g.enumerate_models();
        assert!(
            !m.is_empty()
                && m.iter()
                    .any(|s: &String| s.contains("igpu") || s.contains("intel-igpu"))
        );
    } else {
        assert!(g.enumerate_models().is_empty());
    }
}

#[test]
fn aa_hal_intel_igpu_supported_precisions_when_present() {
    let g = IntelIgpu::new();
    let p = g.supported_precisions();
    if g.is_present() {
        assert_eq!(p.len(), 3, "{p:?}");
    } else {
        // Still advertises the lane surface; allocate returns NotApplicable
        assert!(!p.is_empty());
    }
}

#[test]
fn aa_hal_intel_igpu_smoke_roundtrip_when_present() {
    let g = IntelIgpu::new();
    if !g.is_present() {
        let err = g
            .allocate(&AllocationSpec {
                bytes: 8,
                class: WorkloadClass::Inference as u32,
            })
            .err();
        assert_eq!(err, Some(HalError::NotApplicable));
        return;
    }
    let id = g
        .allocate(&AllocationSpec {
            bytes: 1024,
            class: WorkloadClass::Inference as u32,
        })
        .expect("a");
    g.infer(
        id,
        &InferenceBatch {
            batch: 0,
            op: SMOKE_INFER_OP,
        },
    )
    .expect("infer");
    g.deallocate(id).expect("d");
}

#[test]
fn aa_hal_intel_igpu_drift_and_power_consistent() {
    let g = IntelIgpu::new();
    assert!(g.drift_window() > 0);
    let ps = g.power_state();
    if g.is_present() {
        let p = ps.expect("ps");
        if !g.debug_readable() {
            // Honest: no fabricated GPU power without debugfs
            assert!(p.reason.is_some() || p.headroom < 0.5);
        }
    } else {
        assert_eq!(ps.err(), Some(HalError::NotApplicable));
    }
}

#[test]
fn aa_hal_intel_igpu_e_bisim_two_runs() {
    let a = two_igpu_snapshots();
    let b = two_igpu_snapshots();
    assert_eq!(a.0, b.0);
    assert_eq!(a.1, b.1);
}

#[test]
fn aa_hal_intel_igpu_inventory_slot_matches_probe() {
    let g = IntelIgpu::new();
    let (inv, _, _) = build_linux_inventory();
    if g.is_present() {
        assert!(matches!(&inv.igpu, UnitPresence::Present(_)));
    } else {
        assert!(matches!(&inv.igpu, UnitPresence::AbsentByArch));
    }
}
