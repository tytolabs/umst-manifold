// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Neural-SIMP topology optimization (Phase 4) — density network + optimizer shell.
//!
//! [`DensityNet`] maps normalized coordinates to pseudo-densities \(\rho \in (0,1)\) via a small MLP
//! and sigmoid output. [`TopologyOptimizer`] holds SIMP metadata (`volume_target`, `penalization`)
//! and the network for [`TopologyOptimizer::pseudo_density_at_coords`].
//!
//! Future wiring: sensitivity filtering on `edges_b1`, coupling to
//! [`crate::physics::mechanics::VectorMechanicsSolver`] and [`crate::ai::adjoint::AdjointNeuralODE`].
//! Differentiable optimization loops live behind autodiff-capable backends (`train` feature) — see
//! [`TopologyOptimizer::optimize_step`]: a no-op placeholder without **`solver-experimental`**;
//! with the feature enabled it runs a differentiable forward (DensityNet → SIMP → equilibrium) and
//! returns compliance plus density for an external optimizer.

use burn::module::Module;
use burn::nn::{Linear, LinearConfig};
use burn::tensor::activation::{relu, sigmoid};
use burn::tensor::{backend::Backend, Tensor};

/// Small MLP: last dimension 3 → hidden → hidden → 1, then **sigmoid** so \(\rho \in (0,1)\).
///
/// Generic over [`Backend`] (NdArray, WGPU, etc.). Forward accepts flattened `[..., 3]` features.
#[derive(Module, Debug)]
pub struct DensityNet<B: Backend> {
    lin1: Linear<B>,
    lin2: Linear<B>,
    lin_out: Linear<B>,
}

impl<B: Backend<FloatElem = f32>> DensityNet<B> {
    /// Builds a network with the given hidden width on `device`.
    pub fn new(hidden_dim: usize, device: &B::Device) -> Self {
        Self {
            lin1: LinearConfig::new(3, hidden_dim).init(device),
            lin2: LinearConfig::new(hidden_dim, hidden_dim).init(device),
            lin_out: LinearConfig::new(hidden_dim, 1).init(device),
        }
    }

    /// Forward on `[..., 3]` → `[..., 1]` (same rank as input).
    pub fn forward<const D: usize>(&self, coords: Tensor<B, D>) -> Tensor<B, D> {
        let x = self.lin1.forward(coords);
        let x = relu(x);
        let x = self.lin2.forward(x);
        let x = relu(x);
        let x = self.lin_out.forward(x);
        sigmoid(x)
    }

    /// Batched node coordinates `[B, N, 3]` → pseudo-density `[B, N, 1]`.
    pub fn forward_batched(&self, coords_bn3: Tensor<B, 3>) -> Tensor<B, 3> {
        let [b, n, three] = coords_bn3.dims();
        debug_assert_eq!(three, 3);
        let flat = coords_bn3.reshape([b * n, 3]);
        let rho = self.forward(flat);
        rho.reshape([b, n, 1])
    }
}

/// Neural-SIMP optimizer shell: volume target \(V^\*\), SIMP exponent \(p\), and [`DensityNet`].
///
/// Not a Burn [`Module`] itself (scalar hyperparameters are plain `f32`); only [`DensityNet`] carries
/// learnable parameters for checkpointing / autodiff.
#[derive(Clone, Debug)]
pub struct TopologyOptimizer<B: Backend> {
    pub volume_target: f32,
    pub penalization: f32,
    pub density_net: DensityNet<B>,
}

impl<B: Backend<FloatElem = f32>> TopologyOptimizer<B> {
    /// `penalization` is the usual SIMP stiffness exponent \(p \geq 1\) (stored as `f32`).
    pub fn new(
        volume_target: f32,
        penalization: f32,
        hidden_dim: usize,
        device: &B::Device,
    ) -> Self {
        Self {
            volume_target,
            penalization,
            density_net: DensityNet::new(hidden_dim, device),
        }
    }

    /// Pseudo-density \(\rho \in (0,1)^{B \times N}\) from coordinates `[B, N, 3]`.
    pub fn pseudo_density_at_coords(&self, coords: Tensor<B, 3>) -> Tensor<B, 3> {
        self.density_net.forward_batched(coords)
    }

    /// **Without `solver-experimental`:** intentionally a no-op. A full step needs geometry,
    /// loads, masks, and solver settings that are not stored on [`TopologyOptimizer`]; enable
    /// **`solver-experimental`** for [`TopologyOptimizer::optimize_step`] and
    /// [`TopologyOptimizer::optimize_step_simplite`].
    ///
    /// **With `solver-experimental`:** see [`TopologyOptimizer::optimize_step`] — one differentiable
    /// forward (no internal weight optimizer); callers backprop externally.
    #[cfg(not(feature = "topology-density-evolution"))]
    pub fn optimize_step(&mut self) {}
}

impl TopologyOptimizer<burn_ndarray::NdArray<f32>> {
    /// Convenience constructor for the CPU NdArray backend shipped as a direct dependency.
    pub fn new_ndarray(volume_target: f32, penalization: f32, hidden_dim: usize) -> Self {
        let device = Default::default();
        Self::new(volume_target, penalization, hidden_dim, &device)
    }
}

/// Historical name for [`TopologyOptimizer`] (same type).
pub type TopologyOptimizerStub<B> = TopologyOptimizer<B>;

/// SIMP-style equilibrium step: compliance \( \mathbf f^\top \mathbf u \) on the free (masked) DOFs
/// and an optional loss handle for future autodiff through [`DensityNet`].
#[cfg(feature = "topology-density-evolution")]
#[derive(Debug)]
pub struct SimpComplianceStepResult<B: Backend> {
    /// Per-batch compliance scalar \( \sum_i f_i u_i m_i \) with the same `boundary_mask` as the solve.
    pub compliance: Tensor<B, 1>,
    /// When using a training backend, set this to the same tensor as `compliance` (or a smoothed
    /// surrogate) so autodiff retains the graph; forward-only CPU runs typically leave this `None`.
    pub loss_for_autodiff: Option<Tensor<B, 1>>,
}

/// Neural-SIMP equilibrium: \(\rho =\) [`DensityNet::forward_batched`], \(E_{\mathrm{eff}} = \rho^p E_0\),
/// then [`crate::physics::mechanics::VectorMechanicsSolver::solve_equilibrium`].
///
/// All batches share one discrete geometry: vertex positions are taken from **`coords_bn3` batch 0**
/// (`[N,3]` passed into the solver); \(\rho\) and \(E_{\mathrm{eff}}\) remain batch-wise `[B,N,…]`.
///
/// `e_base_bn1` must be `[B, N, 1]` (Young’s modulus factor before SIMP penalization). Poisson’s ratio
/// column is set to `0.3` (same layout as [`TopologyOptimizer::optimize_step`]; reserved for continuum
/// coupling in the bar solver).
#[cfg(feature = "topology-density-evolution")]
#[allow(clippy::too_many_arguments)]
pub fn simp_compliance_step<B: Backend<FloatElem = f32>>(
    density_net: &DensityNet<B>,
    penalization: f32,
    coords_bn3: Tensor<B, 3>,
    e_base_bn1: Tensor<B, 3>,
    body_force: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, burn::tensor::Int>,
    boundary_mask: Tensor<B, 3>,
    cross_section_area: f32,
    inner_cfg: &crate::physics::time_orchestration::MechanicsInnerLoopConfig,
    retain_loss_for_autodiff: bool,
) -> SimpComplianceStepResult<B> {
    use crate::physics::linear::masked_dot;
    use crate::physics::mechanics::VectorMechanicsSolver;

    let [b, n, three] = coords_bn3.dims();
    debug_assert_eq!(three, 3);
    let device = coords_bn3.device();
    let coords_n3 = coords_bn3.clone().slice([0..1, 0..n, 0..3]).reshape([n, 3]);
    let rho = density_net.forward_batched(coords_bn3);
    let e_eff = rho.powf_scalar(penalization).mul(e_base_bn1.clone());
    let nu = Tensor::<B, 3>::full([b, n, 1], 0.3, &device);
    let stiffness = Tensor::cat(vec![e_eff, nu], 2);
    let damage = Tensor::<B, 3>::zeros([b, n, 1], &device);
    let displacement = Tensor::<B, 3>::zeros([b, n, 3], &device);
    let (u, _stress) = VectorMechanicsSolver::solve_equilibrium(
        displacement,
        coords_n3,
        stiffness,
        body_force.clone(),
        edges_b1,
        damage,
        boundary_mask.clone(),
        cross_section_area,
        inner_cfg,
    );
    let compliance = masked_dot(&body_force, &u, &boundary_mask);
    let loss_for_autodiff = if retain_loss_for_autodiff {
        Some(compliance.clone())
    } else {
        None
    };
    SimpComplianceStepResult {
        compliance,
        loss_for_autodiff,
    }
}

#[cfg(feature = "topology-density-evolution")]
impl<B: Backend<FloatElem = f32>> TopologyOptimizer<B> {
    /// Differentiable forward: \(\rho =\) [`DensityNet::forward_batched`],
    /// \(E_{\mathrm{eff}} = \rho^p E_{\mathrm{base}}\), stiffness \([E_{\mathrm{eff}}, \nu]\) with
    /// \(\nu = 0.3\), then [`crate::physics::mechanics::VectorMechanicsSolver::solve_equilibrium`].
    ///
    /// Compliance is \(L = \sum_{i,\alpha} f_{i,\alpha}\, u_{i,\alpha}\) over all nodes and components
    /// (per-batch, rank-1 `[B]`). The caller applies an external optimizer (no AdamW here). No
    /// `.into_scalar()` on the forward path.
    ///
    /// Geometry is batch 0 of `coords_bn3` (`[N,3]`) shared across the batch, matching
    /// [`simp_compliance_step`].
    #[allow(clippy::too_many_arguments)]
    pub fn optimize_step(
        &self,
        coords_bn3: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, burn::tensor::Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        base_stiffness: f32,
        cross_section_area: f32,
        inner_cfg: &crate::physics::time_orchestration::MechanicsInnerLoopConfig,
    ) -> (Tensor<B, 1>, Tensor<B, 3>) {
        use crate::physics::mechanics::VectorMechanicsSolver;

        let [b, n, three] = coords_bn3.dims();
        debug_assert_eq!(three, 3);
        let device = coords_bn3.device();
        let coords_n3 = coords_bn3.clone().slice([0..1, 0..n, 0..3]).reshape([n, 3]);
        let rho = self.density_net.forward_batched(coords_bn3);
        let e_eff = rho
            .clone()
            .powf_scalar(self.penalization)
            .mul_scalar(base_stiffness);
        let nu = Tensor::<B, 3>::full([b, n, 1], 0.3, &device);
        let stiffness = Tensor::cat(vec![e_eff, nu], 2);
        let displacement = Tensor::<B, 3>::zeros([b, n, 3], &device);
        let (u, _stress) = VectorMechanicsSolver::solve_equilibrium(
            displacement,
            coords_n3,
            stiffness,
            body_force.clone(),
            edges_b1,
            damage,
            boundary_mask.clone(),
            cross_section_area,
            inner_cfg,
        );
        use crate::physics::linear::masked_dot;
        let compliance = masked_dot(&body_force, &u, &boundary_mask);
        (compliance, rho)
    }

    /// One SIMP-style forward step: effective modulus \(E_{\mathrm{eff}} = \rho^p E_0\), equilibrium
    /// solve, and masked compliance \( \mathbf f^\top \mathbf u \). See [`simp_compliance_step`].
    #[allow(clippy::too_many_arguments)]
    pub fn optimize_step_simplite(
        &self,
        coords_bn3: Tensor<B, 3>,
        e_base_bn1: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, burn::tensor::Int>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &crate::physics::time_orchestration::MechanicsInnerLoopConfig,
        retain_loss_for_autodiff: bool,
    ) -> SimpComplianceStepResult<B> {
        simp_compliance_step(
            &self.density_net,
            self.penalization,
            coords_bn3,
            e_base_bn1,
            body_force,
            edges_b1,
            boundary_mask,
            cross_section_area,
            inner_cfg,
            retain_loss_for_autodiff,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Shape;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn density_net_batched_shape_and_bounded() {
        let device = Default::default();
        let net = DensityNet::<B>::new(16, &device);
        let coords = Tensor::<B, 3>::zeros(Shape::new([2, 5, 3]), &device);
        let rho = net.forward_batched(coords);
        assert_eq!(rho.dims(), [2, 5, 1]);
        rho.into_data().value.iter().copied().for_each(|v| {
            assert!(
                v > 0.0 && v < 1.0,
                "sigmoid output should lie strictly in (0,1), got {v}"
            );
        });
    }

    #[test]
    fn topology_optimizer_pseudo_density_matches_net() {
        let opt = TopologyOptimizer::<B>::new_ndarray(0.4, 3.0, 8);
        let device = Default::default();
        let coords = Tensor::<B, 3>::ones(Shape::new([1, 4, 3]), &device);
        let rho = opt.pseudo_density_at_coords(coords);
        assert_eq!(rho.dims(), [1, 4, 1]);
    }
}

#[cfg(all(test, feature = "topology-density-evolution"))]
mod simp_step_tests {
    use super::*;
    use crate::physics::time_orchestration::MechanicsInnerLoopConfig;
    use burn::module::{Module, ModuleMapper, ParamId};
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    /// Zero all [`DensityNet`] floats so every forward is `sigmoid(0)=0.5` — deterministic SIMP stiffness
    /// for bar tests (avoids RNG-heavy `Linear` init destabilizing f32 CG / compliance).
    fn topology_optimizer_zero_density_weights(
        volume_target: f32,
        penalization: f32,
        hidden_dim: usize,
        device: &<B as Backend>::Device,
    ) -> TopologyOptimizer<B> {
        struct ZeroFloats;
        impl ModuleMapper<B> for ZeroFloats {
            fn map_float<const D: usize>(
                &mut self,
                _id: &ParamId,
                tensor: Tensor<B, D>,
            ) -> Tensor<B, D> {
                Tensor::zeros(tensor.shape(), &tensor.device())
            }
        }
        let mut opt = TopologyOptimizer::new(volume_target, penalization, hidden_dim, device);
        let mut mapper = ZeroFloats;
        opt.density_net = opt.density_net.map(&mut mapper);
        opt
    }

    #[test]
    fn optimize_step_simplite_two_node_bar_compliance_positive() {
        let dev = Default::default();
        let opt = topology_optimizer_zero_density_weights(0.4, 3.0, 8, &dev);
        let n: usize = 2;
        let l_total = 1.0_f32;
        let dx = l_total / (n - 1) as f32;
        let e = 200e9_f32;
        let a = 0.01_f32;
        let f = 1000.0_f32;

        let mut coords_data = Vec::with_capacity(n * 3);
        for i in 0..n {
            coords_data.push(i as f32 * dx);
            coords_data.push(0.0);
            coords_data.push(0.0);
        }
        let coords_bn3: Tensor<B, 3> =
            Tensor::from_data(Data::new(coords_data, Shape::new([1, n, 3])), &dev);

        let mut edges = Vec::with_capacity((n - 1) * 2);
        for eid in 0..(n - 1) {
            edges.push(eid as i64);
        }
        for eid in 0..(n - 1) {
            edges.push((eid + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, n - 1])), &dev);

        let e_base_bn1 = Tensor::<B, 3>::full([1, n, 1], e, &dev);
        let mut bf_data = vec![0.0_f32; n * 3];
        bf_data[(n - 1) * 3] = f;
        let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([1, n, 3])), &dev);

        let mut bm_data = vec![1.0_f32; n * 3];
        for i in 0..n {
            bm_data[i * 3 + 1] = 0.0;
            bm_data[i * 3 + 2] = 0.0;
        }
        bm_data[0] = 0.0;
        let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([1, n, 3])), &dev);

        let cfg = MechanicsInnerLoopConfig {
            max_cg_iterations: 200,
            cg_tolerance: 1e-7,
            max_equilibrium_substeps: 1,
        };

        let out = opt.optimize_step_simplite(
            coords_bn3,
            e_base_bn1,
            body_force,
            edges_b1,
            boundary_mask,
            a,
            &cfg,
            false,
        );
        let c = out.compliance.into_scalar();
        assert!(
            c.is_finite() && c > 0.0,
            "compliance should be finite and positive, got {c}"
        );
        assert!(out.loss_for_autodiff.is_none());
    }

    #[test]
    fn optimize_step_forward_agrees_with_simplite_bar() {
        let dev = Default::default();
        let opt = topology_optimizer_zero_density_weights(0.4, 3.0, 8, &dev);
        let n: usize = 2;
        let l_total = 1.0_f32;
        let dx = l_total / (n - 1) as f32;
        let e = 200e9_f32;
        let a = 0.01_f32;
        let f = 1000.0_f32;

        let mut coords_data = Vec::with_capacity(n * 3);
        for i in 0..n {
            coords_data.push(i as f32 * dx);
            coords_data.push(0.0);
            coords_data.push(0.0);
        }
        let coords_bn3: Tensor<B, 3> =
            Tensor::from_data(Data::new(coords_data, Shape::new([1, n, 3])), &dev);

        let mut edges = Vec::with_capacity((n - 1) * 2);
        for eid in 0..(n - 1) {
            edges.push(eid as i64);
        }
        for eid in 0..(n - 1) {
            edges.push((eid + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, n - 1])), &dev);

        let e_base_bn1 = Tensor::<B, 3>::full([1, n, 1], e, &dev);
        let mut bf_data = vec![0.0_f32; n * 3];
        bf_data[(n - 1) * 3] = f;
        let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([1, n, 3])), &dev);

        let mut bm_data = vec![1.0_f32; n * 3];
        for i in 0..n {
            bm_data[i * 3 + 1] = 0.0;
            bm_data[i * 3 + 2] = 0.0;
        }
        bm_data[0] = 0.0;
        let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([1, n, 3])), &dev);

        let cfg = MechanicsInnerLoopConfig {
            max_cg_iterations: 200,
            cg_tolerance: 1e-7,
            max_equilibrium_substeps: 1,
        };

        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let (compliance_step, _rho_step) = opt.optimize_step(
            coords_bn3.clone(),
            body_force.clone(),
            edges_b1.clone(),
            damage.clone(),
            boundary_mask.clone(),
            e,
            a,
            &cfg,
        );
        let via_simplite = opt.optimize_step_simplite(
            coords_bn3,
            e_base_bn1,
            body_force,
            edges_b1,
            boundary_mask,
            a,
            &cfg,
            false,
        );
        let c_step = compliance_step.clone().into_scalar();
        let c_simplite = via_simplite.compliance.into_scalar();
        assert!(
            (c_step - c_simplite).abs() < 1e-5 * c_simplite.abs().max(1.0),
            "optimize_step Σ(f·u) should match simplite masked compliance on this bar, got {c_step} vs {c_simplite}"
        );
    }
}
