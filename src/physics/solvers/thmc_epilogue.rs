// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Post-step THMC epilogue — Kleisli composition: **fracture → sync → gate** (RW-FP-P51).
//!
//! Extracted from [`super::thmc::ThmcSolver::step_experimental`] so the operator-split Newton loop
//! ends with a named epilogue morphism instead of imperative tail statements.
//!
//! ## Composition diagram
//!
//! ```text
//! ThmcState
//!   --map_state(apply_fracture_damage)-->
//! ThmcState
//!   --and_then_state(sync_thmc_to_umst; identity)-->
//! ThmcState
//!   --attach_gate_evidence-->
//! (ThmcState, ThmcStepGateEvidence)
//! ```
//!
//! **Order invariant (W4/W5):** fracture updates plan `damage` first; UMST scalar writeback mirrors
//! post-fracture fields; gate evidence lifts the synced post-step snapshot. Caller advances `time`
//! after this function returns.

use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor};

use crate::core::field::Field;
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;
use crate::physics::error::PhysicsError;
use crate::physics::pipeline::{and_then_state, map_state};
use crate::physics::solvers::fracture_field::{
    strain_tensor_for_fracture_from_manifold, strain_tensor_from_bar_network_displacement,
    PhaseFieldFractureSolver,
};
use crate::physics::thmc_umst_sync::sync_thmc_to_umst;

use super::thmc::{ThmcSolver, ThmcState};
use super::thmc_step::{ThmcSolverStep, ThmcStepGateEvidence};

/// Immutable context for the post-step epilogue (no `&mut ThmcSolver` in pure passes).
pub struct ThmcEpilogueCtx<'a, B: Backend> {
    pub batch: usize,
    pub n: usize,
    pub device: &'a <B as Backend>::Device,
    pub edges_b1: Tensor<B, 2, Int>,
}

/// Phase-field fracture pass: update `damage` from post-mechanics strain (infallible map).
fn apply_fracture_damage<B: Backend<FloatElem = f32>>(
    mut state: ThmcState<B>,
    manifold: &UnifiedMaterialStateTensor<B>,
    ctx: &ThmcEpilogueCtx<'_, B>,
) -> ThmcState<B> {
    let strain_tensor = if let Some(coords_n3) = manifold.node_positions.as_ref() {
        if coords_n3.dims() == [ctx.n, 3] {
            strain_tensor_from_bar_network_displacement::<B>(
                state.mechanical.displacement.as_tensor().clone(),
                coords_n3.clone(),
                ctx.edges_b1.clone(),
                ctx.n,
            )
        } else {
            strain_tensor_for_fracture_from_manifold::<B>(
                manifold,
                ctx.batch,
                ctx.n,
                ctx.device,
            )
        }
    } else {
        strain_tensor_for_fracture_from_manifold::<B>(manifold, ctx.batch, ctx.n, ctx.device)
    };
    let strain = crate::core::field::SmallStrainField::from_tensor(strain_tensor);
    let gc = crate::core::field::FractureEnergyField::from_tensor(Tensor::<B, 3>::ones(
        [ctx.batch, ctx.n, 1],
        ctx.device,
    ));
    let fracture = PhaseFieldFractureSolver { length_scale: 1.0 };

    let d_last = state.damage.as_tensor().dims()[2];
    let damage_core = match d_last {
        1 => state.damage.clone(),
        _ => state
            .damage
            .clone()
            .map(|t| t.slice([0..ctx.batch, 0..ctx.n, 0..1])),
    };
    let damage_new = fracture.update_damage(strain, damage_core, gc, ctx.edges_b1.clone());

    state.damage = if d_last == 1 {
        damage_new
    } else {
        let tail = state
            .damage
            .as_tensor()
            .clone()
            .slice([0..ctx.batch, 0..ctx.n, 1..d_last]);
        damage_new.map(|core| Tensor::cat(vec![core, tail], 2))
    };
    state
}

/// Post-step epilogue: **fracture → sync → gate** via [`map_state`] / [`and_then_state`].
pub fn thmc_post_step_epilogue<B, C>(
    solver: &ThmcSolver,
    cartridge: &C,
    pre_step: &ThmcState<B>,
    state: ThmcState<B>,
    manifold: &mut UnifiedMaterialStateTensor<B>,
    ctx: &ThmcEpilogueCtx<'_, B>,
) -> Result<(ThmcState<B>, ThmcStepGateEvidence), PhysicsError>
where
    B: Backend<FloatElem = f32>,
    C: IScienceCartridge<B>,
{
    let state = map_state(state, |s| apply_fracture_damage(s, manifold, ctx))?;
    let state = and_then_state(state, |s| {
        sync_thmc_to_umst(&s, manifold)?;
        Ok(s)
    })?;
    let gate_evidence = ThmcSolverStep::attach_gate_evidence(
        solver,
        cartridge,
        pre_step,
        &state,
        manifold,
        solver.dt,
    )?;
    Ok((state, gate_evidence))
}
