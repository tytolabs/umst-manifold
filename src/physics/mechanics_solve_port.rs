// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Mechanics solve **port** trait — bar→Q1 adoption boundary (`integration-mechanics-trait`).
//!
//! Complements [`super::mechanics_operator::MechanicsOperator`] (tensor morphism without witness)
//! with a [`crate::solve_report::SolveReport`] at the port boundary for adjoint / gate consumers.
//!
//! **This wave:** trait + bar port stub only — no `thmc`, `fracture_field`, or `adjoint` rewiring.

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::solve_report::{PrecisionLane, ReportedSolve, SolveReport};

use super::mechanics::VectorMechanicsSolver;
use super::time_orchestration::MechanicsInnerLoopConfig;

/// Port morphism: quasi-static equilibrium \(K(\rho)u = f\) plus immutable [`SolveReport`] witness.
///
/// | Port | `PrecisionLane` | Feature gate | SSOT module (today) |
/// | --- | --- | --- | --- |
/// | Bar network | [`PrecisionLane::F64AdjointBarPcg`] | `mechanics-adjoint` | [`VectorMechanicsSolver`] |
/// | Q1 hex brick | [`PrecisionLane::HexQ1Pcg`] | `mechanics-adjoint-q1-hex` | `extruded_plate`, `q1_hex_elasticity` |
pub trait MechanicsSolvePort<B: Backend<FloatElem = f32>> {
    /// Numeric lane tag recorded on every [`SolveReport`] from this port.
    fn precision_lane(&self) -> PrecisionLane;

    /// Equilibrium solve returning `(u, σ, witness)`.
    ///
    /// `rel_tol` is the converged predicate tolerance forwarded into [`SolveReport::converged`].
    #[allow(clippy::too_many_arguments)]
    fn solve_equilibrium_reported(
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
        rel_tol: f32,
    ) -> (Tensor<B, 3>, Tensor<B, 4>, SolveReport);
}

/// Bar-network port — lifts [`VectorMechanicsSolver::solve_equilibrium_with_pcg_report`] into
/// [`SolveReport`] via [`ReportedSolve::into_solve_report`].
pub struct BarNetworkMechanicsSolvePort;

impl<B: Backend<FloatElem = f32>> MechanicsSolvePort<B> for BarNetworkMechanicsSolvePort {
    fn precision_lane(&self) -> PrecisionLane {
        PrecisionLane::F64AdjointBarPcg
    }

    fn solve_equilibrium_reported(
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
        rel_tol: f32,
    ) -> (Tensor<B, 3>, Tensor<B, 4>, SolveReport) {
        let (u, stress, pcg) = VectorMechanicsSolver::solve_equilibrium_with_pcg_report(
            displacement,
            coords,
            stiffness,
            body_force,
            edges_b1,
            damage,
            boundary_mask,
            cross_section_area,
            inner_cfg,
        );
        let report = pcg.into_solve_report(rel_tol, PrecisionLane::F64AdjointBarPcg);
        (u, stress, report)
    }
}

#[cfg(test)]
mod tests {
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
    fn bar_port_emits_converged_solve_report() {
        let (coords, edges, stiff, bf, mask, damage, area, cfg) = chain_fixture(2);
        let dev = NdArrayDevice::Cpu;
        let u0 = Tensor::<B, 3>::zeros([1, 2, 3], &dev);
        let rel_tol = 1e-6_f32;

        let (_u, _stress, report) = BarNetworkMechanicsSolvePort.solve_equilibrium_reported(
            u0, coords, stiff, bf, edges, damage, mask, area, &cfg, rel_tol,
        );

        assert_eq!(report.lane, PrecisionLane::F64AdjointBarPcg);
        assert!(report.converged());
    }
}
