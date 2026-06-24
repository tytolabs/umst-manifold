// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Post-step THMC gate evidence wire — [`CdTransitionCartridge::transition_evidence`] connectivity.

#[cfg(feature = "thmc-coupled")]
mod thmc_gate_evidence_wire {
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::core::tensors::{StatePoint, UnifiedMaterialStateTensor};
    use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
    use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
    use umst_manifold::physics::solvers::{
        ChemicalPlan, HydrologicPlan, MechanicalPlan, ThermalPlan, ThmcSolver, ThmcState,
    };
    use umst_manifold::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;
    use umst_manifold::runtime::gate::AdmissibilityToken;

    type B = NdArray<f32>;

    fn dev() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    struct Stub;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for Stub {
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

    fn umst(n: usize) -> UnifiedMaterialStateTensor<B> {
        let dev = dev();
        let f = UMST_SCALAR_CHANNEL_COUNT;
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
            scalar_features: Tensor::<B, 2>::zeros([n, f], &dev),
            vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &dev),
            matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev),
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions: None,
            displacement_bc_mask: Tensor::<B, 3>::ones([n, 3, 1], &dev),
            policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    fn mk_state(
        dev: &NdArrayDevice,
        n: usize,
        temp: f32,
        humidity: f32,
        alpha: f32,
        time: f32,
    ) -> ThmcState<B> {
        ThmcState {
            thermal: ThermalPlan {
                temperature: Tensor::full([1, n, 1], temp, dev),
            },
            hydro: HydrologicPlan {
                humidity: Tensor::full([1, n, 1], humidity, dev),
            },
            mechanical: MechanicalPlan {
                displacement: Tensor::zeros([1, n, 3], dev),
            },
            chemical: ChemicalPlan {
                reaction_extent: Tensor::full([1, n, 1], alpha, dev),
            },
            damage: Tensor::zeros([1, n, 1], dev),
            time,
        }
    }

    #[test]
    fn wire_gate_evidence_cement_default_strength_is_240_mpa() {
        let solver = ThmcSolver::default();
        assert!(
            (solver.gate_intrinsic_strength_mpa - 240.0).abs() < 1e-9,
            "expected cement SSOT 240 MPa default, got {}",
            solver.gate_intrinsic_strength_mpa
        );
    }

    #[test]
    fn attach_gate_evidence_identity_transition_is_admissible() {
        let n = 2usize;
        let umst = umst(n);
        let dev = dev();
        let pre = mk_state(&dev, n, 293.0_f32, 0.5_f32, 0.42_f32, 0.0_f32);
        let post = pre.clone();
        let solver = ThmcSolver::default();
        let stub = Stub;
        let evidence = umst_manifold::physics::solvers::ThmcSolverStep::attach_gate_evidence(
            &solver, &stub, &pre, &post, &umst, 1.0_f32,
        )
        .expect("identity lift should succeed");
        assert_eq!(evidence.transition.catalog_id, CD_TRANSITION_CATALOG_ID);
        assert_eq!(
            evidence.transition.admissibility,
            AdmissibilityToken::Admissible
        );
        assert!(evidence.wiring_tag.contains("GateCartridge"));
    }

    #[test]
    fn with_gate_cartridge_injection_uses_configured_witness() {
        use umst_manifold::runtime::gate::CdTransitionCartridge;

        let n = 2usize;
        let umst = umst(n);
        let dev = dev();
        let pre = mk_state(&dev, n, 293.0_f32, 0.5_f32, 0.42_f32, 0.0_f32);
        let post = pre.clone();
        let solver = ThmcSolver::default()
            .with_gate_intrinsic_strength_mpa(240.0)
            .with_gate_cartridge(&CdTransitionCartridge);
        let stub = Stub;
        let evidence = umst_manifold::physics::solvers::ThmcSolverStep::attach_gate_evidence(
            &solver, &stub, &pre, &post, &umst, 1.0_f32,
        )
        .expect("injected cartridge lift should succeed");
        assert_eq!(evidence.transition.catalog_id, CD_TRANSITION_CATALOG_ID);
        assert_eq!(
            evidence.transition.admissibility,
            AdmissibilityToken::Admissible
        );
    }
}
