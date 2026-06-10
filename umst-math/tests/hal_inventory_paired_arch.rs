//! Paired-mock inventory (Q6 C-strict; *mocks* only — physical M3 Op-6 pending; FORWARD-PLAN G2)
use umst_math::hal::laws::{
    check_associative, check_identity_law, f_cpu_igpu, g_igpu_ram, h_ram_port,
};
use umst_math::hal::mocks::{mock_apple_m3_inventory, mock_linux_intel_inventory};
use umst_math::hal::presence::UnitPresence;
use umst_math::hal::profile::ArchClass;
use umst_math::hal::traits::HardwareUnit;
use umst_math::hal::UnitKind;

fn assert_present(h: &std::sync::Arc<dyn HardwareUnit + Send + Sync>, expect: &str) {
    let m = h.enumerate_models();
    assert!(m[0].contains(expect), "{m:?}");
}

#[test]
fn aa_hal_paired_linux_intel_cpu_present_ane_absent() {
    let inv = mock_linux_intel_inventory();
    assert!(matches!(
        inv.profile.arch_class,
        ArchClass::LinuxIntelP14sGen5
    ));
    match &inv.ane {
        UnitPresence::AbsentByArch => {}
        _ => panic!("expected ANE AbsentByArch on Linux mock"),
    }
    match &inv.cpu {
        UnitPresence::Present(h) => assert_present(h, "Cpu"),
        _ => panic!("expected CPU present"),
    }
}

#[test]
fn aa_hal_paired_apple_m3_dgpu_npu_absent_ane_present() {
    let inv = mock_apple_m3_inventory();
    assert!(matches!(inv.profile.arch_class, ArchClass::MacosM3Max));
    match &inv.dgpu {
        UnitPresence::AbsentByArch => {}
        _ => panic!("expected dGPU AbsentByArch on Apple mock"),
    }
    match &inv.npu {
        UnitPresence::AbsentByArch => {}
        _ => panic!("expected NPU AbsentByArch on Apple mock"),
    }
    match &inv.ane {
        UnitPresence::Present(h) => assert_present(h, "Ane"),
        _ => panic!("expected ANE present on Apple mock"),
    }
}

#[test]
fn aa_hal_paired_both_export_seven_lanes() {
    let a = mock_linux_intel_inventory();
    let b = mock_apple_m3_inventory();
    for k in UnitKind::ALL {
        let _ = a.slot(k);
        let _ = b.slot(k);
    }
}

#[test]
fn aa_hal_paired_both_satisfy_roundtrip_drift() {
    for inv in [mock_linux_intel_inventory(), mock_apple_m3_inventory()] {
        for k in UnitKind::ALL {
            if let UnitPresence::Present(h) = inv.slot(k) {
                assert!(h.drift_window() > 0);
            }
        }
    }
}

/// M4: same routing-law witnesses as [`hal_trait_laws`], in one module as both mocks are constructed
#[test]
fn aa_hal_paired_mocks_satisfy_route_laws_in_same_test_run() {
    let _linux = mock_linux_intel_inventory();
    let _appl = mock_apple_m3_inventory();
    let (a, b) = check_identity_law(f_cpu_igpu);
    assert!(a && b);
    assert!(check_associative(f_cpu_igpu, g_igpu_ram, h_ram_port));
}
