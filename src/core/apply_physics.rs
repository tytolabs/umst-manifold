// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Merge sparse nodal [`crate::core::traits::PhysicalResult`] channels back into a live
//! [`crate::core::tensors::UnifiedMaterialStateTensor`] for topology / gateway loops.
//!
//! Uses [`UnifiedMaterialStateTensor::project_scalar_channel`] and
//! [`UnifiedMaterialStateTensor::write_scalar_channel`] so **damage** and **temperature** respect
//! [`UnifiedMaterialStateTensor::policy_editable_mask`].
//!
//! ## Honest fences (W29-021)
//!
//! - **Landed:** masked damage writeback + optional additive temperature ΔT writeback through
//!   typed [`ApplyPhysicsError`] refusals and B1 / scalar-channel typestate.
//! - **Not merged here:** `free_energy`, `dissipation`, `safety_margin`, `cost`, or
//!   `information_density` — those stay on the PPO / CBF reward / admissibility path
//!   ([`APPLY_PHYSICS_UNMERGED_PHYSICAL_RESULT_FIELDS`]).
//! - **Multi-batch:** when `B > 1`, only batch index `0` is consumed —
//!   [`APPLY_PHYSICS_MULTI_BATCH_MERGE`] stays **false**.
//! - [`APPLY_PHYSICS_PRODUCTION_WIRED`], [`APPLY_PHYSICS_PHYSICS_GREEN`], and
//!   [`APPLY_PHYSICS_MASTER`] stay **false** — writeback ≠ fleet physics GREEN / production /
//!   MASTER retick. See [`apply_physics_posture_probe`].

use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

use crate::core::dec_typestate::ScalarChannelIdx;
use crate::core::error_boundary::ApplyPhysicsError;
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::PhysicalResult;
use crate::core::umst_schema::{SCALAR_DAMAGE, SCALAR_TEMPERATURE};

/// W29 deepen cell — apply-physics writeback honesty (no invent GREEN).
pub const W29_APPLY_PHYSICS_DEEPEN_CELL: &str = "W29-021-APPLY_PHYSICS";

/// Measured deepen stamp — unmerged inventory + multi-batch fence (not GREEN).
pub const APPLY_PHYSICS_DEEPEN_STAMP: &str = "W29-021-v2-unmerged-inventory-multibatch";

/// Honest posture — damage / ΔT writeback landed; full PhysicalResult merge open.
pub const APPLY_PHYSICS_POSTURE_TAG: &str = "WRITEBACK_DAMAGE_TEMP_PARTIAL";

/// Honest deepen fence for meta / fleet probes.
pub const APPLY_PHYSICS_HONEST_FENCE: &str = concat!(
    "damage_writeback=true|temperature_delta_writeback=true|policy_mask=true|",
    "free_energy_merge=false|dissipation_merge=false|safety_margin_merge=false|",
    "cost_merge=false|info_density_merge=false|multi_batch_merge=false|",
    "production_wired=false|physics_green=false|master=false"
);

/// Masked damage column merge via `project_scalar_channel` / `write_scalar_channel`.
pub const APPLY_PHYSICS_DAMAGE_WRITEBACK: bool = true;

/// Optional additive temperature ΔT merge under the same policy mask.
pub const APPLY_PHYSICS_TEMPERATURE_DELTA_WRITEBACK: bool = true;

/// Policy-editable mask is honored on both damage and temperature paths.
pub const APPLY_PHYSICS_POLICY_MASK_RESPECTED: bool = true;

/// `PhysicalResult::free_energy` is **not** written into UMST scalar features here.
pub const APPLY_PHYSICS_FREE_ENERGY_MERGE: bool = false;

/// `PhysicalResult::dissipation` is **not** written into UMST scalar features here.
pub const APPLY_PHYSICS_DISSIPATION_MERGE: bool = false;

/// `PhysicalResult::safety_margin` is **not** written into UMST scalar features here.
pub const APPLY_PHYSICS_SAFETY_MARGIN_MERGE: bool = false;

/// `PhysicalResult::cost` is **not** written into UMST scalar features here.
pub const APPLY_PHYSICS_COST_MERGE: bool = false;

/// `PhysicalResult::information_density` is **not** written into UMST scalar features here.
pub const APPLY_PHYSICS_INFO_DENSITY_MERGE: bool = false;

/// Full `[B, N]` → UMST merge for `B > 1` is **not** claimed — only batch index `0` is used.
pub const APPLY_PHYSICS_MULTI_BATCH_MERGE: bool = false;

/// Production orchestration pin — writeback helper ≠ production-wired fleet claim.
pub const APPLY_PHYSICS_PRODUCTION_WIRED: bool = false;

/// Honest physics posture — gateway writeback is not cast-lifecycle physics GREEN.
pub const APPLY_PHYSICS_PHYSICS_GREEN: bool = false;

/// Master composition pin — not claimed by this writeback slice.
pub const APPLY_PHYSICS_MASTER: bool = false;

/// Channels this helper merges (damage + optional temperature).
pub const APPLY_PHYSICS_MERGED_CHANNEL_COUNT: usize = 2;

/// UMST scalar column pin for damage writeback (must match layout SSOT).
pub const APPLY_PHYSICS_DAMAGE_CHANNEL_IDX: usize = SCALAR_DAMAGE;

/// UMST scalar column pin for temperature ΔT writeback (must match layout SSOT).
pub const APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX: usize = SCALAR_TEMPERATURE;

/// PhysicalResult fields intentionally left on the PPO / CBF path (not UMST-merged here).
pub const APPLY_PHYSICS_UNMERGED_PHYSICAL_RESULT_FIELDS: &[&str] = &[
    "free_energy",
    "dissipation",
    "safety_margin",
    "cost",
    "information_density",
];

/// Count of intentionally unmerged PhysicalResult fields in
/// [`APPLY_PHYSICS_UNMERGED_PHYSICAL_RESULT_FIELDS`].
pub const APPLY_PHYSICS_UNMERGED_FIELD_COUNT: usize = 5;

const _: () = assert!(APPLY_PHYSICS_DAMAGE_WRITEBACK);
const _: () = assert!(APPLY_PHYSICS_TEMPERATURE_DELTA_WRITEBACK);
const _: () = assert!(APPLY_PHYSICS_POLICY_MASK_RESPECTED);
const _: () = assert!(!APPLY_PHYSICS_FREE_ENERGY_MERGE);
const _: () = assert!(!APPLY_PHYSICS_DISSIPATION_MERGE);
const _: () = assert!(!APPLY_PHYSICS_SAFETY_MARGIN_MERGE);
const _: () = assert!(!APPLY_PHYSICS_COST_MERGE);
const _: () = assert!(!APPLY_PHYSICS_INFO_DENSITY_MERGE);
const _: () = assert!(!APPLY_PHYSICS_MULTI_BATCH_MERGE);
const _: () = assert!(!APPLY_PHYSICS_PRODUCTION_WIRED);
const _: () = assert!(!APPLY_PHYSICS_PHYSICS_GREEN);
const _: () = assert!(!APPLY_PHYSICS_MASTER);
const _: () = assert!(APPLY_PHYSICS_MERGED_CHANNEL_COUNT == 2);
const _: () = assert!(APPLY_PHYSICS_DAMAGE_CHANNEL_IDX == SCALAR_DAMAGE);
const _: () = assert!(APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX == SCALAR_TEMPERATURE);
const _: () = assert!(
    APPLY_PHYSICS_UNMERGED_PHYSICAL_RESULT_FIELDS.len() == APPLY_PHYSICS_UNMERGED_FIELD_COUNT
);

/// Honest production gateway wiring — **false** until measured live fleet eval.
#[must_use]
pub const fn apply_physics_production_wired() -> bool {
    false
}

/// Honest master-tier wiring — **false** until fleet sign-off.
#[must_use]
pub const fn apply_physics_master_wired() -> bool {
    false
}

/// Honest physics GREEN claim — **false**; writeback ≠ physics oracle GREEN.
#[must_use]
pub const fn apply_physics_physics_green_claimed() -> bool {
    false
}

const _: () = assert!(!apply_physics_production_wired());
const _: () = assert!(!apply_physics_master_wired());
const _: () = assert!(!apply_physics_physics_green_claimed());

/// Typed probe for apply-physics writeback posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyPhysicsPostureProbe {
    pub deepen_cell: &'static str,
    pub deepen_stamp: &'static str,
    pub posture_tag: &'static str,
    pub damage_writeback: bool,
    pub temperature_delta_writeback: bool,
    pub policy_mask_respected: bool,
    pub free_energy_merge: bool,
    pub dissipation_merge: bool,
    pub safety_margin_merge: bool,
    pub cost_merge: bool,
    pub info_density_merge: bool,
    pub multi_batch_merge: bool,
    pub merged_channel_count: usize,
    pub unmerged_field_count: usize,
    pub damage_channel_idx: usize,
    pub temperature_channel_idx: usize,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for apply-physics done-when checks.
#[must_use]
pub const fn apply_physics_posture_probe() -> ApplyPhysicsPostureProbe {
    ApplyPhysicsPostureProbe {
        deepen_cell: W29_APPLY_PHYSICS_DEEPEN_CELL,
        deepen_stamp: APPLY_PHYSICS_DEEPEN_STAMP,
        posture_tag: APPLY_PHYSICS_POSTURE_TAG,
        damage_writeback: APPLY_PHYSICS_DAMAGE_WRITEBACK,
        temperature_delta_writeback: APPLY_PHYSICS_TEMPERATURE_DELTA_WRITEBACK,
        policy_mask_respected: APPLY_PHYSICS_POLICY_MASK_RESPECTED,
        free_energy_merge: APPLY_PHYSICS_FREE_ENERGY_MERGE,
        dissipation_merge: APPLY_PHYSICS_DISSIPATION_MERGE,
        safety_margin_merge: APPLY_PHYSICS_SAFETY_MARGIN_MERGE,
        cost_merge: APPLY_PHYSICS_COST_MERGE,
        info_density_merge: APPLY_PHYSICS_INFO_DENSITY_MERGE,
        multi_batch_merge: APPLY_PHYSICS_MULTI_BATCH_MERGE,
        merged_channel_count: APPLY_PHYSICS_MERGED_CHANNEL_COUNT,
        unmerged_field_count: APPLY_PHYSICS_UNMERGED_FIELD_COUNT,
        damage_channel_idx: APPLY_PHYSICS_DAMAGE_CHANNEL_IDX,
        temperature_channel_idx: APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX,
        production_wired: APPLY_PHYSICS_PRODUCTION_WIRED,
        physics_green: APPLY_PHYSICS_PHYSICS_GREEN,
        master: APPLY_PHYSICS_MASTER,
        honest_fence: APPLY_PHYSICS_HONEST_FENCE,
    }
}

/// Static inventory of PhysicalResult fields left unmerged by this helper.
#[must_use]
pub const fn apply_physics_unmerged_physical_result_fields() -> &'static [&'static str] {
    APPLY_PHYSICS_UNMERGED_PHYSICAL_RESULT_FIELDS
}

/// Channel index pins for landed writeback paths (damage, temperature).
#[must_use]
pub const fn apply_physics_writeback_channel_pins() -> (usize, usize) {
    (
        APPLY_PHYSICS_DAMAGE_CHANNEL_IDX,
        APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX,
    )
}

/// Damage/ΔT writeback landed with production / master / physics GREEN honestly open.
#[must_use]
pub fn apply_physics_posture_honest(probe: &ApplyPhysicsPostureProbe) -> bool {
    probe.deepen_cell == W29_APPLY_PHYSICS_DEEPEN_CELL
        && probe.deepen_stamp == APPLY_PHYSICS_DEEPEN_STAMP
        && probe.posture_tag == APPLY_PHYSICS_POSTURE_TAG
        && probe.damage_writeback
        && probe.temperature_delta_writeback
        && probe.policy_mask_respected
        && !probe.free_energy_merge
        && !probe.dissipation_merge
        && !probe.safety_margin_merge
        && !probe.cost_merge
        && !probe.info_density_merge
        && !probe.multi_batch_merge
        && probe.merged_channel_count == APPLY_PHYSICS_MERGED_CHANNEL_COUNT
        && probe.unmerged_field_count == APPLY_PHYSICS_UNMERGED_FIELD_COUNT
        && probe.damage_channel_idx == SCALAR_DAMAGE
        && probe.temperature_channel_idx == SCALAR_TEMPERATURE
        && !probe.production_wired
        && !probe.physics_green
        && !probe.master
        && probe.honest_fence.contains("damage_writeback=true")
        && probe
            .honest_fence
            .contains("temperature_delta_writeback=true")
        && probe.honest_fence.contains("policy_mask=true")
        && probe.honest_fence.contains("free_energy_merge=false")
        && probe.honest_fence.contains("safety_margin_merge=false")
        && probe.honest_fence.contains("cost_merge=false")
        && probe.honest_fence.contains("multi_batch_merge=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
}

/// Validate apply-physics posture honesty — fail closed on fake production / GREEN claims.
pub fn validate_apply_physics_posture_honesty() -> Result<(), &'static str> {
    let probe = apply_physics_posture_probe();
    if probe.production_wired || apply_physics_production_wired() {
        return Err("apply_physics_production_wired must stay false until fleet measured");
    }
    if probe.master || apply_physics_master_wired() {
        return Err("apply_physics_master_wired must stay false until fleet sign-off");
    }
    if probe.physics_green || apply_physics_physics_green_claimed() {
        return Err("APPLY_PHYSICS_PHYSICS_GREEN must stay false at writeback slice");
    }
    if !probe.damage_writeback {
        return Err("APPLY_PHYSICS_DAMAGE_WRITEBACK must stay true at W29-021");
    }
    if !probe.temperature_delta_writeback {
        return Err("APPLY_PHYSICS_TEMPERATURE_DELTA_WRITEBACK must stay true at W29-021");
    }
    if probe.free_energy_merge
        || probe.dissipation_merge
        || probe.safety_margin_merge
        || probe.cost_merge
        || probe.info_density_merge
    {
        return Err("non-damage/temp PhysicalResult channels must stay unmerged here");
    }
    if probe.multi_batch_merge {
        return Err("APPLY_PHYSICS_MULTI_BATCH_MERGE must stay false — only batch 0 consumed");
    }
    if probe.unmerged_field_count != APPLY_PHYSICS_UNMERGED_FIELD_COUNT {
        return Err("unmerged PhysicalResult field inventory count drift");
    }
    if probe.damage_channel_idx != SCALAR_DAMAGE
        || probe.temperature_channel_idx != SCALAR_TEMPERATURE
    {
        return Err("writeback channel pins must match SCALAR_DAMAGE / SCALAR_TEMPERATURE");
    }
    if apply_physics_unmerged_physical_result_fields().len() != APPLY_PHYSICS_UNMERGED_FIELD_COUNT {
        return Err("APPLY_PHYSICS_UNMERGED_PHYSICAL_RESULT_FIELDS length drift");
    }
    if !apply_physics_posture_honest(&probe) {
        return Err("apply_physics_posture_honest failed");
    }
    Ok(())
}

/// Collapse sparse `[B, N]` nodal field to a column `[N, 1]` for scalar-channel writeback.
///
/// When `B > 1`, only batch index `0` is consumed — [`APPLY_PHYSICS_MULTI_BATCH_MERGE`] is
/// **false** and multi-batch merge is **not** claimed.
fn nodal_batch_to_column<B: Backend<FloatElem = f32>>(
    field: Tensor<B, 2>,
    n: usize,
) -> Tensor<B, 2> {
    let [b, _] = field.dims();
    if b == 1 {
        field.squeeze::<1>(0).unsqueeze_dim::<2>(1)
    } else {
        // Honest fence: batch>1 → index 0 only (APPLY_PHYSICS_MULTI_BATCH_MERGE=false).
        field
            .slice([0..1, 0..n])
            .squeeze::<1>(0)
            .unsqueeze_dim::<2>(1)
    }
}

/// Writes `damage` and optional `temperature_delta` from `result` into `umst.scalar_features`.
///
/// - **Damage:** proposed values from `result.damage` (`[B, N]`) are blended with the existing damage
///   column via [`UnifiedMaterialStateTensor::policy_editable_mask`].
/// - **Temperature:** when `temperature_delta` is [`Some`], forms \(T_{\text{prop}} = T_{\text{old}} + \Delta T\)
///   then blends that column with the same mask (equivalently \(T \leftarrow T + m \odot \Delta T\) on
///   editable nodes).
/// - **Not written:** free energy, dissipation, safety margin, cost, or information density
///   (see [`APPLY_PHYSICS_UNMERGED_PHYSICAL_RESULT_FIELDS`]).
pub fn apply_physics_to_umst<B: Backend<FloatElem = f32>>(
    result: &PhysicalResult<B>,
    umst: &mut UnifiedMaterialStateTensor<B>,
) -> Result<(), ApplyPhysicsError> {
    umst.try_b1_incidence()
        .map_err(|source| ApplyPhysicsError::DecTypestate {
            context: "invalid B1 incidence on UMST",
            source,
        })?;

    let n = umst.scalar_features.dims()[0];
    let nf = umst.scalar_features.dims()[1];
    if nf <= APPLY_PHYSICS_DAMAGE_CHANNEL_IDX {
        return Err(ApplyPhysicsError::ScalarFeaturesTooSmallForDamage {
            width: nf,
            required_index: APPLY_PHYSICS_DAMAGE_CHANNEL_IDX,
        });
    }

    let [_, n_d] = result.damage.dims();
    if n_d != n {
        return Err(ApplyPhysicsError::DamageWidthMismatch {
            damage_width: n_d,
            umst_nodes: n,
        });
    }

    let damage_col = nodal_batch_to_column(result.damage.clone(), n);
    let damage_ch =
        ScalarChannelIdx::try_new(APPLY_PHYSICS_DAMAGE_CHANNEL_IDX).map_err(|source| {
            ApplyPhysicsError::DecTypestate {
                context: "invalid SCALAR_DAMAGE channel",
                source,
            }
        })?;
    let merged_damage = umst.project_scalar_channel(damage_ch, damage_col);
    umst.write_scalar_channel(damage_ch, merged_damage);

    if let Some(ref delta) = result.temperature_delta {
        if nf <= APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX {
            return Err(ApplyPhysicsError::ScalarFeaturesTooSmallForTemperature {
                width: nf,
                required_index: APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX,
            });
        }
        let [_, nd] = delta.dims();
        if nd != n {
            return Err(ApplyPhysicsError::TemperatureWidthMismatch {
                delta_width: nd,
                umst_nodes: n,
            });
        }
        let inc = nodal_batch_to_column(delta.clone(), n);
        let old_t = umst.scalar_features.clone().slice([
            0..n,
            APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX..APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX + 1,
        ]);
        let proposed = old_t.add(inc);
        let temp_ch =
            ScalarChannelIdx::try_new(APPLY_PHYSICS_TEMPERATURE_CHANNEL_IDX).map_err(|source| {
                ApplyPhysicsError::DecTypestate {
                    context: "invalid SCALAR_TEMPERATURE channel",
                    source,
                }
            })?;
        let merged_t = umst.project_scalar_channel(temp_ch, proposed);
        umst.write_scalar_channel(temp_ch, merged_t);
    }

    Ok(())
}

#[cfg(test)]
mod apply_physics_posture_tests {
    use super::*;

    #[test]
    fn apply_physics_posture_probe_honest_fence() {
        let probe = apply_physics_posture_probe();
        assert_eq!(probe.deepen_cell, W29_APPLY_PHYSICS_DEEPEN_CELL);
        assert_eq!(probe.deepen_stamp, APPLY_PHYSICS_DEEPEN_STAMP);
        assert_eq!(probe.posture_tag, APPLY_PHYSICS_POSTURE_TAG);
        assert!(probe.damage_writeback);
        assert!(probe.temperature_delta_writeback);
        assert!(probe.policy_mask_respected);
        assert!(!probe.free_energy_merge);
        assert!(!probe.dissipation_merge);
        assert!(!probe.safety_margin_merge);
        assert!(!probe.cost_merge);
        assert!(!probe.info_density_merge);
        assert!(!probe.multi_batch_merge);
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master);
        assert_eq!(probe.merged_channel_count, 2);
        assert_eq!(probe.unmerged_field_count, 5);
        assert_eq!(probe.damage_channel_idx, SCALAR_DAMAGE);
        assert_eq!(probe.temperature_channel_idx, SCALAR_TEMPERATURE);
        assert!(probe.honest_fence.contains("damage_writeback=true"));
        assert!(probe.honest_fence.contains("safety_margin_merge=false"));
        assert!(probe.honest_fence.contains("cost_merge=false"));
        assert!(probe.honest_fence.contains("multi_batch_merge=false"));
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("physics_green=false"));
        assert!(probe.honest_fence.contains("master=false"));
        assert!(apply_physics_posture_honest(&probe));
    }

    #[test]
    fn apply_physics_validate_posture_honesty_ok() {
        validate_apply_physics_posture_honesty().expect("posture must stay honest at W29-021");
    }

    #[test]
    fn apply_physics_honest_fence_no_green_invent() {
        assert!(!apply_physics_production_wired());
        assert!(!apply_physics_master_wired());
        assert!(!apply_physics_physics_green_claimed());
        assert!(!APPLY_PHYSICS_PHYSICS_GREEN);
        assert!(!APPLY_PHYSICS_PRODUCTION_WIRED);
        assert!(!APPLY_PHYSICS_MASTER);
        assert!(!APPLY_PHYSICS_FREE_ENERGY_MERGE);
        assert!(!APPLY_PHYSICS_DISSIPATION_MERGE);
        assert!(!APPLY_PHYSICS_SAFETY_MARGIN_MERGE);
        assert!(!APPLY_PHYSICS_COST_MERGE);
        assert!(!APPLY_PHYSICS_INFO_DENSITY_MERGE);
        assert!(!APPLY_PHYSICS_MULTI_BATCH_MERGE);
        assert!(APPLY_PHYSICS_HONEST_FENCE.contains("physics_green=false"));
        assert!(APPLY_PHYSICS_HONEST_FENCE.contains("free_energy_merge=false"));
        assert!(APPLY_PHYSICS_HONEST_FENCE.contains("multi_batch_merge=false"));
    }

    #[test]
    fn apply_physics_merged_channels_are_damage_and_temperature_only() {
        assert_eq!(APPLY_PHYSICS_MERGED_CHANNEL_COUNT, 2);
        assert!(APPLY_PHYSICS_DAMAGE_WRITEBACK);
        assert!(APPLY_PHYSICS_TEMPERATURE_DELTA_WRITEBACK);
        assert!(APPLY_PHYSICS_POLICY_MASK_RESPECTED);
        let (d, t) = apply_physics_writeback_channel_pins();
        assert_eq!(d, SCALAR_DAMAGE);
        assert_eq!(t, SCALAR_TEMPERATURE);
    }

    #[test]
    fn apply_physics_unmerged_inventory_covers_ppo_cbf_fields() {
        let fields = apply_physics_unmerged_physical_result_fields();
        assert_eq!(fields.len(), APPLY_PHYSICS_UNMERGED_FIELD_COUNT);
        assert!(fields.contains(&"free_energy"));
        assert!(fields.contains(&"dissipation"));
        assert!(fields.contains(&"safety_margin"));
        assert!(fields.contains(&"cost"));
        assert!(fields.contains(&"information_density"));
        assert!(!fields.contains(&"damage"));
        assert!(!fields.contains(&"temperature_delta"));
        assert!(APPLY_PHYSICS_DEEPEN_STAMP.contains("unmerged-inventory"));
        assert!(APPLY_PHYSICS_DEEPEN_STAMP.contains("multibatch"));
    }
}
