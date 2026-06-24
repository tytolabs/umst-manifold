// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Merge sparse nodal [`crate::core::traits::PhysicalResult`] channels back into a live
//! [`crate::core::tensors::UnifiedMaterialStateTensor`] for topology / gateway loops.
//!
//! Uses [`UnifiedMaterialStateTensor::project_scalar_channel`] and
//! [`UnifiedMaterialStateTensor::write_scalar_channel`] so **damage** and **temperature** respect
//! [`UnifiedMaterialStateTensor::policy_editable_mask`].

use burn::tensor::backend::Backend;

use crate::core::dec_typestate::{DecTypestateError, ScalarChannelIdx};
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::PhysicalResult;
use crate::core::umst_schema::{SCALAR_DAMAGE, SCALAR_TEMPERATURE};

fn dec_typestate_err(context: &str, err: DecTypestateError) -> String {
    format!("apply_physics_to_umst: {context}: {err:?}")
}

/// Writes `damage` and optional `temperature_delta` from `result` into `umst.scalar_features`.
///
/// - **Damage:** proposed values from `result.damage` (`[B, N]`) are blended with the existing damage
///   column via [`UnifiedMaterialStateTensor::policy_editable_mask`].
/// - **Temperature:** when `temperature_delta` is [`Some`], forms \(T_{\text{prop}} = T_{\text{old}} + \Delta T\)
///   then blends that column with the same mask (equivalently \(T \leftarrow T + m \odot \Delta T\) on
///   editable nodes).
pub fn apply_physics_to_umst<B: Backend<FloatElem = f32>>(
    result: &PhysicalResult<B>,
    umst: &mut UnifiedMaterialStateTensor<B>,
) -> Result<(), String> {
    umst.try_b1_incidence()
        .map_err(|e| dec_typestate_err("invalid B1 incidence on UMST", e))?;

    let n = umst.scalar_features.dims()[0];
    let nf = umst.scalar_features.dims()[1];
    if nf <= SCALAR_DAMAGE {
        return Err(format!(
            "apply_physics_to_umst: scalar_features width {nf} too small for SCALAR_DAMAGE={SCALAR_DAMAGE}"
        ));
    }

    let [b, n_d] = result.damage.dims();
    if n_d != n {
        return Err(format!(
            "apply_physics_to_umst: damage width {n_d} != UMST nodes {n}"
        ));
    }

    let damage_col = if b == 1 {
        result.damage.clone().squeeze::<1>(0).unsqueeze_dim::<2>(1)
    } else {
        result
            .damage
            .clone()
            .slice([0..1, 0..n])
            .squeeze::<1>(0)
            .unsqueeze_dim::<2>(1)
    };
    let damage_ch = ScalarChannelIdx::try_new(SCALAR_DAMAGE)
        .map_err(|e| dec_typestate_err("invalid SCALAR_DAMAGE channel", e))?;
    let merged_damage = umst.project_scalar_channel(damage_ch, damage_col);
    umst.write_scalar_channel(damage_ch, merged_damage);

    if let Some(ref delta) = result.temperature_delta {
        if nf <= SCALAR_TEMPERATURE {
            return Err(format!(
                "apply_physics_to_umst: scalar_features width {nf} too small for SCALAR_TEMPERATURE={SCALAR_TEMPERATURE}"
            ));
        }
        let [bd, nd] = delta.dims();
        if nd != n {
            return Err(format!(
                "apply_physics_to_umst: temperature_delta width {nd} != UMST nodes {n}"
            ));
        }
        let inc = if bd == 1 {
            delta.clone().squeeze::<1>(0).unsqueeze_dim::<2>(1)
        } else {
            delta
                .clone()
                .slice([0..1, 0..n])
                .squeeze::<1>(0)
                .unsqueeze_dim::<2>(1)
        };
        let old_t = umst
            .scalar_features
            .clone()
            .slice([0..n, SCALAR_TEMPERATURE..SCALAR_TEMPERATURE + 1]);
        let proposed = old_t.add(inc);
        let temp_ch = ScalarChannelIdx::try_new(SCALAR_TEMPERATURE)
            .map_err(|e| dec_typestate_err("invalid SCALAR_TEMPERATURE channel", e))?;
        let merged_t = umst.project_scalar_channel(temp_ch, proposed);
        umst.write_scalar_channel(temp_ch, merged_t);
    }

    Ok(())
}
