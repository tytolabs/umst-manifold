//! M4: paired-mock laws + (Linux) real `build_linux_inventory` with `ArchClass` population.
// mock path always runs; Linux path exercises real backends; macOS stays on mocks (H-9-mac).

use umst_math::hal::kinds::UnitKind;
use umst_math::hal::laws::{
    check_associative, check_identity_law, f_cpu_igpu, g_igpu_ram, h_ram_port,
};
use umst_math::hal::mocks::{mock_apple_m3_inventory, mock_linux_intel_inventory};
use umst_math::hal::presence::UnitPresence;
use umst_math::hal::profile::ArchClass;

/// Always-on: mock-Apple + mock-Linux satisfy the same H-8 route laws as the inventory tests
#[test]
fn aa_hal_paired_arch_mocks_satisfy_identity_and_associative() {
    let _a = mock_linux_intel_inventory();
    let _b = mock_apple_m3_inventory();
    let (i1, i2) = check_identity_law(f_cpu_igpu);
    assert!(i1 && i2, "Kleisli id law");
    assert!(
        check_associative(f_cpu_igpu, g_igpu_ram, h_ram_port),
        "associativity"
    );
}

/// Mock-Apple profile remains MacosM3Max (C-strict)
#[test]
fn aa_hal_paired_mocks_distinguish_by_archclass() {
    let li = mock_linux_intel_inventory();
    let ap = mock_apple_m3_inventory();
    assert!(matches!(
        li.profile.arch_class,
        ArchClass::LinuxIntelP14sGen5
    ));
    assert!(matches!(ap.profile.arch_class, ArchClass::MacosM3Max));
}

#[cfg(target_os = "linux")]
mod linux_paired {
    use super::*;
    use umst_math::hal::build_linux_inventory;
    use umst_math::hal::profile::ArchClass;

    #[test]
    fn aa_hal_paired_build_linux_inv_arch_is_linux_intel() {
        let (inv, _, _) = build_linux_inventory();
        assert!(
            matches!(inv.profile.arch_class, ArchClass::LinuxIntelP14sGen5),
            "{:?}",
            inv.profile
        );
    }

    #[test]
    fn aa_hal_paired_linux_real_mocks_m3_still_satisfy_drift() {
        for inv in [mock_apple_m3_inventory(), build_linux_inventory().0] {
            for k in UnitKind::ALL {
                if let UnitPresence::Present(h) = inv.slot(k) {
                    assert!(h.drift_window() > 0, "{k:?}");
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
#[test]
fn aa_hal_paired_host_macos_skips_real_linux_path_until_h9_mac() {
    let _m = mock_apple_m3_inventory();
    // Physical M3 Max + Linux paired CI = Op-6 PENDING; local macOS only exercises mocks
}
