// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Mechanics solve **port** trait — bar→Q1 adoption boundary (`integration-mechanics-trait`).
//!
//! Complements [`super::mechanics_operator::MechanicsOperator`] (tensor morphism without witness)
//! with a [`crate::solve_report::SolveReport`] at the port boundary for adjoint / gate consumers.
//!
//! **Wave 9:** first production consumer via [`bar_network_equilibrium_reported`] (THMC operator-split).
//! **FP P3.4:** displacement / damage / body_force operands are [`Field`] newtypes at the port boundary.
//!
//! # Honest boundary (W29-060)
//!
//! Bar-network [`MechanicsSolvePort`] + THMC consumer helper are landed with Field-typed operands.
//! Q1-hex remains **catalogued** (`PrecisionLane::HexQ1Pcg`) without a trait impl on this surface.
//! Unit contracts: `cargo test -p umst-manifold mechanics_solve_port`. Not physics GREEN, not
//! `PRODUCTION_WIRED`, not `MASTER` / OP-5.

/// W29 deepen cell — mechanics solve port honest fence bundle.
pub const W29_MECHANICS_SOLVE_PORT_DEEPEN_CELL: &str = "W29-060-MECHANICS_SOLVE_PORT";

/// Honest posture tag — bar SolveReport port landed; Q1 hex trait impl deferred.
pub const MECHANICS_SOLVE_PORT_POSTURE_TAG: &str = "honest-bar-solve-report-port-research-lane";

/// Honest physics posture — port unit contracts pass; does not certify fleet physics GREEN.
pub const MECHANICS_SOLVE_PORT_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by the bar port / THMC helper alone.
pub const MECHANICS_SOLVE_PORT_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const MECHANICS_SOLVE_PORT_MASTER: bool = false;

/// Bar-network [`MechanicsSolvePort`] + fail-closed [`SolveReport`] consumer landed.
pub const MECHANICS_SOLVE_PORT_BAR_LANDED: bool = true;

/// THMC operator-split consumer via [`bar_network_equilibrium_reported`].
pub const MECHANICS_SOLVE_PORT_THMC_CONSUMER_WIRED: bool = true;

/// FP P3.4 Field newtypes at the port boundary (displacement / damage / body_force / mask / stiffness).
pub const MECHANICS_SOLVE_PORT_FIELD_TYPED: bool = true;

/// Q1-hex [`MechanicsSolvePort`] impl — catalogued in the lane table; trait surface not landed here.
pub const MECHANICS_SOLVE_PORT_Q1_HEX_IMPL: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const MECHANICS_SOLVE_PORT_HONEST_FENCE: &str =
    "bar_port_landed=true thmc_consumer_wired=true field_typed_operands=true q1_hex_port_impl=false production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!MECHANICS_SOLVE_PORT_PHYSICS_GREEN);
const _: () = assert!(!MECHANICS_SOLVE_PORT_PRODUCTION_WIRED);
const _: () = assert!(!MECHANICS_SOLVE_PORT_MASTER);
const _: () = assert!(!MECHANICS_SOLVE_PORT_Q1_HEX_IMPL);
const _: () = assert!(MECHANICS_SOLVE_PORT_BAR_LANDED);
const _: () = assert!(MECHANICS_SOLVE_PORT_THMC_CONSUMER_WIRED);
const _: () = assert!(MECHANICS_SOLVE_PORT_FIELD_TYPED);

/// Typed probe for mechanics solve-port posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MechanicsSolvePortPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub bar_port_landed: bool,
    pub thmc_consumer_wired: bool,
    pub field_typed_operands: bool,
    pub q1_hex_port_impl: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the mechanics solve port.
#[must_use]
pub fn mechanics_solve_port_honest_posture_bundle() -> MechanicsSolvePortPostureProbe {
    MechanicsSolvePortPostureProbe {
        physics_green: MECHANICS_SOLVE_PORT_PHYSICS_GREEN,
        production_wired: MECHANICS_SOLVE_PORT_PRODUCTION_WIRED,
        master: MECHANICS_SOLVE_PORT_MASTER,
        bar_port_landed: MECHANICS_SOLVE_PORT_BAR_LANDED,
        thmc_consumer_wired: MECHANICS_SOLVE_PORT_THMC_CONSUMER_WIRED,
        field_typed_operands: MECHANICS_SOLVE_PORT_FIELD_TYPED,
        q1_hex_port_impl: MECHANICS_SOLVE_PORT_Q1_HEX_IMPL,
        honest_fence: MECHANICS_SOLVE_PORT_HONEST_FENCE,
        posture_tag: MECHANICS_SOLVE_PORT_POSTURE_TAG,
        deepen_cell: W29_MECHANICS_SOLVE_PORT_DEEPEN_CELL,
    }
}

/// Bar SolveReport port landed with Q1/production/master/GREEN composition honestly open.
#[must_use]
pub fn mechanics_solve_port_posture_honest(probe: &MechanicsSolvePortPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.q1_hex_port_impl
        && probe.bar_port_landed
        && probe.thmc_consumer_wired
        && probe.field_typed_operands
        && probe.honest_fence.contains("bar_port_landed=true")
        && probe.honest_fence.contains("q1_hex_port_impl=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER / fake Q1-port claims on this surface.
#[must_use]
pub fn mechanics_solve_port_refuse_overclaim(
    probe: &MechanicsSolvePortPostureProbe,
) -> Result<(), &'static str> {
    if probe.physics_green {
        return Err(
            "MECHANICS_SOLVE_PORT_PHYSICS_GREEN must stay false until fleet physics closes",
        );
    }
    if probe.production_wired {
        return Err(
            "MECHANICS_SOLVE_PORT_PRODUCTION_WIRED must stay false until embodied loop closes",
        );
    }
    if probe.master {
        return Err("MECHANICS_SOLVE_PORT_MASTER must stay false — not claimed by bar port alone");
    }
    if probe.q1_hex_port_impl {
        return Err("Q1-hex MechanicsSolvePort impl must stay false until HexQ1 port lands");
    }
    if !mechanics_solve_port_posture_honest(probe) {
        return Err("mechanics_solve_port posture fence inconsistent");
    }
    Ok(())
}

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::core::field::{
    BodyForceField, BoundaryMaskField, DamageField, DisplacementField, Field, StiffnessField,
};
use crate::physics::error::PhysicsError;
use crate::solve_report::{PrecisionLane, ReportedSolve, SolveReport};

use super::mechanics::VectorMechanicsSolver;
use super::time_orchestration::MechanicsInnerLoopConfig;

/// Port morphism: quasi-static equilibrium \(K(\rho)u = f\) plus immutable [`SolveReport`] witness.
///
/// | Port | `PrecisionLane` | Feature gate | SSOT module (today) |
/// | --- | --- | --- | --- |
/// | Bar network | [`PrecisionLane::F64AdjointBarPcg`] | `mechanics-adjoint` | [`VectorMechanicsSolver`] |
/// | Q1 hex brick | [`PrecisionLane::HexQ1Pcg`] | `mechanics-adjoint-q1-hex` | `extruded_plate`, `q1_hex_elasticity` (catalog only — [`MECHANICS_SOLVE_PORT_Q1_HEX_IMPL`]=false) |
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
#[deprecated(
    since = "0.2.0",
    note = "use Field-typed bar_network_equilibrium_reported — FP P3.4"
)]
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
        let boundary_mask = BoundaryMaskField::from_tensor(Tensor::from_data(
            Data::new(bm, Shape::new([1, n, 3])),
            &dev,
        ));

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

        let (_u, _stress, report) = BarNetworkMechanicsSolvePort
            .solve_equilibrium_reported(
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

    #[test]
    fn mechanics_solve_port_honest_fence_blocks_production_master_green() {
        let probe = mechanics_solve_port_honest_posture_bundle();
        assert_eq!(probe.deepen_cell, W29_MECHANICS_SOLVE_PORT_DEEPEN_CELL);
        assert!(mechanics_solve_port_posture_honest(&probe));
        mechanics_solve_port_refuse_overclaim(&probe).expect("honest refuse");
        assert!(MECHANICS_SOLVE_PORT_HONEST_FENCE.contains("production_wired=false"));
        assert!(MECHANICS_SOLVE_PORT_HONEST_FENCE.contains("physics_green=false"));
        assert!(MECHANICS_SOLVE_PORT_HONEST_FENCE.contains("q1_hex_port_impl=false"));
        assert!(!MECHANICS_SOLVE_PORT_PHYSICS_GREEN);
        assert!(!MECHANICS_SOLVE_PORT_PRODUCTION_WIRED);
        assert!(!MECHANICS_SOLVE_PORT_MASTER);
        assert!(!MECHANICS_SOLVE_PORT_Q1_HEX_IMPL);
    }

    #[test]
    fn bar_network_consumer_emits_converged_solve_report() {
        let (coords, edges, stiff, bf, mask, damage, area, cfg) = chain_fixture(2);
        let dev = NdArrayDevice::Cpu;
        let u0 = Field::new(Tensor::<B, 3>::zeros([1, 2, 3], &dev));
        let rel_tol = 1e-6_f32;

        let (_u, _stress, report) = bar_network_equilibrium_reported(
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
        .expect("THMC bar consumer equilibrium");

        assert_eq!(report.lane, PrecisionLane::F64AdjointBarPcg);
        assert!(report.converged());
        assert_eq!(
            BarNetworkMechanicsSolvePort.precision_lane(),
            PrecisionLane::F64AdjointBarPcg
        );
    }

    #[test]
    fn bar_port_fail_closed_on_impossible_rel_tol() {
        let (coords, edges, stiff, bf, mask, damage, area, cfg) = chain_fixture(2);
        let dev = NdArrayDevice::Cpu;
        let u0 = Field::new(Tensor::<B, 3>::zeros([1, 2, 3], &dev));
        // Positive but unreachable relative tolerance → Diverged (not a silent Ok).
        let rel_tol = 1e-30_f32;

        let err = BarNetworkMechanicsSolvePort
            .solve_equilibrium_reported(
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
            .expect_err("unreachable rel_tol must fail closed");

        match err {
            PhysicsError::Diverged { .. } => {}
            other => panic!("expected Diverged, got {other:?}"),
        }
    }
}
