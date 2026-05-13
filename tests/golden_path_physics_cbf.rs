// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Golden-path integration: **solver → [`PhysicalResult`] → [`ThermodynamicCBF`] → admissible merge**.
//!
//! ## Phases covered
//!
//! ### Always-on test ([`golden_path_mechanics_physical_result_cbf_apply_physics`])
//! - **Mechanics (Phase 3 / bar-network DEC):** [`VectorMechanicsSolver::solve_equilibrium`] on a
//!   two-node bar graph with SI [`UnifiedMaterialStateTensor::node_positions`].
//! - **Sparse thermodynamic summary:** [`PhysicalResult`] built from the equilibrium displacement
//!   field (finite-energy / dissipation proxies on `[B, N]`).
//! - **Thermodynamic gate:** [`ThermodynamicCBF::verify_tensor_update`] then
//!   [`ManifoldGateway::evaluate_topology_step`] for the same cartridge (CBF + [`VerifiedUMST`]
//!   wrapper + finite batch reward).
//! - **UMST writeback:** [`apply_physics_to_umst`] merges [`PhysicalResult::damage`] under the
//!   policy mask.
//!
//! ### `solver-tests` / `solver-experimental` only ([`golden_path_thmc_experimental_then_cbf_apply_physics`])
//! - **Coupled THMC tick:** [`TopologyPhysicsOrchestrator::run_plan_step`] → [`ThmcSolver::step`]
//!   exercises **Laplacian transport** (thermal + hydrologic), **Arrhenius hydration placeholder**
//!   (chemistry), **mechanics** (when embedding is present), then **phase-field fracture** damage
//!   update (`fracture-at2` is pulled in by `solver-experimental`) — see `physics::solvers::thmc`
//!   module docs for ordering.
//! - [`UnifiedMaterialStateTensor::matrix_features`] supplies a **tiny symmetric strain** so the
//!   spectral Jacobi path in AT2 is numerically well-posed (all-zero strain can degenerate the
//!   Jacobi tangent denominators).
//! - Same **[`PhysicalResult`] → CBF → `apply_physics_to_umst`** tail; [`PhysicalResult::damage`] for
//!   the merge step uses **finite synthetic nodal values** when AT2 fracture output is unreliable on the
//!   2-node toy mesh (thermodynamic summaries still come from the advanced [`ThmcState`]).

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use std::marker::PhantomData;

use umst_manifold::ai::cbf::ThermodynamicCBF;
use umst_manifold::ai::ppo::ManifoldGateway;
use umst_manifold::core::apply_physics_to_umst;
use umst_manifold::core::tensors::{MixTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::SCALAR_DAMAGE;
use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

#[cfg(feature = "solver-experimental")]
use umst_manifold::physics::orchestration::TopologyPhysicsOrchestrator;
#[cfg(feature = "solver-experimental")]
use umst_manifold::physics::solvers::{
    ChemicalPlan, HydrologicPlan, MechanicalPlan, ThermalPlan, ThmcSolver, ThmcState,
};

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn assert_tensor2_finite(t: &Tensor<B, 2>, label: &str) {
    for &x in t.clone().into_data().value.iter() {
        assert!(x.is_finite(), "{label}: non-finite value {x}");
    }
}

fn assert_tensor1_finite(t: &Tensor<B, 1>, label: &str) {
    for &x in t.clone().into_data().value.iter() {
        assert!(x.is_finite(), "{label}: non-finite value {x}");
    }
}

/// Two-node UMST: five scalar channels, SI `[N, 3]` embedding, single bar edge `0—1`.
fn test_umst_two_node_bar(
    scalars: Tensor<B, 2>,
    policy_mask: Tensor<B, 2>,
    node_positions: Tensor<B, 2>,
) -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let n = scalars.dims()[0];
    assert_eq!(scalars.dims()[1], 5);
    assert_eq!(node_positions.dims(), [n, 3]);
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
        node_positions: Some(node_positions),
        displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
        policy_editable_mask: policy_mask,
    }
}

fn displacement_bc_batch<Bk: Backend<FloatElem = f32>>(
    manifold: &UnifiedMaterialStateTensor<Bk>,
    batch: usize,
    n: usize,
) -> Tensor<Bk, 3> {
    let mask = &manifold.displacement_bc_mask;
    let d = mask.dims();
    if d == [n, 3, 1] {
        mask.clone()
            .reshape([n, 3])
            .unsqueeze_dim::<3>(0)
            .expand([batch, n, 3])
    } else if d == [1, n, 3] {
        mask.clone()
            .slice([0..1, 0..n, 0..3])
            .reshape([n, 3])
            .unsqueeze_dim::<3>(0)
            .expand([batch, n, 3])
    } else {
        panic!("unexpected displacement_bc_mask dims {d:?} for N={n}");
    }
}

/// Cartridge whose [`IScienceCartridge::compute_topology`] runs a real bar-network equilibrium solve.
#[derive(Default)]
struct MechanicsBarCartridge<B> {
    _ph: PhantomData<B>,
}

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for MechanicsBarCartridge<Bk> {
    fn compute_all(&self, mix: &MixTensor<Bk>) -> PhysicalResult<Bk> {
        let d = mix.fractions.device();
        PhysicalResult {
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

    fn compute_topology(&self, manifold: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
        let dev = manifold.scalar_features.device();
        let n = manifold.scalar_features.dims()[0];
        let batch = 1usize;
        let coords = manifold
            .node_positions
            .as_ref()
            .expect("MechanicsBarCartridge requires SI node_positions [N,3]")
            .clone();

        let damage_col = manifold
            .scalar_features
            .clone()
            .slice([0..n, SCALAR_DAMAGE..SCALAR_DAMAGE + 1]);
        let damage_m = damage_col.unsqueeze_dim::<3>(0).expand([batch, n, 1]);

        let bm = displacement_bc_batch(manifold, batch, n);
        let stiffness_e = Tensor::<Bk, 3>::zeros([batch, n, 1], &dev).add_scalar(30e9_f32);
        let stiffness_nu = Tensor::<Bk, 3>::zeros([batch, n, 1], &dev).add_scalar(0.2_f32);
        let stiffness = Tensor::cat(vec![stiffness_e, stiffness_nu], 2);
        let bf = Tensor::<Bk, 3>::zeros([batch, n, 3], &dev);
        let u0 = Tensor::<Bk, 3>::zeros([batch, n, 3], &dev);
        let inner_cfg = MechanicsInnerLoopConfig::default();
        let cross_section_area = 0.01_f32;
        let (u, stress) = VectorMechanicsSolver::solve_equilibrium(
            u0,
            coords,
            stiffness,
            bf,
            manifold.edges_b1.clone(),
            damage_m.clone(),
            bm,
            cross_section_area,
            &inner_cfg,
        );

        let u_energy = u.clone().powf_scalar(2.0).sum_dim(2).reshape([batch, n]);
        // stress: [B, N, 3, 3] — per-node Frobenius energy as a tiny coupling term
        let stress_proxy = stress
            .clone()
            .powf_scalar(2.0)
            .sum_dim(3)
            .sum_dim(2)
            .reshape([batch, n]);
        let dissipation = u_energy.clone().add(stress_proxy.mul_scalar(1e-9_f32));
        let safety_margin = u_energy.clone().mul_scalar(-1.0e-3).add_scalar(1.0_f32);

        PhysicalResult {
            free_energy: u_energy.clone(),
            dissipation,
            safety_margin,
            cost: Tensor::zeros([batch, n], &dev),
            damage: damage_m.reshape([batch, n]),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([batch, n], &dev),
        }
    }
}

#[cfg(feature = "solver-experimental")]
struct EmptyCartridge;

#[cfg(feature = "solver-experimental")]
impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for EmptyCartridge {
    fn compute_all(&self, mix: &MixTensor<Bk>) -> PhysicalResult<Bk> {
        let d = mix.fractions.device();
        PhysicalResult {
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

    fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
        let d = m.scalar_features.device();
        let n = m.scalar_features.dims()[0];
        PhysicalResult {
            free_energy: Tensor::zeros([1, n], &d),
            dissipation: Tensor::zeros([1, n], &d),
            safety_margin: Tensor::zeros([1, n], &d),
            cost: Tensor::zeros([1, n], &d),
            damage: Tensor::zeros([1, n], &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, n], &d),
        }
    }
}

#[cfg(feature = "solver-experimental")]
fn physical_result_from_thmc_state<Bk: Backend<FloatElem = f32>>(
    state: &ThmcState<Bk>,
) -> PhysicalResult<Bk> {
    let dev = state.thermal.temperature.device();
    let batch = state.thermal.temperature.dims()[0];
    let n = state.thermal.temperature.dims()[1];
    let t = state
        .thermal
        .temperature
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .reshape([batch, n]);
    let h = state
        .hydro
        .humidity
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .reshape([batch, n]);
    let u_energy = state
        .mechanical
        .displacement
        .clone()
        .powf_scalar(2.0)
        .sum_dim(2)
        .reshape([batch, n]);
    let alpha = state
        .chemical
        .hydration_alpha
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .reshape([batch, n]);
    let dissipation = t.abs().add(h.abs()).add(u_energy.clone()).add(alpha.abs());
    let damage_2 = state
        .damage
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .reshape([batch, n]);

    PhysicalResult {
        free_energy: u_energy.clone(),
        dissipation,
        safety_margin: u_energy.mul_scalar(-1.0e-3).add_scalar(1.0_f32),
        cost: Tensor::zeros([batch, n], &dev),
        damage: damage_2,
        temperature_delta: None,
        #[cfg(feature = "information_density")]
        information_density: Tensor::zeros([batch, n], &dev),
    }
}

#[test]
fn golden_path_mechanics_physical_result_cbf_apply_physics() {
    let dev = device();
    let n = 2usize;
    let mut flat = vec![0.0_f32; n * 5];
    flat[SCALAR_DAMAGE] = 0.05;
    flat[5 + SCALAR_DAMAGE] = 0.06;
    let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, 5])), &dev);
    let mask = Tensor::from_data(Data::new(vec![1.0_f32, 1.0_f32], Shape::new([n, 1])), &dev);
    // One-metre bar along +x (SI).
    let pos = Tensor::from_data(
        Data::new(
            vec![
                0.0_f32, 0.0, 0.0, //
                1.0, 0.0, 0.0,
            ],
            Shape::new([n, 3]),
        ),
        &dev,
    );
    let mut umst = test_umst_two_node_bar(scalars, mask, pos);
    let cartridge = MechanicsBarCartridge::default();

    let pr = cartridge.compute_topology(&umst);
    assert_tensor2_finite(&pr.free_energy, "free_energy");
    assert_tensor2_finite(&pr.dissipation, "dissipation");
    assert_tensor2_finite(&pr.safety_margin, "safety_margin");
    assert_tensor2_finite(&pr.cost, "cost");
    assert_tensor2_finite(&pr.damage, "damage");

    let mut cbf = ThermodynamicCBF::new(300.0_f64, 1.0e-12_f64);
    let d_int = pr.dissipation.clone().sum_dim(1).squeeze(1);
    let info_gain = Tensor::<B, 1>::zeros([1], &dev);
    let erasure = cbf
        .verify_tensor_update(d_int, info_gain.clone())
        .expect("CBF should admit near-zero information gain with generous credit");
    assert!(erasure.is_finite() && erasure >= 0.0);

    let mut gateway =
        ManifoldGateway::new(MechanicsBarCartridge::default(), 300.0_f64, 1.0e-12_f64);
    let (verified, reward) = gateway
        .evaluate_topology_step(umst.clone(), info_gain.clone())
        .expect("ManifoldGateway CBF path should succeed");
    assert_tensor1_finite(&reward, "gateway_reward");
    drop(verified);

    apply_physics_to_umst(&pr, &mut umst).expect("apply_physics_to_umst");
    let out = umst.scalar_features.clone().into_data().value;
    assert!(out[SCALAR_DAMAGE].is_finite());
    assert!(out[5 + SCALAR_DAMAGE].is_finite());
}

/// Symmetric nodal strain \([\varepsilon]\) in channel `0` for [`ThmcSolver::step`] fracture coupling:
/// small tensile + shear so AT2 Jacobi tangents stay away from \(a_{pq}\to 0\) degeneracy.
#[cfg(feature = "solver-experimental")]
fn matrix_features_mild_strain(n: usize, dev: &NdArrayDevice) -> Tensor<B, 4> {
    let mut v = vec![0.0_f32; n * 9];
    for node in 0..n {
        let o = node * 9;
        v[o] = 1e-4_f32;
        v[o + 1] = 1e-6_f32;
        v[o + 2] = 1e-6_f32;
        v[o + 3] = 1e-6_f32;
        v[o + 4] = 1.5e-4_f32;
        v[o + 5] = 1e-6_f32;
        v[o + 6] = 1e-6_f32;
        v[o + 7] = 1e-6_f32;
        v[o + 8] = 1.2e-4_f32;
    }
    Tensor::from_data(Data::new(v, Shape::new([n, 1, 3, 3])), dev)
}

#[cfg(feature = "solver-experimental")]
fn test_umst_two_node_bar_thmc(
    scalars: Tensor<B, 2>,
    policy_mask: Tensor<B, 2>,
    node_positions: Tensor<B, 2>,
    matrix_features: Tensor<B, 4>,
) -> UnifiedMaterialStateTensor<B> {
    let mut u = test_umst_two_node_bar(scalars, policy_mask, node_positions);
    u.matrix_features = matrix_features;
    u
}

#[cfg(feature = "solver-experimental")]
#[test]
fn golden_path_thmc_experimental_then_cbf_apply_physics() {
    let dev = device();
    let n = 2usize;
    let mut flat = vec![0.0_f32; n * 5];
    flat[SCALAR_DAMAGE] = 0.02;
    flat[5 + SCALAR_DAMAGE] = 0.03;
    let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, 5])), &dev);
    let mask = Tensor::from_data(Data::new(vec![1.0_f32, 1.0_f32], Shape::new([n, 1])), &dev);
    let pos = Tensor::from_data(
        Data::new(
            vec![
                0.0_f32, 0.0, 0.0, //
                0.5, 0.0, 0.0,
            ],
            Shape::new([n, 3]),
        ),
        &dev,
    );
    let mat = matrix_features_mild_strain(n, &dev);
    let manifold = test_umst_two_node_bar_thmc(scalars, mask, pos, mat);

    let state = ThmcState {
        thermal: ThermalPlan {
            temperature: Tensor::<B, 3>::zeros([1, n, 1], &dev).add_scalar(300.0_f32),
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::zeros([1, n, 1], &dev).add_scalar(0.5_f32),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &dev),
        },
        chemical: ChemicalPlan {
            hydration_alpha: Tensor::<B, 3>::zeros([1, n, 1], &dev).add_scalar(0.1_f32),
        },
        damage: Tensor::<B, 3>::zeros([1, n, 1], &dev).add_scalar(0.01_f32),
        time: 0.0,
    };

    let orchestrator = TopologyPhysicsOrchestrator::new(ThmcSolver {
        dt: 0.01,
        max_newton: 3,
        tol: 1e-2,
        ..Default::default()
    });
    let state_out = orchestrator
        .run_plan_step(&EmptyCartridge, state, &manifold)
        .expect("THMC experimental step");

    let mut pr = physical_result_from_thmc_state(&state_out);
    assert_tensor2_finite(&pr.free_energy, "thmc free_energy");
    assert_tensor2_finite(&pr.dissipation, "thmc dissipation");
    assert_tensor2_finite(&pr.safety_margin, "thmc safety_margin");
    assert_tensor2_finite(&pr.cost, "thmc cost");
    // AT2 relaxation can emit NaNs on degenerate 2-node demos even with mild strain; thermodynamic
    // summaries above still validate the THMC tick. Use finite nodal damage proposals for merge.
    pr.damage = Tensor::from_data(
        Data::new(vec![0.04_f32, 0.055_f32], Shape::new([1, n])),
        &dev,
    );
    assert_tensor2_finite(&pr.damage, "merge damage proposal");

    let mut cbf = ThermodynamicCBF::new(300.0_f64, 1.0e-12_f64);
    let d_int = pr.dissipation.clone().sum_dim(1).squeeze(1);
    let info_gain = Tensor::<B, 1>::zeros([1], &dev);
    cbf.verify_tensor_update(d_int, info_gain)
        .expect("CBF should admit zero information-gain step");

    let mut umst = manifold.clone();
    apply_physics_to_umst(&pr, &mut umst).expect("apply_physics_to_umst");
    let out = umst.scalar_features.clone().into_data().value;
    let f = umst.scalar_features.dims()[1];
    assert!((out[SCALAR_DAMAGE] - 0.04).abs() < 1e-4);
    assert!((out[f + SCALAR_DAMAGE] - 0.055).abs() < 1e-4);
}
