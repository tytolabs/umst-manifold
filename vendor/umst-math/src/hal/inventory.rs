//! `HardwareInventory` — one [`super::presence::UnitPresence`] per [`super::kinds::UnitKind`], plus [`super::profile::ArchitectureProfile`] (B-2.5 §14.2).
//!
//! # I4 (H-Arc lane)
//! This struct is the *node* in the H-Arc cross-arc map (FORWARD-PLAN §4) opened by H-8.
//!
//! # I5
//! A row [`super::presence::UnitPresence::AbsentByConfig`] means *no fabrication*; empty inventory
//! in egoff is valid (stub).

use std::sync::Arc;

use super::kinds::UnitKind;
use super::presence::UnitPresence;
use super::profile::ArchitectureProfile;
use super::traits::HardwareUnit;

/// One slot: present (`Arc` shared unit) or one of the four absences
pub type UnitSlot = UnitPresence<Arc<dyn HardwareUnit + Send + Sync + 'static>>;

/// The seven-lane inventory + global architecture descriptor (H-8) — `Debug` is intentionally not derived (trait object lanes).
#[derive(Clone)]
pub struct HardwareInventory {
    /// B-2.5 field (H-8: may be `ArchitectureProfile::h8_stub()`)
    pub profile: ArchitectureProfile,
    /// CPU lane
    pub cpu: UnitSlot,
    /// Integrated GPU
    pub igpu: UnitSlot,
    /// Discrete GPU
    pub dgpu: UnitSlot,
    /// NPU (non-Apple)
    pub npu: UnitSlot,
    /// ANE (Apple) / neural accelerator
    pub ane: UnitSlot,
    /// RAM
    pub ram: UnitSlot,
    /// I/O / port *cluster* enumeration
    pub port: UnitSlot,
}

impl HardwareInventory {
    /// Look up a lane by kind (total, exhaustive on [`UnitKind`])
    #[must_use]
    pub fn slot(&self, k: UnitKind) -> &UnitSlot {
        match k {
            UnitKind::Cpu => &self.cpu,
            UnitKind::Igpu => &self.igpu,
            UnitKind::Dgpu => &self.dgpu,
            UnitKind::Npu => &self.npu,
            UnitKind::Ane => &self.ane,
            UnitKind::Ram => &self.ram,
            UnitKind::Port => &self.port,
        }
    }

    /// H-8 **stub** inventory: every unit [`UnitPresence::AbsentByConfig`]; profile is
    /// [`ArchitectureProfile::h8_stub`]. *No* fabricated `Present` rows.
    pub fn h8_all_absent_by_config() -> Self {
        let absent = || UnitPresence::AbsentByConfig;
        Self {
            profile: ArchitectureProfile::h8_stub(),
            cpu: absent(),
            igpu: absent(),
            dgpu: absent(),
            npu: absent(),
            ane: absent(),
            ram: absent(),
            port: absent(),
        }
    }
}
