// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Mechanics solve **port** trait — bar→Q1 adoption boundary (`integration-mechanics-trait`).
//!
//! Complements [`super::mechanics_operator::MechanicsOperator`] (tensor morphism without witness)
//! with a [`crate::solve_report::SolveReport`] at the port boundary for adjoint / gate consumers.
//!
//! **Wave 9:** first production consumer via [`bar_network_equilibrium_reported`] (THMC operator-split).
//! **FP P3.4:** displacement / damage / body_force operands are [`Field`] newtypes at the port boundary.

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::core::field::{BodyForceField, BoundaryMaskField, DamageField, DisplacementField, Field, StiffnessField};
use crate::physics::error::PhysicsError;
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
        displacement: DisplacementField<B>,
        coords: Tensor<B, 2>,
        stiffness: StiffnessField<B>,
        body_force: BodyForceField<B>,
        edges_b1: Tensor<B, 2, Int>,
        damage: DamageField<B>,
        boundary_mask: BoundaryMaskField<B>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
        rel_tol: f32,
    ) -> Result<(DisplacementField<B>, Tensor<B, 4>, SolveReport), PhysicsError>;
}

/// Bar-network port — lifts [`VectorMechanicsSolver::solve_equilibrium_with_pcg_report_typed`] into
/// [`SolveReport`] via [`ReportedSolve::into_solve_report`].
pub struct BarNetworkMechanicsSolvePort;

impl<B: Backend<FloatElem = f32>> MechanicsSolvePort<B> for BarNetworkMechanicsSolvePort {
    fn precision_lane(&self) -> PrecisionLane {
        PrecisionLane::F64AdjointBarPcg
    }

    fn solve_equilibrium_reported(
        &self,
        displacement: DisplacementField<B>,
        coords: Tensor<B, 2>,
        stiffness: StiffnessField<B>,
        body_force: BodyForceField<B>,
        edges_b1: Tensor<B, 2, Int>,
        damage: DamageField<B>,
        boundary_mask: BoundaryMaskField<B>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
        rel_tol: f32,
    ) -> Result<(DisplacementField<B>, Tensor<B, 4>, SolveReport), PhysicsError> {
        let (u, stress, pcg) = VectorMechanicsSolver::solve_equilibrium_with_pcg_report_typed(
            displacement,
            coords,
            stiffness,
            body_force,
            edges_b1,
            damage,
            boundary_mask,
            cross_section_area,
            inner_cfg,
        )?;
        let report = pcg.into_solve_report(rel_tol, PrecisionLane::F64AdjointBarPcg);
        if !report.converged() {
            return Err(PhysicsError::Diverged {
                eq_rel: report.rel_residual,
                pcg_iterations: report.iterations,
            });
        }
        Ok((u, stress, report))
    }
}

/// Fail-closed bar equilibrium via [`BarNetworkMechanicsSolvePort`] (`ops-mechanics-port-consumer`).
#[allow(clippy::too_many_arguments)]
pub fn bar_network_equilibrium_reported<B: Backend<FloatElem = f32>>(
    displacement: DisplacementField<B>,
    coords: Tensor<B, 2>,
    stiffness: StiffnessField<B>,
    body_force: BodyForceField<B>,
    edges_b1: Tensor<B, 2, Int>,
    damage: DamageField<B>,
    boundary_mask: BoundaryMaskField<B>,
    cross_section_area: f32,
    inner_cfg: &MechanicsInnerLoopConfig,
    rel_tol: f32,
) -> Result<(DisplacementField<B>, Tensor<B, 4>, SolveReport), PhysicsError> {
    let port = BarNetworkMechanicsSolvePort;
    let (u, stress, report) = port.solve_equilibrium_reported(
        displacement,
        coords,
        stiffness,
        body_force,
        edges_b1,
        damage,
        boundary_mask,
        cross_section_area,
        inner_cfg,
        rel_tol,
    )?;
    Ok((u, stress, report))
}

/// One-release tensor shim for legacy callers — wraps operands and unwraps displacement.
#[deprecated(since = "0.2.0", note = "use Field-typed bar_network_equilibrium_reported — FP P3.4")]
#[allow(clippy::too_many_arguments)]
pub fn bar_network_equilibrium_reported_from_tensors<B: Backend<FloatElem = f32>>(
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
) -> Result<(Tensor<B, 3>, Tensor<B, 4>, SolveReport), PhysicsError> {
    let (u, stress, report) = bar_network_equilibrium_reported(
        Field::new(displacement),
        coords,
        StiffnessField::from_tensor(stiffness),
        BodyForceField::from_tensor(body_force),
        edges_b1,
        Field::new(damage),
        BoundaryMaskField::from_tensor(boundary_mask),
        cross_section_area,
        inner_cfg,
        rel_tol,
    )?;
    Ok((u.into_tensor(), stress, report))
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    type ChainFixture = (
        Tensor<B, 2>,
        Tensor<B, 2, Int>,
        Tensor<B, 3>,
        BodyForceField<B>,
        BoundaryMaskField<B>,
        DamageField<B>,
        f32,
        MechanicsInnerLoopConfig,
    );

    fn chain_fixture(n: usize) -> ChainFixture {
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
        let body_force = BodyForceField::from_tensor(Tensor::from_data(
            Data::new(bf, Shape::new([1, n, 3])),
            &dev,
        ));

        let mut bm = vec![1.0_f32; n * 3];
        bm[0] = 0.0;
        bm[1] = 0.0;
        bm[2] = 0.0;
        let boundary_mask = BoundaryMaskField::from_tensor(Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &dev));

        let damage = Field::new(Tensor::<B, 3>::zeros([1, n, 1], &dev));
        let cfg = MechanicsInnerLoopConfig {
            max_cg_iterations: 500,
            cg_tolerance: 1e-6,
            pcg_tolerance: 1e-6,
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
        let u0 = Field::new(Tensor::<B, 3>::zeros([1, 2, 3], &dev));
        let rel_tol = 1e-6_f32;

        let (_u, _stress, report) = BarNetworkMechanicsSolvePort.solve_equilibrium_reported(
            u0,
            coords,
            StiffnessField::from_tensor(stiff),
            bf,
            edges,
            damage,
            mask,
            area,
            &cfg,
            rel_tol,
        )
            .expect("bar port equilibrium");

        assert_eq!(report.lane, PrecisionLane::F64AdjointBarPcg);
        assert!(report.converged());
    }

    #[test]
    fn bar_port_rejects_body_force_boundary_mask_operand_swap_at_compile_time() {
        fn accept_body_force(_: BodyForceField<B>) {}
        fn accept_boundary_mask(_: BoundaryMaskField<B>) {}

        let device = NdArrayDevice::Cpu;
        let raw = Tensor::<B, 3>::zeros([1, 2, 3], &device);
        accept_body_force(BodyForceField::from_tensor(raw.clone()));
        accept_boundary_mask(BoundaryMaskField::from_tensor(raw));
    }

    #[test]
    fn bar_port_rejects_displacement_damage_operand_swap_at_compile_time() {
        fn accept_displacement(_: DisplacementField<B>) {}
        fn accept_damage(_: DamageField<B>) {}

        let device = NdArrayDevice::Cpu;
        let raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        accept_damage(Field::new(raw.clone()));
        // `accept_displacement(Field::new(raw))` would not compile — distinct space markers.
    }
}
