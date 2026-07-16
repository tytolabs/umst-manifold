// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! THMC ↔ UMST reverse writeback morphism (FP P3.5 / RW-FP-P35).
//!
//! Closes **W4**: after [`crate::physics::solvers::ThmcSolver::step`], nodal plan fields are
//! mirrored into [`crate::core::tensors::UnifiedMaterialStateTensor::scalar_features`] so gate
//! evidence and UMST columns reference the same post-step state (**W5**).
//!
//! Distinct from [`crate::core::apply_physics_to_umst`] (gateway ΔT additive path).

use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

use crate::core::dec_typestate::{DecTypestateError, ScalarChannelIdx};
use crate::core::field::{DamageField, HumidityField, TemperatureField};
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::umst_schema::{SCALAR_DAMAGE, SCALAR_HUMIDITY, SCALAR_TEMPERATURE};
use crate::physics::error::PhysicsError;
use crate::physics::solvers::ThmcState;

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
    Ok(field
        .clone()
        .slice([0..1, 0..n, 0..1])
        .reshape([n, 1]))
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

    let t_col = plan_field_nodal_col(
        temperature.as_tensor(),
        "sync_thmc_to_umst: temperature",
        n,
    )?;
    let h_col = plan_field_nodal_col(
        humidity.as_tensor(),
        "sync_thmc_to_umst: humidity",
        n,
    )?;
    let d_col = plan_field_nodal_col(
        damage.as_tensor(),
        "sync_thmc_to_umst: damage",
        n,
    )?;

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
