//! Linux/Intel H-9 assembly

use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

use super::super::inventory::HardwareInventory;
use super::super::kinds::UnitKind;
use super::super::presence::UnitPresence;
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

/// Assemble the seven-lane inventory + P14s profile; also returns NED permission strings for `egoff` `tracing::warn!`.
pub fn build_linux_inventory() -> (HardwareInventory, Vec<String>, usize) {
    let mut warn = Vec::new();
    let cpu = IntelCpu::new();
    if !cpu.power_readable {
        warn.push(format!(
            "H-9: RAPL not readable for current user: {}; energy numbers are not from RAPL (NED §0.5).",
            cpu.rapl_path_display()
        ));
    }
    let i = IntelIgpu::new();
    if i.is_present() && !i.debug_readable() {
        warn.push(format!(
            "H-9: i915 debug not readable: {}; iGPU power view limited.",
            i.i915_path_str()
        ));
    }
    let tbt = std::path::Path::new("/sys/bus/thunderbolt/devices");
    if tbt.exists() {
        if let Err(e) = std::fs::read_dir(tbt) {
            if e.raw_os_error() == Some(13) || e.kind() == std::io::ErrorKind::PermissionDenied {
                warn.push(format!(
                    "H-9: thunderbolt enumeration denied: /sys/bus/thunderbolt/devices ({e})"
                ));
            }
        }
    }
    let n = IntelNpu::new();
    let r = LinuxRam::new();
    let p = LinuxPort::new();
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

pub use permissions::read_probe;
pub use permissions::PermissionState;
pub use sysfs::default_rapl_package_energy;
pub use sysfs::read_line;
