// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `ThmcSolver::step` smoke test when `thmc-coupled` is enabled: SI `node_positions` present
//! so mechanics sub-solve runs and [`ThmcSolver::step`] returns `Ok`.

#[cfg(feature = "thmc-coupled")]
mod thmc_ok {
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
    use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
    use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
    use umst_manifold::physics::solvers::{
        ChemicalPlan, HydrologicPlan, MechanicalPlan, ThermalPlan, ThmcSolver, ThmcState,
    };

    type B = NdArray<f32>;

    fn dev() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    struct Stub;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for Stub {
        fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
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

    fn umst_with_positions() -> UnifiedMaterialStateTensor<B> {
        let dev = dev();
        let n = 2usize;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let coords: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
            &dev,
        );
        let faces_b2: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
        let scalar_features = Tensor::<B, 2>::zeros([n, f], &dev);
        let vector_features = Tensor::<B, 3>::zeros([n, 1, 3], &dev);
        let matrix_features = Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev);
        let mut pos = vec![0.0_f32; n * 3];
        pos[3] = 1.0;
        let node_positions = Some(Tensor::from_data(Data::new(pos, Shape::new([n, 3])), &dev));
        UnifiedMaterialStateTensor {
            coords,
            edges_b1,
            faces_b2,
            scalar_features,
            vector_features,
            matrix_features,
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions,
            displacement_bc_mask: Tensor::<B, 3>::ones([n, 3, 1], &dev),
            policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    #[test]
    fn thmc_step_ok_with_node_positions() {
        let umst = umst_with_positions();
        let dev = dev();
        let n = umst.scalar_features.dims()[0];
        let state =         ThmcState::from_tensors(
            Tensor::zeros([1, n, 1], &dev),
            Tensor::zeros([1, n, 1], &dev),
            Tensor::zeros([1, n, 3], &dev),
            Tensor::zeros([1, n, 1], &dev),
            Tensor::zeros([1, n, 1], &dev),
            0.0_f32,
        );
        let mut solver = ThmcSolver {
            dt: 0.01_f32,
            max_newton: 2_usize,
            tol: 1e-3_f32,
            ..Default::default()
        };
        let mut umst_mut = umst;
        let out = solver.step(&Stub, state, &mut umst_mut);
        assert!(out.is_ok(), "expected Ok, got {:?}", out.err());
    }
}
