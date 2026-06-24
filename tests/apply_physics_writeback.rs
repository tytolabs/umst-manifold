// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Integration tests for [`umst_manifold::core::apply_physics_to_umst`] (damage / masked temperature
//! via [`UnifiedMaterialStateTensor::project_scalar_channel`] /
//! [`UnifiedMaterialStateTensor::write_scalar_channel`]) and default-feature [`ThmcSolver::step`] errors.

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::apply_physics_to_umst;
use umst_manifold::core::tensors::{StatePoint, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::{SCALAR_DAMAGE, SCALAR_TEMPERATURE, UMST_SCALAR_CHANNEL_COUNT};

#[cfg(not(feature = "thmc-coupled"))]
use umst_manifold::physics::solvers::{
    ChemicalPlan, HydrologicPlan, MechanicalPlan, ThermalPlan, ThmcSolver, ThmcState,
};

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

/// Two-node UMST with [`UMST_SCALAR_CHANNEL_COUNT`] scalar channels; optional SI `[N, 3]` embedding.
fn test_umst(
    scalars: Tensor<B, 2>,
    policy_mask: Tensor<B, 2>,
    node_positions: Option<Tensor<B, 2>>,
) -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let n = scalars.dims()[0];
    assert_eq!(scalars.dims()[1], UMST_SCALAR_CHANNEL_COUNT);
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
        node_positions,
        displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
        policy_editable_mask: policy_mask,
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
    }
}

fn physical_result(
    dev: &NdArrayDevice,
    n: usize,
    damage: Vec<f32>,
    temp_delta: Option<Vec<f32>>,
) -> PhysicalResult<B> {
    PhysicalResult {
        free_energy: Tensor::zeros([1, n], dev),
        dissipation: Tensor::zeros([1, n], dev),
        safety_margin: Tensor::zeros([1, n], dev),
        cost: Tensor::zeros([1, n], dev),
        damage: Tensor::from_data(Data::new(damage, Shape::new([1, n])), dev),
        temperature_delta: temp_delta
            .map(|v| Tensor::from_data(Data::new(v, Shape::new([1, n])), dev)),
        #[cfg(feature = "information_density")]
        information_density: Tensor::zeros([1, n], dev),
    }
}

#[test]
fn apply_physics_damage_writeback_respects_policy_mask() {
    let dev = device();
    let n = 2usize;
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let mut flat = vec![0.0_f32; n * f];
    flat[SCALAR_DAMAGE] = 0.1;
    flat[f + SCALAR_DAMAGE] = 0.2;
    let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, f])), &dev);
    let mask = Tensor::from_data(Data::new(vec![1.0_f32, 0.0_f32], Shape::new([n, 1])), &dev);
    let pos = Tensor::from_data(Data::new(vec![0.0_f32; n * 3], Shape::new([n, 3])), &dev);
    let mut umst = test_umst(scalars, mask, Some(pos));

    let pr = physical_result(&dev, n, vec![0.9_f32, 0.9_f32], None);
    apply_physics_to_umst(&pr, &mut umst).unwrap();

    let out = umst.scalar_features.clone().into_data().value;
    let d0 = out[SCALAR_DAMAGE];
    let d1 = out[f + SCALAR_DAMAGE];
    assert!(
        (d0 - 0.9).abs() < 1e-5,
        "editable node should take physics damage"
    );
    assert!(
        (d1 - 0.2).abs() < 1e-5,
        "masked node should keep prior damage"
    );
}

#[test]
fn apply_physics_temperature_delta_respects_policy_mask() {
    let dev = device();
    let n = 2usize;
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let mut flat = vec![0.0_f32; n * f];
    flat[SCALAR_TEMPERATURE] = 100.0;
    flat[f + SCALAR_TEMPERATURE] = 50.0;
    let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, f])), &dev);
    let mask = Tensor::from_data(Data::new(vec![1.0_f32, 0.0_f32], Shape::new([n, 1])), &dev);
    let mut umst = test_umst(scalars, mask, None);

    let pr = physical_result(&dev, n, vec![0.0_f32; n], Some(vec![10.0_f32, 10.0_f32]));
    apply_physics_to_umst(&pr, &mut umst).unwrap();

    let out = umst.scalar_features.clone().into_data().value;
    let t0 = out[SCALAR_TEMPERATURE];
    let t1 = out[f + SCALAR_TEMPERATURE];
    assert!((t0 - 110.0).abs() < 1e-4);
    assert!((t1 - 50.0).abs() < 1e-4);
}

#[cfg_attr(feature = "thmc-coupled", allow(dead_code))]
struct EmptyCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for EmptyCartridge {
    fn compute_all(&self, mix: &StatePoint<Bk>) -> PhysicalResult<Bk> {
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

#[cfg(not(feature = "thmc-coupled"))]
#[test]
fn apply_physics_thmc_step_errors_without_thmc_coupled() {
    let dev = device();
    let n = 2usize;
    let scalars = Tensor::<B, 2>::zeros([n, UMST_SCALAR_CHANNEL_COUNT], &dev);
    let mask = Tensor::<B, 2>::ones([n, 1], &dev);
    let manifold = test_umst(scalars, mask, None);

    let state = ThmcState {
        thermal: ThermalPlan {
            temperature: Tensor::<B, 3>::zeros([1, n, 1], &dev),
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::zeros([1, n, 1], &dev),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &dev),
        },
        chemical: ChemicalPlan {
            reaction_extent: Tensor::<B, 3>::zeros([1, n, 1], &dev),
        },
        damage: Tensor::<B, 3>::zeros([1, n, 1], &dev),
        time: 0.0,
    };

    let solver = ThmcSolver {
        dt: 0.01,
        max_newton: 1,
        tol: 1e-6,
        ..Default::default()
    };
    let cartridge = EmptyCartridge;
    match solver.step(&cartridge, state, &manifold) {
        Err(e) => assert!(e.contains("thmc-coupled"), "unexpected error: {e}"),
        Ok(_) => panic!("expected Err when thmc-coupled is disabled"),
    }
}
