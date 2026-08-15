// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! THMC ↔ UMST reverse writeback morphism (FP P3.5 / RW-FP-P35).
//!
//! Closes **W4**: after [`crate::physics::solvers::ThmcSolver::step`], nodal plan fields are
//! mirrored into [`crate::core::tensors::UnifiedMaterialStateTensor::scalar_features`] so gate
//! evidence and UMST columns reference the same post-step state (**W5**).
//!
//! Distinct from [`crate::core::apply_physics_to_umst`] (gateway ΔT additive path).
//!
//! # Honest boundary (W29-090)
//!
//! Absolute nodal writeback + batch/shape fences are landed. Not physics GREEN, not
//! `PRODUCTION_WIRED`, not `MASTER`, not OP-5.

use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

use crate::core::dec_typestate::{DecTypestateError, ScalarChannelIdx};
use crate::core::field::{DamageField, HumidityField, TemperatureField};
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::umst_schema::{SCALAR_DAMAGE, SCALAR_HUMIDITY, SCALAR_TEMPERATURE};
use crate::physics::error::PhysicsError;
use crate::physics::solvers::ThmcState;

// ---------------------------------------------------------------------------
// W29-090 deepen — honest fences (no invent GREEN / PRODUCTION_WIRED / MASTER / OP-5)
// ---------------------------------------------------------------------------

/// Swarm cell id for this deepen (W29-090).
pub const W29_090_CELL_ID: &str = "W29-090-THMC_UMST_SYNC";

/// Honest posture — THMC→UMST absolute writeback deepen only; no ceremony invent.
pub const W29_090_HONEST_POSTURE: &str = "THMC_UMST_SYNC_DEEPEN_ONLY";

/// Explicit non-claims (gate text).
pub const W29_090_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// Deepen schema version pin.
pub const W29_090_DEEPEN_SCHEMA_VERSION: &str = "thmc_umst_sync_w29_090_deepen_v1";

/// Absolute temperature sync path is the only supported write mode.
pub const THMC_UMST_SYNC_ABSOLUTE_LANDED: bool = true;

/// ΔT additive mode remains reserved for the gateway path — not landed here.
pub const THMC_UMST_SYNC_DELTA_ADDITIVE_LANDED: bool = false;

/// Honest physics posture — unit fences pass; does not certify fleet physics GREEN.
pub const THMC_UMST_SYNC_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by reverse writeback deepen alone.
pub const THMC_UMST_SYNC_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const THMC_UMST_SYNC_MASTER: bool = false;

/// OP-5 ceremony claim — not claimed by this deepen.
pub const THMC_UMST_SYNC_OP5_PASS: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const THMC_UMST_SYNC_HONEST_FENCE: &str = concat!(
    "absolute_sync_landed=true delta_additive_landed=false ",
    "production_wired=false physics_green=false master=false op5_pass=false"
);

/// Compile-time fence — production/master/physics GREEN / OP-5 flip not authorized.
const _: () = assert!(!THMC_UMST_SYNC_PHYSICS_GREEN);
const _: () = assert!(!THMC_UMST_SYNC_PRODUCTION_WIRED);
const _: () = assert!(!THMC_UMST_SYNC_MASTER);
const _: () = assert!(!THMC_UMST_SYNC_OP5_PASS);
const _: () = assert!(!THMC_UMST_SYNC_DELTA_ADDITIVE_LANDED);
const _: () = assert!(THMC_UMST_SYNC_ABSOLUTE_LANDED);

/// Honest fence flags for the THMC↔UMST sync deepen (W29-090).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThmcUmstSyncW29090DeepenProbe {
    pub schema_version: &'static str,
    pub cell_id: &'static str,
    pub honest_posture: &'static str,
    pub non_claim: &'static str,
    pub absolute_sync_landed: bool,
    pub delta_additive_landed: bool,
    pub production_wired_claimed: bool,
    pub green_claimed: bool,
    pub op5_pass_claimed: bool,
    pub master_retick_claimed: bool,
    pub deepen_honest: bool,
}

/// Build the W29-090 THMC↔UMST sync deepen honesty probe.
#[must_use]
pub fn thmc_umst_sync_w29_090_deepen_probe() -> ThmcUmstSyncW29090DeepenProbe {
    let production_wired_claimed = THMC_UMST_SYNC_PRODUCTION_WIRED;
    let green_claimed = THMC_UMST_SYNC_PHYSICS_GREEN;
    let op5_pass_claimed = THMC_UMST_SYNC_OP5_PASS;
    let master_retick_claimed = THMC_UMST_SYNC_MASTER;

    let deepen_honest = W29_090_CELL_ID == "W29-090-THMC_UMST_SYNC"
        && W29_090_DEEPEN_SCHEMA_VERSION == "thmc_umst_sync_w29_090_deepen_v1"
        && W29_090_HONEST_POSTURE == "THMC_UMST_SYNC_DEEPEN_ONLY"
        && THMC_UMST_SYNC_ABSOLUTE_LANDED
        && !THMC_UMST_SYNC_DELTA_ADDITIVE_LANDED
        && !production_wired_claimed
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && THMC_UMST_SYNC_HONEST_FENCE.contains("absolute_sync_landed=true")
        && THMC_UMST_SYNC_HONEST_FENCE.contains("production_wired=false")
        && THMC_UMST_SYNC_HONEST_FENCE.contains("physics_green=false")
        && W29_090_NON_CLAIM.contains("not GREEN")
        && W29_090_NON_CLAIM.contains("not OP-5 PASS")
        && W29_090_NON_CLAIM.contains("not production_wired")
        && W29_090_NON_CLAIM.contains("not MASTER_RETICK");

    ThmcUmstSyncW29090DeepenProbe {
        schema_version: W29_090_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_090_CELL_ID,
        honest_posture: W29_090_HONEST_POSTURE,
        non_claim: W29_090_NON_CLAIM,
        absolute_sync_landed: THMC_UMST_SYNC_ABSOLUTE_LANDED,
        delta_additive_landed: THMC_UMST_SYNC_DELTA_ADDITIVE_LANDED,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-090 deepen honesty probe passes.
#[must_use]
pub fn thmc_umst_sync_w29_090_deepen_honest() -> bool {
    thmc_umst_sync_w29_090_deepen_probe().deepen_honest
}

/// Module fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn thmc_umst_sync_honest_fence_holds() -> bool {
    let p = thmc_umst_sync_w29_090_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
}

/// Invent-claim: production wire — always **false** at this deepen.
#[must_use]
pub const fn thmc_umst_sync_production_wired() -> bool {
    THMC_UMST_SYNC_PRODUCTION_WIRED
}

/// Invent-claim: physics/fleet GREEN — always **false** at this deepen.
#[must_use]
pub const fn thmc_umst_sync_physics_green() -> bool {
    THMC_UMST_SYNC_PHYSICS_GREEN
}

fn dec_typestate_err(context: &str, err: DecTypestateError) -> PhysicsError {
    PhysicsError::Domain {
        detail: format!("sync_thmc_to_umst: {context}: {err:?}"),
    }
}

/// Temperature write policy at the THMC ↔ UMST boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemperatureSyncMode {
    /// Overwrite `SCALAR_TEMPERATURE` with nodal T from [`ThmcState`] (THMC post-step sync).
    Absolute,
    /// ΔT additive — reserved for [`crate::core::apply_physics_to_umst`] gateway path (W3).
    DeltaAdditive,
}

/// Extract batch-0 channel-0 nodal column `[N, 1]` from a rank-3 THMC plan field.
fn plan_field_nodal_col<B: Backend<FloatElem = f32>>(
    field: &Tensor<B, 3>,
    context: &'static str,
    n_umst: usize,
) -> Result<Tensor<B, 2>, PhysicsError> {
    let [batch, n, _f] = field.dims();
    if batch > 1 {
        return Err(PhysicsError::InvariantViolation {
            context: "sync_thmc_to_umst: batch size > 1 rejected (W1)",
        });
    }
    if n != n_umst {
        return Err(PhysicsError::ShapeMismatch {
            context,
            detail: "plan field N != UMST nodes",
        });
    }
    Ok(field.clone().slice([0..1, 0..n, 0..1]).reshape([n, 1]))
}

/// Write typed THMC plan fields into UMST scalar columns (absolute T, humidity, damage).
pub fn sync_thmc_fields_to_umst<B: Backend<FloatElem = f32>>(
    temperature: &TemperatureField<B>,
    humidity: &HumidityField<B>,
    damage: &DamageField<B>,
    umst: &mut UnifiedMaterialStateTensor<B>,
    temperature_mode: TemperatureSyncMode,
) -> Result<(), PhysicsError> {
    if temperature_mode != TemperatureSyncMode::Absolute {
        return Err(PhysicsError::InvariantViolation {
            context: "sync_thmc_fields_to_umst: only TemperatureSyncMode::Absolute is supported",
        });
    }

    umst.try_b1_incidence()
        .map_err(|e| dec_typestate_err("invalid B1 incidence on UMST", e))?;

    let n = umst.scalar_features.dims()[0];
    let nf = umst.scalar_features.dims()[1];

    if nf <= SCALAR_DAMAGE {
        return Err(PhysicsError::Domain {
            detail: format!(
                "sync_thmc_to_umst: scalar_features width {nf} too small for SCALAR_DAMAGE={SCALAR_DAMAGE}"
            ),
        });
    }
    if nf <= SCALAR_HUMIDITY {
        return Err(PhysicsError::Domain {
            detail: format!(
                "sync_thmc_to_umst: scalar_features width {nf} too small for SCALAR_HUMIDITY={SCALAR_HUMIDITY}"
            ),
        });
    }
    if nf <= SCALAR_TEMPERATURE {
        return Err(PhysicsError::Domain {
            detail: format!(
                "sync_thmc_to_umst: scalar_features width {nf} too small for SCALAR_TEMPERATURE={SCALAR_TEMPERATURE}"
            ),
        });
    }

    let t_col = plan_field_nodal_col(temperature.as_tensor(), "sync_thmc_to_umst: temperature", n)?;
    let h_col = plan_field_nodal_col(humidity.as_tensor(), "sync_thmc_to_umst: humidity", n)?;
    let d_col = plan_field_nodal_col(damage.as_tensor(), "sync_thmc_to_umst: damage", n)?;

    let temp_ch = ScalarChannelIdx::try_new(SCALAR_TEMPERATURE)
        .map_err(|e| dec_typestate_err("invalid SCALAR_TEMPERATURE channel", e))?;
    let hum_ch = ScalarChannelIdx::try_new(SCALAR_HUMIDITY)
        .map_err(|e| dec_typestate_err("invalid SCALAR_HUMIDITY channel", e))?;
    let damage_ch = ScalarChannelIdx::try_new(SCALAR_DAMAGE)
        .map_err(|e| dec_typestate_err("invalid SCALAR_DAMAGE channel", e))?;

    umst.write_scalar_channel(temp_ch, t_col);
    umst.write_scalar_channel(hum_ch, h_col);
    umst.write_scalar_channel(damage_ch, d_col);

    Ok(())
}

/// Convenience wrapper: sync all scalar channels from a post-step [`ThmcState`].
pub fn sync_thmc_to_umst<B: Backend<FloatElem = f32>>(
    state: &ThmcState<B>,
    umst: &mut UnifiedMaterialStateTensor<B>,
) -> Result<(), PhysicsError> {
    sync_thmc_fields_to_umst(
        &state.thermal.temperature,
        &state.hydro.humidity,
        &state.damage,
        umst,
        TemperatureSyncMode::Absolute,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::field::Field;
    use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
    use crate::physics::error::PhysicsError;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    fn device() -> <B as Backend>::Device {
        Default::default()
    }

    fn toy_umst(n: usize, nf: usize) -> UnifiedMaterialStateTensor<B> {
        let dev = device();
        let scalars = Tensor::<B, 2>::zeros([n, nf], &dev);
        let coords: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
            &dev,
        );
        let faces_b2: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
        UnifiedMaterialStateTensor {
            coords,
            edges_b1,
            faces_b2,
            scalar_features: scalars,
            vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &dev),
            matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev),
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions: None,
            displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
            policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    fn plan_fields(
        batch: usize,
        n: usize,
        t: f32,
        h: f32,
        d: f32,
    ) -> (TemperatureField<B>, HumidityField<B>, DamageField<B>) {
        let dev = device();
        (
            Field::new(Tensor::<B, 3>::full([batch, n, 1], t, &dev)),
            Field::new(Tensor::<B, 3>::full([batch, n, 1], h, &dev)),
            Field::new(Tensor::<B, 3>::full([batch, n, 1], d, &dev)),
        )
    }

    #[test]
    fn thmc_umst_sync_w29_090_deepen_honest_probe() {
        let probe = thmc_umst_sync_w29_090_deepen_probe();
        assert_eq!(probe.cell_id, W29_090_CELL_ID);
        assert_eq!(probe.schema_version, W29_090_DEEPEN_SCHEMA_VERSION);
        assert_eq!(probe.honest_posture, W29_090_HONEST_POSTURE);
        assert!(probe.absolute_sync_landed);
        assert!(!probe.delta_additive_landed);
        assert!(!probe.green_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(thmc_umst_sync_w29_090_deepen_honest());
        assert!(thmc_umst_sync_honest_fence_holds());
        assert!(!thmc_umst_sync_production_wired());
        assert!(!thmc_umst_sync_physics_green());
    }

    #[test]
    fn thmc_umst_sync_non_claim_text_covers_forbidden_invent() {
        for needle in [
            "not GREEN",
            "not OP-5 PASS",
            "not production_wired",
            "not MASTER_RETICK",
        ] {
            assert!(
                W29_090_NON_CLAIM.contains(needle),
                "missing non-claim fragment: {needle}"
            );
        }
    }

    #[test]
    fn thmc_umst_sync_absolute_writes_scalar_channels() {
        let n = 2usize;
        let mut umst = toy_umst(n, UMST_SCALAR_CHANNEL_COUNT);
        let (t, h, d) = plan_fields(1, n, 305.0, 0.55, 0.2);
        sync_thmc_fields_to_umst(&t, &h, &d, &mut umst, TemperatureSyncMode::Absolute)
            .expect("sync_thmc_fields_to_umst Absolute on 2-node toy UMST (W29-090 deepen fence)");
        let out = umst.scalar_features.clone().into_data().value;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        assert!((out[SCALAR_TEMPERATURE] - 305.0).abs() < 1e-5);
        assert!((out[SCALAR_HUMIDITY] - 0.55).abs() < 1e-5);
        assert!((out[SCALAR_DAMAGE] - 0.2).abs() < 1e-5);
        assert!((out[f + SCALAR_TEMPERATURE] - 305.0).abs() < 1e-5);
        assert!((out[f + SCALAR_HUMIDITY] - 0.55).abs() < 1e-5);
        assert!((out[f + SCALAR_DAMAGE] - 0.2).abs() < 1e-5);
    }

    #[test]
    fn thmc_umst_sync_rejects_delta_additive_mode() {
        let n = 2usize;
        let mut umst = toy_umst(n, UMST_SCALAR_CHANNEL_COUNT);
        let (t, h, d) = plan_fields(1, n, 300.0, 0.5, 0.1);
        let err =
            sync_thmc_fields_to_umst(&t, &h, &d, &mut umst, TemperatureSyncMode::DeltaAdditive)
                .unwrap_err();
        assert!(matches!(err, PhysicsError::InvariantViolation { .. }));
    }

    #[test]
    fn thmc_umst_sync_rejects_batch_gt_one() {
        let n = 2usize;
        let mut umst = toy_umst(n, UMST_SCALAR_CHANNEL_COUNT);
        let (t, h, d) = plan_fields(2, n, 310.0, 0.6, 0.15);
        let err = sync_thmc_fields_to_umst(&t, &h, &d, &mut umst, TemperatureSyncMode::Absolute)
            .unwrap_err();
        assert!(matches!(err, PhysicsError::InvariantViolation { .. }));
    }

    #[test]
    fn thmc_umst_sync_rejects_node_count_mismatch() {
        let mut umst = toy_umst(2, UMST_SCALAR_CHANNEL_COUNT);
        let (t, h, d) = plan_fields(1, 3, 300.0, 0.5, 0.1);
        let err = sync_thmc_fields_to_umst(&t, &h, &d, &mut umst, TemperatureSyncMode::Absolute)
            .unwrap_err();
        assert!(matches!(err, PhysicsError::ShapeMismatch { .. }));
    }

    #[test]
    fn thmc_umst_sync_rejects_narrow_scalar_width() {
        // Width must exceed SCALAR_DAMAGE; truncate to fail the fence honestly.
        let narrow = SCALAR_DAMAGE; // nf <= SCALAR_DAMAGE → Domain
        let mut umst = toy_umst(2, narrow);
        let (t, h, d) = plan_fields(1, 2, 300.0, 0.5, 0.1);
        let err = sync_thmc_fields_to_umst(&t, &h, &d, &mut umst, TemperatureSyncMode::Absolute)
            .unwrap_err();
        assert!(matches!(err, PhysicsError::Domain { .. }));
    }

    #[test]
    fn thmc_umst_sync_idempotent_via_thmc_state() {
        let n = 2usize;
        let mut umst = toy_umst(n, UMST_SCALAR_CHANNEL_COUNT);
        let state = ThmcState::from_tensors(
            Tensor::<B, 3>::full([1, n, 1], 310.0, &device()),
            Tensor::<B, 3>::full([1, n, 1], 0.6, &device()),
            Tensor::<B, 3>::zeros([1, n, 3], &device()),
            Tensor::<B, 3>::zeros([1, n, 1], &device()),
            Tensor::<B, 3>::full([1, n, 1], 0.2, &device()),
            0.0,
        );
        sync_thmc_to_umst(&state, &mut umst)
            .expect("sync_thmc_to_umst first pass (W29-090 deepen idempotency baseline)");
        let snap = umst.scalar_features.clone().into_data().value;
        sync_thmc_to_umst(&state, &mut umst)
            .expect("sync_thmc_to_umst re-application (W29-090 deepen idempotency witness)");
        let again = umst.scalar_features.clone().into_data().value;
        assert_eq!(snap, again);
    }
}
