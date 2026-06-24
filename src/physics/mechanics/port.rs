// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Mechanics solve **port** trait — bar→Q1 adoption boundary (`integration-mechanics-trait`).
//!
//! Complements [`crate::physics::mechanics_operator::MechanicsOperator`] (tensor morphism without witness)
//! with lane telemetry at the port boundary that lifts to [`crate::solve_report::SolveReport`].
//!
//! **This wave:** trait + bar port stub only — no `thmc`, `fracture_field`, or `adjoint` rewiring.

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::physics::time_orchestration::MechanicsInnerLoopConfig;
use crate::solve_report::{PrecisionLane, ReportedSolve};

use super::{BarNetworkPcgReport, VectorMechanicsSolver};

/// Port morphism: quasi-static equilibrium \(K(\rho)u = f\) plus lane telemetry lifting to [`SolveReport`].
///
/// | Port | `PrecisionLane` | Feature gate | SSOT module (today) |
/// | --- | --- | --- | --- |
/// | Bar network | [`PrecisionLane::F64AdjointBarPcg`] | `mechanics-adjoint` | [`VectorMechanicsSolver`] |
/// | Q1 hex brick | [`PrecisionLane::HexQ1Pcg`] | `mechanics-adjoint-q1-hex` | `extruded_plate`, `q1_hex_elasticity` |
pub trait MechanicsSolvePort<B: Backend<FloatElem = f32>> {
    /// Lane-specific telemetry — every impl lifts to [`SolveReport`] via [`ReportedSolve`].
    type Report: ReportedSolve;

    /// Numeric lane tag recorded on every [`SolveReport`] from this port.
    fn precision_lane(&self) -> PrecisionLane;

    /// Equilibrium solve returning `(u, σ, lane_report)`.
    ///
    /// `rel_tol` is the converged predicate tolerance forwarded into [`SolveReport::converged`]
    /// when callers lift `Report` with [`Self::into_solve_report`].
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
    ) -> (Tensor<B, 3>, Tensor<B, 4>, Self::Report);
}

/// Bar-network port — lifts [`VectorMechanicsSolver::solve_equilibrium_with_pcg_report`] into
/// [`BarNetworkPcgReport`] then [`SolveReport`] via [`ReportedSolve`].
pub struct BarNetworkMechanicsSolvePort;

impl<B: Backend<FloatElem = f32>> MechanicsSolvePort<B> for BarNetworkMechanicsSolvePort {
    type Report = BarNetworkPcgReport;

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
        _rel_tol: f32,
    ) -> (Tensor<B, 3>, Tensor<B, 4>, Self::Report) {
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
        (u, stress, pcg)
    }
}
