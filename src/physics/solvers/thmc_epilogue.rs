// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP5.1 — post-step epilogue: fracture → sync → gate → time (RW-FP-P51).
//!
//! ```text
//! thmc_post_step_epilogue
//!   ok_state → and_then fracture → and_then sync → gate witness → map advance_time
//! ```

#[cfg(feature = "thmc-coupled")]
use burn::tensor::backend::Backend;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::{Int, Tensor};

#[cfg(feature = "thmc-coupled")]
use crate::core::field::{Field, FractureEnergyField, SmallStrainField};
#[cfg(feature = "thmc-coupled")]
use crate::core::tensors::UnifiedMaterialStateTensor;
#[cfg(feature = "thmc-coupled")]
use crate::core::traits::IScienceCartridge;
#[cfg(feature = "thmc-coupled")]
use crate::physics::error::PhysicsError;
#[cfg(feature = "thmc-coupled")]
use crate::physics::pipeline::{and_then_unit, map_result, ok_state};

#[cfg(feature = "thmc-coupled")]
use super::fracture_field::{
    strain_tensor_for_fracture_from_manifold, strain_tensor_from_bar_network_displacement,
    PhaseFieldFractureSolver,
};
#[cfg(feature = "thmc-coupled")]
use super::thmc::{ThmcSolver, ThmcState};
#[cfg(feature = "thmc-coupled")]
use super::thmc_step::{ThmcSolverStep, ThmcStepGateEvidence};

#[cfg(feature = "thmc-coupled")]
#[derive(Clone)]
pub struct ThmcPostStepCtx<B: Backend> {
    pub batch: usize,
    pub n: usize,
    pub edges_b1: Tensor<B, 2, Int>,
    pub dt: f32,
}

#[cfg(feature = "thmc-coupled")]
pub fn thmc_post_step_epilogue<B, C>(
    solver: &ThmcSolver,
    cartridge: &C,
    pre_step: &ThmcState<B>,
    state: ThmcState<B>,
    manifold: &mut UnifiedMaterialStateTensor<B>,
    ctx: &ThmcPostStepCtx<B>,
) -> Result<(ThmcState<B>, ThmcStepGateEvidence), PhysicsError>
where
    B: Backend<FloatElem = f32>,
    C: IScienceCartridge<B>,
{
    let edges_b1 = ctx.edges_b1.clone();
    let state = ok_state(state)
        .and_then(|s| apply_fracture_damage(s, manifold, ctx.batch, ctx.n, edges_b1.clone()))
        .and_then(|s| and_then_unit(s, |st| crate::physics::thmc_umst_sync::sync_thmc_to_umst(st, manifold)))?;
    let gate_evidence = ThmcSolverStep::attach_gate_evidence(
        solver, cartridge, pre_step, &state, manifold, ctx.dt,
    )?;
    let state = map_result(ok_state(state), |mut s| {
        s.time += ctx.dt;
        s
    })?;
    Ok((state, gate_evidence))
}

#[cfg(feature = "thmc-coupled")]
fn apply_fracture_damage<B: Backend<FloatElem = f32>>(
    mut state: ThmcState<B>,
    manifold: &UnifiedMaterialStateTensor<B>,
    batch: usize,
    n: usize,
    edges_b1: Tensor<B, 2, Int>,
) -> Result<ThmcState<B>, PhysicsError> {
    let device = state.thermal.temperature.as_tensor().device();
    let strain_tensor = if let Some(coords_n3) = manifold.node_positions.as_ref() {
        if coords_n3.dims() == [n, 3] {
            strain_tensor_from_bar_network_displacement::<B>(
                state.mechanical.displacement.as_tensor().clone(),
                coords_n3.clone(), edges_b1.clone(), n,
            )
        } else {
            strain_tensor_for_fracture_from_manifold::<B>(manifold, batch, n, &device)
        }
    } else {
        strain_tensor_for_fracture_from_manifold::<B>(manifold, batch, n, &device)
    };
    let strain = SmallStrainField::from_tensor(strain_tensor);
    let gc = FractureEnergyField::from_tensor(Tensor::<B, 3>::ones([batch, n, 1], &device));
    let fracture = PhaseFieldFractureSolver { length_scale: 1.0 };
    let d_last = state.damage.as_tensor().dims()[2];
    let damage_core = match d_last {
        1 => state.damage.clone(),
        _ => state.damage.clone().map(|t| t.slice([0..batch, 0..n, 0..1])),
    };
    let damage_new = fracture.update_damage(strain, damage_core, gc, edges_b1)?;
    state.damage = if d_last == 1 {
        damage_new
    } else {
        let tail = state.damage.as_tensor().clone().slice([0..batch, 0..n, 1..d_last]);
        Field::new(Tensor::cat(
            vec![damage_new.as_tensor().clone(), tail],
            2,
        ))
    };
    Ok(state)
}
