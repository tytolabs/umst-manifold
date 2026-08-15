//! Host IO boundary for Linux H-9 sysfs/proc probes (FP §4).
//!
//! Enabled with the `linux-hal-sysfs` feature. Runtime/CLI callers should probe once
//! and inject [`super::probe_snapshot::HalProbeSnapshot`] into pure assembly paths.

#![cfg(target_os = "linux")]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::backends::linux::rapl;
use super::backends::linux::sysfs;
use super::permission_state::{classify_read_probe, PermissionState};
use super::probe_snapshot::{CpuProbe, HalProbeSnapshot, IgpuProbe, NpuProbe, PortProbe, RamProbe};

const RAPL_LABEL: &str = "/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj";

/// Probe the live host and return a pure snapshot for HAL assembly.
#[must_use]
pub fn probe_sysfs_snapshot() -> HalProbeSnapshot {
    let rapl_path = sysfs::default_rapl_package_energy();
    let rapl_permission = read_probe(&rapl_path, RAPL_LABEL);
    let rapl_readable = matches!(rapl_permission, PermissionState::Granted);
    let rapl_uj = if rapl_readable {
        rapl::read_package_energy_uj(&rapl_path)
    } else {
        None
    };

    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let logical_cores = sysfs::parse_cpuinfo_logical_cores(&cpuinfo);
    let l3_cache_kb = sysfs::parse_l3_cache_kb(&cpuinfo);
    let model_name = sysfs::parse_cpu_model_name(&cpuinfo);

    let igpu = probe_igpu();
    let npu = NpuProbe {
        present: sysfs::class_accel0().exists(),
    };

    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let total_kb = sysfs::parse_mem_total_kb(&meminfo).unwrap_or(0);

    let port = PortProbe {
        net: net_iface_count().unwrap_or(0),
        usb: usb_device_entry_count().unwrap_or(0),
    };

    let thunderbolt_enum_denied = probe_thunderbolt_denied();

    HalProbeSnapshot {
        cpu: CpuProbe {
            rapl_path,
            rapl_permission,
            rapl_uj,
            logical_cores,
            l3_cache_kb,
            model_name,
        },
        igpu,
        npu,
        ram: RamProbe {
            total_kb,
            rapl_ok: rapl_readable,
            rapl_uj,
        },
        port,
        thunderbolt_enum_denied,
    }
}

fn probe_igpu() -> IgpuProbe {
    let mut has_intel = false;
    for card in list_drm_cards().unwrap_or_default() {
        let v = card.join("device/vendor");
        if let Some(s) = read_line(&v).ok() {
            if s.trim() == "0x8086" && card.join("device/uevent").exists() {
                has_intel = true;
                break;
            }
        }
    }
    let p = sysfs::i915_gpu_info();
    let debug_readable = p.exists() && fs::read(&p).is_ok();
    IgpuProbe {
        has_intel_render: has_intel,
        i915_path: p,
        debug_readable,
    }
}

fn probe_thunderbolt_denied() -> bool {
    let tbt = Path::new("/sys/bus/thunderbolt/devices");
    if !tbt.exists() {
        return false;
    }
    match fs::read_dir(tbt) {
        Err(e) => e.raw_os_error() == Some(13) || e.kind() == io::ErrorKind::PermissionDenied,
        Ok(_) => false,
    }
}

/// Read entire small file: success → `Granted`; EACCES → `Denied(13)`.
pub fn read_probe(path: &Path, label: &'static str) -> PermissionState {
    if !path.exists() {
        return PermissionState::NotApplicable;
    }
    classify_read_probe(fs::read(path), label)
}

/// First line of a text file, trimmed.
pub fn read_line(path: &Path) -> io::Result<String> {
    let s = fs::read_to_string(path)?;
    Ok(s.lines().next().unwrap_or("").trim().to_string())
}

/// DRM card paths under `/sys/class/drm`.
pub fn list_drm_cards() -> io::Result<Vec<PathBuf>> {
    let mut v = Vec::new();
    let p = Path::new("/sys/class/drm");
    for e in fs::read_dir(p).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
        let e = e?;
        let p = e.path();
        if p.file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s.starts_with("card") && !s.contains('-'))
        {
            v.push(p);
        }
    }
    v.sort();
    Ok(v)
}

/// Count of network interfaces in `/sys/class/net` excluding `lo`.
pub fn net_iface_count() -> io::Result<usize> {
    let mut c = 0u32;
    for e in fs::read_dir("/sys/class/net")? {
        let e = e?;
        if let Some(n) = e.file_name().to_str() {
            if n != "lo" {
                c = c.saturating_add(1);
            }
        }
    }
    Ok(c as usize)
}

/// USB bus device directory count (rough port-ish enumeration).
pub fn usb_device_entry_count() -> io::Result<usize> {
    let mut c = 0usize;
    for e in fs::read_dir("/sys/bus/usb/devices")? {
        let e = e?;
        if e.file_type()?.is_dir() {
            c += 1;
        }
    }
    Ok(c)
}
