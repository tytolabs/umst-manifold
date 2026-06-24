// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Single delegation surface for topology-coupled physics stepping.
//!
//! Callers that want a **named pipeline** (transport → chemistry → mechanics → fracture → optional
//! rheology) should use [`TopologyPhysicsOrchestrator::run_plan_step`]. That method **folds** the
//! default [`TopologyPlanIntent`] sequence (today a singleton) into successive state updates and
//! ultimately forwards each coupled tick to [`crate::physics::solvers::ThmcSolver::step`] — it does
//! **not** duplicate that implementation.
//!
//! ## Integration contract (execution order — design intent)
//!
//! The canonical ordering below is what higher-level planners should assume when composing solvers.
//! Today, the solver sub-steps live inside [`crate::physics::solvers::ThmcSolver`] (see its module
//! docs and `--features solver-experimental` implementation). This module documents the **contract**;
//! evolution of `ThmcSolver` is expected to stay aligned with these phases rather than scattering
//! duplicate loops across the codebase.
//!
//! 1. **Laplacian transport hints** — discrete diffusion / Laplacian-style updates on nodal fields
//!    (thermal, hydrologic proxies) using graph topology and masks (e.g. damage-degraded flux).
//! 2. **Chemistry** — reaction extent / reaction channels on [`crate::physics::solvers::ChemicalPlan`];
//!    **placeholder** until kinetics are wired; must not silently change conserved quantities without
//!    documenting closures via [`crate::core::traits::IScienceCartridge`].
//! 3. **Mechanics** — equilibrium or pseudo-time step for displacement / stress; requires consistent
//!    embeddings when Euclidean coordinates exist (integer-only manifold indices skip sub-solves until
//!    an embedding map is supplied — see `Thmc` solver docs).
//! 4. **Fracture** — phase-field or damage evolution coupled to strain / energy release proxies on
//!    the same node batch as transport.
//! 5. **Rheology (optional)** — Bingham / flow-like updates ([`crate::physics::solvers::BinghamFlowSolver`])
//!    are **not** folded into [`crate::physics::solvers::ThmcSolver::step`] yet. When pore flow must run in
//!    the same tick, compose **after** `run_plan_step` with explicit velocity/pressure tensors and document
//!    data dependencies in your cartridge pipeline.
//!
//! ## Errors (default builds)
//!
//! [`crate::physics::solvers::ThmcSolver::step`] returns `Err` when `solver-experimental` is disabled.
//! The orchestrator forwards that `Result` so callers can branch without panicking.
//!
//! ## Categorical vocabulary ([`Category-of-Material-Updates`](../../docs/Category-of-Material-Updates.md))
//!
//! - **Object:** [`crate::core::tensors::UnifiedMaterialStateTensor`] plus inner [`crate::physics::solvers::ThmcState`]
//!   carried across a tick.
//! - **Morphism / sequential composition:** one orchestrated plan step is a **fold** over
//!   [`TopologyPlanIntent`] values; each `ThmcCoupledIntegration` intent applies the composed
//!   transport → chemistry → mechanics → fracture chain inside [`ThmcSolver::step`]; optional rheology
//!   stays **outside** the default tick when documented (same table as the Category note).
//! - **Cartridge injection:** generic `C: IScienceCartridge<B>` supplies constitutive closures inside
//!   the solver without expanding the orchestrator’s surface area.
//! - **Second law hook:** results ultimately feed [`crate::core::traits::PhysicalResult`] fields
//!   (`dissipation`, `free_energy`, …) via cartridge + merge paths—keep numerical dissipation
//!   consistent with the same step that updates damage/temperature channels.
//!
//! Epic cross-ref: `fp-categorical-v04`.

use std::ops::ControlFlow;

use burn::tensor::backend::Backend;

use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;
use crate::physics::solvers::fixed_point::repeat_controlled;
use crate::physics::solvers::{ThmcSolver, ThmcState};

/// Back-compat alias used in v0.4 planning notes (`PhysicsOrchestrator` ↔ topology plan driver).
pub type PhysicsOrchestrator = TopologyPhysicsOrchestrator;

/// One named intent in a topology physics **plan** folded by [`TopologyPhysicsOrchestrator::fold_plan_step`].
///
/// Vocabulary aligns with *sequential composition* in [`Category-of-Material-Updates`](../../docs/Category-of-Material-Updates.md):
/// each variant names a morphism family the orchestrator schedules; expansion of the enum is how new
/// outer-level steps join the fold without rewriting `run_plan_step` call sites.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TopologyPlanIntent {
    /// Coupled THMC integration tick: one call to [`ThmcSolver::step`] (internal sub-phases per solver docs).
    ThmcCoupledIntegration,
}

impl TopologyPlanIntent {
    fn apply<B, C>(
        self,
        thmc: &mut ThmcSolver,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        match self {
            TopologyPlanIntent::ThmcCoupledIntegration => thmc.step(cartridge, state, manifold),
        }
    }
}

/// Shipped default: one coupled THMC morphism per outer tick (see module **Integration contract**).
pub fn default_topology_plan_intents() -> impl Iterator<Item = TopologyPlanIntent> {
    [TopologyPlanIntent::ThmcCoupledIntegration].into_iter()
}

/// Names the topology physics step and holds the sole [`ThmcSolver`] used for coupled advancement.
///
/// Use [`Self::run_plan_step`] as the **one** call site that performs a full plan tick; avoid calling
/// [`ThmcSolver::step`] directly elsewhere if you want a single integration chokepoint for logging,
/// profiling, or future middleware (e.g. validation gates between phases).
pub struct TopologyPhysicsOrchestrator {
    /// Coupled THMC Newton / explicit scaffold controls and tolerances.
    pub thmc: ThmcSolver,
}

impl TopologyPhysicsOrchestrator {
    /// Wrap an existing [`ThmcSolver`] configuration.
    pub fn new(thmc: ThmcSolver) -> Self {
        Self { thmc }
    }

    /// Fold an explicit iterator of [`TopologyPlanIntent`] into state, left-to-right (**composition**).
    ///
    /// Short-circuits on the first `Err` from an intent. An **empty** iterator returns `Ok(state)`
    /// without invoking the solver — that is **not** equivalent to [`Self::run_plan_step`], which always
    /// runs the default singleton plan.
    pub fn fold_plan_step<B, C, I>(
        &mut self,
        intents: I,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
        I: IntoIterator<Item = TopologyPlanIntent>,
    {
        intents.into_iter().try_fold(state, |state, intent| {
            intent.apply(&mut self.thmc, cartridge, state, manifold)
        })
    }

    /// Advance one orchestrated plan step: folds [`default_topology_plan_intents`] via [`Self::fold_plan_step`].
    ///
    /// This method intentionally contains no second copy of Laplacian, fracture, or rheology logic.
    /// Refer to this module’s **Integration contract** for the semantic ordering guaranteed by the
    /// solver implementation behind each [`TopologyPlanIntent::ThmcCoupledIntegration`] tick.
    ///
    /// When [`crate::core::tensors::UnifiedMaterialStateTensor::node_positions`] is `Some` and
    /// `solver-experimental` is enabled, the inner [`ThmcSolver`] consumes it for mechanics edge lengths
    /// (see [`crate::physics::solvers::ThmcSolver::step`]).
    ///
    /// # Errors
    ///
    /// Forwards [`ThmcSolver::step`] errors (including the default-feature `Err` when experimental
    /// coupling is disabled).
    pub fn run_plan_step<B, C>(
        &mut self,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        self.fold_plan_step(default_topology_plan_intents(), cartridge, state, manifold)
    }

    /// Same as [`Self::run_plan_step`] — alias for planners that prefer an explicit “full integration” name.
    pub fn run_full_integration_step<B, C>(
        &mut self,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        self.run_plan_step(cartridge, state, manifold)
    }

    /// Apply [`Self::run_plan_step`] **`steps`** times in order, using [`repeat_controlled`] as the
    /// outer fixed-point driver (same semantics as an open `for` over `0..steps` with early `break`
    /// on the first solver `Err`).
    ///
    /// **Side effects / IO:** identical to calling [`Self::run_plan_step`] in a loop — this
    /// orchestrator performs no file or network I/O; any tracing inside [`ThmcSolver::step`] is
    /// unchanged and still runs once per successful sub-step.
    ///
    /// `steps == 0` returns `Ok(state)` without invoking the solver (same empty-fold spirit as
    /// [`Self::fold_plan_step`] on an empty iterator).
    pub fn run_plan_step_repeated<B, C>(
        &mut self,
        steps: usize,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        if steps == 0 {
            return Ok(state);
        }
        let mut state_cell: Option<ThmcState<B>> = Some(state);
        let mut last_err: Option<String> = None;
        repeat_controlled(steps, || {
            let Some(s) = state_cell.take() else {
                return ControlFlow::Break(());
            };
            match self.run_plan_step(cartridge, s, manifold) {
                Ok(next) => {
                    state_cell = Some(next);
                    ControlFlow::Continue(())
                }
                Err(e) => {
                    last_err = Some(e);
                    ControlFlow::Break(())
                }
            }
        });
        match last_err {
            Some(e) => Err(e),
            None => state_cell.ok_or_else(|| {
                "TopologyPhysicsOrchestrator::run_plan_step_repeated: internal state lost"
                    .to_string()
            }),
        }
    }

    /// Borrow the inner solver for tuning `dt` / Newton counts between steps (experimental workflows).
    #[cfg(feature = "thmc-coupled")]
    pub fn thmc_solver_mut(&mut self) -> &mut ThmcSolver {
        &mut self.thmc
    }

    /// Immutable access to inner solver parameters (experimental workflows).
    #[cfg(feature = "thmc-coupled")]
    pub fn thmc_solver(&self) -> &ThmcSolver {
        &self.thmc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::NdArray;

    use crate::core::tensors::MaterialCompositionTensor;
    use crate::core::traits::PhysicalResult;
    use crate::physics::solvers::{
        ChemicalPlan, HydrologicPlan, MechanicalPlan, ThermalPlan, ThmcState,
    };

    type TestBackend = NdArray<f32>;

    fn ndarray_device() -> <TestBackend as Backend>::Device {
        Default::default()
    }

    fn physical_zeros(
        dev: &<TestBackend as Backend>::Device,
        batch: usize,
        n: usize,
    ) -> PhysicalResult<TestBackend> {
        PhysicalResult {
            free_energy: Tensor::zeros([batch, n], dev),
            dissipation: Tensor::zeros([batch, n], dev),
            safety_margin: Tensor::zeros([batch, n], dev),
            cost: Tensor::zeros([batch, n], dev),
            damage: Tensor::zeros([batch, n], dev),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([batch, n], dev),
        }
    }

    fn toy_umst_two_node(
        dev: &<TestBackend as Backend>::Device,
    ) -> UnifiedMaterialStateTensor<TestBackend> {
        let n = 2usize;
        let scalars = Tensor::<TestBackend, 2>::zeros([n, 5], dev);
        let policy_mask = Tensor::<TestBackend, 2>::ones([n, 1], dev);
        let pos = Tensor::from_data(
            Data::new(vec![0.0_f32, 0.0, 0.0, 1.0, 0.0, 0.0], Shape::new([n, 3])),
            dev,
        );
        let coords: Tensor<TestBackend, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), dev);
        let edges_b1: Tensor<TestBackend, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
            dev,
        );
        let faces_b2: Tensor<TestBackend, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), dev);
        UnifiedMaterialStateTensor {
            coords,
            edges_b1,
            faces_b2,
            scalar_features: scalars,
            vector_features: Tensor::<TestBackend, 3>::zeros([n, 1, 3], dev),
            matrix_features: Tensor::<TestBackend, 4>::zeros([n, 1, 3, 3], dev),
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions: Some(pos),
            displacement_bc_mask: Tensor::<TestBackend, 3>::ones([1, n, 3], dev),
            policy_editable_mask: policy_mask,
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    fn toy_thmc_state(dev: &<TestBackend as Backend>::Device, n: usize) -> ThmcState<TestBackend> {
        ThmcState {
            thermal: ThermalPlan {
                temperature: Tensor::<TestBackend, 3>::zeros([1, n, 1], dev).add_scalar(300.0_f32),
            },
            hydro: HydrologicPlan {
                humidity: Tensor::<TestBackend, 3>::zeros([1, n, 1], dev).add_scalar(0.5_f32),
            },
            mechanical: MechanicalPlan {
                displacement: Tensor::<TestBackend, 3>::zeros([1, n, 3], dev),
            },
            chemical: ChemicalPlan {
                reaction_extent: Tensor::<TestBackend, 3>::zeros([1, n, 1], dev)
                    .add_scalar(0.1_f32),
            },
            damage: Tensor::<TestBackend, 3>::zeros([1, n, 1], dev).add_scalar(0.01_f32),
            time: 0.0,
        }
    }

    struct EmptyCartridge;

    impl IScienceCartridge<TestBackend> for EmptyCartridge {
        fn compute_all(&self, mix: &MaterialCompositionTensor<TestBackend>) -> PhysicalResult<TestBackend> {
            physical_zeros(&mix.fractions.device(), 1, 1)
        }

        fn compute_topology(
            &self,
            m: &UnifiedMaterialStateTensor<TestBackend>,
        ) -> PhysicalResult<TestBackend> {
            let n = m.scalar_features.dims()[0];
            let d = m.scalar_features.device();
            physical_zeros(&d, 1, n)
        }
    }

    #[test]
    fn run_plan_step_repeated_zero_leaves_state() {
        let mut o = TopologyPhysicsOrchestrator::new(ThmcSolver::default());
        let dev = ndarray_device();
        let state = toy_thmc_state(&dev, 2);
        let manifold = toy_umst_two_node(&dev);
        let out = o
            .run_plan_step_repeated(0, &EmptyCartridge, state.clone(), &manifold)
            .expect("zero steps");
        assert_eq!(
            out.thermal.temperature.into_data(),
            state.thermal.temperature.into_data()
        );
    }

    #[test]
    fn run_plan_step_repeated_one_matches_run_plan_step() {
        let mut o = TopologyPhysicsOrchestrator::new(ThmcSolver::default());
        let dev = ndarray_device();
        let n = 2usize;
        let state = toy_thmc_state(&dev, n);
        let manifold = toy_umst_two_node(&dev);
        let a = o.run_plan_step(&EmptyCartridge, state.clone(), &manifold);
        let b = o.run_plan_step_repeated(1, &EmptyCartridge, state, &manifold);
        match (&a, &b) {
            (Err(ea), Err(eb)) => assert_eq!(ea, eb),
            (Ok(sa), Ok(sb)) => assert_eq!(
                sa.thermal.temperature.clone().into_data(),
                sb.thermal.temperature.clone().into_data()
            ),
            _ => panic!("run_plan_step vs repeated mismatch: {a:?} vs {b:?}"),
        }
    }

    #[test]
    fn orchestrator_wraps_solver_config() {
        let o = TopologyPhysicsOrchestrator::new(ThmcSolver {
            dt: 0.01,
            max_newton: 4,
            tol: 1e-4,
            ..Default::default()
        });
        assert!((o.thmc.dt - 0.01).abs() < f32::EPSILON);
        assert_eq!(o.thmc.max_newton, 4);
    }

    #[test]
    fn default_plan_yields_single_thmc_intent() {
        let mut it = default_topology_plan_intents();
        assert_eq!(it.next(), Some(TopologyPlanIntent::ThmcCoupledIntegration));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn run_plan_step_matches_fold_over_default_intents() {
        let mut o = TopologyPhysicsOrchestrator::new(ThmcSolver::default());
        let dev = ndarray_device();
        let n = 2usize;
        let state = toy_thmc_state(&dev, n);
        let manifold = toy_umst_two_node(&dev);
        let a = o.run_plan_step(&EmptyCartridge, state.clone(), &manifold);
        let b = o.fold_plan_step(
            default_topology_plan_intents(),
            &EmptyCartridge,
            state,
            &manifold,
        );
        match (&a, &b) {
            (Err(ea), Err(eb)) => assert_eq!(ea, eb),
            (Ok(sa), Ok(sb)) => assert_eq!(
                sa.thermal.temperature.clone().into_data(),
                sb.thermal.temperature.clone().into_data()
            ),
            _ => panic!("run_plan_step vs fold mismatch: {a:?} vs {b:?}"),
        }
    }

    #[test]
    fn fold_empty_plan_is_noop_ok() {
        let mut o = TopologyPhysicsOrchestrator::new(ThmcSolver::default());
        let dev = ndarray_device();
        let state = toy_thmc_state(&dev, 2);
        let manifold = toy_umst_two_node(&dev);
        let out = o
            .fold_plan_step(
                std::iter::empty(),
                &EmptyCartridge,
                state.clone(),
                &manifold,
            )
            .expect("empty fold");
        assert_eq!(
            out.thermal.temperature.into_data(),
            state.thermal.temperature.into_data()
        );
    }
}
