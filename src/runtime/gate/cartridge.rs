// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `GateCartridge` evidence contract — cold-edge witness for transition admissibility.
//!
//! # Honesty fences (W29-112-CARTRIDGE)
//!
//! Host Clausius–Duhem witness only (`CdTransitionCartridge`). This module does **not** claim:
//! - swarm/physics `GREEN`
//! - `PRODUCTION_WIRED` / live operator or concrete-cartridge production wire
//! - `MASTER` retick eligibility
//! - `OP-5` clearance
//!
//! Concrete-cartridge dissipation/strength/hydration lift remains a separate write_set.
//! Hot-path evaluators stay pure via [`crate::gate::transition_proposal::transition_outcome`].

use crate::gate::transition_proposal::{ThermodynamicStateSnapshot, TRANSITION_TOLERANCE};

use super::evidence::{explain_cd_transition_host, TransitionEvidence};

/// Cell id for this deepen write_set.
pub const GATE_CARTRIDGE_CELL_ID: &str = "W29-112-CARTRIDGE";

/// Honest posture tag — host-CD cold witness, not production wire.
pub const GATE_CARTRIDGE_POSTURE_TAG: &str =
    "runtime-gate-cartridge-host-cd-witness-not-production";

/// Honest fence — host-CD cartridge is not a production wire.
pub const GATE_CARTRIDGE_PRODUCTION_WIRED: bool = false;
/// Honest fence — GREEN claims stay blocked at this surface.
pub const GATE_CARTRIDGE_GREEN_CLAIM_BLOCKED: bool = true;
/// Honest fence — MASTER retick not claimed from cartridge deepen.
pub const GATE_CARTRIDGE_MASTER_RETICK_ELIGIBLE: bool = false;
/// Honest fence — OP-5 not cleared from cartridge deepen.
pub const GATE_CARTRIDGE_OP5_CLEARED: bool = false;
/// Honest measured fact — [`CdTransitionCartridge`] sources host CD, not concrete cartridge.
pub const GATE_CARTRIDGE_HOST_CD_ONLY: bool = true;
/// Honest measured fact — concrete-cartridge-backed evidence not claimed here.
pub const GATE_CARTRIDGE_CONCRETE_BACKED: bool = false;

const _: () = assert!(!GATE_CARTRIDGE_PRODUCTION_WIRED);
const _: () = assert!(GATE_CARTRIDGE_GREEN_CLAIM_BLOCKED);
const _: () = assert!(!GATE_CARTRIDGE_MASTER_RETICK_ELIGIBLE);
const _: () = assert!(!GATE_CARTRIDGE_OP5_CLEARED);
const _: () = assert!(GATE_CARTRIDGE_HOST_CD_ONLY);
const _: () = assert!(!GATE_CARTRIDGE_CONCRETE_BACKED);

/// Cartridge-facing evidence hook for gate transitions (cold-edge host witness).
///
/// Implementations produce structured witnesses at the Warm/Cold boundary; hot-path
/// evaluators remain pure via [`crate::gate::transition_proposal::transition_outcome`].
pub trait GateCartridge {
    #[must_use]
    fn transition_evidence(
        &self,
        old: &ThermodynamicStateSnapshot,
        new: &ThermodynamicStateSnapshot,
        dt: f64,
    ) -> TransitionEvidence;
}

/// Clausius–Duhem transition cartridge — first [`GateCartridge`] witness (host CD only).
#[derive(Debug, Clone, Copy, Default)]
pub struct CdTransitionCartridge;

impl GateCartridge for CdTransitionCartridge {
    fn transition_evidence(
        &self,
        old: &ThermodynamicStateSnapshot,
        new: &ThermodynamicStateSnapshot,
        dt: f64,
    ) -> TransitionEvidence {
        TransitionEvidence::from_constraint_explanation(explain_cd_transition_host(
            old,
            new,
            dt,
            TRANSITION_TOLERANCE,
        ))
    }
}

/// Measured honesty posture for this write_set (never invents GREEN/PRODUCTION_WIRED).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateCartridgeHonestyProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub production_wired: bool,
    pub green_claim_blocked: bool,
    pub master_retick_eligible: bool,
    pub op5_cleared: bool,
    pub host_cd_only: bool,
    pub concrete_backed: bool,
    pub deepen_honest: bool,
}

/// Snapshot honesty fences for the gate cartridge surface.
#[must_use]
pub fn gate_cartridge_honesty_probe() -> GateCartridgeHonestyProbe {
    let production_wired = GATE_CARTRIDGE_PRODUCTION_WIRED;
    let green_claim_blocked = GATE_CARTRIDGE_GREEN_CLAIM_BLOCKED;
    let master_retick_eligible = GATE_CARTRIDGE_MASTER_RETICK_ELIGIBLE;
    let op5_cleared = GATE_CARTRIDGE_OP5_CLEARED;
    let host_cd_only = GATE_CARTRIDGE_HOST_CD_ONLY;
    let concrete_backed = GATE_CARTRIDGE_CONCRETE_BACKED;
    let deepen_honest = GATE_CARTRIDGE_CELL_ID == "W29-112-CARTRIDGE"
        && GATE_CARTRIDGE_POSTURE_TAG
            == "runtime-gate-cartridge-host-cd-witness-not-production"
        && !production_wired
        && green_claim_blocked
        && !master_retick_eligible
        && !op5_cleared
        && host_cd_only
        && !concrete_backed;
    GateCartridgeHonestyProbe {
        cell_id: GATE_CARTRIDGE_CELL_ID,
        posture_tag: GATE_CARTRIDGE_POSTURE_TAG,
        production_wired,
        green_claim_blocked,
        master_retick_eligible,
        op5_cleared,
        host_cd_only,
        concrete_backed,
        deepen_honest,
    }
}

/// Fail-closed honesty check for cartridge deepen.
#[must_use]
pub fn validate_gate_cartridge_honesty() -> Result<(), &'static str> {
    let p = gate_cartridge_honesty_probe();
    if p.cell_id != "W29-112-CARTRIDGE" {
        return Err("gate_cartridge cell_id drift");
    }
    if p.production_wired {
        return Err("gate_cartridge production_wired must stay honest false");
    }
    if !p.green_claim_blocked {
        return Err("gate_cartridge GREEN claims must stay blocked");
    }
    if p.master_retick_eligible {
        return Err("gate_cartridge must not claim MASTER retick");
    }
    if p.op5_cleared {
        return Err("gate_cartridge must not claim OP-5 cleared");
    }
    if !p.host_cd_only {
        return Err("gate_cartridge must remain host-CD-only until concrete lift measured");
    }
    if p.concrete_backed {
        return Err("gate_cartridge must not invent concrete_backed");
    }
    if !p.deepen_honest {
        return Err("gate_cartridge deepen_honest failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::transition_proposal::transition_outcome;
    use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;
    use crate::runtime::gate::evidence::AdmissibilityToken;

    #[test]
    fn cd_transition_cartridge_admissible_evidence() {
        let old = ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        let new = old;
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(host.is_energy_positive(), "sanity: identity transition admits");

        let evidence = CdTransitionCartridge.transition_evidence(&old, &new, dt);
        assert_eq!(evidence.catalog_id, CD_TRANSITION_CATALOG_ID);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
        assert!(
            (evidence.margin.value() - host.dissipation as f32).abs() < 1e-5,
            "cartridge margin must track host D_int"
        );
    }

    #[test]
    fn cd_transition_cartridge_inadmissible_evidence() {
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(!host.is_energy_positive(), "sanity: ψ spike rejects on host");

        let evidence = CdTransitionCartridge.transition_evidence(&old, &new, dt);
        assert_eq!(evidence.catalog_id, CD_TRANSITION_CATALOG_ID);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Inadmissible);
        assert!(
            (evidence.margin.value() - host.dissipation as f32).abs() < 1e-5,
            "cartridge margin must track host D_int on reject"
        );
        assert!(evidence.margin.violation() > 0.0);
    }

    #[test]
    fn cd_transition_cartridge_margin_parity_with_host_explanation() {
        let old = ThermodynamicStateSnapshot {
            density: 2300.0,
            temperature: 295.0,
            free_energy: -1.2e5,
            entropy: 0.1,
            reaction_extent: 0.3,
            strength: 15.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.25e5,
            entropy: 0.11,
            ..old
        };
        let dt = 0.5_f64;
        let explanation = explain_cd_transition_host(&old, &new, dt, TRANSITION_TOLERANCE);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &new, dt);
        assert_eq!(evidence.admissibility, explanation.admissibility);
        assert_eq!(evidence.margin, explanation.margin);
        assert_eq!(evidence.catalog_id, explanation.channel_id);
        assert!(evidence.observed_at.is_none());
    }

    #[test]
    fn honesty_fences_block_green_production_master_op5() {
        assert_eq!(GATE_CARTRIDGE_CELL_ID, "W29-112-CARTRIDGE");
        assert!(!GATE_CARTRIDGE_PRODUCTION_WIRED);
        assert!(GATE_CARTRIDGE_GREEN_CLAIM_BLOCKED);
        assert!(!GATE_CARTRIDGE_MASTER_RETICK_ELIGIBLE);
        assert!(!GATE_CARTRIDGE_OP5_CLEARED);
        assert!(GATE_CARTRIDGE_HOST_CD_ONLY);
        assert!(!GATE_CARTRIDGE_CONCRETE_BACKED);
        let probe = gate_cartridge_honesty_probe();
        assert_eq!(probe.cell_id, "W29-112-CARTRIDGE");
        assert!(!probe.production_wired);
        assert!(probe.green_claim_blocked);
        assert!(!probe.master_retick_eligible);
        assert!(!probe.op5_cleared);
        assert!(probe.host_cd_only);
        assert!(!probe.concrete_backed);
        assert!(probe.deepen_honest);
        assert!(validate_gate_cartridge_honesty().is_ok());
    }
}
