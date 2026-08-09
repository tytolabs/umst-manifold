// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Density evolution on the primal 1-skeleton (“sheaf-carried” scalar field).
//!
//! [`TopologySolver`] holds nodal pseudo-density \(\rho\) (`[B, N, 1]`) and advances it with an
//! explicit diffusion step built from [`crate::physics::laplacian::TopologicalLaplacian::scalar_laplacian`]
//! — the same DEC gather/scatter pattern as mechanics and THMC transport.
//!
//! **Relation to [`crate::ai::topology::TopologyOptimizer`]**: the optimizer runs Neural-SIMP training
//! forwards (density net \(\rightarrow\) SIMP modulus \(\rightarrow\) equilibrium) and does not own a
//! persistent \(\rho\) buffer. Use `TopologySolver::set_rho_from_optimizer` (behind
//! **`topology-density-evolution`**, also enabled via **`solver-experimental`** /
//! **`solver-tests`**) to copy network output into this physics-side state, then diffuse /
//! mask-filter \(\rho\) without duplicating the training loop.
//!
//! Re-exports: [`DensityNet`], [`TopologyOptimizer`], [`TopologyOptimizerStub`] for callers that
//! previously imported them only from this module.
//!
//! # Honest boundary (W29-089)
//!
//! **In scope today:** explicit CFL-guarded graph diffusion of nodal \(\rho\), policy∩BC edit masks,
//! optional pre/post filter hooks, and feature-gated copy-in from [`TopologyOptimizer`] /
//! [`DensityNet`]. Unit witnesses cover harmonic stationarity, two-node mix, mask freezes, CFL /
//! `dt` domain errors, mass preservation under full-editable Neumann steps (no clamp hit), and
//! damage-severed edges.
//!
//! **Out of scope / not claimed:** Neural-SIMP training loop ownership, volume-fraction projection
//! (lives on AI [`crate::ai::topology`] helpers / continuation tests), cartridge B6/B8 / Track L
//! shell acceptance, and fleet posture flags. Not physics GREEN, not `PRODUCTION_WIRED`, not
//! `MASTER`, not OP-5.

pub use crate::ai::topology::{DensityNet, TopologyOptimizer, TopologyOptimizerStub};

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::core::field::{Field, NodalDensityField};
use crate::physics::error::PhysicsError;
use crate::physics::laplacian::TopologicalLaplacian;

/// W29 deepen cell — topology density-diffusion honest fence bundle.
pub const W29_TOPOLOGY_SOLVER_DEEPEN_CELL: &str = "W29-089-TOPOLOGY_SOLVER";

/// Honest posture tag — explicit graph diffusion + masks; training/volume/cartridge refused.
pub const TOPOLOGY_SOLVER_POSTURE_TAG: &str = "honest-topology-solver-density-diffusion";

/// Honest physics posture — unit witnesses pass; does not certify fleet physics GREEN.
pub const TOPOLOGY_SOLVER_PHYSICS_GREEN: bool = false;

/// Production / cartridge shell wiring — B6/B8 Track L not claimed by this module.
pub const TOPOLOGY_SOLVER_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by topology_solver alone.
pub const TOPOLOGY_SOLVER_MASTER: bool = false;

/// OP-5 fleet pass — not claimed.
pub const TOPOLOGY_SOLVER_OP5: bool = false;

/// Whether explicit density diffusion + edit masks are landed.
pub const TOPOLOGY_SOLVER_DIFFUSION_LANDED: bool = true;

/// Whether Neural-SIMP training ownership lives in this module (refused — AI optimizer owns it).
pub const TOPOLOGY_SOLVER_TRAINING_LOOP_OWNED: bool = false;

/// Whether cartridge B6/B8 acceptance is certified from this module (open / elsewhere).
pub const TOPOLOGY_SOLVER_CARTRIDGE_ACCEPTANCE_LANDED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const TOPOLOGY_SOLVER_HONEST_FENCE: &str = "topology_density_diffusion_landed=true training_loop_owned=false cartridge_acceptance_landed=false production_wired=false master_composition_wired=false physics_green=false op5_pass=false";

/// Typed probe for topology-solver posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopologySolverPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub diffusion_landed: bool,
    pub training_loop_owned: bool,
    pub cartridge_acceptance_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for topology density evolution.
#[must_use]
pub fn topology_solver_honest_posture_bundle() -> TopologySolverPostureProbe {
    TopologySolverPostureProbe {
        physics_green: TOPOLOGY_SOLVER_PHYSICS_GREEN,
        production_wired: TOPOLOGY_SOLVER_PRODUCTION_WIRED,
        master: TOPOLOGY_SOLVER_MASTER,
        op5: TOPOLOGY_SOLVER_OP5,
        diffusion_landed: TOPOLOGY_SOLVER_DIFFUSION_LANDED,
        training_loop_owned: TOPOLOGY_SOLVER_TRAINING_LOOP_OWNED,
        cartridge_acceptance_landed: TOPOLOGY_SOLVER_CARTRIDGE_ACCEPTANCE_LANDED,
        honest_fence: TOPOLOGY_SOLVER_HONEST_FENCE,
        posture_tag: TOPOLOGY_SOLVER_POSTURE_TAG,
        deepen_cell: W29_TOPOLOGY_SOLVER_DEEPEN_CELL,
    }
}

/// Diffusion + masks landed; training/cartridge/GREEN/production/master/OP-5 honestly open/false.
#[must_use]
pub fn topology_solver_posture_honest(probe: &TopologySolverPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && probe.diffusion_landed
        && !probe.training_loop_owned
        && !probe.cartridge_acceptance_landed
        && probe.honest_fence.contains("topology_density_diffusion_landed=true")
        && probe.honest_fence.contains("training_loop_owned=false")
        && probe.honest_fence.contains("cartridge_acceptance_landed=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("op5_pass=false")
}

/// Validate posture honesty; returns Err with a static reason on fence violation.
pub fn validate_topology_solver_posture_honesty() -> Result<(), &'static str> {
    let probe = topology_solver_honest_posture_bundle();
    if !topology_solver_posture_honest(&probe) {
        return Err("topology_solver_posture_honest failed");
    }
    if probe.physics_green || TOPOLOGY_SOLVER_PHYSICS_GREEN {
        return Err("invented physics_green");
    }
    if probe.production_wired || TOPOLOGY_SOLVER_PRODUCTION_WIRED {
        return Err("invented production_wired");
    }
    if probe.master || TOPOLOGY_SOLVER_MASTER {
        return Err("invented master");
    }
    if probe.op5 || TOPOLOGY_SOLVER_OP5 {
        return Err("invented op5");
    }
    if probe.training_loop_owned || TOPOLOGY_SOLVER_TRAINING_LOOP_OWNED {
        return Err("invented training_loop_owned");
    }
    if probe.cartridge_acceptance_landed || TOPOLOGY_SOLVER_CARTRIDGE_ACCEPTANCE_LANDED {
        return Err("invented cartridge_acceptance_landed");
    }
    Ok(())
}

/// Explicit graph-diffusion CFL bound \(\Delta t_{\max} = 1/\overline{\deg}\) with
/// \(\overline{\deg}=\max(1, 2E/N)\).
#[must_use]
pub fn topology_density_cfl_dt_max(n_nodes: usize, n_edges: usize) -> f32 {
    let n_f = n_nodes.max(1) as f32;
    let e_f = n_edges.max(1) as f32;
    let mean_degree = (2.0 * e_f / n_f).max(1.0);
    1.0 / mean_degree
}

/// Bounds for \(\rho\) after each step (SIMP-style \((0,1)\) interval).
#[derive(Clone, Debug)]
pub struct TopologySolverConfig {
    pub rho_min: f32,
    pub rho_max: f32,
}

impl Default for TopologySolverConfig {
    fn default() -> Self {
        Self {
            rho_min: 1e-6,
            rho_max: 1.0 - 1e-6,
        }
    }
}

/// Pre-step validation for explicit graph diffusion (CFL + shape guards).
fn validate_density_diffusion_inputs<B: Backend>(
    dt: f32,
    rho: &Tensor<B, 3>,
    edges_b1: &Tensor<B, 2, Int>,
    damage: &Tensor<B, 3>,
    boundary_mask: &Tensor<B, 3>,
    policy_editable_mask: &Tensor<B, 2>,
) -> Result<(), PhysicsError> {
    if !dt.is_finite() || dt <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: format!(
                "topology density diffusion: dt must be finite and positive (got {dt})"
            ),
        });
    }
    let [b, n, c] = rho.dims();
    if c != 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: "TopologySolver::step_density_diffusion",
            detail: "rho last dim must be 1",
        });
    }
    let [eb_two, e] = edges_b1.dims();
    if eb_two != 2 {
        return Err(PhysicsError::ShapeMismatch {
            context: "TopologySolver::step_density_diffusion",
            detail: "edges_b1 must be [2, E]",
        });
    }
    if e == 0 && n > 1 {
        return Err(PhysicsError::Domain {
            detail: "topology density diffusion: zero edges with multiple nodes".into(),
        });
    }
    let [db, dn, dc] = damage.dims();
    if db != b || dn != n || dc != 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: "TopologySolver::step_density_diffusion",
            detail: "damage shape must match [B, N, 1]",
        });
    }
    let [bb, bn, bt] = boundary_mask.dims();
    if bb != b || bn != n || bt != 3 {
        return Err(PhysicsError::ShapeMismatch {
            context: "TopologySolver::step_density_diffusion",
            detail: "boundary_mask must be [B, N, 3]",
        });
    }
    let [pn, p1] = policy_editable_mask.dims();
    if pn != n || p1 != 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: "TopologySolver::step_density_diffusion",
            detail: "policy_editable_mask must be [N, 1]",
        });
    }
    let cfl_dt_max = topology_density_cfl_dt_max(n, e);
    let mean_degree = 1.0 / cfl_dt_max;
    if dt > cfl_dt_max {
        return Err(PhysicsError::Domain {
            detail: format!(
                "topology density diffusion: dt={dt} exceeds CFL bound {cfl_dt_max} (mean degree {mean_degree})"
            ),
        });
    }
    Ok(())
}

/// Physics-side carrier for nodal density with optional filter hooks on each step.
#[derive(Clone, Debug)]
pub struct TopologySolver<B: Backend> {
    /// Pseudo-density \(\rho\), shape `[B, N, 1]`.
    pub rho: Tensor<B, 3>,
    pub config: TopologySolverConfig,
}

impl<B: Backend<FloatElem = f32>> TopologySolver<B> {
    /// Canonical field-wrapped constructor (R25).
    #[must_use]
    pub fn new_from_field(rho: NodalDensityField<B>, config: TopologySolverConfig) -> Self {
        Self {
            rho: rho.into_tensor(),
            config,
        }
    }

    /// New solver with initial \(\rho\) and clamp bounds from `config`.
    pub fn new(rho: Tensor<B, 3>, config: TopologySolverConfig) -> Self {
        Self::new_from_field(Field::new(rho), config)
    }

    /// Batch-wise nodal sum of \(\rho\) (shape `[B]`), for mass-preservation witnesses.
    ///
    /// Under a pure Neumann graph Laplacian with full edit mask and no clamp saturation,
    /// \(\sum_i\rho_i\) is invariant to \(\rho\leftarrow\rho+\Delta t\,\Delta\rho\).
    #[must_use]
    pub fn rho_nodal_sum(&self) -> Tensor<B, 1> {
        self.rho.clone().sum_dim(2).sum_dim(1).reshape([self.rho.dims()[0]])
    }

    /// Scalar mask `≥ 0` where **both** policy may edit density (cf. [`crate::core::tensors::UnifiedMaterialStateTensor::policy_editable_mask`])
    /// **and** mechanics BCs do not fix every translational DOF on that node (product of the three
    /// `boundary_mask` channels; shape `[B, N, 3]`, `1` = free).
    ///
    /// Result shape `[B, N, 1]`, suitable for blending with [`TopologySolver::blend_masked_update`].
    pub fn combined_edit_mask(
        boundary_mask: Tensor<B, 3>,
        policy_editable_mask: Tensor<B, 2>,
    ) -> Result<Tensor<B, 3>, PhysicsError> {
        let [b, n, three] = boundary_mask.dims();
        if three != 3 {
            return Err(PhysicsError::ShapeMismatch {
                context: "TopologySolver::combined_edit_mask",
                detail: "boundary_mask last dim must be 3",
            });
        }
        let [n_pol, one] = policy_editable_mask.dims();
        if one != 1 {
            return Err(PhysicsError::ShapeMismatch {
                context: "TopologySolver::combined_edit_mask",
                detail: "policy_editable_mask last dim must be 1",
            });
        }
        if n_pol != n {
            return Err(PhysicsError::ShapeMismatch {
                context: "TopologySolver::combined_edit_mask",
                detail: "policy_editable_mask rows N must match nodal N",
            });
        }
        let mx = boundary_mask.clone().slice([0..b, 0..n, 0..1]);
        let my = boundary_mask.clone().slice([0..b, 0..n, 1..2]);
        let mz = boundary_mask.slice([0..b, 0..n, 2..3]);
        let bc_scalar = mx.mul(my).mul(mz);
        let pol = policy_editable_mask.reshape([1, n, 1]).expand([b, n, 1]);
        Ok(bc_scalar.mul(pol))
    }

    /// Blend `proposed` toward `current` where `mask` is low: `out = proposed * mask + current * (1 - mask)`.
    #[inline]
    pub fn blend_masked_update(
        current: Tensor<B, 3>,
        proposed: Tensor<B, 3>,
        mask: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        let one = Tensor::<B, 3>::ones_like(&mask);
        proposed.mul(mask.clone()).add(current.mul(one.sub(mask)))
    }

    /// One explicit diffusion step: \(\rho \leftarrow \mathrm{clamp}(\rho + \Delta t\,\Delta\rho)\) then
    /// mask projection using [`Self::combined_edit_mask`].
    pub fn step_density_diffusion(
        &mut self,
        dt: f32,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        policy_editable_mask: Tensor<B, 2>,
    ) -> Result<(), PhysicsError> {
        self.step_density_diffusion_filtered(
            dt,
            edges_b1,
            damage,
            boundary_mask,
            policy_editable_mask,
            |t| t,
            |t| t,
        )
    }

    /// Same as [`Self::step_density_diffusion`] with optional **pre-Laplacian** and **post-clamp**
    /// hooks (e.g. sensitivity filter surrogates or projection onto a feasible set).
    #[allow(clippy::too_many_arguments)]
    pub fn step_density_diffusion_filtered<F, G>(
        &mut self,
        dt: f32,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        policy_editable_mask: Tensor<B, 2>,
        pre_filter: F,
        post_filter: G,
    ) -> Result<(), PhysicsError>
    where
        F: Fn(Tensor<B, 3>) -> Tensor<B, 3>,
        G: Fn(Tensor<B, 3>) -> Tensor<B, 3>,
    {
        validate_density_diffusion_inputs(
            dt,
            &self.rho,
            &edges_b1,
            &damage,
            &boundary_mask,
            &policy_editable_mask,
        )?;
        let rho_old = self.rho.clone();
        let rho_work = pre_filter(rho_old.clone());
        let lap = TopologicalLaplacian::scalar_laplacian(rho_work, edges_b1, damage);
        let proposed = rho_old.clone().add(lap.mul_scalar(dt));
        let clamped = proposed.clamp(self.config.rho_min, self.config.rho_max);
        let filtered = post_filter(clamped);
        let m = Self::combined_edit_mask(boundary_mask, policy_editable_mask)?;
        self.rho = Self::blend_masked_update(rho_old, filtered, m);
        Ok(())
    }
}

#[cfg(feature = "topology-density-evolution")]
impl<B: Backend<FloatElem = f32>> TopologySolver<B> {
    pub fn set_rho_from_density_net(
        &mut self,
        density_net: &DensityNet<B>,
        coords_bn3: Tensor<B, 3>,
    ) {
        self.rho = density_net.forward_batched(coords_bn3);
    }

    pub fn set_rho_from_optimizer(&mut self, opt: &TopologyOptimizer<B>, coords_bn3: Tensor<B, 3>) {
        self.rho = opt.pseudo_density_at_coords(coords_bn3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    fn two_node_edge_topology() -> Tensor<B, 2, Int> {
        let dev = Default::default();
        Tensor::from_data(Data::new(vec![0_i64, 1_i64], Shape::new([2, 1])), &dev)
    }

    #[test]
    fn uniform_rho_unchanged_on_ring() {
        let dev = Default::default();
        let n = 4_usize;
        let mut edges = Vec::with_capacity(n * 2);
        for e in 0..n {
            edges.push(e as i64);
            edges.push(((e + 1) % n) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, n])), &dev);
        let rho = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);
        solver
            .step_density_diffusion(0.2, edges_b1, damage, boundary_mask, policy)
            .expect("TopologySolver::step_density_diffusion on uniform rho ring (FP §6 topology density diffusion verification)");
        let expected = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        assert!(
            solver.rho.clone().all_close(expected, Some(1e-5), Some(1e-6)),
            "uniform rho should be harmonic / stationary"
        );
    }

    #[test]
    fn two_bar_mixes_toward_equilibrium() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho = Tensor::from_data(Data::new(vec![1.0_f32, 0.0_f32], Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);
        solver
            .step_density_diffusion(0.5, edges_b1, damage, boundary_mask, policy)
            .expect("TopologySolver::step_density_diffusion on two-node bar toward 0.5 equilibrium (FP §6 topology density diffusion verification)");
        let expected = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        assert!(
            solver.rho.clone().all_close(expected, Some(1e-4), Some(1e-5)),
            "expected ~0.5 equilibrium on two-node bar"
        );
    }

    #[test]
    fn policy_mask_freezes_masked_nodes() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho = Tensor::from_data(Data::new(vec![1.0_f32, 0.0_f32], Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let mut pol = vec![0.0_f32; n];
        pol[1] = 1.0;
        let policy = Tensor::from_data(Data::new(pol, Shape::new([n, 1])), &dev);
        solver
            .step_density_diffusion(0.5, edges_b1, damage, boundary_mask, policy)
            .expect("TopologySolver::step_density_diffusion with policy mask freezing node 0 (FP §6 topology density diffusion verification)");
        let rho = solver.rho.clone();
        let n0 = rho.clone().slice([0..1, 0..1, 0..1]);
        let n1 = rho.slice([0..1, 1..2, 0..1]);
        let one = Tensor::<B, 3>::ones([1, 1, 1], &dev);
        assert!(n0.all_close(Tensor::<B, 3>::full([1, 1, 1], 1.0, &dev), Some(1e-5), Some(1e-6)));
        assert!(n1.clone().greater_elem(0.0_f32).float().all_close(one.clone(), Some(0.0), Some(0.0)));
        assert!(n1.lower_elem(1.0_f32).float().all_close(one, Some(0.0), Some(0.0)));
    }

    #[test]
    fn boundary_mask_freezes_fully_fixed_node() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho = Tensor::from_data(Data::new(vec![1.0_f32, 0.3_f32], Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let mut bm = vec![1.0_f32; n * 3];
        bm[3] = 0.0;
        bm[4] = 0.0;
        bm[5] = 0.0;
        let boundary_mask = Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);
        solver
            .step_density_diffusion(0.5, edges_b1, damage, boundary_mask, policy)
            .expect("TopologySolver::step_density_diffusion with fully fixed boundary node (FP §6 topology density diffusion verification)");
        let rho = solver.rho.clone();
        assert!(rho.clone().slice([0..1, 1..2, 0..1]).all_close(
            Tensor::<B, 3>::full([1, 1, 1], 0.3, &dev),
            Some(1e-5),
            Some(1e-6)
        ));
        assert!(rho.slice([0..1, 0..1, 0..1]).all_close(
            Tensor::<B, 3>::full([1, 1, 1], 0.65, &dev),
            Some(1e-4),
            Some(1e-5)
        ));
    }

    #[test]
    fn step_density_diffusion_filtered_invokes_pre_post_hooks() {
        use std::cell::Cell;
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);
        let pre_calls = Cell::new(0_u32);
        let post_calls = Cell::new(0_u32);
        solver
            .step_density_diffusion_filtered(
                0.1,
                edges_b1,
                damage,
                boundary_mask,
                policy,
                |t| {
                    pre_calls.set(pre_calls.get() + 1);
                    t
                },
                |t| {
                    post_calls.set(post_calls.get() + 1);
                    t
                },
            )
            .expect("TopologySolver::step_density_diffusion_filtered pre/post hook invocation (FP §6 topology density diffusion verification)");
        assert_eq!(pre_calls.get(), 1);
        assert_eq!(post_calls.get(), 1);
    }

    #[test]
    fn cfl_violation_returns_domain_error() {
        let dev = Default::default();
        let n = 4_usize;
        let mut edges = Vec::with_capacity(n * 2);
        for e in 0..n {
            edges.push(e as i64);
            edges.push(((e + 1) % n) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, n])), &dev);
        let rho = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);
        let err = solver
            .step_density_diffusion(2.0, edges_b1, damage, boundary_mask, policy)
            .unwrap_err();
        assert!(matches!(err, PhysicsError::Domain { .. }));
    }

    #[test]
    fn non_positive_dt_returns_domain_error() {
        let dev = Default::default();
        let edges_b1 = two_node_edge_topology();
        let rho = Tensor::<B, 3>::full([1, 2, 1], 0.5, &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, 2, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, 2, 3], &dev);
        let policy = Tensor::<B, 2>::ones([2, 1], &dev);
        let err = solver
            .step_density_diffusion(0.0, edges_b1, damage, boundary_mask, policy)
            .unwrap_err();
        assert!(matches!(err, PhysicsError::Domain { .. }));
    }

    #[test]
    fn honest_posture_fence_refuses_fleet_affirmatives() {
        validate_topology_solver_posture_honesty().expect("topology_solver honest fence");
        let probe = topology_solver_honest_posture_bundle();
        assert!(topology_solver_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert_eq!(probe.deepen_cell, W29_TOPOLOGY_SOLVER_DEEPEN_CELL);
        assert!((topology_density_cfl_dt_max(4, 4) - 0.5).abs() < 1e-6);
        assert!((topology_density_cfl_dt_max(2, 1) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn mass_preserved_on_full_editable_neumann_step() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        // Interior of (rho_min, rho_max) so clamp does not spoil discrete mass.
        let rho = Tensor::from_data(Data::new(vec![0.8_f32, 0.2_f32], Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let sum0 = solver.rho_nodal_sum();
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);
        solver
            .step_density_diffusion(0.25, edges_b1, damage, boundary_mask, policy)
            .expect("TopologySolver mass-preservation Neumann step (W29-089 deepen)");
        let sum1 = solver.rho_nodal_sum();
        assert!(
            sum0.all_close(sum1, Some(1e-5), Some(1e-6)),
            "graph Laplacian diffusion must preserve nodal sum under full edit mask"
        );
    }

    #[test]
    fn full_damage_severs_edge_flow() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho = Tensor::from_data(Data::new(vec![1.0_f32, 0.0_f32], Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);
        solver
            .step_density_diffusion(0.5, edges_b1, damage, boundary_mask, policy)
            .expect("TopologySolver damage-severed edge step (W29-089 deepen)");
        let expected =
            Tensor::from_data(Data::new(vec![1.0_f32, 0.0_f32], Shape::new([1, n, 1])), &dev);
        assert!(
            solver.rho.clone().all_close(expected, Some(1e-5), Some(1e-6)),
            "damage=1 must zero edge flow (no mix)"
        );
    }

    #[test]
    fn combined_edit_mask_shape_mismatch() {
        let dev = Default::default();
        let boundary_mask = Tensor::<B, 3>::ones([1, 2, 3], &dev);
        let policy_bad = Tensor::<B, 2>::ones([3, 1], &dev);
        let err = TopologySolver::<B>::combined_edit_mask(boundary_mask, policy_bad).unwrap_err();
        assert!(matches!(err, PhysicsError::ShapeMismatch { .. }));
    }

    #[test]
    fn clamp_bounds_hold_after_diffusion_step() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let cfg = TopologySolverConfig {
            rho_min: 0.1,
            rho_max: 0.9,
        };
        let rho = Tensor::from_data(Data::new(vec![0.95_f32, 0.05_f32], Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, cfg);
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);
        solver
            .step_density_diffusion(0.5, edges_b1, damage, boundary_mask, policy)
            .expect("TopologySolver clamp-bounds step (W29-089 deepen)");
        let vals = solver.rho.into_data().value;
        for v in vals {
            assert!(
                v >= 0.1 - 1e-5 && v <= 0.9 + 1e-5,
                "rho {v} escaped config clamp"
            );
        }
    }
}

#[cfg(all(test, feature = "topology-density-evolution"))]
mod optimizer_sync_tests {
    use super::*;
    use burn::tensor::{Shape, Tensor};
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn set_rho_from_optimizer_matches_forward_batched() {
        let dev = Default::default();
        let opt = TopologyOptimizer::<B>::new(0.4, 3.0, 8, &dev);
        let coords = Tensor::<B, 3>::zeros(Shape::new([1, 3, 3]), &dev);
        let rho_direct = opt.pseudo_density_at_coords(coords.clone());
        let mut solver = TopologySolver::new(
            Tensor::<B, 3>::zeros([1, 3, 1], &dev),
            TopologySolverConfig::default(),
        );
        solver.set_rho_from_optimizer(&opt, coords);
        assert!(rho_direct.all_close(solver.rho.clone(), Some(1e-6), Some(1e-7)));
    }
}
