// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Read-only sysfs path constants and pure parsers; no `std::fs` (FP §4).

use std::io;

/// Default RAPL package (socket) energy counter; may be unreadable.
pub fn default_rapl_package_energy() -> std::path::PathBuf {
    std::path::PathBuf::from("/sys/class/powercap/intel-rapl/intel-rapl:0/energy_uj")
}

/// i915 **debug** GPU info (may require `debugfs` + `cap_sys_admin`).
pub fn i915_gpu_info() -> std::path::PathBuf {
    std::path::PathBuf::from("/sys/kernel/debug/dri/0/i915_gpu_info")
}

/// First NPU/AI accelerator device node, if any.
pub fn class_accel0() -> std::path::PathBuf {
    std::path::PathBuf::from("/sys/class/accel/accel0")
}

/// Parse `MemTotal:` kB from `/proc/meminfo` text.
pub fn parse_mem_total_kb(meminfo: &str) -> io::Result<u64> {
    for l in meminfo.lines() {
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

/// Logical CPU count from `processor` line count in `/proc/cpuinfo`.
pub fn parse_cpuinfo_logical_cores(cpuinfo: &str) -> u32 {
    let n = cpuinfo
        .lines()
        .filter(|l| l.to_lowercase().starts_with("processor"))
        .count();
    n.max(1) as u32
}

/// Best-effort L3 cache in KB from `/proc/cpuinfo` text.
pub fn parse_l3_cache_kb(cpuinfo: &str) -> u32 {
    for l in cpuinfo.lines() {
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

/// Parse `model name` from `/proc/cpuinfo` text.
pub fn parse_cpu_model_name(cpuinfo: &str) -> Option<String> {
    for line in cpuinfo.lines() {
        if line.to_lowercase().starts_with("model name") {
            if let Some(n) = line.split(':').nth(1) {
                return Some(n.trim().to_string());
            }
        }
    }
    None
}
