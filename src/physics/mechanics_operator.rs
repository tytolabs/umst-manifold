// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Quasi-static mechanics **morphism** trait (integration-contracts D2).
//!
//! Two discretizations coexist today: **bar network** ([`VectorMechanicsSolver`]) and **Q1 hex**
//! (`extruded_plate` / `q1_hex_elasticity`). This trait is the SSOT boundary for Wave 3 consumer
//! migration; **no call sites are ported in this wave**.
//!
//! # Honest boundary (W29-059)
//!
//! Trait + bar adapters are a **research-lane** equilibrium morphism SSOT. Bar parity harnesses
//! pass unit tests. Q1-hex `MechanicsOperator` impl and production consumer migration remain
//! open. Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`.

/// W29 deepen cell — mechanics-operator honest fence bundle.
pub const W29_MECHANICS_OPERATOR_DEEPEN_CELL: &str = "W29-059-MECHANICS_OPERATOR";

/// Honest posture tag — bar trait adapters landed; Q1 / production migration refused.
pub const MECHANICS_OPERATOR_POSTURE_TAG: &str = "honest-mechanics-operator-research-lane";

/// Honest physics posture — bar parity tests pass; does not certify fleet physics GREEN.
pub const MECHANICS_OPERATOR_PHYSICS_GREEN: bool = false;

/// Production consumer wiring — Wave 3 call-site port not claimed by this module.
pub const MECHANICS_OPERATOR_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by mechanics_operator alone.
pub const MECHANICS_OPERATOR_MASTER: bool = false;

/// Whether the bar-network trait adapters are landed (deprecated adapter + VectorMechanicsSolver).
pub const MECHANICS_OPERATOR_BAR_ADAPTERS_LANDED: bool = true;

/// Whether a Q1-hex `MechanicsOperator` impl is landed in this module (Wave 3 — still open).
pub const MECHANICS_OPERATOR_Q1_HEX_IMPL_LANDED: bool = false;

/// Whether production consumers have been migrated onto `dyn MechanicsOperator` (Wave 3 — open).
pub const MECHANICS_OPERATOR_CONSUMER_MIGRATION_LANDED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const MECHANICS_OPERATOR_HONEST_FENCE: &str = "mechanics_operator_trait_landed=true bar_adapters_landed=true q1_hex_impl_landed=false consumer_migration_landed=false production_wired=false master_composition_wired=false physics_green=false";

/// Typed probe for mechanics-operator posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MechanicsOperatorPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub bar_adapters_landed: bool,
    pub q1_hex_impl_landed: bool,
    pub consumer_migration_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for mechanics operator morphism.
#[must_use]
pub fn mechanics_operator_honest_posture_bundle() -> MechanicsOperatorPostureProbe {
    MechanicsOperatorPostureProbe {
        physics_green: MECHANICS_OPERATOR_PHYSICS_GREEN,
        production_wired: MECHANICS_OPERATOR_PRODUCTION_WIRED,
        master: MECHANICS_OPERATOR_MASTER,
        bar_adapters_landed: MECHANICS_OPERATOR_BAR_ADAPTERS_LANDED,
        q1_hex_impl_landed: MECHANICS_OPERATOR_Q1_HEX_IMPL_LANDED,
        consumer_migration_landed: MECHANICS_OPERATOR_CONSUMER_MIGRATION_LANDED,
        honest_fence: MECHANICS_OPERATOR_HONEST_FENCE,
        posture_tag: MECHANICS_OPERATOR_POSTURE_TAG,
        deepen_cell: W29_MECHANICS_OPERATOR_DEEPEN_CELL,
    }
}

/// Bar trait SSOT landed with Q1 / production / master composition honestly open.
#[must_use]
pub fn mechanics_operator_posture_honest(probe: &MechanicsOperatorPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.bar_adapters_landed
        && !probe.q1_hex_impl_landed
        && !probe.consumer_migration_landed
        && probe.honest_fence.contains("mechanics_operator_trait_landed=true")
        && probe.honest_fence.contains("bar_adapters_landed=true")
        && probe.honest_fence.contains("q1_hex_impl_landed=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Validate posture honesty; returns Err with a static reason on fence violation.
pub fn validate_mechanics_operator_posture_honesty() -> Result<(), &'static str> {
    let probe = mechanics_operator_honest_posture_bundle();
    if !mechanics_operator_posture_honest(&probe) {
        return Err("mechanics_operator_posture_honest failed");
    }
    if probe.physics_green || MECHANICS_OPERATOR_PHYSICS_GREEN {
        return Err("invented physics_green");
    }
    if probe.production_wired || MECHANICS_OPERATOR_PRODUCTION_WIRED {
        return Err("invented production_wired");
    }
    if probe.master || MECHANICS_OPERATOR_MASTER {
        return Err("invented master");
    }
    if probe.q1_hex_impl_landed || MECHANICS_OPERATOR_Q1_HEX_IMPL_LANDED {
        return Err("invented q1_hex_impl_landed");
    }
    if probe.consumer_migration_landed || MECHANICS_OPERATOR_CONSUMER_MIGRATION_LANDED {
        return Err("invented consumer_migration_landed");
    }
    Ok(())
}

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::core::field::{BodyForceField, BoundaryMaskField, Field, StiffnessField};

use super::mechanics::VectorMechanicsSolver;
use super::error::PhysicsError;
use super::time_orchestration::MechanicsInnerLoopConfig;

/// Equilibrium morphism \(K(\rho)\,u = f\) on the DEC 1-skeleton (bar today; Q1 hex in Wave 3).
///
/// **FP XS-3 step 4:** `stiffness` is [`StiffnessField`] so callers cannot pass damage/α tensors
/// where E/ν cat is required (mirrors [`super::mechanics_solve_port::MechanicsSolvePort`]).
pub trait MechanicsOperator<B: Backend<FloatElem = f32>> {
    #[allow(clippy::too_many_arguments)]
    fn solve_equilibrium(
        &self,
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: StiffnessField<B>,
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
        stiffness: StiffnessField<B>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError> {
        let (u, stress) = VectorMechanicsSolver::solve_equilibrium_typed(
            Field::new(displacement),
            coords,
            stiffness,
            BodyForceField::from_tensor(body_force),
            edges_b1,
            Field::new(damage),
            BoundaryMaskField::from_tensor(boundary_mask),
            cross_section_area,
            inner_cfg,
        )?;
        Ok((u.into_tensor(), stress))
    }
}

impl<B: Backend<FloatElem = f32>> MechanicsOperator<B> for VectorMechanicsSolver {
    fn solve_equilibrium(
        &self,
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: StiffnessField<B>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError> {
        let (u, stress) = VectorMechanicsSolver::solve_equilibrium_typed(
            Field::new(displacement),
            coords,
            stiffness,
            BodyForceField::from_tensor(body_force),
            edges_b1,
            Field::new(damage),
            BoundaryMaskField::from_tensor(boundary_mask),
            cross_section_area,
            inner_cfg,
        )?;
        Ok((u.into_tensor(), stress))
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
        StiffnessField<B>,
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
        let stiffness = StiffnessField::from_tensor(Tensor::from_data(
            Data::new(stiff, Shape::new([1, n, 2])),
            &dev,
        ));

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
    fn mechanics_operator_honest_fence_consts_refuse_green_production_master() {
        assert!(!MECHANICS_OPERATOR_PHYSICS_GREEN);
        assert!(!MECHANICS_OPERATOR_PRODUCTION_WIRED);
        assert!(!MECHANICS_OPERATOR_MASTER);
        assert!(MECHANICS_OPERATOR_BAR_ADAPTERS_LANDED);
        assert!(!MECHANICS_OPERATOR_Q1_HEX_IMPL_LANDED);
        assert!(!MECHANICS_OPERATOR_CONSUMER_MIGRATION_LANDED);
        assert!(MECHANICS_OPERATOR_HONEST_FENCE.contains("production_wired=false"));
        assert!(MECHANICS_OPERATOR_HONEST_FENCE.contains("physics_green=false"));
        assert!(MECHANICS_OPERATOR_HONEST_FENCE.contains("master_composition_wired=false"));
        assert!(MECHANICS_OPERATOR_HONEST_FENCE.contains("q1_hex_impl_landed=false"));
        assert_eq!(
            W29_MECHANICS_OPERATOR_DEEPEN_CELL,
            "W29-059-MECHANICS_OPERATOR"
        );
    }

    #[test]
    fn mechanics_operator_posture_probe_honest() {
        let probe = mechanics_operator_honest_posture_bundle();
        assert!(mechanics_operator_posture_honest(&probe));
        assert_eq!(probe.deepen_cell, W29_MECHANICS_OPERATOR_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, MECHANICS_OPERATOR_POSTURE_TAG);
        validate_mechanics_operator_posture_honesty()
            .expect("validate_mechanics_operator_posture_honesty");
    }

    #[test]
    fn vector_solver_via_trait_two_node_bit_identical_to_direct() {
        let (coords, edges, stiff, bf, mask, damage, area, cfg) = chain_fixture(2);
        let dev = NdArrayDevice::Cpu;
        let u0 = Tensor::<B, 3>::zeros([1, 2, 3], &dev);

        let direct = VectorMechanicsSolver::solve_equilibrium_typed(
            Field::new(u0.clone()),
            coords.clone(),
            stiff.clone(),
            BodyForceField::from_tensor(bf.clone()),
            edges.clone(),
            Field::new(damage.clone()),
            BoundaryMaskField::from_tensor(mask.clone()),
            area,
            &cfg,
        )
        .expect("solve_equilibrium_typed");
        let via_trait = VectorMechanicsSolver
            .solve_equilibrium(u0, coords, stiff, bf, edges, damage, mask, area, &cfg)
            .expect("MechanicsOperator::solve_equilibrium");
        assert_eq!(
            direct.0.clone().into_tensor().into_data().value,
            via_trait.0.into_data().value
        );
        assert_eq!(direct.1.into_data().value, via_trait.1.into_data().value);
    }

    #[test]
    fn bar_adapter_two_node_bit_identical_to_direct() {
        let (coords, edges, stiff, bf, mask, damage, area, cfg) = chain_fixture(2);
        let dev = NdArrayDevice::Cpu;
        let u0 = Tensor::<B, 3>::zeros([1, 2, 3], &dev);

        let direct = VectorMechanicsSolver::solve_equilibrium_typed(
            Field::new(u0.clone()),
            coords.clone(),
            stiff.clone(),
            BodyForceField::from_tensor(bf.clone()),
            edges.clone(),
            Field::new(damage.clone()),
            BoundaryMaskField::from_tensor(mask.clone()),
            area,
            &cfg,
        )
        .expect("solve_equilibrium_typed");
        #[allow(deprecated)]
        let via_trait = BarNetworkMechanicsAdapter
            .solve_equilibrium(u0, coords, stiff, bf, edges, damage, mask, area, &cfg)
            .expect("solve_equilibrium");
        assert_eq!(
            direct.0.clone().into_tensor().into_data().value,
            via_trait.0.into_data().value
        );
        assert_eq!(direct.1.into_data().value, via_trait.1.into_data().value);
    }

    #[test]
    fn bar_adapter_nine_node_bit_identical_to_direct() {
        let (coords, edges, stiff, bf, mask, damage, area, cfg) = chain_fixture(9);
        let dev = NdArrayDevice::Cpu;
        let n = 9usize;
        let u0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);

        let direct = VectorMechanicsSolver::solve_equilibrium_typed(
            Field::new(u0.clone()),
            coords.clone(),
            stiff.clone(),
            BodyForceField::from_tensor(bf.clone()),
            edges.clone(),
            Field::new(damage.clone()),
            BoundaryMaskField::from_tensor(mask.clone()),
            area,
            &cfg,
        )
        .expect("solve_equilibrium_typed");
        #[allow(deprecated)]
        let via_trait = BarNetworkMechanicsAdapter
            .solve_equilibrium(u0, coords, stiff, bf, edges, damage, mask, area, &cfg)
            .expect("solve_equilibrium");
        assert_eq!(
            direct.0.clone().into_tensor().into_data().value,
            via_trait.0.into_data().value
        );
        assert_eq!(direct.1.into_data().value, via_trait.1.into_data().value);
    }
}
