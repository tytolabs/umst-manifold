// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Compile witness: [`MechanicsSolvePort::Report`] associated type lifts to [`SolveReport`].

#![cfg(feature = "mechanics-adjoint")]

use burn_ndarray::NdArray;
use umst_manifold::physics::mechanics::port::{BarNetworkMechanicsSolvePort, MechanicsSolvePort};
use umst_manifold::solve_report::{PrecisionLane, ReportedSolve, SolveReport};

type B = NdArray<f32>;

#[test]
fn mechanics_solve_port_associated_report_compiles() {
    type BarPort = dyn MechanicsSolvePort<B, Report = umst_manifold::physics::mechanics::BarNetworkPcgReport>;

    fn object_safe(_: &BarPort) {}

    let port = BarNetworkMechanicsSolvePort;
    object_safe(&port);

    type PortReport = <BarNetworkMechanicsSolvePort as MechanicsSolvePort<B>>::Report;

    let witness: SolveReport = PortReport::default().into_solve_report(
        1e-6,
        PrecisionLane::F64AdjointBarPcg,
    );

    assert_eq!(witness.lane, PrecisionLane::F64AdjointBarPcg);
}
