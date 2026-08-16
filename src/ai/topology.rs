// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Neural-SIMP topology optimization (Phase 4) — density network + optimizer shell.
//!
//! [`DensityNet`] maps normalized coordinates to pseudo-densities \(\rho \in (0,1)\) via a small MLP
//! and sigmoid output. [`TopologyOptimizer`] holds SIMP metadata (`volume_target`, `penalization`)
//! and the network for [`TopologyOptimizer::pseudo_density_at_coords`].
//!
//! Future wiring: sensitivity filtering on `edges_b1`, coupling to
//! [`crate::physics::mechanics::VectorMechanicsSolver`] and [`crate::ai::adjoint::AdjointNeuralODE`].
//! Differentiable optimization loops live behind **`topology-density-evolution`** /
//! **`solver-experimental`** — see `TopologyOptimizer::optimize_step` for DensityNet → SIMP →
//! equilibrium without `.into_scalar()` on the forward path.
//!
//! # Honest boundary (W29-016)
//!
//! This module is a **Neural-SIMP shell** — DensityNet + optimizer hyperparameters + optional
//! feature-gated equilibrium / projection helpers. It does **not** attest physics GREEN,
//! production closed-loop topology design, MASTER retick, or Sigmund mesh-independence filter
//! completeness (`SensitivityFilter::r_min` remains reserved). Feature
//! `topology-density-evolution` unlocks measured SIMP forward steps; that is **not** fleet GREEN.
//! Greyness / VF-gap / \(\rho^p\) helpers below are **descriptive witnesses**, not TO certificates.

/// W29 wave cell id — Neural-SIMP topology deepen.
pub const TOPOLOGY_CELL_ID: &str = "W29-016-TOPOLOGY";

/// Primary morphism @ SSOT — density decode / pseudo-density forward (not gate alias).
pub const TOPOLOGY_MORPHISM_ID: &str = "pseudo_density_at_coords";

/// Honest posture — density shell partial; evolution path feature-gated (`MASTER_RETICK=no`).
pub const TOPOLOGY_POSTURE_TAG: &str = "honest-neural-simp-shell-partial";

/// Compile-time honest fence — no production / GREEN / MASTER / Sigmund-complete / closed-loop flip.
pub const TOPOLOGY_HONEST_FENCE: &str =
    "density_net_landed=true optimizer_shell_landed=true production_wired=false physics_green=false master_retick=false sigmund_filter_complete=false closed_loop_to=false";

/// Honest non-claim @ source — bar / projection tests are local witnesses, not fleet GREEN.
pub const TOPOLOGY_SOURCE_NON_CLAIM: &str =
    "DensityNet + TopologyOptimizer shell measured in crate tests; SIMP equilibrium / Heaviside / volume match live behind topology-density-evolution; SensitivityFilter is 1-hop neighbor average (r_min reserved); greyness/VF-gap/rho^p helpers are descriptive only; not physics GREEN / not production closed-loop TO.";

/// Whether a production closed-loop topology optimizer is wired end-to-end — **false** @ W29.
pub const TOPOLOGY_PRODUCTION_WIRED: bool = false;

/// Whether physics GREEN is claimed for Neural-SIMP — **false** (shell + optional feature path ≠ GREEN).
pub const TOPOLOGY_PHYSICS_GREEN: bool = false;

/// Whether MASTER retick is authorized for this cell — **false** @ W29 deepen tier.
pub const TOPOLOGY_MASTER_RETICK: bool = false;

/// Whether Sigmund mesh-independence filter (distance-weighted \(r_{\min}\)) is complete — **false**.
pub const TOPOLOGY_SIGMUND_FILTER_COMPLETE: bool = false;

/// Whether end-to-end closed-loop topology optimisation is claimed — **false** @ W29.
pub const TOPOLOGY_CLOSED_LOOP_TO: bool = false;

/// DensityNet MLP + sigmoid path is landed (always-on).
pub const TOPOLOGY_DENSITY_NET_LANDED: bool = true;

/// TopologyOptimizer hyperparameter shell is landed (always-on).
pub const TOPOLOGY_OPTIMIZER_SHELL_LANDED: bool = true;

/// Whether `topology-density-evolution` is compiled into this binary (feature presence ≠ GREEN).
#[cfg(feature = "topology-density-evolution")]
pub const TOPOLOGY_DENSITY_EVOLUTION_COMPILED: bool = true;
#[cfg(not(feature = "topology-density-evolution"))]
pub const TOPOLOGY_DENSITY_EVOLUTION_COMPILED: bool = false;

/// Compile-time fence — production / MASTER / GREEN / Sigmund-complete / closed-loop flip not authorized.
const _: () = assert!(!TOPOLOGY_PRODUCTION_WIRED);
const _: () = assert!(!TOPOLOGY_PHYSICS_GREEN);
const _: () = assert!(!TOPOLOGY_MASTER_RETICK);
const _: () = assert!(!TOPOLOGY_SIGMUND_FILTER_COMPLETE);
const _: () = assert!(!TOPOLOGY_CLOSED_LOOP_TO);
const _: () = assert!(TOPOLOGY_DENSITY_NET_LANDED);
const _: () = assert!(TOPOLOGY_OPTIMIZER_SHELL_LANDED);

/// Typed probe for W29 Neural-SIMP topology posture honesty (meta / fleet probes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyPostureProbe {
    pub cell_id: &'static str,
    pub morphism_id: &'static str,
    pub posture_tag: &'static str,
    pub honest_fence: &'static str,
    pub source_non_claim: &'static str,
    pub density_net_landed: bool,
    pub optimizer_shell_landed: bool,
    pub density_evolution_compiled: bool,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master_retick: bool,
    pub sigmund_filter_complete: bool,
    pub closed_loop_to: bool,
}

/// Build live posture probe from compile-time SSOT constants.
#[must_use]
pub fn topology_posture_probe() -> TopologyPostureProbe {
    TopologyPostureProbe {
        cell_id: TOPOLOGY_CELL_ID,
        morphism_id: TOPOLOGY_MORPHISM_ID,
        posture_tag: TOPOLOGY_POSTURE_TAG,
        honest_fence: TOPOLOGY_HONEST_FENCE,
        source_non_claim: TOPOLOGY_SOURCE_NON_CLAIM,
        density_net_landed: TOPOLOGY_DENSITY_NET_LANDED,
        optimizer_shell_landed: TOPOLOGY_OPTIMIZER_SHELL_LANDED,
        density_evolution_compiled: TOPOLOGY_DENSITY_EVOLUTION_COMPILED,
        production_wired: TOPOLOGY_PRODUCTION_WIRED,
        physics_green: TOPOLOGY_PHYSICS_GREEN,
        master_retick: TOPOLOGY_MASTER_RETICK,
        sigmund_filter_complete: TOPOLOGY_SIGMUND_FILTER_COMPLETE,
        closed_loop_to: TOPOLOGY_CLOSED_LOOP_TO,
    }
}

/// Whether topology morphism metadata is pinned @ HEAD (visibility only; no GREEN invent).
#[must_use]
pub fn topology_morphism_pinned() -> bool {
    TOPOLOGY_CELL_ID == "W29-016-TOPOLOGY"
        && TOPOLOGY_MORPHISM_ID == "pseudo_density_at_coords"
        && TOPOLOGY_POSTURE_TAG == "honest-neural-simp-shell-partial"
        && TOPOLOGY_DENSITY_NET_LANDED
        && TOPOLOGY_OPTIMIZER_SHELL_LANDED
        && !TOPOLOGY_PRODUCTION_WIRED
        && !TOPOLOGY_PHYSICS_GREEN
        && !TOPOLOGY_MASTER_RETICK
        && !TOPOLOGY_SIGMUND_FILTER_COMPLETE
        && !TOPOLOGY_CLOSED_LOOP_TO
}

/// Validate Neural-SIMP posture honesty — fail closed on fake production / GREEN / MASTER claims.
pub fn validate_topology_posture_honesty() -> Result<(), &'static str> {
    let probe = topology_posture_probe();
    if !topology_morphism_pinned() {
        return Err("topology_morphism_pinned failed");
    }
    if probe.posture_tag.to_ascii_lowercase().contains("green") {
        return Err("posture_tag must not claim green");
    }
    let lower = probe.source_non_claim.to_ascii_lowercase();
    if lower.contains("not physics green") && probe.physics_green {
        return Err("source_non_claim must not pair with physics_green=true");
    }
    if !probe.honest_fence.contains("density_net_landed=true") {
        return Err("honest_fence missing density_net_landed=true");
    }
    if !probe.honest_fence.contains("optimizer_shell_landed=true") {
        return Err("honest_fence missing optimizer_shell_landed=true");
    }
    if !probe.honest_fence.contains("production_wired=false") {
        return Err("honest_fence missing production_wired=false");
    }
    if !probe.honest_fence.contains("physics_green=false") {
        return Err("honest_fence missing physics_green=false");
    }
    if !probe.honest_fence.contains("master_retick=false") {
        return Err("honest_fence missing master_retick=false");
    }
    if !probe.honest_fence.contains("sigmund_filter_complete=false") {
        return Err("honest_fence missing sigmund_filter_complete=false");
    }
    if !probe.honest_fence.contains("closed_loop_to=false") {
        return Err("honest_fence missing closed_loop_to=false");
    }
    if !probe.density_net_landed || !probe.optimizer_shell_landed {
        return Err("density_net / optimizer_shell must stay landed at W29 deepen tier");
    }
    if probe.production_wired
        || probe.physics_green
        || probe.master_retick
        || probe.sigmund_filter_complete
        || probe.closed_loop_to
    {
        return Err("honest refusal bits must stay false at W29 deepen tier");
    }
    // Feature compile bit is informational — must never upgrade refusal bits.
    let _ = probe.density_evolution_compiled;
    Ok(())
}

/// Volume fraction \(V^\*\) admissible for SIMP VF constraints: finite and in \((0,1)\).
#[must_use]
pub fn volume_target_admissible(v: f32) -> bool {
    v.is_finite() && v > 0.0 && v < 1.0
}

/// SIMP penalization \(p\) admissible: finite and \(p \ge 1\).
#[must_use]
pub fn penalization_admissible(p: f32) -> bool {
    p.is_finite() && p >= 1.0
}

/// Heaviside continuation \(\beta\) admissible: finite and \(\beta > 0\).
#[must_use]
pub fn heaviside_beta_admissible(beta: f32) -> bool {
    beta.is_finite() && beta > 0.0
}

/// Heaviside threshold \(\eta\) admissible: finite and in \([0,1]\).
#[must_use]
pub fn heaviside_eta_admissible(eta: f32) -> bool {
    eta.is_finite() && (0.0..=1.0).contains(&eta)
}

/// Sigmund-style greyness on a host density slice: \(4\,\mathrm{mean}(\rho(1-\rho))\in[0,1]\).
///
/// Descriptive intermediate-density witness for plateau / continuation callers — **not** a
/// mesh-independence certificate and **not** physics GREEN.
#[must_use]
pub fn greyness_from_density_slice(rho: &[f32]) -> f32 {
    if rho.is_empty() {
        return 0.0;
    }
    let n = rho.len() as f32;
    let sum: f32 = rho.iter().map(|&r| r * (1.0 - r)).sum();
    (4.0 * sum / n).clamp(0.0, 1.0)
}

/// SIMP stiffness factor \(\rho^p\) on a detached host slice (always-on; no equilibrium solve).
///
/// Returns `None` when `p` fails [`penalization_admissible`]. Output length matches `rho`.
#[must_use]
pub fn simp_penalized_modulus_factor_slice(rho: &[f32], p: f32) -> Option<Vec<f32>> {
    if !penalization_admissible(p) {
        return None;
    }
    Some(rho.iter().map(|&r| r.clamp(0.0, 1.0).powf(p)).collect())
}

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

    /// MLP trunk through `lin_out` — **pre-sigmoid logits** (unbounded).
    pub fn forward_logits<const D: usize>(&self, coords: Tensor<B, D>) -> Tensor<B, D> {
        let x = self.lin1.forward(coords);
        let x = relu(x);
        let x = self.lin2.forward(x);
        let x = relu(x);
        self.lin_out.forward(x)
    }

    /// Forward on `[..., 3]` → `[..., 1]` (same rank as input).
    pub fn forward<const D: usize>(&self, coords: Tensor<B, D>) -> Tensor<B, D> {
        sigmoid(self.forward_logits(coords))
    }

    /// Batched node coordinates `[B, N, 3]` → pseudo-density `[B, N, 1]`.
    pub fn forward_batched(&self, coords_bn3: Tensor<B, 3>) -> Tensor<B, 3> {
        let [b, n, three] = coords_bn3.dims();
        debug_assert_eq!(three, 3);
        let flat = coords_bn3.reshape([b * n, 3]);
        let rho = self.forward(flat);
        rho.reshape([b, n, 1])
    }

    /// Batched node coordinates `[B, N, 3]` → pre-sigmoid logits `[B, N, 1]`.
    pub fn forward_logits_batched(&self, coords_bn3: Tensor<B, 3>) -> Tensor<B, 3> {
        let [b, n, three] = coords_bn3.dims();
        debug_assert_eq!(three, 3);
        let flat = coords_bn3.reshape([b * n, 3]);
        let z = self.forward_logits(flat);
        z.reshape([b, n, 1])
    }
}

#[cfg(feature = "topology-density-evolution")]
use crate::core::traits::{DesignDecodeError, DesignLatent, DesignRepresentation, Geometry};
#[cfg(feature = "topology-density-evolution")]
use crate::physics::error::PhysicsError;

/// R4 adapter: nodal voxel density via [`DensityNet`] (parity-first).
#[cfg(feature = "topology-density-evolution")]
#[derive(Clone, Debug)]
pub struct VoxelDensity<B: Backend> {
    pub density_net: DensityNet<B>,
}

#[cfg(feature = "topology-density-evolution")]
impl<B: Backend<FloatElem = f32>> VoxelDensity<B> {
    pub fn new(density_net: DensityNet<B>) -> Self {
        Self { density_net }
    }

    /// Single choke point for B6 / harness decode (zero offset when latent is zero).
    pub fn decode_voxel_density(
        &self,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<Geometry<B>, DesignDecodeError> {
        self.decode(latent, query_coords)
    }
}

#[cfg(feature = "topology-density-evolution")]
impl<B: Backend<FloatElem = f32>> DesignRepresentation<B> for VoxelDensity<B> {
    fn repr_id(&self) -> &'static str {
        "umst.design.voxel_density"
    }

    fn decode(
        &self,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<Geometry<B>, DesignDecodeError> {
        let mut density = self.density_net.forward_batched(query_coords.clone());
        let [b, n, _] = density.dims();
        let latent_dim = latent.tensor.dims()[1];
        if latent_dim == 1 {
            let offset = latent.tensor.clone().reshape([b, 1, 1]);
            density = density.add(offset);
        } else if latent_dim == n {
            let offset = latent.tensor.clone().reshape([b, n, 1]);
            density = density.add(offset);
        } else if latent_dim != 1 && latent_dim != n {
            return Err(DesignDecodeError::ShapeMismatch);
        }
        if density
            .clone()
            .into_data()
            .value
            .iter()
            .any(|x| !x.is_finite())
        {
            return Err(DesignDecodeError::NonFinite);
        }
        Ok(Geometry {
            density,
            signed_distance: None,
            coords: query_coords,
        })
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
    ///
    /// Does **not** invent GREEN: stores hyperparameters as given. Prefer [`Self::try_new`] when
    /// callers need an honest domain fence on \(V^\*\) and \(p\).
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

    /// Fail-closed constructor: refuses non-admissible \(V^\*\) or \(p\) (honest domain fence).
    ///
    /// Returns `Err` with a static reason — never upgrades refusal into a GREEN claim.
    pub fn try_new(
        volume_target: f32,
        penalization: f32,
        hidden_dim: usize,
        device: &B::Device,
    ) -> Result<Self, &'static str> {
        if !volume_target_admissible(volume_target) {
            return Err("volume_target must be finite and in (0,1)");
        }
        if !penalization_admissible(penalization) {
            return Err("penalization must be finite and >= 1");
        }
        if hidden_dim == 0 {
            return Err("hidden_dim must be > 0");
        }
        Ok(Self::new(volume_target, penalization, hidden_dim, device))
    }

    /// Whether stored SIMP hyperparameters pass the honest domain fence.
    #[must_use]
    pub fn hyperparameters_admissible(&self) -> bool {
        volume_target_admissible(self.volume_target) && penalization_admissible(self.penalization)
    }

    /// Pseudo-density \(\rho \in (0,1)^{B \times N}\) from coordinates `[B, N, 3]`.
    ///
    /// For the full SIMP equilibrium forward (`optimize_step`), enable **`topology-density-evolution`**
    /// or **`solver-experimental`**.
    pub fn pseudo_density_at_coords(&self, coords: Tensor<B, 3>) -> Tensor<B, 3> {
        self.density_net.forward_batched(coords)
    }

    /// Graph-mean pseudo-density \(\mathrm{mean}(\rho)\) as rank‑1 `[1]` (volume-fraction surrogate).
    ///
    /// Descriptive statistic over the current DensityNet field — **not** a certificate that the
    /// volume constraint is satisfied under production TO, and **not** physics GREEN.
    pub fn mean_pseudo_density(&self, coords: Tensor<B, 3>) -> Tensor<B, 1> {
        let rho = self.pseudo_density_at_coords(coords);
        let [b, n, c] = rho.dims();
        let count = (b * n * c) as f32;
        rho.sum().div_scalar(count).reshape([1])
    }

    /// Absolute VF gap \(|\mathrm{mean}(\rho) - V^\*|\) as rank‑1 `[1]`.
    ///
    /// Descriptive mismatch vs stored `volume_target` — **not** a production volume certificate
    /// and **not** physics GREEN / closed-loop TO.
    pub fn volume_fraction_gap(&self, coords: Tensor<B, 3>) -> Tensor<B, 1> {
        let mean = self.mean_pseudo_density(coords);
        mean.sub_scalar(self.volume_target).abs()
    }

    /// Sigmund-style greyness \(4\,\mathrm{mean}(\rho(1-\rho))\) as rank‑1 `[1]` (in \([0,1]\)).
    ///
    /// Intermediate-density witness for continuation / plateau callers — **not** Sigmund
    /// mesh-independence completeness (`TOPOLOGY_SIGMUND_FILTER_COMPLETE` stays false).
    pub fn pseudo_density_greyness(&self, coords: Tensor<B, 3>) -> Tensor<B, 1> {
        let rho = self.pseudo_density_at_coords(coords);
        let [b, n, c] = rho.dims();
        let count = (b * n * c) as f32;
        let grey = rho
            .clone()
            .mul(rho.mul_scalar(-1.0).add_scalar(1.0))
            .sum()
            .div_scalar(count)
            .mul_scalar(4.0);
        grey.reshape([1]).clamp(0.0, 1.0)
    }

    /// Always-on SIMP factor \(\rho^p\) from DensityNet coords — **no** equilibrium solve.
    ///
    /// Feature `topology-density-evolution` remains required for masked compliance / Heaviside
    /// projection. This helper does **not** invent production wiring.
    pub fn simp_penalized_modulus_factor(&self, coords: Tensor<B, 3>) -> Tensor<B, 3> {
        let rho = self.pseudo_density_at_coords(coords);
        rho.powf_scalar(self.penalization)
    }

    /// Optimizer vs gate penalization pair for [`crate::design::query::DesignQueryContext`] (R3 v0).
    #[cfg(feature = "design-query")]
    pub fn design_query_penalizations(
        &self,
        outer: usize,
        total: usize,
    ) -> (
        crate::physics::compliance_functional::CompliancePenalization,
        crate::physics::compliance_functional::CompliancePenalization,
    ) {
        use crate::physics::compliance_functional::CompliancePenalization;
        (
            CompliancePenalization::Schedule { outer, total },
            CompliancePenalization::Gate(3.0),
        )
    }
}

impl TopologyOptimizer<burn_ndarray::NdArray<f32>> {
    /// Convenience constructor for the CPU NdArray backend shipped as a direct dependency.
    pub fn new_ndarray(volume_target: f32, penalization: f32, hidden_dim: usize) -> Self {
        let device = Default::default();
        Self::new(volume_target, penalization, hidden_dim, &device)
    }

    /// Fail-closed NdArray constructor — see [`TopologyOptimizer::try_new`].
    pub fn try_new_ndarray(
        volume_target: f32,
        penalization: f32,
        hidden_dim: usize,
    ) -> Result<Self, &'static str> {
        let device = Default::default();
        Self::try_new(volume_target, penalization, hidden_dim, &device)
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
/// column is set to `0.3` (same layout as `TopologyOptimizer::optimize_step`; reserved for continuum
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
) -> Result<SimpComplianceStepResult<B>, PhysicsError> {
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
    )?;
    let compliance = masked_dot(&body_force, &u, &boundary_mask);
    let loss_for_autodiff = if retain_loss_for_autodiff {
        Some(compliance.clone())
    } else {
        None
    };
    Ok(SimpComplianceStepResult {
        compliance,
        loss_for_autodiff,
    })
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
    ) -> Result<(Tensor<B, 1>, Tensor<B, 3>), PhysicsError> {
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
        )?;
        use crate::physics::linear::masked_dot;
        let compliance = masked_dot(&body_force, &u, &boundary_mask);
        Ok((compliance, rho))
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
    ) -> Result<SimpComplianceStepResult<B>, PhysicsError> {
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

#[cfg(feature = "topology-density-evolution")]
/// Heaviside projection with \(\tanh\) stiffening ([Wang, Lazarov & Sigmund 2011]).
///
/// formal_anchor: Literature  
/// formal_citation: Wang, Lazarov & Sigmund 2011, Struct. Multidisc. Optim. 43:767-784
#[derive(Clone, Debug)]
pub struct HeavisideProjection {
    beta: f32,
    initial_beta: f32,
    pub eta: f32,
}

#[cfg(feature = "topology-density-evolution")]
impl HeavisideProjection {
    #[must_use]
    pub fn new(initial_beta: f32, eta: f32) -> Self {
        Self {
            beta: initial_beta,
            initial_beta,
            eta,
        }
    }

    #[must_use]
    pub fn beta(&self) -> f32 {
        self.beta
    }

    /// Replace \(\beta\) (e.g. from [`BetaContinuation::beta`]).
    pub fn set_beta(&mut self, beta: f32) {
        self.beta = beta.max(1e-20);
    }

    pub fn step_continuation(
        &mut self,
        iterations_per_doubling: usize,
        current_iter: usize,
        beta_max: f32,
    ) {
        let d = iterations_per_doubling.max(1);
        let k = current_iter / d;
        self.beta = (self.initial_beta * 2_f32.powi(k as i32)).min(beta_max);
    }

    pub fn project<B: Backend<FloatElem = f32>>(&self, rho_tilde: Tensor<B, 3>) -> Tensor<B, 3> {
        let b = self.beta;
        let eta = self.eta;
        let tabn = (b * eta).tanh();
        let den = (tabn + (b * (1.0 - eta)).tanh()).max(1e-20);
        rho_tilde
            .sub_scalar(eta)
            .mul_scalar(b)
            .tanh()
            .add_scalar(tabn)
            .div_scalar(den)
    }
}

#[cfg(feature = "topology-density-evolution")]
/// Augmented Lagrangian for a global volume fraction constraint ([Bertsekas 1996]).
///
/// formal_anchor: Literature  
/// formal_citation: Bertsekas 1996, Constrained Optimization and Lagrange Multiplier Methods, Athena Scientific
#[derive(Clone, Debug)]
pub struct AugmentedLagrangianVolume {
    pub target_volume_fraction: f32,
    pub lambda: f32,
    pub mu: f32,
    pub update_period: usize,
    pub gamma: f32,
    pub tau: f32,
    step: usize,
    v_ring: std::collections::VecDeque<f32>,
}

#[cfg(feature = "topology-density-evolution")]
impl AugmentedLagrangianVolume {
    #[must_use]
    pub fn new(target_volume_fraction: f32) -> Self {
        Self {
            target_volume_fraction,
            lambda: 0.0,
            mu: 1.0,
            update_period: 10,
            gamma: 2.0,
            tau: 0.5,
            step: 0,
            v_ring: std::collections::VecDeque::new(),
        }
    }

    /// Scalar penalty \( \lambda (V-V^\*) + \tfrac12 \mu (V-V^\*)^2 \) as rank‑1 tensor (graph‑mean \(V\)).
    pub fn loss_term<B: Backend<FloatElem = f32>>(
        &self,
        rho_projected: Tensor<B, 3>,
    ) -> Tensor<B, 1> {
        let d = rho_projected.dims();
        let n = (d[0] * d[1] * d[2]) as f32;
        let v = rho_projected.sum().div_scalar(n);
        let err = v.sub_scalar(self.target_volume_fraction);
        let term = err
            .clone()
            .mul_scalar(self.lambda)
            .add(err.clone().mul(err).mul_scalar(0.5 * self.mu));
        term.reshape([1]).clamp(-1e6, 1e6)
    }

    pub fn update_multipliers(&mut self, current_volume_fraction: f32) {
        self.step += 1;
        let g = current_volume_fraction - self.target_volume_fraction;
        if self.step % self.update_period != 0 {
            self.v_ring.push_back(current_volume_fraction);
            if self.v_ring.len() > self.update_period {
                self.v_ring.pop_front();
            }
            return;
        }
        self.lambda += self.mu * g;
        if let Some(&v_old) = self.v_ring.front() {
            let g_old = v_old - self.target_volume_fraction;
            if g.abs() <= self.tau * g_old.abs() {
                // keep mu
            } else {
                self.mu *= self.gamma;
            }
        } else {
            self.mu *= self.gamma;
        }
        self.v_ring.push_back(current_volume_fraction);
        while self.v_ring.len() > self.update_period {
            self.v_ring.pop_front();
        }
    }

    /// Damp carried \(\lambda\) after a \(\beta\) step — stale multiplier from the old projection regime.
    pub fn decay_lambda(&mut self, tau_beta: f32) {
        self.lambda *= tau_beta.clamp(0.0, 1.0);
    }
}

#[cfg(feature = "topology-density-evolution")]
/// Volume-preserving projection onto \(\{\rho \in \[0,1\]^{B\times N} : \text{mean}_b(\rho)=V^\*\}\)
/// per batch \(b\) (mean over node and channel dims). Finds \(\lambda\) with bisection such that
/// \(\rho \leftarrow \mathrm{clamp}(\rho + \lambda,\,0,\,1)\) has batch-wise mean \(V^\*\); \(\lambda\) is
/// bracketed in \(\[-1,1\]\) when \(\rho\in\[0,1\]\) (see [`Self::project`]).
///
/// formal_anchor: Track B2 — `composer_prompts/v0.4_solver_completion_no_namesakes.md`
#[derive(Clone, Debug)]
pub struct VolumeProjection {
    pub target: f32,
    pub max_bisection: usize,
}

#[cfg(feature = "topology-density-evolution")]
impl VolumeProjection {
    #[must_use]
    pub fn new(target: f32, max_bisection: usize) -> Self {
        Self {
            target,
            max_bisection,
        }
    }

    /// Per-batch mean of `tensor` over axes \(1\) and \(2\) → shape `[B,1,1]`.
    fn batch_mean<B: Backend<FloatElem = f32>>(tensor: Tensor<B, 3>) -> Tensor<B, 3> {
        let [batch, n, c] = tensor.dims();
        let count = (n * c) as f32;
        tensor
            .sum_dim(2)
            .sum_dim(1)
            .div_scalar(count)
            .reshape([batch, 1, 1])
    }

    pub fn project<B: Backend<FloatElem = f32>>(&self, rho: Tensor<B, 3>) -> Tensor<B, 3> {
        let target = self.target.clamp(0.0, 1.0);
        let [batch, _n, _c] = rho.dims();
        let device = rho.device();
        let iters = self.max_bisection.max(1);
        let mut lo = Tensor::<B, 3>::full([batch, 1, 1], -1.0_f32, &device);
        let mut hi = Tensor::<B, 3>::full([batch, 1, 1], 1.0_f32, &device);
        for _ in 0..iters {
            let mid = lo.clone().add(hi.clone()).mul_scalar(0.5_f32);
            let mean = Self::batch_mean(rho.clone().add(mid.clone()).clamp(0.0, 1.0));
            let gt = mean.greater_elem(target).float();
            let one = Tensor::<B, 3>::ones_like(&gt);
            hi = one
                .clone()
                .sub(gt.clone())
                .mul(hi)
                .add(gt.clone().mul(mid.clone()));
            lo = gt.clone().mul(lo).add(one.sub(gt).mul(mid));
        }
        let lambda = lo.add(hi).mul_scalar(0.5_f32);
        rho.add(lambda).clamp(0.0, 1.0)
    }
}

#[cfg(feature = "topology-density-evolution")]
/// Scalar \(\tanh\) Heaviside ([`HeavisideProjection::project`]) for bisection on CPU slices.
#[must_use]
pub fn heaviside_tanh_scalar(r: f32, beta: f32, eta: f32) -> f32 {
    let b = beta.max(1e-20);
    let tabn = (b * eta).tanh();
    let den = (tabn + (b * (1.0 - eta)).tanh()).max(1e-20);
    (((r - eta) * b).tanh() + tabn) / den
}

#[cfg(feature = "topology-density-evolution")]
/// Logistic sigmoid on a scalar (CPU bisection helper).
#[must_use]
pub fn sigmoid_scalar(x: f32) -> f32 {
    if x >= 0.0 {
        1.0 / (1.0 + (-x).exp())
    } else {
        let ex = x.exp();
        ex / (1.0 + ex)
    }
}

#[cfg(feature = "topology-density-evolution")]
/// Mean projected VF on a detached logits slice: \(\mathrm{mean}(H_{\beta,\eta}( \sigma(z+b) ))\)
/// with identity filter (caller applies spatial filter externally when needed).
#[must_use]
pub fn logit_offset_vf_from_slice(logits: &[f32], b: f32, beta: f32, eta: f32) -> f32 {
    let n = logits.len().max(1) as f32;
    logits
        .iter()
        .map(|&z| heaviside_tanh_scalar(sigmoid_scalar(z + b), beta, eta))
        .sum::<f32>()
        / n
}

#[cfg(feature = "topology-density-evolution")]
/// Monotone bisection on logit offset \(b\) so
/// \(\mathrm{mean}(H_{\beta,\eta=0.5}(\sigma(z+b))) = V^\*\) within `tol` (Hoyer et al. 2019).
///
/// Bracket \(b \in [-B,+B]\) expands on demand; returns [`PhysicsError::Domain`] if no bracket is found.
pub fn logit_offset_matching_from_slice(
    logits: &[f32],
    beta: f32,
    target_vf: f32,
    tol: f32,
    max_iters: usize,
) -> Result<f32, PhysicsError> {
    if logits.is_empty() {
        return Err(PhysicsError::Domain {
            detail: "logit_offset_matching_from_slice: empty logits".into(),
        });
    }
    if !beta.is_finite() || beta <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: format!(
                "logit_offset_matching_from_slice: beta must be finite and positive (got {beta})"
            ),
        });
    }
    const ETA: f32 = 0.5;
    let target = target_vf.clamp(0.0, 1.0);
    let eval = |b: f32| logit_offset_vf_from_slice(logits, b, beta, ETA);
    let mut width = 8.0_f32;
    let (mut lo, mut hi) = loop {
        let vf_lo = eval(-width);
        let vf_hi = eval(width);
        if vf_lo <= target + tol && vf_hi >= target - tol {
            break (-width, width);
        }
        if width > 1_000_000.0 {
            return Err(PhysicsError::Domain {
                detail: format!(
                    "logit_offset_matching_from_slice: bracket failed — vf@b=-{width}={vf_lo:.6} vf@b=+{width}={vf_hi:.6} target={target:.6} beta={beta:.3}"
                ),
            });
        }
        width *= 2.0;
    };
    for _ in 0..max_iters.max(1) {
        let mid = 0.5 * (lo + hi);
        let vf = eval(mid);
        if vf > target + tol {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Ok(0.5 * (lo + hi))
}

#[cfg(feature = "topology-density-evolution")]
/// Volume match via uniform logit shift \(b\) on detached logits (no post-hoc \(\lambda\) shift).
#[derive(Clone, Copy, Debug)]
pub struct VolumeLogitOffsetProjection {
    pub max_bisection: usize,
    pub tol: f32,
}

#[cfg(feature = "topology-density-evolution")]
impl VolumeLogitOffsetProjection {
    #[must_use]
    pub fn new(max_bisection: usize, tol: f32) -> Self {
        Self {
            max_bisection,
            tol: tol.max(1e-8),
        }
    }

    /// Scalar \(b^\*\) from [`logit_offset_matching_from_slice`] on detached logits.
    pub fn bisect_b_from_logits_slice(
        &self,
        logits: &[f32],
        beta: f32,
        target_vf: f32,
    ) -> Result<f32, PhysicsError> {
        logit_offset_matching_from_slice(logits, beta, target_vf, self.tol, self.max_bisection)
    }

    /// Taped apply: \(\rho = \sigma(z + b)\) with constant \(b\) (bisected on detached \(z\)).
    pub fn apply_shift<B: Backend<FloatElem = f32>>(
        &self,
        logits: Tensor<B, 3>,
        b: f32,
    ) -> Tensor<B, 3> {
        sigmoid(logits.add_scalar(b))
    }
}

#[cfg(feature = "topology-density-evolution")]
/// Monotone bisection on Heaviside threshold \(\eta\) so \(\mathrm{mean}(\rho_\eta)=V^\*\) within `tol`.
///
/// Pure on `rho_tilde` scalars — used to pick \(\eta\) without \(\lambda\)-shift grey inflation (B6 H1).
#[must_use]
pub fn volume_matching_threshold_from_slice(
    rho_tilde: &[f32],
    beta: f32,
    target_vf: f32,
    tol: f32,
    max_iters: usize,
) -> f32 {
    let target = target_vf.clamp(0.0, 1.0);
    let n = rho_tilde.len().max(1) as f32;
    let mut lo = 0.0_f32;
    let mut hi = 1.0_f32;
    for _ in 0..max_iters.max(1) {
        let mid = 0.5 * (lo + hi);
        let vf = rho_tilde
            .iter()
            .map(|&r| heaviside_tanh_scalar(r, beta, mid))
            .sum::<f32>()
            / n;
        // VF decreases as \(\eta\) increases (Wang \(\tanh\) Heaviside on fixed \(\tilde\rho\)).
        if vf > target + tol {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

#[cfg(feature = "topology-density-evolution")]
/// Mask-aware \(\eta\) bisection: fixed nodes (`mask` \< 0.5) contribute VF=1; editable nodes bisect to
/// hit global `target_vf` (excludes non-design skin from the η solve — D2 Tier 4a).
#[must_use]
pub fn volume_matching_threshold_masked_from_slice(
    rho_tilde: &[f32],
    editable_mask: &[f32],
    beta: f32,
    target_vf: f32,
    tol: f32,
    max_iters: usize,
) -> f32 {
    if editable_mask.len() != rho_tilde.len() {
        return volume_matching_threshold_from_slice(rho_tilde, beta, target_vf, tol, max_iters);
    }
    let n = rho_tilde.len().max(1);
    let mut n_edit = 0usize;
    let mut vf_fixed = 0.0_f32;
    for (i, &m) in editable_mask.iter().enumerate() {
        if m > 0.5 {
            n_edit += 1;
        } else {
            // Policy-fixed solid skin (mask·ρ + (1−mask)·1).
            let _ = i;
            vf_fixed += 1.0;
        }
    }
    if n_edit == 0 {
        return volume_matching_threshold_from_slice(rho_tilde, beta, target_vf, tol, max_iters);
    }
    let target = target_vf.clamp(0.0, 1.0);
    let n_f = n as f32;
    let mut lo = 0.0_f32;
    let mut hi = 1.0_f32;
    for _ in 0..max_iters.max(1) {
        let mid = 0.5 * (lo + hi);
        let mut vf_sum = vf_fixed;
        for (i, &r) in rho_tilde.iter().enumerate() {
            if editable_mask[i] > 0.5 {
                vf_sum += heaviside_tanh_scalar(r, beta, mid);
            }
        }
        let vf = vf_sum / n_f;
        if vf > target + tol {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

#[cfg(feature = "topology-density-evolution")]
/// Volume match via \(\eta\) on [`HeavisideProjection`] (no post-hoc \(\lambda\) shift).
#[derive(Clone, Copy, Debug)]
pub struct VolumeEtaProjection {
    pub max_bisection: usize,
    pub tol: f32,
}

#[cfg(feature = "topology-density-evolution")]
impl VolumeEtaProjection {
    #[must_use]
    pub fn new(max_bisection: usize, tol: f32) -> Self {
        Self {
            max_bisection,
            tol: tol.max(1e-8),
        }
    }

    /// \(\rho_\eta = H_\beta(\tilde\rho;\eta^\*)\) with \(\eta^\*\) from [`volume_matching_threshold_from_slice`].
    pub fn project<B: Backend<FloatElem = f32>>(
        &self,
        rho_tilde: Tensor<B, 3>,
        beta: f32,
        target_vf: f32,
    ) -> Tensor<B, 3> {
        self.project_with_mask(rho_tilde, beta, target_vf, None)
    }

    /// Like [`Self::project`] but bisects \(\eta\) on editable DOFs only when `editable_mask` is set.
    pub fn project_with_mask<B: Backend<FloatElem = f32>>(
        &self,
        rho_tilde: Tensor<B, 3>,
        beta: f32,
        target_vf: f32,
        editable_mask: Option<&[f32]>,
    ) -> Tensor<B, 3> {
        let flat = rho_tilde.clone().detach().into_data().value;
        let eta = match editable_mask {
            Some(mask) => volume_matching_threshold_masked_from_slice(
                &flat,
                mask,
                beta,
                target_vf,
                self.tol,
                self.max_bisection,
            ),
            None => volume_matching_threshold_from_slice(
                &flat,
                beta,
                target_vf,
                self.tol,
                self.max_bisection,
            ),
        };
        HeavisideProjection::new(beta, eta).project(rho_tilde)
    }
}

#[cfg(feature = "topology-density-evolution")]
/// Plateau-triggered \(\beta\) boost when greyness history is flat (B6 H3).
#[derive(Clone, Copy, Debug)]
pub struct PlateauBetaContinuation {
    pub window: usize,
    pub plateau_eps: f32,
}

#[cfg(feature = "topology-density-evolution")]
impl PlateauBetaContinuation {
    #[must_use]
    pub fn new(window: usize, plateau_eps: f32) -> Self {
        Self {
            window: window.max(2),
            plateau_eps: plateau_eps.max(1e-8),
        }
    }

    /// Returns schedule `base_beta` (monotone vs `prev_beta`) or `min(2·schedule, beta_max)` when
    /// the last `window` greyness samples plateau.
    #[must_use]
    pub fn effective_beta(
        &self,
        base_beta: f32,
        greyness_history: &[f32],
        beta_max: f32,
        prev_beta: f32,
    ) -> f32 {
        let schedule = base_beta.max(prev_beta);
        if greyness_history.len() < self.window {
            return schedule;
        }
        let tail = &greyness_history[greyness_history.len() - self.window..];
        let plateau = tail
            .windows(2)
            .all(|w| (w[1] - w[0]).abs() <= self.plateau_eps);
        let boosted = if plateau && schedule < beta_max * 0.99 {
            (schedule * 2.0).min(beta_max)
        } else {
            schedule
        };
        let effective = boosted.max(prev_beta);
        debug_assert!(
            effective + 1e-6 >= prev_beta,
            "PlateauBetaContinuation: beta must be monotone (effective={effective} prev={prev_beta})"
        );
        effective
    }
}

#[cfg(feature = "topology-density-evolution")]
/// AL × β handshake: hold applied \(\beta\) until the volume AL has settled, then allow
/// [`PlateauBetaContinuation`] / schedule targets to advance (B6 H4).
#[derive(Clone, Debug)]
pub struct BetaAlHandshake {
    pub applied_beta: f32,
    pub vf_settle_tol: f32,
    pub lambda_settle_frac: f32,
    lambda_history: std::collections::VecDeque<f32>,
}

#[cfg(feature = "topology-density-evolution")]
impl BetaAlHandshake {
    /// `lambda_settle_frac`: max \(|\Delta\lambda|\) over the last three outers must be below
    /// this fraction of \(|\lambda|\) (with floor `1e-4` on \(|\lambda|\)).
    #[must_use]
    pub fn new(initial_beta: f32, vf_settle_tol: f32, lambda_settle_frac: f32) -> Self {
        Self {
            applied_beta: initial_beta.max(1e-20),
            vf_settle_tol: vf_settle_tol.max(1e-8),
            lambda_settle_frac: lambda_settle_frac.clamp(1e-6, 1.0),
            lambda_history: std::collections::VecDeque::new(),
        }
    }

    /// Record \(\lambda\) after each outer multiplier update (settlement uses prior history).
    pub fn record_lambda(&mut self, lambda: f32) {
        self.lambda_history.push_back(lambda);
        while self.lambda_history.len() > 4 {
            self.lambda_history.pop_front();
        }
    }

    /// Volume AL settled: \(|V-V^\*|\) small and recent \(\lambda\) steps are quiet.
    #[must_use]
    pub fn constraint_settled(&self, vf_err: f32, lambda: f32) -> bool {
        if vf_err.abs() >= self.vf_settle_tol {
            return false;
        }
        if self.lambda_history.len() < 4 {
            return false;
        }
        let lam_ref = lambda.abs().max(1e-4);
        let hist: Vec<f32> = self.lambda_history.iter().copied().collect();
        let n = hist.len();
        let mut max_delta = 0.0_f32;
        for i in (n - 3)..n {
            if i > 0 {
                max_delta = max_delta.max((hist[i] - hist[i - 1]).abs());
            }
        }
        max_delta <= self.lambda_settle_frac * lam_ref
    }

    /// Candidate \(\beta\) from schedule + plateau; applied only when settled (unless `bypass_settle`).
    ///
    /// Returns `(applied_beta, beta_stepped, settled)`.
    #[allow(clippy::too_many_arguments)]
    pub fn effective_beta(
        &mut self,
        plateau: &PlateauBetaContinuation,
        schedule_beta: f32,
        greyness_history: &[f32],
        beta_max: f32,
        vf_err: f32,
        lambda: f32,
        bypass_settle: bool,
    ) -> (f32, bool, bool) {
        let candidate =
            plateau.effective_beta(schedule_beta, greyness_history, beta_max, self.applied_beta);
        let settled = bypass_settle || self.constraint_settled(vf_err, lambda);
        let mut beta_stepped = false;
        if settled && candidate > self.applied_beta * (1.0 + 1e-6) {
            self.applied_beta = candidate;
            beta_stepped = true;
        }
        (self.applied_beta, beta_stepped, settled)
    }
}

#[cfg(feature = "topology-density-evolution")]
/// SIMP continuation: penalization \(p\) as a function of outer iteration.
#[derive(Clone, Copy, Debug, Default)]
pub struct ContinuationSchedule;

#[cfg(feature = "topology-density-evolution")]
impl ContinuationSchedule {
    /// SIMP exponent \(p(\texttt{iter})\): \(p=1\) at \(\texttt{iter}=0\), linear ramp to \(3\) over the
    /// first **30%** of the schedule (\(\texttt{total}\) outer iterations), then \(p=3\).
    #[must_use]
    pub fn value(iter: usize, total: usize) -> f32 {
        let t = iter as f32 / total.max(1) as f32;
        if t <= 0.3 {
            1.0 + (t / 0.3) * 2.0
        } else {
            3.0
        }
    }
}

#[cfg(feature = "topology-density-evolution")]
/// Heaviside / \(\tanh\) projection continuation: \(\beta\) spaced linearly in \(\log \beta\).
#[derive(Clone, Copy, Debug, Default)]
pub struct BetaContinuation;

#[cfg(feature = "topology-density-evolution")]
impl BetaContinuation {
    /// \(\beta_k = \exp\bigl(\log \beta_0 + \frac{k}{T}\bigl(\log \beta_{\max} - \log \beta_0\bigr)\bigr)\)
    /// with \(k=\min(\texttt{iter}, \texttt{total})\), \(T=\max(1,\texttt{total})\). Requires \(\beta_0,\beta_{\max}>0\).
    #[must_use]
    pub fn beta(iter: usize, total: usize, beta0: f32, beta_max: f32) -> f32 {
        let b0 = beta0.max(1e-20);
        let b1 = beta_max.max(1e-20);
        let t = (iter as f32 / total.max(1) as f32).clamp(0.0, 1.0);
        let log_b0 = b0.ln();
        let log_b1 = b1.ln();
        (log_b0 + t * (log_b1 - log_b0)).exp()
    }
}

#[cfg(feature = "topology-density-evolution")]
/// Simplified **gradient** smoothing on the primal graph (not Sigmund’s full mesh-independence
/// filter with density weights and \(w_{ej}=\max(0,r_{\min}-\mathrm{dist})\); that formula is open).
///
/// Here: each node receives the arithmetic mean of **neighbor** values (1‑hop via `edges_b1`),
/// then \(\mathbf g \leftarrow (1-\gamma)\mathbf g + \gamma \overline{\mathbf g}_{\mathcal N}\).
/// Isolated nodes (\(\deg=0\)) keep \(\mathbf g\) unchanged. [`Self::r_min`] is reserved for future
/// distance‑weighted or multi‑hop kernels aligned with Sigmund (1997).
#[derive(Clone, Debug)]
pub struct SensitivityFilter {
    /// Minimum filter radius (weight kernel); **not** applied in this 1‑hop neighbor average.
    pub r_min: f32,
    pub gamma: f32,
}

#[cfg(feature = "topology-density-evolution")]
impl SensitivityFilter {
    #[must_use]
    pub fn new(r_min: f32, gamma: f32) -> Self {
        Self { r_min, gamma }
    }

    /// Filter nodal sensitivities `[B, N, C]` using undirected edges `[2, E]`.
    pub fn filter_nodal<B: Backend<FloatElem = f32>>(
        &self,
        grad_bn_c: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, burn::tensor::Int>,
    ) -> Tensor<B, 3> {
        use crate::physics::topology::EdgeTopology;

        let [batch, n_v, channels] = grad_bn_c.dims();
        let device = grad_bn_c.device();
        let topo = EdgeTopology::new(edges_b1);
        let n_e = topo.n_edges();
        if n_e == 0 || self.gamma == 0.0 {
            return grad_bn_c;
        }
        let src_ix = topo.expand_src_gather_indices(batch, channels);
        let tgt_ix = topo.expand_tgt_gather_indices(batch, channels);
        let g_src = grad_bn_c.clone().gather(1, src_ix.clone());
        let g_tgt = grad_bn_c.clone().gather(1, tgt_ix.clone());
        let idx_st = Tensor::cat(vec![src_ix.clone(), tgt_ix.clone()], 1);
        let val_st = Tensor::cat(vec![g_tgt, g_src], 1);
        let neigh_sum =
            Tensor::<B, 3>::zeros([batch, n_v, channels], &device).scatter(1, idx_st, val_st);
        let src1 = topo.expand_src_gather_indices(batch, 1);
        let tgt1 = topo.expand_tgt_gather_indices(batch, 1);
        let ones_e = Tensor::<B, 3>::ones([batch, n_e, 1], &device);
        let idx_deg = Tensor::cat(vec![src1, tgt1], 1);
        let ones_2e = Tensor::cat(vec![ones_e.clone(), ones_e], 1);
        let deg = Tensor::<B, 3>::zeros([batch, n_v, 1], &device).scatter(1, idx_deg, ones_2e);
        let mask = deg.clone().greater_elem(0.0_f32).float();
        let one_m = Tensor::<B, 3>::ones_like(&mask).sub(mask.clone());
        let neigh_avg = mask
            .mul(neigh_sum.div(deg.clamp_min(1.0)))
            .add(one_m.mul(grad_bn_c.clone()));
        grad_bn_c
            .mul_scalar(1.0 - self.gamma)
            .add(neigh_avg.mul_scalar(self.gamma))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::activation::sigmoid;
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

    #[test]
    fn w29_016_topology_posture_probe_honest_not_green() {
        let probe = topology_posture_probe();
        assert_eq!(probe.cell_id, TOPOLOGY_CELL_ID);
        assert_eq!(probe.morphism_id, TOPOLOGY_MORPHISM_ID);
        assert!(probe.posture_tag.contains("honest"));
        assert!(!probe.posture_tag.to_ascii_lowercase().contains("green"));
        assert!(probe.density_net_landed);
        assert!(probe.optimizer_shell_landed);
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master_retick);
        assert!(!probe.sigmund_filter_complete);
        assert!(!probe.closed_loop_to);
        assert!(!TOPOLOGY_PRODUCTION_WIRED);
        assert!(!TOPOLOGY_PHYSICS_GREEN);
        assert!(!TOPOLOGY_CLOSED_LOOP_TO);
        // Feature compile bit is informational only — must not upgrade refusal bits.
        let _ = probe.density_evolution_compiled;
        validate_topology_posture_honesty().expect("topology posture must validate");
    }

    #[test]
    fn w29_016_topology_morphism_pinned() {
        assert!(topology_morphism_pinned());
        assert_eq!(
            TOPOLOGY_HONEST_FENCE,
            "density_net_landed=true optimizer_shell_landed=true production_wired=false physics_green=false master_retick=false sigmund_filter_complete=false closed_loop_to=false"
        );
        assert!(TOPOLOGY_SOURCE_NON_CLAIM.contains("not physics GREEN"));
        assert!(TOPOLOGY_SOURCE_NON_CLAIM.contains("r_min reserved"));
        assert!(TOPOLOGY_SOURCE_NON_CLAIM.contains("descriptive only"));
    }

    #[test]
    fn w29_016_hyperparameter_domain_fence() {
        assert!(volume_target_admissible(0.4));
        assert!(!volume_target_admissible(0.0));
        assert!(!volume_target_admissible(1.0));
        assert!(!volume_target_admissible(f32::NAN));
        assert!(penalization_admissible(1.0));
        assert!(penalization_admissible(3.0));
        assert!(!penalization_admissible(0.5));
        assert!(!penalization_admissible(f32::INFINITY));
        assert!(heaviside_beta_admissible(1.0));
        assert!(!heaviside_beta_admissible(0.0));
        assert!(!heaviside_beta_admissible(-1.0));
        assert!(heaviside_eta_admissible(0.5));
        assert!(heaviside_eta_admissible(0.0));
        assert!(heaviside_eta_admissible(1.0));
        assert!(!heaviside_eta_admissible(1.5));

        let ok = TopologyOptimizer::<B>::try_new_ndarray(0.4, 3.0, 8).expect("admissible");
        assert!(ok.hyperparameters_admissible());
        assert!(TopologyOptimizer::<B>::try_new_ndarray(0.0, 3.0, 8).is_err());
        assert!(TopologyOptimizer::<B>::try_new_ndarray(0.4, 0.5, 8).is_err());
        assert!(TopologyOptimizer::<B>::try_new_ndarray(0.4, 3.0, 0).is_err());
    }

    #[test]
    fn w29_016_density_logits_batched_then_sigmoid_matches_forward() {
        let device = Default::default();
        let net = DensityNet::<B>::new(8, &device);
        let coords = Tensor::<B, 3>::ones(Shape::new([1, 3, 3]), &device);
        let rho = net.forward_batched(coords.clone());
        let z = net.forward_logits_batched(coords);
        assert_eq!(z.dims(), [1, 3, 1]);
        let rho_from_z = sigmoid(z);
        let a = rho.into_data().value;
        let b = rho_from_z.into_data().value;
        for (x, y) in a.iter().zip(b.iter()) {
            assert!(
                (x - y).abs() < 1e-6,
                "forward vs sigmoid(logits) {x} vs {y}"
            );
        }
    }

    #[test]
    fn w29_016_mean_pseudo_density_in_unit_interval() {
        let opt = TopologyOptimizer::<B>::try_new_ndarray(0.35, 3.0, 8).expect("admissible");
        let device = Default::default();
        let coords = Tensor::<B, 3>::zeros(Shape::new([2, 4, 3]), &device);
        let mean = opt.mean_pseudo_density(coords).into_scalar();
        assert!(
            mean.is_finite() && mean > 0.0 && mean < 1.0,
            "mean rho surrogate in (0,1), got {mean}"
        );
        // Descriptive VF surrogate ≠ GREEN / production volume certificate.
        assert!(!TOPOLOGY_PHYSICS_GREEN);
        assert!(!TOPOLOGY_PRODUCTION_WIRED);
        assert!(!TOPOLOGY_CLOSED_LOOP_TO);
    }

    #[test]
    fn w29_016_greyness_and_vf_gap_descriptive_witnesses() {
        // Host greyness: binary field → 0; mid-grey → 1.
        assert!((greyness_from_density_slice(&[0.0, 1.0, 0.0, 1.0]) - 0.0).abs() < 1e-6);
        assert!((greyness_from_density_slice(&[0.5, 0.5]) - 1.0).abs() < 1e-6);
        assert!((greyness_from_density_slice(&[]) - 0.0).abs() < 1e-6);

        let factors = simp_penalized_modulus_factor_slice(&[0.5, 1.0], 3.0).expect("p ok");
        assert!((factors[0] - 0.125).abs() < 1e-6);
        assert!((factors[1] - 1.0).abs() < 1e-6);
        assert!(simp_penalized_modulus_factor_slice(&[0.5], 0.5).is_none());

        let opt = TopologyOptimizer::<B>::try_new_ndarray(0.4, 3.0, 8).expect("admissible");
        let device = Default::default();
        let coords = Tensor::<B, 3>::zeros(Shape::new([1, 6, 3]), &device);
        let grey = opt.pseudo_density_greyness(coords.clone()).into_scalar();
        assert!(
            grey.is_finite() && (0.0..=1.0).contains(&grey),
            "greyness in [0,1], got {grey}"
        );
        let gap = opt.volume_fraction_gap(coords.clone()).into_scalar();
        assert!(
            gap.is_finite() && gap >= 0.0,
            "VF gap non-negative, got {gap}"
        );
        let e_factor = opt.simp_penalized_modulus_factor(coords);
        assert_eq!(e_factor.dims(), [1, 6, 1]);
        e_factor.into_data().value.iter().copied().for_each(|v| {
            assert!(
                v.is_finite() && v >= 0.0 && v <= 1.0,
                "rho^p in [0,1], got {v}"
            );
        });
        // Descriptive helpers must not invent GREEN / production / closed-loop.
        assert!(!TOPOLOGY_PHYSICS_GREEN);
        assert!(!TOPOLOGY_PRODUCTION_WIRED);
        assert!(!TOPOLOGY_CLOSED_LOOP_TO);
        assert!(!TOPOLOGY_SIGMUND_FILTER_COMPLETE);
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
            pcg_tolerance: 1e-7,
            use_preconditioner: true,
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
        ).expect("TopologyOptimizer::optimize_step_simplite on two-node bar (FP §6 neural-SIMP compliance verification)");
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
            pcg_tolerance: 1e-7,
            use_preconditioner: true,
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
        ).expect("TopologyOptimizer::optimize_step on two-node bar (FP §6 neural-SIMP compliance verification)");
        let via_simplite = opt.optimize_step_simplite(
            coords_bn3,
            e_base_bn1,
            body_force,
            edges_b1,
            boundary_mask,
            a,
            &cfg,
            false,
        ).expect("TopologyOptimizer::optimize_step_simplite parity on two-node bar (FP §6 neural-SIMP compliance verification)");
        let c_step = compliance_step.clone().into_scalar();
        let c_simplite = via_simplite.compliance.into_scalar();
        assert!(
            (c_step - c_simplite).abs() < 1e-5 * c_simplite.abs().max(1.0),
            "optimize_step Σ(f·u) should match simplite masked compliance on this bar, got {c_step} vs {c_simplite}"
        );
    }
}

#[cfg(all(test, feature = "topology-density-evolution"))]
mod topology_density_evolution_tests {
    use super::{
        heaviside_tanh_scalar, logit_offset_matching_from_slice, logit_offset_vf_from_slice,
        volume_matching_threshold_from_slice, volume_matching_threshold_masked_from_slice,
        BetaContinuation, ContinuationSchedule, PlateauBetaContinuation, SensitivityFilter,
        VolumeEtaProjection, VolumeLogitOffsetProjection, VolumeProjection,
    };
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn logit_offset_matching_hits_target_vf() {
        let logits: Vec<f32> = (0..64).map(|i| -2.0 + 4.0 * (i as f32 / 63.0)).collect();
        let beta = 16.0_f32;
        let target = 0.35_f32;
        let b = logit_offset_matching_from_slice(&logits, beta, target, 1e-3, 48).expect(
            "logit_offset_matching_from_slice bisect b on 64-node logits field (FP §6 neural-SIMP volume verification witness)",
        );
        let vf = logit_offset_vf_from_slice(&logits, b, beta, 0.5);
        assert!(
            (vf - target).abs() < 1e-2,
            "vf {vf} vs target {target} at b {b}"
        );
    }

    #[test]
    fn logit_offset_vf_monotone_in_b() {
        let logits: Vec<f32> = (0..20).map(|i| -1.0 + 2.0 * (i as f32 / 19.0)).collect();
        let beta = 8.0_f32;
        let mut prev = 0.0_f32;
        for k in -5..=5 {
            let b = k as f32 * 0.5;
            let vf = logit_offset_vf_from_slice(&logits, b, beta, 0.5);
            assert!(
                vf >= prev - 1e-5,
                "VF should increase in b: {vf} < {prev} at b={b}"
            );
            prev = vf;
        }
    }

    #[test]
    fn volume_logit_offset_projection_apply_shift_bounded() {
        let dev = Default::default();
        let proj = VolumeLogitOffsetProjection::new(48, 1e-3);
        let z = Tensor::<B, 3>::from_data(
            Data::new(vec![0.0_f32, 1.0, -1.0], Shape::new([1, 3, 1])),
            &dev,
        );
        let rho = proj.apply_shift(z, 0.5);
        rho.into_data().value.iter().copied().for_each(|v| {
            assert!(
                v > 0.0 && v < 1.0,
                "sigmoid output should lie strictly in (0,1), got {v}"
            );
        });
    }

    #[test]
    fn volume_matching_threshold_masked_excludes_fixed_skin() {
        let mut rho: Vec<f32> = (0..64).map(|i| 0.2 + 0.6 * (i as f32 / 63.0)).collect();
        let mut mask = vec![1.0_f32; 64];
        for m in mask.iter_mut().take(16) {
            *m = 0.0;
        }
        for r in rho.iter_mut().take(16) {
            *r = 0.5;
        }
        let beta = 16.0_f32;
        let target = 0.35_f32;
        let eta = volume_matching_threshold_masked_from_slice(&rho, &mask, beta, target, 1e-3, 48);
        let mut vf_sum = 0.0_f32;
        for (i, &r) in rho.iter().enumerate() {
            let h = if mask[i] > 0.5 {
                heaviside_tanh_scalar(r, beta, eta)
            } else {
                1.0
            };
            vf_sum += h;
        }
        let vf = vf_sum / 64.0;
        assert!(
            (vf - target).abs() < 1e-2,
            "masked vf {vf} vs target {target} at eta {eta}"
        );
    }

    #[test]
    fn volume_matching_threshold_hits_target_vf() {
        let rho: Vec<f32> = (0..64).map(|i| 0.2 + 0.6 * (i as f32 / 63.0)).collect();
        let beta = 16.0_f32;
        let target = 0.35_f32;
        let eta = volume_matching_threshold_from_slice(&rho, beta, target, 1e-3, 48);
        let vf = rho
            .iter()
            .map(|&r| heaviside_tanh_scalar(r, beta, eta))
            .sum::<f32>()
            / rho.len() as f32;
        assert!(
            (vf - target).abs() < 1e-2,
            "vf {vf} vs target {target} at eta {eta}"
        );
    }

    #[test]
    fn volume_eta_projection_idempotent_on_binary_field() {
        let dev = Default::default();
        let mut v = vec![0.0_f32; 32];
        for x in v.iter_mut().take(12) {
            *x = 1.0;
        }
        let rho = Tensor::<B, 3>::from_data(Data::new(v, Shape::new([1, 32, 1])), &dev);
        let proj = VolumeEtaProjection::new(48, 1e-3);
        let target = 12.0 / 32.0;
        let out = proj.project(rho.clone(), 32.0, target);
        let out2 = proj.project(out.clone(), 32.0, target);
        let a = out.into_data().value;
        let b = out2.into_data().value;
        for (x, y) in a.iter().zip(b.iter()) {
            assert!((x - y).abs() < 1e-5, "idempotent mismatch {x} vs {y}");
        }
    }

    #[test]
    fn volume_matching_vf_monotone_in_eta() {
        let rho: Vec<f32> = (0..20).map(|i| i as f32 / 19.0).collect();
        let beta = 8.0_f32;
        let mut prev = 1.0_f32;
        for k in 1..=10 {
            let eta = k as f32 / 10.0;
            let vf = rho
                .iter()
                .map(|&r| heaviside_tanh_scalar(r, beta, eta))
                .sum::<f32>()
                / rho.len() as f32;
            assert!(
                vf <= prev + 1e-5,
                "VF should decrease in eta: {vf} > {prev}"
            );
            prev = vf;
        }
    }

    #[test]
    fn plateau_beta_doubles_on_flat_greyness() {
        let p = PlateauBetaContinuation::new(4, 0.01);
        let hist = [0.5_f32, 0.501, 0.499, 0.5005];
        let b = p.effective_beta(8.0, &hist, 64.0, 8.0);
        assert!((b - 16.0).abs() < 1e-3);
    }

    #[test]
    fn plateau_beta_monotone_when_schedule_dips() {
        let p = PlateauBetaContinuation::new(4, 0.01);
        let hist = [0.9_f32, 0.91, 0.89, 0.905];
        // Prior outer plateau-doubled to 2.514; schedule would read 1.283 without monotonicity guard.
        let b = p.effective_beta(1.283, &hist, 64.0, 2.514);
        assert!(
            b + 1e-6 >= 2.514,
            "beta must not drop when schedule base_beta < prev_beta: got {b}"
        );
    }

    #[test]
    fn volume_projection_restores_batch_mean() {
        let dev = Default::default();
        let rho = Tensor::<B, 3>::full([2, 4, 1], 0.25_f32, &dev);
        let proj = VolumeProjection::new(0.6_f32, 48);
        let out = proj.project(rho);
        let m0 = out
            .clone()
            .slice([0..1, 0..4, 0..1])
            .sum()
            .div_scalar(4.0)
            .into_scalar();
        let m1 = out
            .clone()
            .slice([1..2, 0..4, 0..1])
            .sum()
            .div_scalar(4.0)
            .into_scalar();
        assert!((m0 - 0.6).abs() < 1e-4, "batch0 mean {m0}");
        assert!((m1 - 0.6).abs() < 1e-4, "batch1 mean {m1}");
        out.into_data().value.iter().copied().for_each(|v| {
            assert!((0.0..=1.0).contains(&v), "clamp [0,1], got {v}");
        });
    }

    #[test]
    fn continuation_schedule_simp_p_ramp() {
        assert!((ContinuationSchedule::value(0, 100) - 1.0).abs() < 1e-6);
        let at_30pct = ContinuationSchedule::value(30, 100);
        assert!((at_30pct - 3.0).abs() < 1e-5, "p at 30% got {at_30pct}");
        assert!((ContinuationSchedule::value(99, 100) - 3.0).abs() < 1e-6);
    }

    #[test]
    fn beta_continuation_log_spacing_endpoints() {
        let b0 = 1.0_f32;
        let b1 = 8.0_f32;
        let t0 = BetaContinuation::beta(0, 10, b0, b1);
        let t10 = BetaContinuation::beta(10, 10, b0, b1);
        assert!((t0 - b0).abs() < 1e-5);
        assert!((t10 - b1).abs() < 1e-4, "beta(10,10)={t10}");
        let mid = BetaContinuation::beta(5, 10, b0, b1);
        let expect = (0.5_f32 * (b1 / b0).ln()).exp() * b0; // exp(log b0 + 0.5 (log b1 - log b0))
        assert!((mid - expect).abs() < 1e-4, "mid {mid} vs {expect}");
    }

    #[test]
    fn sensitivity_filter_chain_averages_neighbors() {
        let dev = Default::default();
        // 3-node line: 0 — 1 — 2
        let g = Tensor::from_data(
            Data::new(vec![1.0_f32, 2.0, 3.0], Shape::new([1, 3, 1])),
            &dev,
        );
        let edges: Vec<i64> = vec![0, 1, 1, 2];
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, 2])), &dev);
        let f = SensitivityFilter::new(0.0, 1.0);
        let out = f.filter_nodal(g, edges_b1);
        let v: Vec<f32> = out.into_data().value.to_vec();
        assert_eq!(v.len(), 3);
        v.iter()
            .for_each(|&x| assert!((x - 2.0).abs() < 1e-5, "got {x}"));
    }

    #[test]
    fn sensitivity_filter_gamma_zero_is_identity() {
        let dev = Default::default();
        let g = Tensor::<B, 3>::full([1, 2, 2], 3.5_f32, &dev);
        let g_data = g.clone().into_data().value.clone();
        let edges: Vec<i64> = vec![0, 1];
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, 1])), &dev);
        let f = SensitivityFilter::new(1.0, 0.0);
        let out = f.filter_nodal(g, edges_b1);
        assert_eq!(out.into_data().value, g_data);
    }
}
