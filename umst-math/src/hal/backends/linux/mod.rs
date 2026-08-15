//! Linux/Intel H-9 assembly

use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

use super::super::inventory::HardwareInventory;
use super::super::kinds::UnitKind;
use super::super::presence::UnitPresence;
use super::super::probe_snapshot::HalProbeSnapshot;
use super::super::profile::{ArchClass, ArchitectureProfile, ParetoReferenceLabel};
use super::super::traits::HardwareUnit;
use super::super::traits::WorkloadClass;

mod cpu;
mod igpu;
mod npu;
mod permissions;
mod port;
mod ram;
mod rapl;
mod smoke;
mod sysfs;

// RED §0.8 / H-9 tests construct backends directly; keep surface narrow (constructors + probes only).
pub use self::cpu::IntelCpu;
pub use self::igpu::IntelIgpu;
pub use self::npu::IntelNpu;
pub use self::port::LinuxPort;
pub use self::ram::LinuxRam;

/// Pure inventory assembly from an injected probe snapshot (FP §4).
pub fn build_linux_inventory_from_snapshot(
    snapshot: &HalProbeSnapshot,
) -> (HardwareInventory, Vec<String>, usize) {
    let mut warn = Vec::new();
    let cpu = IntelCpu::from_snapshot(snapshot);
    if !cpu.power_readable {
        warn.push(format!(
            "H-9: RAPL not readable for current user: {}; energy numbers are not from RAPL (NED §0.5).",
            cpu.rapl_path_display()
        ));
    }
    let i = IntelIgpu::from_snapshot(snapshot);
    if i.is_present() && !i.debug_readable() {
        warn.push(format!(
            "H-9: i915 debug not readable: {}; iGPU power view limited.",
            i.i915_path_str()
        ));
    }
    if snapshot.thunderbolt_enum_denied {
        warn.push(
            "H-9: thunderbolt enumeration denied: /sys/bus/thunderbolt/devices (permission denied)"
                .to_string(),
        );
    }
    let n = IntelNpu::from_snapshot(snapshot);
    let r = LinuxRam::from_snapshot(snapshot);
    let p = LinuxPort::from_snapshot(snapshot);
    let port_count = p.total_ports();
    let profile = ArchitectureProfile {
        arch_class: ArchClass::LinuxIntelP14sGen5,
        canonical_fallback_chains: vec![(
            WorkloadClass::Inference,
            vec![UnitKind::Cpu, UnitKind::Igpu, UnitKind::Npu],
        )],
        architectural_constraints: "H-9 Linux/Intel; dgpu=AbsByCfg; ane=AbsByArch".to_string(),
        pareto_reference: Some(ParetoReferenceLabel {
            reg_token: "B2_H9_LINUX".to_string(),
        }),
    };
    let inv = HardwareInventory {
        profile,
        cpu: UnitPresence::Present(Arc::new(cpu) as Arc<dyn HardwareUnit + Send + Sync + 'static>),
        igpu: if i.is_present() {
            UnitPresence::Present(Arc::new(i) as Arc<dyn HardwareUnit + Send + Sync + 'static>)
        } else {
            UnitPresence::AbsentByArch
        },
        dgpu: UnitPresence::AbsentByConfig,
        npu: if n.is_present() {
            UnitPresence::Present(Arc::new(n) as Arc<dyn HardwareUnit + Send + Sync + 'static>)
        } else {
            UnitPresence::AbsentByConfig
        },
        ane: UnitPresence::AbsentByArch,
        ram: UnitPresence::Present(Arc::new(r) as Arc<dyn HardwareUnit + Send + Sync + 'static>),
        port: UnitPresence::Present(Arc::new(p) as Arc<dyn HardwareUnit + Send + Sync + 'static>),
    };
    (inv, warn, port_count)
}

/// Probe live host sysfs and assemble inventory (`linux-hal-sysfs` feature).
#[cfg(feature = "linux-hal-sysfs")]
pub fn build_linux_inventory() -> (HardwareInventory, Vec<String>, usize) {
    build_linux_inventory_from_snapshot(&crate::hal::probe_host::probe_sysfs_snapshot())
}

pub use permissions::PermissionState;
pub use sysfs::default_rapl_package_energy;

#[cfg(feature = "linux-hal-sysfs")]
pub use crate::hal::probe_host::read_probe;

#[cfg(feature = "linux-hal-sysfs")]
pub use crate::hal::probe_host::read_line;
