// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP Manifesto §6 — THMC idempotency at composite equilibrium and hydrate/sync roundtrip.

#![cfg(feature = "thmc-coupled")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::field::{
    Field, HumidityField, ReactionExtentField, StepEntryDamageMask, TemperatureField,
};
use umst_manifold::core::tensors::UnifiedMaterialStateTensor;
use umst_manifold::core::traits::IScienceCartridge;
use umst_manifold::core::umst_schema::{UMST_SCALAR_CHANNEL_COUNT, SCALAR_DAMAGE, SCALAR_HUMIDITY, SCALAR_TEMPERATURE};
use umst_manifold::physics::laplacian::TopologicalLaplacian;
use umst_manifold::physics::solvers::thmc::{reaction_extent_rate_field, ThmcNewtonConfig};
use umst_manifold::physics::solvers::{ReactionExtentKinetics, ThmcSolver, ThmcState};
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
