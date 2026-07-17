// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Quasi-static mechanics **morphism** trait (integration-contracts D2).
//!
//! Two discretizations coexist today: **bar network** ([`VectorMechanicsSolver`]) and **Q1 hex**
//! (`extruded_plate` / `q1_hex_elasticity`). This trait is the SSOT boundary for Wave 3 consumer
//! migration; **no call sites are ported in this wave**.

use burn::tensor::{backend::Backend, Int, Tensor};

use super::mechanics::VectorMechanicsSolver;
use super::error::PhysicsError;
use super::time_orchestration::MechanicsInnerLoopConfig;

/// Equilibrium morphism \(K(\rho)\,u = f\) on the DEC 1-skeleton (bar today; Q1 hex in Wave 3).
pub trait MechanicsOperator<B: Backend<FloatElem = f32>> {
    #[allow(clippy::too_many_arguments)]
    fn solve_equilibrium(
        &self,
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError>;
}

/// Deprecated bar-network adapter — bit-identical to [`VectorMechanicsSolver::solve_equilibrium`].
#[deprecated(
    since = "0.1.0",
    note = "Wave 3: migrate consumers to `dyn MechanicsOperator` or Q1 operator; bar remains interim SSOT"
)]
pub struct BarNetworkMechanicsAdapter;

#[allow(deprecated)]
impl<B: Backend<FloatElem = f32>> MechanicsOperator<B> for BarNetworkMechanicsAdapter {
    fn solve_equilibrium(
        &self,
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError> {
        VectorMechanicsSolver::solve_equilibrium(
            displacement,
            coords,
            stiffness,
            body_force,
            edges_b1,
            damage,
            boundary_mask,
            cross_section_area,
            inner_cfg,
        )
    }
}

impl<B: Backend<FloatElem = f32>> MechanicsOperator<B> for VectorMechanicsSolver {
    fn solve_equilibrium(
        &self,
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError> {
        VectorMechanicsSolver::solve_equilibrium(
            displacement,
            coords,
            stiffness,
            body_force,
            edges_b1,
            damage,
            boundary_mask,
            cross_section_area,
            inner_cfg,
        )
    }
}

#[cfg(test)]
#[allow(clippy::type_complexity)]
mod parity_tests {
    use super::*;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn chain_fixture(
        n: usize,
    ) -> (
        Tensor<B, 2>,
        Tensor<B, 2, Int>,
        Tensor<B, 3>,
        Tensor<B, 3>,
        Tensor<B, 3>,
        Tensor<B, 3>,
        f32,
        MechanicsInnerLoopConfig,
    ) {
        let dev = NdArrayDevice::Cpu;
        let dx = 0.1_f32;
        let e = 200e9_f32;
        let a_sec = 0.01_f32;

        let mut coords_data = Vec::with_capacity(n * 3);
        for i in 0..n {
            coords_data.push(i as f32 * dx);
            coords_data.push(0.0);
            coords_data.push(0.0);
        }
        let coords: Tensor<B, 2> =
            Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);

        let mut edges = Vec::with_capacity((n - 1) * 2);
        for eid in 0..(n - 1) {
            edges.push(eid as i64);
        }
        for eid in 0..(n - 1) {
            edges.push((eid + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, n - 1])), &dev);

        let mut stiff = vec![0.0_f32; n * 2];
        for i in 0..n {
            stiff[i * 2] = e;
            stiff[i * 2 + 1] = 0.3;
        }
        let stiffness = Tensor::from_data(Data::new(stiff, Shape::new([1, n, 2])), &dev);

        let mut bf = vec![0.0_f32; n * 3];
        bf[(n - 1) * 3] = 1000.0;
        let body_force = Tensor::from_data(Data::new(bf, Shape::new([1, n, 3])), &dev);

        let mut bm = vec![1.0_f32; n * 3];
        bm[0] = 0.0;
        bm[1] = 0.0;
        bm[2] = 0.0;
        let boundary_mask = Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &dev);

        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let cfg = MechanicsInnerLoopConfig {
            max_cg_iterations: 500,
            cg_tolerance: 1e-10,
            pcg_tolerance: 1e-10,
            use_preconditioner: true,
            max_equilibrium_substeps: 1,
        };

        (
            coords,
            edges_b1,
            stiffness,
            body_force,
            boundary_mask,
            damage,
            a_sec,
            cfg,
        )
    }

    #[test]
    fn bar_adapter_two_node_bit_identical_to_direct() {
        let (coords, edges, stiff, bf, mask, damage, area, cfg) = chain_fixture(2);
        let dev = NdArrayDevice::Cpu;
        let u0 = Tensor::<B, 3>::zeros([1, 2, 3], &dev);

        let direct = VectorMechanicsSolver::solve_equilibrium(
            u0.clone(),
            coords.clone(),
            stiff.clone(),
            bf.clone(),
            edges.clone(),
            damage.clone(),
            mask.clone(),
            area,
            &cfg,
        );
        #[allow(deprecated)]
        let via_trait = BarNetworkMechanicsAdapter
            .solve_equilibrium(u0, coords, stiff, bf, edges, damage, mask, area, &cfg);
        assert_eq!(direct.0.into_data().value, via_trait.0.into_data().value);
        assert_eq!(direct.1.into_data().value, via_trait.1.into_data().value);
    }

    #[test]
    fn bar_adapter_nine_node_bit_identical_to_direct() {
        let (coords, edges, stiff, bf, mask, damage, area, cfg) = chain_fixture(9);
        let dev = NdArrayDevice::Cpu;
        let n = 9usize;
        let u0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);

        let direct = VectorMechanicsSolver::solve_equilibrium(
            u0.clone(),
            coords.clone(),
            stiff.clone(),
            bf.clone(),
            edges.clone(),
            damage.clone(),
            mask.clone(),
            area,
            &cfg,
        );
        #[allow(deprecated)]
        let via_trait = BarNetworkMechanicsAdapter
            .solve_equilibrium(u0, coords, stiff, bf, edges, damage, mask, area, &cfg);
        assert_eq!(direct.0.into_data().value, via_trait.0.into_data().value);
        assert_eq!(direct.1.into_data().value, via_trait.1.into_data().value);
    }
}
