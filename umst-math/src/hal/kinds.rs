// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Closed enum of the **seven** hardware **object** kinds in category **𝓗** (FORWARD-PLAN §0.2; H-8).
//! REGISTRY: `hal_unit_kind_count` = 7.
//!
//! # I1 (type discipline)
//! No additional variants without a new slice + REGISTRY + wide-gate update.

/// ZCI-EXEMPT: the seven `HardwareInventory` lanes (H-8; Q4 vocabulary)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnitKind {
    /// Host CPU
    Cpu,
    /// Integrated GPU
    Igpu,
    /// Discrete GPU
    Dgpu,
    /// Intel / other discrete NPU (OpenVINO path in H-9)
    Npu,
    /// Apple ANE (Core ML path in H-9-mac)
    Ane,
    /// System RAM (UMA/DRAM)
    Ram,
    /// Exposed I/O port cluster (USB-C / HDMI / RJ-45; H-10)
    Port,
}

impl UnitKind {
    /// All kinds in CGD order (total iterator for law (c) “closed enum” checks)
    pub const ALL: [UnitKind; 7] = [
        UnitKind::Cpu,
        UnitKind::Igpu,
        UnitKind::Dgpu,
        UnitKind::Npu,
        UnitKind::Ane,
        UnitKind::Ram,
        UnitKind::Port,
    ];
}
