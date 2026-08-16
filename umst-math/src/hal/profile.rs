// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! `ArchitectureProfile` — B-2.5 bridge field (FORWARD-PLAN §14.2); **schema only** in H-8.
//!
//! # I5 (NED)
//! `ArchClass::Unknown` uses a **host fingerprint hash** (never a raw serial in ledgers; B-7).
//! `pareto_reference` is `None` when we have no measured front yet.

use std::string::String;
use std::vec::Vec;

use super::kinds::UnitKind;
use super::traits::WorkloadClass;

/// Pareto front *label* when a B-2 reference is applicable (H-8: optional stub)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParetoReferenceLabel {
    /// ZCI-EXEMPT: token pointing at a future `REGISTRY` row id (B-2)
    pub reg_token: String,
}

/// The closed `arch_class` three-way (linux reference laptop / mac M3 / unknown; FORWARD-PLAN §14.2, §0.1)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArchClass {
    /// P14* Gen5-class Linux+Intel (reference *name*, not a guarantee of exact SKU)
    LinuxIntelP14sGen5,
    /// macOS + Apple Silicon M-Max (reference *name* for functor staging)
    MacosM3Max,
    /// Honest “we don’t classify yet” (host fingerprint is sha256, hex)
    Unknown {
        /// 64-hex *hash* of host identity bundle (B-0 / B-7; H-8 uses stub literal)
        host_fingerprint_hash: String,
    },
}

/// B-2.5 architecture-self-aware *descriptor* (H-8 holds structure; measurements land in B-2+)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArchitectureProfile {
    /// Theorem: which reference world we think we are in
    pub arch_class: ArchClass,
    /// Canonical per-workload fallbacks: ordered [`UnitKind`] chain (max length
    /// bounded by `REGISTRY` `hal_canonical_fallback_chain_max_len` = 8)
    pub canonical_fallback_chains: Vec<(WorkloadClass, Vec<UnitKind>)>,
    /// NED: freeform *constraints* string (e.g. "dedicated_vram=None" vs VRAM 8Gib)
    pub architectural_constraints: String,
    /// Some when a Tier-2 Pareto ref exists; `None` for cold hosts
    pub pareto_reference: Option<ParetoReferenceLabel>,
}

impl ArchitectureProfile {
    /// H-8 *stub* profile: unknown class, empty chains, no Pareto
    pub fn h8_stub() -> Self {
        Self {
            arch_class: ArchClass::Unknown {
                host_fingerprint_hash: "stub_no_probe".to_string(),
            },
            canonical_fallback_chains: Vec::new(),
            architectural_constraints: "H-8 stub: no B-0 calibration yet".to_string(),
            pareto_reference: None,
        }
    }
}
