// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! `UnitPresence<U>` — Q5 *tagged sum* (FORWARD-PLAN §0.1) with four *honest* absence reasons (NED §0.5).
//!
//! - [`UnitPresence::Present`]: a witness `U` (in H-8, a [`super::traits::HardwareUnit`] impl)
//! - [`UnitPresence::AbsentByArch`]: *physically* absent (e.g. ANE on Linux, dGPU on MacBook w/o dGPU)
//! - [`UnitPresence::AbsentByConfig`]: not exposed / policy-off (H-8 stub: **all** units start here)
//! - [`UnitPresence::AbsentByFault(Reason)`]: *could* exist but we failed to read it
//!
//! # I2 (category)
//! `UnitPresence` is the object carrier per [`super::kinds::UnitKind`] in [`super::inventory::HardwareInventory`].
//! Pattern matching on this ADT is **exhaustive** in Rust; law (c) in `hal_trait_laws` reifies totality.
//!
//! # I3
//! No hidden `null`: every row is *one* of the four.

use std::string::String;

/// NED-typed fault reason (serial patterns scrubbed in B-7; H-8 carries structure only)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AbsentReason {
    /// Opaque u32 for RED fixtures / stable hashing
    pub code: u32,
    /// Human/operator message (not a device serial in H-8)
    pub message: String,
}

/// The four-state presence ADT (Q5)
#[derive(Clone, Debug, PartialEq)]
pub enum UnitPresence<U> {
    /// Present: hardware unit *witness*
    Present(U),
    /// Not on this architecture
    AbsentByArch,
    /// Policy / configuration excludes this lane (H-8 *stub* default for every kind)
    AbsentByConfig,
    /// Probing failed honestly
    AbsentByFault(AbsentReason),
}

impl<U> UnitPresence<U> {
    /// Stable string for operator logs (H-9 `tracing::info!`); not a `REGISTRY` token.
    #[must_use]
    pub fn short_label(&self) -> &'static str {
        match self {
            Self::Present(_) => "Present",
            Self::AbsentByArch => "AbsentByArch",
            Self::AbsentByConfig => "AbsentByConfig",
            Self::AbsentByFault(_) => "AbsentByFault",
        }
    }

    /// Map `Present` through `f`; preserve absence ** functorially ** on the four cases.
    #[must_use]
    pub fn map<V, F: FnOnce(U) -> V>(self, f: F) -> UnitPresence<V> {
        match self {
            Self::Present(u) => UnitPresence::Present(f(u)),
            Self::AbsentByArch => UnitPresence::AbsentByArch,
            Self::AbsentByConfig => UnitPresence::AbsentByConfig,
            Self::AbsentByFault(r) => UnitPresence::AbsentByFault(r),
        }
    }
}
