// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//
// AGAP-1920-B4-076 @ 19:20 IST — runtime T4 seam bridge for R-atoms-scalar / Burn deepen.
// W29-099-ATOMS_SCALAR_BRIDGE — Composer RL honesty deepen (umst-admit-grok).
//
// Cartridge atoms remain f64 scalar-first (SC-01..03 in `umst-cartridge-api` / continuum).
// This module owns the **one cold boundary** between Burn `f32` tensor physics and f64 gate
// snapshots — R-ATOMS-SC-05 per `old/residuals/residuals/misc-outputs-tmp/R_ATOMS_SCALAR_RESIDUE_MAP_1640.md` §4.5.
//
// **Honesty:** slice-2 `BurnAlgebra` 0D prototype stays in `umst-cartridge-continuum/tensor_lift`.
// Production `burn::Tensor` monomorphization remains **[open]** under `R-faithful-decomp-B1`.
// **Invent fence:** not GREEN / not PRODUCTION_WIRED / not MASTER / not OP-5.

use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// Parent G40-R10 / R40-27 residue id — landed @ AGENT-081.
pub const PARENT_RESIDUE_ID: &str = "R-atoms-scalar";

/// Sub-residue id for runtime manifold dual-surface (T4).
pub const SUB_RESIDUE_ID: &str = "R-ATOMS-SC-05";

/// Depth tier — repo-boundary seam (not per-crate atom debt).
pub const DEPTH_TIER: &str = "T4";

/// Honest posture — cast bridge landed; production tensor lift deferred.
pub const POSTURE_TAG: &str = "DEFERRED";

/// Primary source anchor for fleet / meta hygiene (R10-b).
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/runtime/atoms_scalar_bridge.rs";

/// F1 owner rollup module cross-ref (E2 @ P1700; H4 @ P1800 deepen chain).
pub const F1_DEEPEN_ROLLUP_PATH: &str = "umst-manifold/src/runtime/atoms_f1_deepen.rs";

/// Slice-3 0D tensor lift step cross-ref (consumer of this T4 cast).
pub const TENSOR_LIFT_STEP_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift.rs";

/// Prior E2 receipt slug for H4 absorb chain.
pub const PRIOR_E2_RECEIPT_SLUG: &str = "COMPOSER_P1700_E2";

/// Whether F1 owner rollup module is on disk @ H4.
pub const F1_DEEPEN_ROLLUP_LANDED: bool = true;

/// Runtime cast bridge is landed on the Burn home (`umst-runtime` alias).
pub const RUNTIME_BRIDGE_LANDED: bool = true;

/// Production `burn::Tensor` rank-1+ monomorphization — still open.
pub const PRODUCTION_TENSOR_DEFERRED: bool = true;

/// Frozen thermo Burn lane field count (density..strength).
pub const BURN_LANE_FIELD_COUNT: usize = 6;

/// Cold-boundary roundtrip epsilon for host↔Burn f32 casts (scalar host).
pub const COLD_BOUNDARY_F32_EPS: f64 = 1e-3;

/// W29-099 Composer RL cell id (honesty deepen attribution).
pub const W29_099_CELL_ID: &str = "W29-099-ATOMS_SCALAR_BRIDGE";

/// Model pin for this deepen lane.
pub const DEEPEN_MODEL_SLUG: &str = "cursor-grok-4.6-high";

/// Admit coding lane pin.
pub const DEEPEN_LANE: &str = "umst-admit-grok";

/// Honest deepen posture — T4 cast bridge measured; production ceremony stays OPEN.
pub const HONEST_DEEPEN_POSTURE: &str = "T4_CAST_BRIDGE_HONEST_PROD_OPEN";

/// Explicit non-claims — deepen must not invent these.
pub const NON_CLAIM: &str =
    "not GREEN; not PRODUCTION_WIRED; not MASTER; not OP-5; production tensor lift remains OPEN";

/// Honest master retick posture — T4 cast census only.
pub const MASTER_RETICK: &str = "no";

/// Honest `production_wired` floor — never true until measured live wire proof.
#[must_use]
pub const fn atoms_scalar_bridge_production_wired() -> bool {
    false
}

const _: () = assert!(!atoms_scalar_bridge_production_wired());

/// OP-5 PASS invent fence — stays false on T4 cast deepen slice.
#[must_use]
pub const fn atoms_scalar_bridge_op5_pass_invented() -> bool {
    false
}

const _: () = assert!(!atoms_scalar_bridge_op5_pass_invented());

/// MASTER invent / retick fence — T4 cast census only.
#[must_use]
pub const fn atoms_scalar_bridge_master_invented() -> bool {
    false
}

const _: () = assert!(!atoms_scalar_bridge_master_invented());

/// GREEN invent fence — stays false (tool readiness ≠ physics GREEN).
#[must_use]
pub const fn atoms_scalar_bridge_green_invented() -> bool {
    false
}

const _: () = assert!(!atoms_scalar_bridge_green_invented());

/// Flip authorization — blocked until operator ceremony.
#[must_use]
pub const fn atoms_scalar_bridge_flip_authorized() -> bool {
    false
}

const _: () = assert!(!atoms_scalar_bridge_flip_authorized());

/// f32 Burn lane for thermodynamic gate fields at the cold boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermodynamicBurnLane {
    pub density: f32,
    pub temperature: f32,
    pub free_energy: f32,
    pub entropy: f32,
    pub reaction_extent: f32,
    pub strength: f32,
}

/// Fleet census line for R10-b (`umst-meta check --fleet` hygiene target).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomsScalarRuntimeDepthSummary {
    pub parent_residue_id: &'static str,
    pub sub_residue_id: &'static str,
    pub depth_tier: &'static str,
    pub posture_tag: &'static str,
    pub runtime_bridge_landed: bool,
    pub production_tensor_deferred: bool,
    /// F1 owner rollup module on disk (`atoms_f1_deepen`).
    pub f1_deepen_rollup_landed: bool,
}

/// Frozen runtime depth summary — honest partial on T4 seam only.
#[must_use]
pub const fn atoms_scalar_runtime_depth_summary() -> AtomsScalarRuntimeDepthSummary {
    AtomsScalarRuntimeDepthSummary {
        parent_residue_id: PARENT_RESIDUE_ID,
        sub_residue_id: SUB_RESIDUE_ID,
        depth_tier: DEPTH_TIER,
        posture_tag: POSTURE_TAG,
        runtime_bridge_landed: RUNTIME_BRIDGE_LANDED,
        production_tensor_deferred: PRODUCTION_TENSOR_DEFERRED,
        f1_deepen_rollup_landed: F1_DEEPEN_ROLLUP_LANDED,
    }
}

/// Cast cartridge / gate `f64` scalar to Burn `f32` lane (single cold boundary).
#[must_use]
pub fn cartridge_scalar_to_burn_f32(value: f64) -> f32 {
    value as f32
}

/// Cast Burn `f32` lane back to cartridge / gate `f64` scalar.
#[must_use]
pub fn burn_f32_to_cartridge_scalar(value: f32) -> f64 {
    f64::from(value)
}

/// Project f64 gate snapshot into f32 Burn lane for hot-path tensor ops.
#[must_use]
pub fn thermodynamic_snapshot_to_burn_lane(
    snapshot: &ThermodynamicStateSnapshot,
) -> ThermodynamicBurnLane {
    ThermodynamicBurnLane {
        density: cartridge_scalar_to_burn_f32(snapshot.density),
        temperature: cartridge_scalar_to_burn_f32(snapshot.temperature),
        free_energy: cartridge_scalar_to_burn_f32(snapshot.free_energy),
        entropy: cartridge_scalar_to_burn_f32(snapshot.entropy),
        reaction_extent: cartridge_scalar_to_burn_f32(snapshot.reaction_extent),
        strength: cartridge_scalar_to_burn_f32(snapshot.strength),
    }
}

/// Rehydrate f64 gate snapshot from f32 Burn lane (post-solve cold edge).
#[must_use]
pub fn thermodynamic_snapshot_from_burn_lane(
    lane: &ThermodynamicBurnLane,
) -> ThermodynamicStateSnapshot {
    ThermodynamicStateSnapshot {
        density: burn_f32_to_cartridge_scalar(lane.density),
        temperature: burn_f32_to_cartridge_scalar(lane.temperature),
        free_energy: burn_f32_to_cartridge_scalar(lane.free_energy),
        entropy: burn_f32_to_cartridge_scalar(lane.entropy),
        reaction_extent: burn_f32_to_cartridge_scalar(lane.reaction_extent),
        strength: burn_f32_to_cartridge_scalar(lane.strength),
    }
}

/// W29-099 Composer RL honesty deepen probe — T4 cast census + invent fences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomsScalarBridgeW29099DeepenProbe {
    pub cell_id: &'static str,
    pub model_slug: &'static str,
    pub lane: &'static str,
    pub honest_posture: &'static str,
    pub non_claim: &'static str,
    pub master_retick: &'static str,
    pub runtime_bridge_landed: bool,
    pub production_tensor_deferred: bool,
    pub f1_deepen_rollup_landed: bool,
    pub burn_lane_field_count: usize,
    pub production_wired: bool,
    pub op5_pass_invented: bool,
    pub master_invented: bool,
    pub green_invented: bool,
    pub flip_authorized: bool,
}

/// Build W29-099 deepen probe from live runtime depth summary + invent fences.
#[must_use]
pub fn atoms_scalar_bridge_w29099_deepen_probe() -> AtomsScalarBridgeW29099DeepenProbe {
    let summary = atoms_scalar_runtime_depth_summary();
    AtomsScalarBridgeW29099DeepenProbe {
        cell_id: W29_099_CELL_ID,
        model_slug: DEEPEN_MODEL_SLUG,
        lane: DEEPEN_LANE,
        honest_posture: HONEST_DEEPEN_POSTURE,
        non_claim: NON_CLAIM,
        master_retick: MASTER_RETICK,
        runtime_bridge_landed: summary.runtime_bridge_landed,
        production_tensor_deferred: summary.production_tensor_deferred,
        f1_deepen_rollup_landed: summary.f1_deepen_rollup_landed,
        burn_lane_field_count: BURN_LANE_FIELD_COUNT,
        production_wired: atoms_scalar_bridge_production_wired(),
        op5_pass_invented: atoms_scalar_bridge_op5_pass_invented(),
        master_invented: atoms_scalar_bridge_master_invented(),
        green_invented: atoms_scalar_bridge_green_invented(),
        flip_authorized: atoms_scalar_bridge_flip_authorized(),
    }
}

/// Honesty gate for W29-099 deepen — T4 cast landed; invent fences hold.
#[must_use]
pub fn atoms_scalar_bridge_w29099_deepen_honest(
    probe: &AtomsScalarBridgeW29099DeepenProbe,
) -> bool {
    probe.cell_id == W29_099_CELL_ID
        && probe.model_slug == DEEPEN_MODEL_SLUG
        && probe.lane == DEEPEN_LANE
        && probe.honest_posture == HONEST_DEEPEN_POSTURE
        && probe.non_claim.contains("not GREEN")
        && probe.non_claim.contains("not PRODUCTION_WIRED")
        && probe.non_claim.contains("not MASTER")
        && probe.non_claim.contains("not OP-5")
        && probe.master_retick == "no"
        && probe.runtime_bridge_landed
        && probe.production_tensor_deferred
        && probe.f1_deepen_rollup_landed
        && probe.burn_lane_field_count == BURN_LANE_FIELD_COUNT
        && !probe.production_wired
        && !probe.op5_pass_invented
        && !probe.master_invented
        && !probe.green_invented
        && !probe.flip_authorized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sc05_runtime_posture_metadata_locked() {
        let summary = atoms_scalar_runtime_depth_summary();
        assert_eq!(summary.parent_residue_id, "R-atoms-scalar");
        assert_eq!(summary.sub_residue_id, "R-ATOMS-SC-05");
        assert_eq!(summary.depth_tier, "T4");
        assert_eq!(summary.posture_tag, POSTURE_TAG);
        assert!(RUNTIME_BRIDGE_LANDED);
        assert!(PRODUCTION_TENSOR_DEFERRED);
        assert!(summary.f1_deepen_rollup_landed);
        assert!(F1_DEEPEN_ROLLUP_PATH.contains("atoms_f1_deepen"));
        assert!(TENSOR_LIFT_STEP_PATH.contains("atoms_tensor_lift"));
        assert!(PRIOR_E2_RECEIPT_SLUG.contains("COMPOSER_P1700_E2"));
        assert_eq!(BURN_LANE_FIELD_COUNT, 6);
    }

    #[test]
    fn cartridge_scalar_cast_roundtrip_within_f32_epsilon() {
        let host = 2400.0_f64;
        let burn = cartridge_scalar_to_burn_f32(host);
        let back = burn_f32_to_cartridge_scalar(burn);
        assert!((back - host).abs() < COLD_BOUNDARY_F32_EPS);
    }

    #[test]
    fn thermodynamic_snapshot_burn_lane_roundtrip() {
        let snapshot = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let lane = thermodynamic_snapshot_to_burn_lane(&snapshot);
        let restored = thermodynamic_snapshot_from_burn_lane(&lane);
        assert!((restored.density - snapshot.density).abs() < 1e-2);
        assert!((restored.free_energy - snapshot.free_energy).abs() < 1e-1);
        assert!((restored.strength - snapshot.strength).abs() < 1e-2);
    }

    #[test]
    fn w29099_invent_fences_hold() {
        assert!(!atoms_scalar_bridge_production_wired());
        assert!(!atoms_scalar_bridge_op5_pass_invented());
        assert!(!atoms_scalar_bridge_master_invented());
        assert!(!atoms_scalar_bridge_green_invented());
        assert!(!atoms_scalar_bridge_flip_authorized());
        assert_eq!(MASTER_RETICK, "no");
        assert_eq!(DEEPEN_MODEL_SLUG, "cursor-grok-4.6-high");
        assert_eq!(DEEPEN_LANE, "umst-admit-grok");
        assert_eq!(W29_099_CELL_ID, "W29-099-ATOMS_SCALAR_BRIDGE");
        assert!(NON_CLAIM.contains("not GREEN"));
        assert!(NON_CLAIM.contains("not PRODUCTION_WIRED"));
        assert!(NON_CLAIM.contains("not MASTER"));
        assert!(NON_CLAIM.contains("not OP-5"));
    }

    #[test]
    fn w29099_deepen_probe_honest() {
        let probe = atoms_scalar_bridge_w29099_deepen_probe();
        assert!(atoms_scalar_bridge_w29099_deepen_honest(&probe));
        assert!(probe.runtime_bridge_landed);
        assert!(probe.production_tensor_deferred);
        assert!(probe.f1_deepen_rollup_landed);
        assert_eq!(probe.burn_lane_field_count, 6);
        assert!(!probe.production_wired);
        assert!(!probe.op5_pass_invented);
        assert!(!probe.master_invented);
        assert!(!probe.green_invented);
        assert!(!probe.flip_authorized);
        assert_eq!(probe.master_retick, "no");
        assert_eq!(probe.honest_posture, HONEST_DEEPEN_POSTURE);
    }
}
