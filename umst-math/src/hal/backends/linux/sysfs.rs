//! Read-only sysfs helpers; no `unsafe`.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Default RAPL package (socket) energy counter; may be unreadable.
pub fn default_rapl_package_energy() -> PathBuf {
    PathBuf::from("/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj")
}

/// i915 **debug** GPU info (may require `debugfs` + `cap_sys_admin`).
pub fn i915_gpu_info() -> PathBuf {
    PathBuf::from("/sys/kernel/debug/dri/0/i915_gpu_info")
}

/// First NPU/AI accelerator device node, if any.
pub fn class_accel0() -> PathBuf {
    PathBuf::from("/sys/class/accel/accel0")
}

// ZCI-EXEMPT: MSRV 1.74; `io::Error::other` is 1.79+ (see comment on allow below)
#[allow(clippy::io_other_error)] // MSRV 1.74: `io::Error::other` is 1.79+
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

/// First line of a text file, trimmed
pub fn read_line(path: &Path) -> io::Result<String> {
    let s = fs::read_to_string(path)?;
    Ok(s.lines().next().unwrap_or("").trim().to_string())
}

/// Parse `0x` vendor id if present
pub fn pci_vendor_hex(path: &Path) -> Option<String> {
    read_line(path).ok()
}

/// Count of network interfaces in `/sys/class/net` excluding `lo`
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

/// USB bus device directory count (rough port-ish enumeration)
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

/// `/proc/meminfo` `MemTotal:` kB
pub fn mem_total_kb() -> io::Result<u64> {
    let s = fs::read_to_string("/proc/meminfo")?;
    for l in s.lines() {
        if l.starts_with("MemTotal:") {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if let Some(n) = parts.get(1) {
                return Ok(n.parse().unwrap_or(0));
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "MemTotal not found",
    ))
}

/// Logical CPU count from `processor` line count in `/proc/cpuinfo`
pub fn cpuinfo_logical_cores() -> io::Result<u32> {
    let s = fs::read_to_string("/proc/cpuinfo")?;
    let n = s
        .lines()
        .filter(|l| l.to_lowercase().starts_with("processor"))
        .count();
    Ok(n.max(1) as u32)
}

/// Best-effort L3 cache in KB
pub fn l3_cache_kb() -> u32 {
    for l in std::fs::read_to_string("/proc/cpuinfo")
        .unwrap_or_default()
        .lines()
    {
        if l.to_lowercase().contains("cache size") {
            if let Some(rest) = l.split(':').nth(1) {
                let t = rest.trim();
                if let Some(kb) = t.strip_suffix(" KB") {
                    if let Ok(n) = kb.trim().parse::<u32>() {
                        return n;
                    }
                }
            }
        }
    }
    0
}
