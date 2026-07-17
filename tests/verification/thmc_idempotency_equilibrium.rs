// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP Manifesto §6 — THMC idempotency at composite equilibrium and hydrate/sync roundtrip.

#![cfg(feature = "thmc-coupled")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::field::{
    Field, FractureEnergyField, HumidityField, ReactionExtentField, SmallStrainField,
    StepEntryDamageMask, StiffnessField, TemperatureField,
};
use umst_manifold::core::tensors::UnifiedMaterialStateTensor;
use umst_manifold::core::traits::IScienceCartridge;
use umst_manifold::core::umst_schema::{UMST_SCALAR_CHANNEL_COUNT, SCALAR_DAMAGE, SCALAR_HUMIDITY, SCALAR_TEMPERATURE};
use umst_manifold::physics::laplacian::TopologicalLaplacian;
use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::orchestration::TopologyPhysicsOrchestrator;
use umst_manifold::physics::solvers::fracture_field::PhaseFieldFractureSolver;
use umst_manifold::physics::solvers::thmc::{reaction_extent_rate_field, ThmcNewtonConfig};
use umst_manifold::physics::solvers::{
    ReactionExtentKinetics, ThmcImplicitEulerThermalHumidityReactionExtentResidual,
    ThmcImplicitEulerThermalReactionExtentResidual, ThmcSolver, ThmcState,
};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
use umst_manifold::physics::thmc_umst_sync::sync_thmc_to_umst;

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn toy_umst(n: usize, t: f32, h: f32, d: f32) -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let mut flat = vec![0.0_f32; n * f];
    flat[SCALAR_TEMPERATURE] = t;
    flat[SCALAR_HUMIDITY] = h;
    flat[SCALAR_DAMAGE] = d;
    if n > 1 {
        flat[f + SCALAR_TEMPERATURE] = t;
        flat[f + SCALAR_HUMIDITY] = h;
        flat[f + SCALAR_DAMAGE] = d;
    }
    let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, f])), &dev);
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

struct StubCartridge;

impl<Bk: burn::tensor::backend::Backend<FloatElem = f32>> IScienceCartridge<Bk> for StubCartridge {
    fn compute_all(
        &self,
        mix: &umst_manifold::core::tensors::MaterialCompositionTensor<Bk>,
    ) -> umst_manifold::core::traits::PhysicalResult<Bk> {
        let d = mix.fractions.device();
        umst_manifold::core::traits::PhysicalResult {
            free_energy: Tensor::zeros([1, 1], &d),
            dissipation: Tensor::zeros([1, 1], &d),
            safety_margin: Tensor::zeros([1, 1], &d),
            cost: Tensor::zeros([1, 1], &d),
            damage: Tensor::zeros([1, 1], &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, 1], &d),
        }
    }

    fn compute_topology(
        &self,
        m: &UnifiedMaterialStateTensor<Bk>,
    ) -> umst_manifold::core::traits::PhysicalResult<Bk> {
        let nn = m.scalar_features.dims()[0];
        let d = m.scalar_features.device();
        umst_manifold::core::traits::PhysicalResult {
            free_energy: Tensor::zeros([1, nn], &d),
            dissipation: Tensor::zeros([1, nn], &d),
            safety_margin: Tensor::zeros([1, nn], &d),
            cost: Tensor::zeros([1, nn], &d),
            damage: Tensor::zeros([1, nn], &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, nn], &d),
        }
    }
}

fn max_abs_tensor3(a: &Tensor<B, 3>, b: &Tensor<B, 3>) -> f32 {
    let da = a.clone().into_data().value;
    let db = b.clone().into_data().value;
    da.iter()
        .zip(db.iter())
        .map(|(&x, &y)| (x - y).abs())
        .fold(0.0_f32, f32::max)
}

fn two_node_edges() -> Tensor<B, 2, Int> {
    Tensor::from_data(
        Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
        &device(),
    )
}

fn zero_damage_mask(batch: usize, n: usize) -> StepEntryDamageMask<B> {
    StepEntryDamageMask::from_damage_field(Field::new(Tensor::<B, 3>::zeros(
        [batch, n, 1],
        &device(),
    )))
}

/// Mirrors the explicit thermal sub-step in [`ThmcSolver::step`] (Laplacian + Euler, no exothermic).
fn apply_explicit_thermal_substep(
    t: &TemperatureField<B>,
    damage: &StepEntryDamageMask<B>,
    edges: Tensor<B, 2, Int>,
    dt: f32,
) -> TemperatureField<B> {
    let lap = TopologicalLaplacian::scalar_laplacian_temperature(t, damage, edges);
    Field::new(
        t.as_tensor()
            .clone()
            .add(lap.as_tensor().clone().mul_scalar(dt)),
    )
}

/// Mirrors the explicit humidity sub-step in [`ThmcSolver::step`] (no tail drying sink).
fn apply_explicit_humidity_substep(
    h: &HumidityField<B>,
    damage: &StepEntryDamageMask<B>,
    edges: Tensor<B, 2, Int>,
    dt: f32,
) -> HumidityField<B> {
    let lap = TopologicalLaplacian::scalar_laplacian_humidity(h, damage, edges);
    Field::new(
        h.as_tensor()
            .clone()
            .add(lap.as_tensor().clone().mul_scalar(dt)),
    )
}

/// Mirrors the explicit reaction-extent sub-step in [`ThmcSolver::step`].
fn apply_explicit_alpha_substep(
    alpha: &ReactionExtentField<B>,
    temperature: &TemperatureField<B>,
    kinetics: &ReactionExtentKinetics,
    dt: f32,
) -> ReactionExtentField<B> {
    let d_alpha = reaction_extent_rate_field(kinetics, alpha, temperature, &device());
    Field::new(
        alpha
            .as_tensor()
            .clone()
            .add(d_alpha.as_tensor().clone().mul_scalar(dt))
            .clamp(0.0_f32, 1.0_f32),
    )
}

fn equilibrated_state(n: usize) -> ThmcState<B> {
    let dev = device();
    ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 300.0, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.5, &dev),
        Tensor::<B, 3>::zeros([1, n, 3], &dev),
        Tensor::<B, 3>::full([1, n, 1], 1.0, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.1, &dev),
        0.0,
    )
}

/// FP §6: explicit thermal Laplacian increment on uniform `T` is a fixed point.
#[test]
fn thmc_thermal_transport_idempotent_at_laplacian_fixed_point() {
    let dev = device();
    let n = 2usize;
    let batch = 1usize;
    let dt = 1e-4_f32;
    let edges = two_node_edges();
    let damage = zero_damage_mask(batch, n);
    let t0 = Field::new(Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &dev));

    let t1 = apply_explicit_thermal_substep(&t0, &damage, edges.clone(), dt);
    let t2 = apply_explicit_thermal_substep(&t1, &damage, edges, dt);

    let tol = 1e-6_f32;
    assert!(
        max_abs_tensor3(t1.as_tensor(), t0.as_tensor()) < tol,
        "uniform T must satisfy discrete Laplacian equilibrium"
    );
    assert!(
        max_abs_tensor3(t2.as_tensor(), t1.as_tensor()) < tol,
        "re-application of explicit thermal sub-step must not drift"
    );
}

/// FP §6: implicit CG thermal solve on uniform Dirichlet field is a fixed point.
#[test]
fn thmc_thermal_implicit_cg_idempotent_at_dirichlet_equilibrium() {
    let dev = device();
    let n = 2usize;
    let edges = two_node_edges();
    let t_uniform = Tensor::<B, 3>::full([1, n, 1], 300.0_f32, &dev);
    let mask = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let solver = ThmcSolver::default();
    let cfg = ThmcNewtonConfig {
        max_iterations: 20,
        residual_tolerance: 1.0e-6_f32,
        finite_diff_eps: 1.0e-6_f32,
        damping: 1.0_f32,
    };

    let (t1, norms1) = solver
        .step_thermal_implicit::<B>(1e-4_f32, t_uniform.clone(), 0.1_f32, edges.clone(), mask.clone(), cfg)
        .expect("first implicit thermal CG");
    let (t2, norms2) = solver
        .step_thermal_implicit::<B>(1e-4_f32, t1.clone(), 0.1_f32, edges, mask, cfg)
        .expect("second implicit thermal CG");

    let tol = 1e-6_f32;
    assert!(
        max_abs_tensor3(&t1, &t_uniform) < tol,
        "uniform T must be implicit-Euler equilibrium"
    );
    assert!(
        max_abs_tensor3(&t2, &t1) < tol,
        "re-application of implicit thermal CG must not drift"
    );
    assert!(
        norms1.last().copied().unwrap_or(f32::INFINITY) < cfg.residual_tolerance,
        "first CG pass should converge: {:?}",
        norms1
    );
    assert!(
        norms2.last().copied().unwrap_or(f32::INFINITY) < cfg.residual_tolerance,
        "second CG pass should converge: {:?}",
        norms2
    );
}

/// FP §6: explicit humidity Laplacian increment on uniform `h` is a fixed point.
#[test]
fn thmc_humidity_transport_idempotent_at_uniform_field() {
    let dev = device();
    let n = 2usize;
    let batch = 1usize;
    let dt = 1e-4_f32;
    let edges = two_node_edges();
    let damage = zero_damage_mask(batch, n);
    let h0 = Field::new(Tensor::<B, 3>::full([batch, n, 1], 0.5_f32, &dev));

    let h1 = apply_explicit_humidity_substep(&h0, &damage, edges.clone(), dt);
    let h2 = apply_explicit_humidity_substep(&h1, &damage, edges, dt);

    let tol = 1e-6_f32;
    assert!(
        max_abs_tensor3(h1.as_tensor(), h0.as_tensor()) < tol,
        "uniform h must satisfy discrete Laplacian equilibrium"
    );
    assert!(
        max_abs_tensor3(h2.as_tensor(), h1.as_tensor()) < tol,
        "re-application of explicit humidity sub-step must not drift"
    );
}

/// FP §6: saturated `α=1` vanishes reaction rate — explicit α update is a fixed point.
#[test]
fn thmc_reaction_extent_idempotent_when_rate_vanishes() {
    let dev = device();
    let n = 2usize;
    let batch = 1usize;
    let dt = 1e-4_f32;
    let kinetics = ReactionExtentKinetics::default();
    let temperature = Field::new(Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &dev));
    let alpha0 = Field::new(Tensor::<B, 3>::full([batch, n, 1], 1.0_f32, &dev));

    let rate = reaction_extent_rate_field(&kinetics, &alpha0, &temperature, &dev);
    assert!(
        max_abs_tensor3(rate.as_tensor(), &Tensor::<B, 3>::zeros([batch, n, 1], &dev)) < 1e-9_f32,
        "saturated α=1 must zero the reaction rate"
    );

    let alpha1 = apply_explicit_alpha_substep(&alpha0, &temperature, &kinetics, dt);
    let alpha2 = apply_explicit_alpha_substep(&alpha1, &temperature, &kinetics, dt);

    let tol = 1e-6_f32;
    assert!(
        max_abs_tensor3(alpha1.as_tensor(), alpha0.as_tensor()) < tol,
        "α update at vanishing rate must be identity"
    );
    assert!(
        max_abs_tensor3(alpha2.as_tensor(), alpha1.as_tensor()) < tol,
        "re-application of explicit α sub-step must not drift"
    );
}

/// FP §6: operator-split `step` on uniform T/h, saturated α, zero u must be a fixed point.
#[test]
fn thmc_operator_split_step_idempotent_at_quiescent_equilibrium() {
    let n = 2usize;
    let mut umst = toy_umst(n, 300.0, 0.5, 0.1);
    let state = equilibrated_state(n);
    let mut solver = ThmcSolver {
        dt: 1e-4,
        max_newton: 1,
        tol: 1e-6,
        drying_last_node_evaporation_k: 0.0,
        ..Default::default()
    };
    let post1 = solver
        .step(&StubCartridge, state, &mut umst)
        .expect("first step");
    let snap = post1.clone();
    let post2 = solver
        .step(&StubCartridge, post1, &mut umst)
        .expect("second step");
    let tol = 1e-5_f32;
    assert!(
        max_abs_tensor3(
            post2.thermal.temperature.as_tensor(),
            snap.thermal.temperature.as_tensor()
        ) < tol,
        "temperature must not drift on re-step"
    );
    assert!(
        max_abs_tensor3(
            post2.hydro.humidity.as_tensor(),
            snap.hydro.humidity.as_tensor()
        ) < tol,
        "humidity must not drift on re-step"
    );
    assert!(
        max_abs_tensor3(
            post2.chemical.reaction_extent.as_tensor(),
            snap.chemical.reaction_extent.as_tensor()
        ) < tol,
        "reaction extent must not drift on re-step"
    );
    assert!(
        max_abs_tensor3(post2.damage.as_tensor(), snap.damage.as_tensor()) < tol,
        "damage must not drift on re-step"
    );
    assert!(
        max_abs_tensor3(
            post2.mechanical.displacement.as_tensor(),
            snap.mechanical.displacement.as_tensor()
        ) < tol,
        "displacement must not drift on re-step"
    );
}

/// FP §6: zero body force with fixed left end — bar equilibrium is a fixed point.
#[test]
fn thmc_mechanics_bar_idempotent_at_zero_load_equilibrium() {
    let dev = device();
    let n = 2usize;
    let coords = Tensor::from_data(
        Data::new(vec![0.0_f32, 0.0, 0.0, 1.0_f32, 0.0, 0.0], Shape::new([n, 3])),
        &dev,
    );
    let edges = two_node_edges();
    let stiffness = StiffnessField::from_e_nu_cat(
        Tensor::<B, 3>::full([1, n, 1], 1.0e6_f32, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.2_f32, &dev),
    );
    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 64,
        cg_tolerance: 1e-8,
        pcg_tolerance: 1e-8,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let mut bm = vec![1.0_f32; n * 3];
    bm[0] = 0.0_f32;
    let boundary_mask = Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &dev);
    let (u1, _) = VectorMechanicsSolver::solve_equilibrium_typed(
        Field::new(Tensor::<B, 3>::zeros([1, n, 3], &dev)),
        coords.clone(),
        stiffness.clone(),
        Field::new(Tensor::<B, 3>::zeros([1, n, 3], &dev)),
        edges.clone(),
        Field::new(Tensor::<B, 3>::zeros([1, n, 1], &dev)),
        boundary_mask.clone(),
        0.01_f32,
        &cfg,
    )
    .expect("first bar equilibrium");
    let (u2, _) = VectorMechanicsSolver::solve_equilibrium_typed(
        u1.clone(),
        coords,
        stiffness,
        Field::new(Tensor::<B, 3>::zeros([1, n, 3], &dev)),
        edges,
        Field::new(Tensor::<B, 3>::zeros([1, n, 1], &dev)),
        boundary_mask,
        0.01_f32,
        &cfg,
    )
    .expect("second bar equilibrium");
    assert!(max_abs_tensor3(u1.as_tensor(), u2.as_tensor()) < 1e-6_f32);
}

/// FP §6: zero strain with frozen damage — AT2 damage update is a fixed point.
#[test]
fn thmc_fracture_update_damage_idempotent_at_zero_strain() {
    let dev = device();
    let n = 2usize;
    let edges = two_node_edges();
    let strain = SmallStrainField::zeros([1, n, 3, 3], &dev);
    let damage = Field::new(Tensor::<B, 3>::zeros([1, n, 1], &dev));
    let gc = FractureEnergyField::from_tensor(Tensor::<B, 3>::full([1, n, 1], 150.0, &dev));
    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
    let d1 = solver
        .update_damage(strain.clone(), damage, gc.clone(), edges.clone())
        .expect("first fracture damage update");
    let d2 = solver
        .update_damage(strain, d1.clone(), gc, edges)
        .expect("second fracture damage update");
    assert!(max_abs_tensor3(d1.as_tensor(), d2.as_tensor()) < 1e-6_f32);
}

/// FP §6: orchestrator `run_plan_step` on quiescent equilibrium is a fixed point.
#[test]
fn orchestrator_thmc_idempotent_at_equilibrium() {
    let n = 2usize;
    let mut manifold = toy_umst(n, 300.0, 0.5, 0.1);
    let state = equilibrated_state(n);
    let mut orch = TopologyPhysicsOrchestrator::new(ThmcSolver {
        dt: 1e-4,
        max_newton: 1,
        tol: 1e-6,
        drying_last_node_evaporation_k: 0.0,
        ..Default::default()
    });
    let post1 = orch
        .run_plan_step(&StubCartridge, state, &mut manifold)
        .expect("first orchestrator step");
    let snap = post1.clone();
    let post2 = orch
        .run_plan_step(&StubCartridge, post1, &mut manifold)
        .expect("second orchestrator step");
    let tol = 1e-5_f32;
    assert!(
        max_abs_tensor3(
            post2.thermal.temperature.as_tensor(),
            snap.thermal.temperature.as_tensor()
        ) < tol
    );
    assert!(
        max_abs_tensor3(post2.hydro.humidity.as_tensor(), snap.hydro.humidity.as_tensor()) < tol
    );
    assert!(
        max_abs_tensor3(
            post2.chemical.reaction_extent.as_tensor(),
            snap.chemical.reaction_extent.as_tensor()
        ) < tol,
        "orchestrator reaction extent must not drift on re-step"
    );
    assert!(
        max_abs_tensor3(post2.damage.as_tensor(), snap.damage.as_tensor()) < tol,
        "orchestrator damage must not drift on re-step"
    );
    assert!(
        max_abs_tensor3(
            post2.mechanical.displacement.as_tensor(),
            snap.mechanical.displacement.as_tensor()
        ) < tol,
        "orchestrator displacement must not drift on re-step"
    );
}

/// FP §6: backward-Euler \((T,\alpha)\) residual at uniform saturated equilibrium — damped Newton re-step is a fixed point.
#[test]
fn thmc_t_alpha_residual_damped_newton_idempotent_at_backward_euler_equilibrium() {
    let dev = device();
    let n = 2usize;
    let batch = 1usize;
    let dt = 1e-4_f32;
    let edges = two_node_edges();
    let damage = zero_damage_mask(batch, n);
    let kinetics = ReactionExtentKinetics::default();
    let t_n = Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &dev);
    let alpha_n = Tensor::<B, 3>::full([batch, n, 1], 1.0_f32, &dev);
    let assembler = ThmcImplicitEulerThermalReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n.clone()),
        alpha_n: Field::new(alpha_n.clone()),
        edges_b1: edges,
        damage_m: damage,
        kinetics,
    };
    let trial = ThmcState::from_tensors(
        t_n,
        Tensor::<B, 3>::full([batch, n, 1], 0.5_f32, &dev),
        Tensor::<B, 3>::zeros([batch, n, 3], &dev),
        alpha_n,
        Tensor::<B, 3>::zeros([batch, n, 1], &dev),
        0.0_f32,
    );
    let r0 = assembler
        .residual_l2(&trial)
        .expect("equilibrium residual norm");
    assert!(
        r0 < 1e-6_f32,
        "uniform saturated (T,α) must satisfy backward-Euler equilibrium, got ||R||={r0}"
    );
    let (after_first, _) = assembler
        .damped_newton_iterations(&trial, 2_usize, 1.0_f32, 1.0e-5_f32)
        .expect("first damped Newton chain on equilibrated (T,α)");
    let (after_second, _) = assembler
        .damped_newton_iterations(&after_first, 2_usize, 1.0_f32, 1.0e-5_f32)
        .expect("second damped Newton chain on equilibrated (T,α)");
    let tol = 1e-5_f32;
    assert!(
        max_abs_tensor3(
            after_first.thermal.temperature.as_tensor(),
            trial.thermal.temperature.as_tensor()
        ) < tol,
        "(T,α) damped Newton must not drift from backward-Euler equilibrium"
    );
    assert!(
        max_abs_tensor3(
            after_first.chemical.reaction_extent.as_tensor(),
            trial.chemical.reaction_extent.as_tensor()
        ) < tol,
        "(T,α) reaction extent must not drift on residual re-step"
    );
    assert!(
        max_abs_tensor3(
            after_second.thermal.temperature.as_tensor(),
            after_first.thermal.temperature.as_tensor()
        ) < tol,
        "re-application of (T,α) damped Newton must not drift"
    );
    assert!(
        max_abs_tensor3(
            after_second.chemical.reaction_extent.as_tensor(),
            after_first.chemical.reaction_extent.as_tensor()
        ) < tol,
        "re-application of (T,α) α must not drift"
    );
}

/// FP §6: backward-Euler \((T,h,\alpha)\) residual at uniform saturated equilibrium — damped Newton re-step is a fixed point.
#[test]
fn thmc_tha_residual_damped_newton_idempotent_at_backward_euler_equilibrium() {
    let dev = device();
    let n = 2usize;
    let batch = 1usize;
    let dt = 1e-4_f32;
    let edges = two_node_edges();
    let damage = zero_damage_mask(batch, n);
    let kinetics = ReactionExtentKinetics::default();
    let t_n = Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &dev);
    let h_n = Tensor::<B, 3>::full([batch, n, 1], 0.5_f32, &dev);
    let alpha_n = Tensor::<B, 3>::full([batch, n, 1], 1.0_f32, &dev);
    let u_n = Tensor::<B, 3>::zeros([batch, n, 3], &dev);
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n.clone()),
        humidity_n: Field::new(h_n.clone()),
        alpha_n: Field::new(alpha_n.clone()),
        displacement_n: u_n.clone(),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1: edges,
        damage_m: damage,
        kinetics,
    };
    let trial = ThmcState::from_tensors(
        t_n,
        h_n,
        u_n,
        alpha_n,
        Tensor::<B, 3>::zeros([batch, n, 1], &dev),
        0.0_f32,
    );
    let r0 = assembler
        .residual_l2(&trial)
        .expect("equilibrium residual norm");
    assert!(
        r0 < 1e-6_f32,
        "uniform saturated (T,h,α) must satisfy backward-Euler equilibrium, got ||R||={r0}"
    );
    let (after_first, _) = assembler
        .damped_newton_iterations(&trial, 2_usize, 1.0_f32, 1.0e-5_f32)
        .expect("first damped Newton chain on equilibrated (T,h,α)");
    let (after_second, _) = assembler
        .damped_newton_iterations(&after_first, 2_usize, 1.0_f32, 1.0e-5_f32)
        .expect("second damped Newton chain on equilibrated (T,h,α)");
    let tol = 1e-5_f32;
    for (label, a, b) in [
        ("T", after_first.thermal.temperature.as_tensor(), trial.thermal.temperature.as_tensor()),
        ("h", after_first.hydro.humidity.as_tensor(), trial.hydro.humidity.as_tensor()),
        (
            "α",
            after_first.chemical.reaction_extent.as_tensor(),
            trial.chemical.reaction_extent.as_tensor(),
        ),
    ] {
        assert!(
            max_abs_tensor3(a, b) < tol,
            "(T,h,α) damped Newton must not drift {label} from backward-Euler equilibrium"
        );
    }
    for (label, a, b) in [
        (
            "T",
            after_second.thermal.temperature.as_tensor(),
            after_first.thermal.temperature.as_tensor(),
        ),
        (
            "h",
            after_second.hydro.humidity.as_tensor(),
            after_first.hydro.humidity.as_tensor(),
        ),
        (
            "α",
            after_second.chemical.reaction_extent.as_tensor(),
            after_first.chemical.reaction_extent.as_tensor(),
        ),
    ] {
        assert!(
            max_abs_tensor3(a, b) < tol,
            "re-application of (T,h,α) damped Newton must not drift {label}"
        );
    }
}

/// FP §6: monolithic quasi-static \((T,h,\alpha,\mathbf u)\) residual at uniform zero-load equilibrium — damped Newton re-step is a fixed point.
#[test]
fn thmc_monolithic_qs_r_u_residual_damped_newton_idempotent_at_equilibrium() {
    let dev = device();
    let n = 2usize;
    let batch = 1usize;
    let dt = 1e-4_f32;
    let edges = two_node_edges();
    let damage = zero_damage_mask(batch, n);
    let kinetics = ReactionExtentKinetics::default();
    let coords = Tensor::from_data(
        Data::new(vec![0.0_f32, 0.0, 0.0, 0.01_f32, 0.0, 0.0], Shape::new([n, 3])),
        &dev,
    );
    let mut bm = vec![1.0_f32; n * 3];
    bm[0] = 0.0_f32;
    let boundary_mask = Tensor::from_data(Data::new(bm, Shape::new([batch, n, 3])), &dev);
    let body_force = Tensor::<B, 3>::zeros([batch, n, 3], &dev);
    let cross_section_area = 0.01_f32;
    let cfg = MechanicsInnerLoopConfig::default();
    let t_n = Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &dev);
    let h_n = Tensor::<B, 3>::full([batch, n, 1], 0.5_f32, &dev);
    let alpha_n = Tensor::<B, 3>::full([batch, n, 1], 1.0_f32, &dev);
    let stiffness = StiffnessField::from_e_nu_cat(
        alpha_n.clone().mul_scalar(kinetics.stiffness_e_scale_pa),
        Tensor::<B, 3>::full([batch, n, 1], kinetics.stiffness_nu, &dev),
    );
    let (u_eq, _) = VectorMechanicsSolver::solve_equilibrium_typed(
        Field::new(Tensor::<B, 3>::zeros([batch, n, 3], &dev)),
        coords.clone(),
        stiffness,
        Field::new(body_force.clone()),
        edges.clone(),
        Field::new(Tensor::<B, 3>::zeros([batch, n, 1], &dev)),
        boundary_mask.clone(),
        cross_section_area,
        &cfg,
    )
    .expect("zero-load bar equilibrium");
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n.clone()),
        humidity_n: Field::new(h_n.clone()),
        alpha_n: Field::new(alpha_n.clone()),
        displacement_n: u_eq.as_tensor().clone(),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1: edges,
        damage_m: damage,
        kinetics,
    };
    let trial = ThmcState::from_tensors(
        t_n,
        h_n,
        u_eq.as_tensor().clone(),
        alpha_n,
        Tensor::<B, 3>::zeros([batch, n, 1], &dev),
        0.0_f32,
    );
    let r0 = assembler
        .residual_l2_including_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("monolithic equilibrium residual norm");
    let stacked_tol = 1e-4_f32;
    assert!(
        r0 < stacked_tol,
        "uniform zero-load monolithic state must satisfy stacked equilibrium, got ||R||={r0}"
    );
    // Production path: tolerance early-exit when ||R||₂ already below threshold (avoids singular FD Jacobian at equilibrium).
    let (after_first, norms1) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            2_usize,
            1.0_f32,
            1.0e-5_f32,
            stacked_tol,
            None,
        )
        .expect("first monolithic damped Newton chain (early-exit at equilibrium)");
    assert_eq!(
        norms1.len(),
        1,
        "equilibrium head iterate must early-exit before inner Newton steps"
    );
    let (after_second, norms2) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &after_first,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            2_usize,
            1.0_f32,
            1.0e-5_f32,
            stacked_tol,
            None,
        )
        .expect("second monolithic damped Newton chain (early-exit at equilibrium)");
    assert_eq!(norms2.len(), 1, "re-step must also early-exit at equilibrium");
    let tol = 1e-5_f32;
    for (label, a, b) in [
        ("T", after_first.thermal.temperature.as_tensor(), trial.thermal.temperature.as_tensor()),
        ("h", after_first.hydro.humidity.as_tensor(), trial.hydro.humidity.as_tensor()),
        (
            "α",
            after_first.chemical.reaction_extent.as_tensor(),
            trial.chemical.reaction_extent.as_tensor(),
        ),
        (
            "u",
            after_first.mechanical.displacement.as_tensor(),
            trial.mechanical.displacement.as_tensor(),
        ),
    ] {
        assert!(
            max_abs_tensor3(a, b) < tol,
            "monolithic damped Newton must not drift {label} from equilibrium"
        );
    }
    for (label, a, b) in [
        (
            "T",
            after_second.thermal.temperature.as_tensor(),
            after_first.thermal.temperature.as_tensor(),
        ),
        (
            "h",
            after_second.hydro.humidity.as_tensor(),
            after_first.hydro.humidity.as_tensor(),
        ),
        (
            "α",
            after_second.chemical.reaction_extent.as_tensor(),
            after_first.chemical.reaction_extent.as_tensor(),
        ),
        (
            "u",
            after_second.mechanical.displacement.as_tensor(),
            after_first.mechanical.displacement.as_tensor(),
        ),
    ] {
        assert!(
            max_abs_tensor3(a, b) < tol,
            "re-application of monolithic damped Newton must not drift {label}"
        );
    }
}

/// FP §6: `sync_thmc_to_umst` ∘ `hydrate_from_umst_typed_views` roundtrip on scalar channels.
#[test]
fn thmc_hydrate_sync_roundtrip_idempotent_on_scalar_channels() {
    let n = 2usize;
    let mut umst = toy_umst(n, 100.0, 0.4, 0.2);
    let dev = device();
    let state = ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 310.0, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.6, &dev),
        Tensor::<B, 3>::zeros([1, n, 3], &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.3, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.25, &dev),
        1.5,
    );
    sync_thmc_to_umst(&state, &mut umst).expect("sync");
    let hydrated =
        ThmcState::hydrate_from_umst_typed_views(&umst, Some(&state)).expect("hydrate");
    let eps = 1e-5_f32;
    assert!(
        max_abs_tensor3(
            hydrated.thermal.temperature.as_tensor(),
            state.thermal.temperature.as_tensor()
        ) < eps
    );
    assert!(
        max_abs_tensor3(
            hydrated.hydro.humidity.as_tensor(),
            state.hydro.humidity.as_tensor()
        ) < eps
    );
    assert!(
        max_abs_tensor3(hydrated.damage.as_tensor(), state.damage.as_tensor()) < eps
    );
    let snap = umst.scalar_features.clone().into_data().value;
    sync_thmc_to_umst(&hydrated, &mut umst).expect("re-sync");
    let again = umst.scalar_features.clone().into_data().value;
    assert_eq!(snap, again, "second sync after hydrate must not drift UMST columns");
}
