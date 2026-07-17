// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Pure operator-split sub-passes for [`super::thmc::ThmcSolver::step_experimental`] (RW-FP-P52).
//!
//! Each morphism is `ThmcState → Result<ThmcState>` with immutable [`ThmcStepCtx`]. Inner CG / dense
//! Newton hosts remain imperative per [`crate::physics::pipeline`] exemption table — functor-wrapping the
//! outer Newton loop is RW-FP-P53.

use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor};

use crate::core::field::{
    Field, HumidityField, ReactionExtentField, StepEntryDamageMask, TemperatureField,
};
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::physics::error::PhysicsError;
use crate::physics::laplacian::TopologicalLaplacian;
use crate::physics::mechanics::VectorMechanicsSolver;
use crate::physics::solvers::thmc_residual::{
    ThmcImplicitEulerThermalHumidityReactionExtentResidual,
    ThmcImplicitEulerThermalReactionExtentResidual, THMC_DENSE_NEWTON_MAX_STACKED_DOFS,
};
use crate::physics::time_orchestration::MechanicsInnerLoopConfig;

use super::thmc::{
    reaction_extent_rate_field, ChemicalPlan, HydrologicPlan, MechanicalPlan, ReactionExtentKinetics,
    ThmcImplicitTAlphaNewtonConfig, ThmcMonolithicNewtonConfig, ThmcSolver, ThmcState, ThermalPlan,
};

/// Immutable per-step context shared by operator-split passes (no `&mut ThmcSolver`).
pub struct ThmcStepCtx<'a, B: Backend> {
    pub solver: &'a ThmcSolver,
    pub manifold: &'a UnifiedMaterialStateTensor<B>,
    pub dt: f32,
    pub batch: usize,
    pub n: usize,
    pub device: &'a <B as Backend>::Device,
    pub edges_b1: Tensor<B, 2, Int>,
    pub damage_m: StepEntryDamageMask<B>,
    pub reaction_extent_kinetics: &'a ReactionExtentKinetics,
    pub drying_last_node_evaporation_k: f32,
    pub drying_ambient_h: f32,
}

/// Per-outer-pass transport inputs frozen at pass entry (Laplacians use step-entry damage mask).
pub struct ThmcTransportPrecompute<B: Backend> {
    pub t_old: TemperatureField<B>,
    pub h_old: HumidityField<B>,
    pub t_old_t: Tensor<B, 3>,
    pub h_old_t: Tensor<B, 3>,
    pub dt_lap_t: Tensor<B, 3>,
    pub dt_lap_h: Tensor<B, 3>,
    pub d_alpha: ReactionExtentField<B>,
    pub exo: Tensor<B, 3>,
    pub alpha_n: ReactionExtentField<B>,
    pub alpha_n_t: Tensor<B, 3>,
    pub d_alpha_t: Tensor<B, 3>,
}

pub fn precompute_transport_inputs<B: Backend<FloatElem = f32>>(
    state: &ThmcState<B>,
    ctx: &ThmcStepCtx<'_, B>,
) -> ThmcTransportPrecompute<B> {
    let t_old = state.thermal.temperature.clone();
    let h_old = state.hydro.humidity.clone();
    let t_old_t = t_old.as_tensor().clone();
    let h_old_t = h_old.as_tensor().clone();

    let lap_t = TopologicalLaplacian::scalar_laplacian_temperature(
        &t_old,
        &ctx.damage_m,
        ctx.edges_b1.clone(),
    );
    let lap_h = TopologicalLaplacian::scalar_laplacian_humidity(
        &h_old,
        &ctx.damage_m,
        ctx.edges_b1.clone(),
    );
    let dt_lap_t = lap_t.as_tensor().clone().mul_scalar(ctx.dt);
    let dt_lap_h = lap_h.as_tensor().clone().mul_scalar(ctx.dt);

    let batch = ctx.batch;
    let n = ctx.n;
    let f_alpha_ch = state.chemical.reaction_extent.as_tensor().dims()[2];
    let t_bn1 = t_old_t.clone().slice([0..batch, 0..n, 0..1]);
    let temperature_for_alpha = Field::new(if f_alpha_ch == 1 {
        t_bn1
    } else {
        t_bn1.expand::<3, _>([batch, n, f_alpha_ch])
    });
    let d_alpha = reaction_extent_rate_field(
        ctx.reaction_extent_kinetics,
        &state.chemical.reaction_extent,
        &temperature_for_alpha,
        ctx.device,
    );

    let f_t_ch = state.thermal.temperature.as_tensor().dims()[2];
    let exo = d_alpha
        .as_tensor()
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .mul_scalar(
            ctx.reaction_extent_kinetics.exothermic_k_per_alpha_rate * ctx.dt,
        )
        .expand::<3, _>([batch, n, f_t_ch]);

    let alpha_n = state.chemical.reaction_extent.clone();
    ThmcTransportPrecompute {
        t_old,
        h_old,
        t_old_t,
        h_old_t,
        dt_lap_t,
        dt_lap_h,
        d_alpha,
        exo,
        alpha_n: alpha_n.clone(),
        alpha_n_t: alpha_n.as_tensor().clone(),
        d_alpha_t: d_alpha.as_tensor().clone(),
    }
}

/// Stacked transport residual \(\|R\|_2\) and nodal \(|R_T|+|R_h|\) tensor (dedupes T-5 blocks).
pub fn transport_residual_l2<B: Backend<FloatElem = f32>>(
    state: &ThmcState<B>,
    pre: &ThmcTransportPrecompute<B>,
) -> (f32, Tensor<B, 3>)
where
    B::FloatElem: num_traits::float::FloatCore,
{
    let r_t = state
        .thermal
        .temperature
        .as_tensor()
        .clone()
        .sub(pre.t_old_t.clone())
        .sub(pre.dt_lap_t.clone())
        .abs();
    let r_h = state
        .hydro
        .humidity
        .as_tensor()
        .clone()
        .sub(pre.h_old_t.clone())
        .sub(pre.dt_lap_h.clone())
        .abs();
    let total_residual_tensor = r_t.add(r_h);
    let l2 = total_residual_tensor
        .clone()
        .powf_scalar(2.0)
        .sum()
        .sqrt()
        .into_scalar()
        .elem::<f32>();
    (l2, total_residual_tensor)
}

pub fn thermal_chemistry_pass<B: Backend<FloatElem = f32>>(
    mut state: ThmcState<B>,
    ctx: &ThmcStepCtx<'_, B>,
    pre: &ThmcTransportPrecompute<B>,
) -> Result<ThmcState<B>, PhysicsError> {
    state.thermal.temperature = Field::new(
        pre.t_old_t.clone().add(pre.dt_lap_t.clone()).add(pre.exo.clone()),
    );
    state.chemical.reaction_extent = Field::new(
        pre.alpha_n_t
            .clone()
            .add(pre.d_alpha_t.clone().mul_scalar(ctx.dt))
            .clamp(0.0_f32, 1.0_f32),
    );
    Ok(state)
}

pub fn monolithic_pass<B: Backend<FloatElem = f32>>(
    mut state: ThmcState<B>,
    ctx: &ThmcStepCtx<'_, B>,
    pre: &ThmcTransportPrecompute<B>,
    mc: &ThmcMonolithicNewtonConfig,
) -> Result<ThmcState<B>, PhysicsError> {
    let batch = ctx.batch;
    let n = ctx.n;
    let device = ctx.device;

    let coords_n3 = ctx
        .manifold
        .node_positions
        .as_ref()
        .filter(|p| p.dims() == [n, 3])
        .ok_or_else(|| {
            "ThmcSolver::step: monolithic_thmc_newton requires manifold.node_positions with shape [N,3]".to_string()
        })?;
    let bm_core = displacement_bc_mask_core(&ctx.manifold.displacement_bc_mask, n)?;
    let bm = bm_core.unsqueeze_dim::<3>(0).expand::<3, _>([batch, n, 3]);
    let bf = Field::new(Tensor::<B, 3>::zeros([batch, n, 3], device));
    let inner_cfg = MechanicsInnerLoopConfig::default();
    let cross_section_area = 0.01_f32;

    let t_predict = pre.t_old_t.clone().add(pre.dt_lap_t.clone()).add(pre.exo.clone());
    let h_predict = pre.h_old_t.clone().add(pre.dt_lap_h.clone());
    let alpha_predict = pre
        .alpha_n_t
        .clone()
        .add(pre.d_alpha_t.clone().mul_scalar(ctx.dt))
        .clamp(0.0_f32, 1.0_f32);

    let alpha_bn1_pred = alpha_predict
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .clamp(1e-6_f32, 1.0_f32);
    let stiffness_e =
        alpha_bn1_pred.mul_scalar(ctx.reaction_extent_kinetics.stiffness_e_scale_pa);
    let stiffness_nu = Tensor::<B, 3>::zeros([batch, n, 1], device)
        .add_scalar(ctx.reaction_extent_kinetics.stiffness_nu);
    let stiffness = Tensor::cat(vec![stiffness_e, stiffness_nu], 2);
    let (u_predict, _) = VectorMechanicsSolver::solve_equilibrium_typed(
        state.mechanical.displacement.clone(),
        coords_n3.clone(),
        stiffness,
        bf.clone(),
        ctx.edges_b1.clone(),
        ctx.damage_m.as_damage_field().clone(),
        bm.clone(),
        cross_section_area,
        &inner_cfg,
    )?;

    let trial = ThmcState {
        thermal: ThermalPlan::from_temperature(t_predict),
        hydro: HydrologicPlan::from_humidity(h_predict),
        mechanical: MechanicalPlan::from_displacement(u_predict.into_tensor()),
        chemical: ChemicalPlan::from_reaction_extent(alpha_predict),
        damage: state.damage.clone(),
        time: state.time,
    };

    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt: ctx.dt,
        temperature_n: pre.t_old.clone(),
        humidity_n: pre.h_old.clone(),
        alpha_n: pre.alpha_n.clone(),
        displacement_n: state.mechanical.displacement.as_tensor().clone(),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1: ctx.edges_b1.clone(),
        damage_m: ctx.damage_m.clone(),
        kinetics: ctx.reaction_extent_kinetics.clone(),
    };

    let (updated, _) = assembler.damped_newton_iterations_with_quasi_static_r_u(
        &trial,
        coords_n3,
        &bm,
        bf.as_tensor(),
        cross_section_area,
        mc.iterations,
        mc.damping,
        mc.fd_eps,
        mc.stacked_residual_l2_tolerance,
        mc.stacked_residual_relative_to_initial,
    )?;

    state.thermal.temperature = updated.thermal.temperature;
    state.hydro.humidity = updated.hydro.humidity;
    state.chemical.reaction_extent = updated.chemical.reaction_extent;
    state.mechanical.displacement = updated.mechanical.displacement;
    Ok(state)
}

pub fn implicit_t_alpha_pass<B: Backend<FloatElem = f32>>(
    mut state: ThmcState<B>,
    ctx: &ThmcStepCtx<'_, B>,
    pre: &ThmcTransportPrecompute<B>,
    im_cfg: &ThmcImplicitTAlphaNewtonConfig,
) -> Result<ThmcState<B>, PhysicsError> {
    let batch = ctx.batch;
    let n = ctx.n;
    let f_alpha_ch = state.chemical.reaction_extent.as_tensor().dims()[2];

    if batch != 1 {
        return Err(format!(
            "ThmcSolver::step: implicit (T,α) Newton requires batch size 1, got {batch}"
        ).into());
    }
    if im_cfg.iterations < 2 {
        return Err(
            "ThmcSolver::step: implicit_t_alpha_newton.iterations must be >= 2".into(),
        );
    }
    let f_t_dof = state.thermal.temperature.as_tensor().dims()[2];
    let stacked = n * f_t_dof + n * f_alpha_ch;
    if stacked > THMC_DENSE_NEWTON_MAX_STACKED_DOFS {
        let cap = THMC_DENSE_NEWTON_MAX_STACKED_DOFS;
        return Err(format!(
            "ThmcSolver::step: implicit (T,α) Newton exceeds dense-Jacobian cap ({cap} DOFs), got {stacked}",
        ).into());
    }

    let t_predict = pre.t_old_t.clone().add(pre.dt_lap_t.clone()).add(pre.exo.clone());
    let alpha_predict = pre
        .alpha_n_t
        .clone()
        .add(pre.d_alpha_t.clone().mul_scalar(ctx.dt))
        .clamp(0.0_f32, 1.0_f32);

    let trial = ThmcState {
        thermal: ThermalPlan::from_temperature(t_predict),
        hydro: HydrologicPlan {
            humidity: state.hydro.humidity.clone(),
        },
        mechanical: MechanicalPlan {
            displacement: state.mechanical.displacement.clone(),
        },
        chemical: ChemicalPlan::from_reaction_extent(alpha_predict),
        damage: state.damage.clone(),
        time: state.time,
    };

    let assembler = ThmcImplicitEulerThermalReactionExtentResidual {
        dt: ctx.dt,
        temperature_n: pre.t_old.clone(),
        alpha_n: pre.alpha_n.clone(),
        edges_b1: ctx.edges_b1.clone(),
        damage_m: ctx.damage_m.clone(),
        kinetics: ctx.reaction_extent_kinetics.clone(),
    };

    let (updated, _) = assembler.damped_newton_iterations(
        &trial,
        im_cfg.iterations,
        im_cfg.damping,
        im_cfg.fd_eps,
    )?;

    state.thermal.temperature = updated.thermal.temperature;
    state.chemical.reaction_extent = updated.chemical.reaction_extent;
    Ok(state)
}

pub fn humidity_pass<B: Backend<FloatElem = f32>>(
    mut state: ThmcState<B>,
    ctx: &ThmcStepCtx<'_, B>,
    pre: &ThmcTransportPrecompute<B>,
) -> Result<ThmcState<B>, PhysicsError> {
    let batch = ctx.batch;
    let n = ctx.n;
    let f_h = state.hydro.humidity.as_tensor().dims()[2];
    let mut h_new = pre.h_old_t.clone().add(pre.dt_lap_h.clone());
    if ctx.drying_last_node_evaporation_k > 0.0_f32 && n > 1 {
        let tail = h_new.clone().slice([0..batch, (n - 1)..n, 0..1]);
        let delta = tail
            .clone()
            .sub_scalar(ctx.drying_ambient_h)
            .mul_scalar(ctx.dt * ctx.drying_last_node_evaporation_k);
        let new_tail = tail.clone().sub(delta);
        let inner = h_new.clone().slice([0..batch, 0..(n - 1), 0..f_h]);
        h_new = Tensor::cat(vec![inner, new_tail], 1);
    }
    state.hydro.humidity = Field::new(h_new);
    Ok(state)
}

pub fn mechanics_pass<B: Backend<FloatElem = f32>>(
    mut state: ThmcState<B>,
    solver: &mut ThmcSolver,
    ctx: &ThmcStepCtx<'_, B>,
) -> Result<ThmcState<B>, PhysicsError> {
    let batch = ctx.batch;
    let n = ctx.n;
    let device = ctx.device;

    let Some(coords_n3) = ctx.manifold.node_positions.as_ref() else {
        return Ok(state);
    };
    if coords_n3.dims() != [n, 3] {
        return Ok(state);
    }

    let bm_core = displacement_bc_mask_core(&ctx.manifold.displacement_bc_mask, n)?;
    let bm = bm_core.unsqueeze_dim::<3>(0).expand::<3, _>([batch, n, 3]);
    let alpha_bn1 = state
        .chemical
        .reaction_extent
        .as_tensor()
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .clamp(1e-6_f32, 1.0_f32);
    let stiffness_e = alpha_bn1.mul_scalar(ctx.reaction_extent_kinetics.stiffness_e_scale_pa);
    let stiffness_nu = Tensor::<B, 3>::zeros([batch, n, 1], device)
        .add_scalar(ctx.reaction_extent_kinetics.stiffness_nu);
    let stiffness = Tensor::cat(vec![stiffness_e, stiffness_nu], 2);
    let bf = Field::new(Tensor::<B, 3>::zeros([batch, n, 3], device));
    let inner_cfg = MechanicsInnerLoopConfig::default();
    let cross_section_area = 0.01_f32;

    #[cfg(feature = "mechanics-adjoint")]
    {
        use crate::physics::mechanics_solve_port::bar_network_equilibrium_reported as solve_bar_equilibrium;
        let rel_tol = inner_cfg
            .pcg_tolerance
            .max(inner_cfg.cg_tolerance)
            .max(1e-6_f32);
        let equilibrium = solve_bar_equilibrium(
            state.mechanical.displacement.clone(),
            coords_n3.clone(),
            stiffness,
            bf.clone(),
            ctx.edges_b1.clone(),
            ctx.damage_m.as_damage_field().clone(),
            bm,
            cross_section_area,
            &inner_cfg,
            rel_tol,
        )?;
        solver.mechanics_solve_reports.push(equilibrium.2);
        state.mechanical.displacement = equilibrium.0;
    }
    #[cfg(not(feature = "mechanics-adjoint"))]
    {
        let (u_new, _stress) = VectorMechanicsSolver::solve_equilibrium_typed(
            state.mechanical.displacement.clone(),
            coords_n3.clone(),
            stiffness,
            bf,
            ctx.edges_b1.clone(),
            ctx.damage_m.as_damage_field().clone(),
            bm,
            cross_section_area,
            &inner_cfg,
        )?;
        state.mechanical.displacement = u_new;
    }

    Ok(state)
}

fn displacement_bc_mask_core<B: Backend>(
    mask: &Tensor<B, 3>,
    n: usize,
) -> Result<Tensor<B, 2>, PhysicsError> {
    match mask.dims()[..] {
        [nn, 3, 1] if nn == n => Ok(mask.reshape([nn, 3])),
        [nn, 1, 3] if nn == n => Ok(mask.clone().reshape([nn, 3])),
        [1, nn, 3] if nn == n => Ok(mask.clone().slice([0..1, 0..n, 0..3]).reshape([nn, 3])),
        _ => Err(format!(
            "ThmcSolver::step: displacement_bc_mask dims {:?} incompatible with N={n} (expected [N,3,1], [N,1,3], or [1,N,3])",
            mask.dims()
        ).into()),
    }
}
