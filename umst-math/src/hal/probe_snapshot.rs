//! Pure host probe snapshot — injected at the runtime/CLI IO boundary (FP §4).
//!
//! [`HalProbeSnapshot`] carries sysfs/proc readings as data; HAL backends assemble
//! [`super::inventory::HardwareInventory`] without `std::fs` when the snapshot is
//! supplied externally.

#![cfg(target_os = "linux")]

use std::path::PathBuf;

use super::permission_state::PermissionState;

const RAPL_PATH: &str = "/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj";
const I915_DEBUG_PATH: &str = "/sys/kernel/debug/dri/0/i915_gpu_info";

/// Linux H-9 host probe readings (pure data; no I/O).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HalProbeSnapshot {
    /// CPU lane probes.
    pub cpu: CpuProbe,
    /// iGPU lane probes.
    pub igpu: IgpuProbe,
    /// NPU lane probes.
    pub npu: NpuProbe,
    /// RAM lane probes.
    pub ram: RamProbe,
    /// Port enumeration probes.
    pub port: PortProbe,
    /// Thunderbolt bus exists but enumeration returned `EACCES`.
    pub thunderbolt_enum_denied: bool,
}

impl Default for HalProbeSnapshot {
    fn default() -> Self {
        Self {
            cpu: CpuProbe::default(),
            igpu: IgpuProbe::default(),
            npu: NpuProbe::default(),
            ram: RamProbe::default(),
            port: PortProbe::default(),
            thunderbolt_enum_denied: false,
        }
    }
}

/// CPU sysfs/proc probe fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CpuProbe {
    /// RAPL package energy path used for permission display.
    pub rapl_path: PathBuf,
    /// Read permission outcome for RAPL.
    pub rapl_permission: PermissionState,
    /// Package energy counter (µJ) when readable.
    pub rapl_uj: Option<u64>,
    /// Logical core count from `/proc/cpuinfo`.
    pub logical_cores: u32,
    /// Best-effort L3 cache size (KB).
    pub l3_cache_kb: u32,
    /// `model name` line from `/proc/cpuinfo`, if present.
    pub model_name: Option<String>,
}

impl Default for CpuProbe {
    fn default() -> Self {
        Self {
            rapl_path: PathBuf::from(RAPL_PATH),
            rapl_permission: PermissionState::Untested,
            rapl_uj: None,
            logical_cores: 1,
            l3_cache_kb: 0,
            model_name: None,
        }
    }
}

/// iGPU sysfs probe fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IgpuProbe {
    /// Intel DRM render/card node detected.
    pub has_intel_render: bool,
    /// i915 debugfs path (may be unreadable).
    pub i915_path: PathBuf,
    /// Whether the i915 debug node is readable.
    pub debug_readable: bool,
}

impl Default for IgpuProbe {
    fn default() -> Self {
        Self {
            has_intel_render: false,
            i915_path: PathBuf::from(I915_DEBUG_PATH),
            debug_readable: false,
        }
    }
}

/// NPU sysfs probe fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NpuProbe {
    /// `/sys/class/accel/accel0` exists.
    pub present: bool,
}

impl Default for NpuProbe {
    fn default() -> Self {
        Self {
            present: false,
        }
    }
}

/// RAM sysfs/proc probe fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RamProbe {
    /// `/proc/meminfo` `MemTotal` kB.
    pub total_kb: u64,
    /// RAPL package counter readable (DRAM policy uses package path).
    pub rapl_ok: bool,
    /// Package energy µJ when readable.
    pub rapl_uj: Option<u64>,
}

impl Default for RamProbe {
    fn default() -> Self {
        Self {
            total_kb: 0,
            rapl_ok: false,
            rapl_uj: None,
        }
    }
}

/// Port enumeration probe fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortProbe {
    /// Network interfaces excluding `lo`.
    pub net: usize,
    /// USB bus device directory entries.
    pub usb: usize,
}

impl Default for PortProbe {
    fn default() -> Self {
        Self {
            net: 0,
            usb: 0,
        }
    }
}
