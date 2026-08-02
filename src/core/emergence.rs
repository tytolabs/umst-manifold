// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Dissipation–geometry diagnostics: **structured grid** hotspots plus **sparse nodal** defect helpers.
//!
//! [`EmergenceMonitor`] builds \(m = D_{int} + \lambda|\nabla \mathrm{SDF}|^2\) on `[B,D,H,W]`.
//! [`nodal_defect_tensor`] uses the same `edges_b1` gather/scatter layout as
//! [`crate::physics::laplacian::TopologicalLaplacian::scalar_laplacian`] (primal edge increment, no damage mask).
//!
//! ## Honest fences (W29-023)
//!
//! - [`GRID_HOTSPOT_MONITOR_LANDED`], [`NODAL_DEFECT_TENSOR_LANDED`],
//!   [`NODAL_DEFECT_NO_EDGES_DEFERRED`], [`COMBINE_NODAL_REWARD_LANDED`] — measured evaluators landed.
//! - [`PPO_TRAINER_HOT_PATH_LANDED`], [`MSDF_EMERGENCE_GRID_BRIDGE_LANDED`] stay **false** — trainer hot-path
//!   and `sdf_grid_for_emergence_sdf` bridge remain **open**.
//! - [`PHYSICS_GREEN`], [`PRODUCTION_WIRED`], and [`MASTER`] stay **false** — no invent GREEN / production /
//!   MASTER. See [`emergence_posture_probe`].

#![allow(clippy::single_range_in_vec_init)] // Burn `Tensor::slice` uses `[Range<usize>; k]` per dimension; Clippy misreads single-row slices.

use burn::tensor::{backend::Backend, Int, Tensor};

/// W29 deepen cell — emergence diagnostics honesty (no invent GREEN / PRODUCTION / MASTER).
pub const W29_EMERGENCE_DEEPEN_CELL: &str = "W29-023-EMERGENCE";

/// Slice identifier for dissipation–geometry emergence diagnostics.
pub const SLICE_ID: &str = "phase-1-emergence-diagnostics";

/// Formal gate catalog surface (hand-aligned to [`crate::ai::formal::FormalReject`]).
pub const CATALOG_ID: &str = "umst.gate.emergence_diagnostics";

/// Honest posture — evaluators landed; trainer / MSDF bridge **open**.
pub const POSTURE_TAG: &str = "EMERGENCE_DIAGNOSTICS_PARTIAL";

/// Default λ for [`EmergenceMonitor`] (registry: `umst_manifold_emergence_lambda` / `UMST_MANIFOLD_EMERGENCE_LAMBDA`).
pub const DEFAULT_EMERGENCE_LAMBDA: f32 = 0.1;

/// Default emergence SDF voxel cap (registry: `umst_msdf_emergence_max_voxels`; enforcement deferred).
pub const DEFAULT_MAX_EMERGENCE_VOXELS: usize = 512;

/// Structured-grid hotspot monitor (`[B,D,H,W]` dissipation + SDF gradient) landed.
pub const GRID_HOTSPOT_MONITOR_LANDED: bool = true;

/// Sparse nodal defect tensor with `edges_b1` gather/scatter landed.
pub const NODAL_DEFECT_TENSOR_LANDED: bool = true;

/// Honest defer when `edges_b1` is absent — returns nodal dissipation unchanged.
pub const NODAL_DEFECT_NO_EDGES_DEFERRED: bool = true;

/// Nodal hotspot aggregation for PPO / trainer logging landed.
pub const COMBINE_NODAL_REWARD_LANDED: bool = true;

/// PPO trainer hot-path wiring for emergence diagnostics — **open** (docs-only cross-ref in `ai/ppo.rs`).
pub const PPO_TRAINER_HOT_PATH_LANDED: bool = false;

/// MSDF → emergence SDF grid bridge (`sdf_grid_for_emergence_sdf`) — **not** landed.
pub const MSDF_EMERGENCE_GRID_BRIDGE_LANDED: bool = false;

/// Honest physics posture — diagnostic evaluators only; not a physics GREEN claim.
pub const PHYSICS_GREEN: bool = false;

/// Honest production emergence gateway path — **false** until measured live eval.
pub const PRODUCTION_WIRED: bool = false;

/// Honest master / fleet-complete posture — **false** at diagnostics slice.
pub const MASTER: bool = false;

/// Fence facet inventory size (landed + open gaps).
pub const EMERGENCE_FENCE_FACET_COUNT: usize = 9;

/// Wired facets today (4/9 measured: grid monitor, nodal defect, no-edges defer, reward combine).
pub const EMERGENCE_FENCE_WIRED_COUNT: usize = 4;

/// Honest deepen fence for meta / fleet probes.
pub const HONEST_FENCE: &str =
    "grid_hotspot_monitor_landed=true|nodal_defect_tensor_landed=true|\
     nodal_defect_no_edges_deferred=true|combine_nodal_reward_landed=true|\
     ppo_trainer_hot_path_landed=false|msdf_emergence_grid_bridge_landed=false|\
     production_wired=false|physics_green=false|master=false";

/// One facet of the emergence production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmergenceFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// Emergence fence facet inventory (honest posture SSOT).
pub const EMERGENCE_FENCE_FACETS: &[EmergenceFenceFacet] = &[
    EmergenceFenceFacet {
        facet: "grid_hotspot_monitor",
        wired: true,
        owning_slice: W29_EMERGENCE_DEEPEN_CELL,
    },
    EmergenceFenceFacet {
        facet: "nodal_defect_tensor",
        wired: true,
        owning_slice: W29_EMERGENCE_DEEPEN_CELL,
    },
    EmergenceFenceFacet {
        facet: "nodal_defect_no_edges_deferred",
        wired: true,
        owning_slice: W29_EMERGENCE_DEEPEN_CELL,
    },
    EmergenceFenceFacet {
        facet: "combine_nodal_reward",
        wired: true,
        owning_slice: W29_EMERGENCE_DEEPEN_CELL,
    },
    EmergenceFenceFacet {
        facet: "ppo_trainer_hot_path",
        wired: false,
        owning_slice: "deferred-ppo-hot-path",
    },
    EmergenceFenceFacet {
        facet: "msdf_emergence_grid_bridge",
        wired: false,
        owning_slice: "deferred-sdf-grid-bridge",
    },
    EmergenceFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    EmergenceFenceFacet {
        facet: "physics_green",
        wired: false,
        owning_slice: "deferred-physics-oracle",
    },
    EmergenceFenceFacet {
        facet: "master_orchestrator_pin",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
];

const _: () = assert!(GRID_HOTSPOT_MONITOR_LANDED);
const _: () = assert!(NODAL_DEFECT_TENSOR_LANDED);
const _: () = assert!(NODAL_DEFECT_NO_EDGES_DEFERRED);
const _: () = assert!(COMBINE_NODAL_REWARD_LANDED);
const _: () = assert!(!PPO_TRAINER_HOT_PATH_LANDED);
const _: () = assert!(!MSDF_EMERGENCE_GRID_BRIDGE_LANDED);
const _: () = assert!(!PHYSICS_GREEN);
const _: () = assert!(!PRODUCTION_WIRED);
const _: () = assert!(!MASTER);
const _: () = assert!(!emergence_production_wired());
const _: () = assert!(!emergence_physics_green());
const _: () = assert!(!emergence_master_wired());

/// Honest production emergence gateway path — **false** until measured live eval.
#[must_use]
pub const fn emergence_production_wired() -> bool {
    false
}

/// Honest physics GREEN claim — **false** at diagnostics slice.
#[must_use]
pub const fn emergence_physics_green() -> bool {
    false
}

/// Honest master-tier wiring — **false** until fleet sign-off.
#[must_use]
pub const fn emergence_master_wired() -> bool {
    false
}

/// Count wired emergence fence facets (must match [`EMERGENCE_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn emergence_fence_wired_count() -> usize {
    EMERGENCE_FENCE_FACETS.iter().filter(|f| f.wired).count()
}

/// Typed probe for emergence posture honesty.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmergencePostureProbe {
    pub deepen_cell: &'static str,
    pub slice_id: &'static str,
    pub catalog_id: &'static str,
    pub posture_tag: &'static str,
    pub grid_hotspot_monitor_landed: bool,
    pub nodal_defect_tensor_landed: bool,
    pub nodal_defect_no_edges_deferred: bool,
    pub combine_nodal_reward_landed: bool,
    pub ppo_trainer_hot_path_landed: bool,
    pub msdf_emergence_grid_bridge_landed: bool,
    pub default_lambda: f32,
    pub default_max_voxels: usize,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for emergence done-when checks.
#[must_use]
pub const fn emergence_posture_probe() -> EmergencePostureProbe {
    EmergencePostureProbe {
        deepen_cell: W29_EMERGENCE_DEEPEN_CELL,
        slice_id: SLICE_ID,
        catalog_id: CATALOG_ID,
        posture_tag: POSTURE_TAG,
        grid_hotspot_monitor_landed: GRID_HOTSPOT_MONITOR_LANDED,
        nodal_defect_tensor_landed: NODAL_DEFECT_TENSOR_LANDED,
        nodal_defect_no_edges_deferred: NODAL_DEFECT_NO_EDGES_DEFERRED,
        combine_nodal_reward_landed: COMBINE_NODAL_REWARD_LANDED,
        ppo_trainer_hot_path_landed: PPO_TRAINER_HOT_PATH_LANDED,
        msdf_emergence_grid_bridge_landed: MSDF_EMERGENCE_GRID_BRIDGE_LANDED,
        default_lambda: DEFAULT_EMERGENCE_LAMBDA,
        default_max_voxels: DEFAULT_MAX_EMERGENCE_VOXELS,
        fence_facet_count: EMERGENCE_FENCE_FACET_COUNT,
        fence_wired_count: EMERGENCE_FENCE_WIRED_COUNT,
        production_wired: emergence_production_wired(),
        physics_green: emergence_physics_green(),
        master: emergence_master_wired(),
        honest_fence: HONEST_FENCE,
    }
}

/// Evaluators landed with production / GREEN / MASTER gateway path honestly open.
#[must_use]
pub fn emergence_posture_honest(probe: &EmergencePostureProbe) -> bool {
    probe.deepen_cell == W29_EMERGENCE_DEEPEN_CELL
        && probe.slice_id == SLICE_ID
        && probe.catalog_id == CATALOG_ID
        && probe.posture_tag == POSTURE_TAG
        && probe.grid_hotspot_monitor_landed
        && probe.nodal_defect_tensor_landed
        && probe.nodal_defect_no_edges_deferred
        && probe.combine_nodal_reward_landed
        && !probe.ppo_trainer_hot_path_landed
        && !probe.msdf_emergence_grid_bridge_landed
        && probe.default_lambda == DEFAULT_EMERGENCE_LAMBDA
        && probe.default_max_voxels == DEFAULT_MAX_EMERGENCE_VOXELS
        && probe.fence_facet_count == EMERGENCE_FENCE_FACET_COUNT
        && probe.fence_wired_count == EMERGENCE_FENCE_WIRED_COUNT
        && !probe.production_wired
        && !probe.physics_green
        && !probe.master
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("ppo_trainer_hot_path_landed=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
}

/// Validate emergence posture honesty — fail closed on fake production / GREEN claims.
pub fn validate_emergence_posture_honesty() -> Result<(), &'static str> {
    let probe = emergence_posture_probe();
    if probe.production_wired || emergence_production_wired() || PRODUCTION_WIRED {
        return Err("emergence_production_wired must stay false until trainer hot-path wire");
    }
    if probe.physics_green || emergence_physics_green() || PHYSICS_GREEN {
        return Err("emergence PHYSICS_GREEN must stay false at diagnostics slice");
    }
    if probe.master || emergence_master_wired() || MASTER {
        return Err("emergence MASTER must stay false until fleet sign-off");
    }
    if probe.ppo_trainer_hot_path_landed || PPO_TRAINER_HOT_PATH_LANDED {
        return Err("ppo_trainer_hot_path_landed must stay false until PPO hot-path wire");
    }
    if probe.msdf_emergence_grid_bridge_landed || MSDF_EMERGENCE_GRID_BRIDGE_LANDED {
        return Err("msdf_emergence_grid_bridge_landed must stay false until sdf grid bridge lands");
    }
    if emergence_fence_wired_count() != EMERGENCE_FENCE_WIRED_COUNT {
        return Err("emergence_fence_wired_count drifted from EMERGENCE_FENCE_WIRED_COUNT");
    }
    if EMERGENCE_FENCE_FACETS.len() != EMERGENCE_FENCE_FACET_COUNT {
        return Err("EMERGENCE_FENCE_FACETS length drifted from EMERGENCE_FENCE_FACET_COUNT");
    }
    if !emergence_posture_honest(&probe) {
        return Err("emergence_posture_honest failed");
    }
    Ok(())
}

/// Numerical floor for `|Δp|` when forming edge-length weights from `node_positions`.
const NODAL_DEFECT_LEN_EPS: f32 = 1e-12;

/// Emergence monitor: dissipation hotspots from \(D_{int}\) and \(|\nabla \mathrm{SDF}|^2\).
///
/// \(m_i = D_{int,i} + \lambda|\nabla \mathrm{SDF}_i|^2\).
pub struct EmergenceMonitor<B: Backend> {
    pub lambda: f32,
    _backend: std::marker::PhantomData<B>,
}

impl<B: Backend> EmergenceMonitor<B> {
    pub fn new(lambda: f32) -> Self {
        Self {
            lambda,
            _backend: std::marker::PhantomData,
        }
    }

    /// Registry-default λ (`DEFAULT_EMERGENCE_LAMBDA` / `UMST_MANIFOLD_EMERGENCE_LAMBDA`).
    pub fn with_default_lambda() -> Self {
        Self::new(DEFAULT_EMERGENCE_LAMBDA)
    }

    /// Computes the thermo-topological defect mass field on a structured lattice `[B,D,H,W]`.
    pub fn compute_dissipation_hotspots(
        &self,
        d_int: Tensor<B, 4>,
        sdf: Tensor<B, 4>,
    ) -> Tensor<B, 4> {
        let dims = sdf.dims();
        let (batch, d, h, w) = (dims[0], dims[1], dims[2], dims[3]);

        let sdf_x_plus = sdf.clone().slice([0..batch, 0..d, 0..h, 2..w]);
        let sdf_x_minus = sdf.clone().slice([0..batch, 0..d, 0..h, 0..(w - 2)]);
        let dx = sdf_x_plus.sub(sdf_x_minus).div_scalar(2.0);

        let sdf_y_plus = sdf.clone().slice([0..batch, 0..d, 2..h, 0..w]);
        let sdf_y_minus = sdf.clone().slice([0..batch, 0..d, 0..(h - 2), 0..w]);
        let dy = sdf_y_plus.sub(sdf_y_minus).div_scalar(2.0);

        let sdf_z_plus = sdf.clone().slice([0..batch, 2..d, 0..h, 0..w]);
        let sdf_z_minus = sdf.clone().slice([0..batch, 0..(d - 2), 0..h, 0..w]);
        let dz = sdf_z_plus.sub(sdf_z_minus).div_scalar(2.0);

        let zero = sdf
            .clone()
            .slice([0..batch, 0..1, 0..1, 0..1])
            .reshape([1])
            .into_scalar();
        let pad_x = dx.pad((1, 1, 0, 0), zero);
        let pad_y = dy.pad((0, 0, 1, 1), zero);
        let dev = dz.device();
        let mut pad_z = Tensor::<B, 4>::zeros([batch, d, h, w], &dev);
        pad_z = pad_z.slice_assign([0..batch, 1..(d - 1), 0..h, 0..w], dz);

        let grad_sdf_sq = pad_x
            .powf_scalar(2.0)
            .add(pad_y.powf_scalar(2.0))
            .add(pad_z.powf_scalar(2.0));

        d_int.add(grad_sdf_sq.mul_scalar(self.lambda))
    }
}

/// Nodal defect \(\dot\varphi_n + \lambda\,\overline{g}_n\) with \(\overline{g}_n\) the mean over incident
/// edges of \(|\dot\varphi_{\mathrm{tgt}}-\dot\varphi_{\mathrm{src}}|/(L_e+\varepsilon)\).
///
/// # Shapes
/// * `nodal_dissipation` — `[B, N, 1]`
/// * `edges_b1` — optional `[2, E]` (row 0 = sources, row 1 = targets), same as [`crate::physics::laplacian::TopologicalLaplacian`]
/// * `node_positions` — optional `[N, 3]` for physical edge lengths; if [`None`], \(L_e \equiv 1\)
///
/// # Open roadmap item
/// Without `edges_b1`, no primal gradient on the graph is defined; returns `nodal_dissipation` unchanged.
pub fn nodal_defect_tensor<B: Backend>(
    nodal_dissipation: Tensor<B, 3>,
    edges_b1: Option<Tensor<B, 2, Int>>,
    node_positions: Option<Tensor<B, 2>>,
    lambda_grad: f32,
) -> Tensor<B, 3> {
    let Some(edges_b1) = edges_b1 else {
        return nodal_dissipation;
    };
    let edges = edges_b1;

    let batch = nodal_dissipation.dims()[0];
    let n = nodal_dissipation.dims()[1];
    let channels = nodal_dissipation.dims()[2];
    let device = nodal_dissipation.device();
    let num_edges = edges.dims()[1];

    let src_ix = edges
        .clone()
        .slice([0..1])
        .reshape([1, num_edges, 1])
        .expand([batch, num_edges, channels]);
    let tgt_ix = edges
        .clone()
        .slice([1..2])
        .reshape([1, num_edges, 1])
        .expand([batch, num_edges, channels]);

    let phi_src = nodal_dissipation.clone().gather(1, src_ix.clone());
    let phi_tgt = nodal_dissipation.clone().gather(1, tgt_ix.clone());
    let dphi = phi_tgt.sub(phi_src);
    let g_e = dphi.abs().div(
        edge_length_per_edge::<B>(&edges, node_positions, batch, n, num_edges, &device)
            .add_scalar(NODAL_DEFECT_LEN_EPS),
    );

    let ch1 = 1usize;
    let src_ix1 = edges
        .clone()
        .slice([0..1])
        .reshape([1, num_edges, 1])
        .expand([batch, num_edges, ch1]);
    let tgt_ix1 = edges
        .clone()
        .slice([1..2])
        .reshape([1, num_edges, 1])
        .expand([batch, num_edges, ch1]);

    let ones_e = Tensor::<B, 3>::ones_like(&g_e);
    let deg_n = Tensor::<B, 3>::zeros([batch, n, 1], &device)
        .scatter(1, src_ix1.clone(), ones_e.clone())
        .scatter(1, tgt_ix1.clone(), ones_e);
    let sum_g_n = Tensor::<B, 3>::zeros([batch, n, 1], &device)
        .scatter(1, src_ix1.clone(), g_e.clone())
        .scatter(1, tgt_ix1, g_e);
    let g_bar_n = sum_g_n.div(deg_n.clamp_min(1.0_f32));

    nodal_dissipation.add(g_bar_n.mul_scalar(lambda_grad))
}

fn edge_length_per_edge<B: Backend>(
    edges_b1: &Tensor<B, 2, Int>,
    node_positions: Option<Tensor<B, 2>>,
    batch: usize,
    n_v: usize,
    num_edges: usize,
    device: &B::Device,
) -> Tensor<B, 3> {
    match node_positions {
        Some(coords) => {
            let src_ix3 = edges_b1
                .clone()
                .slice([0..1])
                .reshape([1, num_edges, 1])
                .expand([batch, num_edges, 3]);
            let tgt_ix3 = edges_b1
                .clone()
                .slice([1..2])
                .reshape([1, num_edges, 1])
                .expand([batch, num_edges, 3]);
            let coords_b = coords.unsqueeze_dim::<3>(0).expand([batch, n_v, 3]);
            let c_src = coords_b.clone().gather(1, src_ix3);
            let c_tgt = coords_b.gather(1, tgt_ix3);
            let delta = c_tgt.sub(c_src);
            delta
                .powf_scalar(2.0)
                .sum_dim(2)
                .sqrt()
                .clamp(NODAL_DEFECT_LEN_EPS, f32::MAX)
                .reshape([batch, num_edges, 1])
        }
        None => Tensor::<B, 3>::ones([batch, num_edges, 1], device),
    }
}

/// Sums nodal hotspot mass over nodes and channels into `[B]` for PPO / trainer logging.
///
/// Accepts `[B, N, C]` (typically `C = 1`). Aggregates with `sum_dim` so batch layout stays
/// consistent; avoids `reshape([B, N])` when `C ≠ 1`, which would mismatch element counts
/// and panic in Burn (`copy_from_slice` length mismatch).
pub fn combine_nodal_for_reward<B: Backend>(hotspots: Tensor<B, 3>) -> Tensor<B, 1> {
    let batch = hotspots.dims()[0];
    hotspots.sum_dim(2).sum_dim(1).reshape([batch])
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn dev() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    fn chain_edges_b1(n: usize) -> Tensor<B, 2, Int> {
        let mut edges = Vec::with_capacity((n - 1) * 2);
        for eid in 0..(n - 1) {
            edges.push(eid as i64);
        }
        for eid in 0..(n - 1) {
            edges.push((eid + 1) as i64);
        }
        Tensor::from_data(Data::new(edges, Shape::new([2, n - 1])), &dev())
    }

    #[test]
    fn nodal_defect_deferred_without_edges() {
        let d = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 2.0, 3.0], Shape::new([1, 3, 1])),
            &dev(),
        );
        let out = nodal_defect_tensor(d.clone(), None, None, 0.5);
        assert_eq!(
            out.into_data().convert::<f32>().value,
            d.into_data().convert::<f32>().value
        );
    }

    #[test]
    fn combine_nodal_for_reward_sums_nodes() {
        let h = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 2.0, 3.0], Shape::new([1, 3, 1])),
            &dev(),
        );
        let s = combine_nodal_for_reward(h);
        let v: Vec<f32> = s.into_data().convert::<f32>().value;
        assert_abs_diff_eq!(v[0], 6.0_f32);
    }

    #[test]
    fn combine_nodal_for_reward_sums_nodes_and_channels() {
        let h = Tensor::<B, 3>::from_data(
            Data::new(
                vec![1.0_f32, 10.0, 2.0, 20.0, 3.0, 30.0],
                Shape::new([1, 3, 2]),
            ),
            &dev(),
        );
        let s = combine_nodal_for_reward(h);
        let v: Vec<f32> = s.into_data().convert::<f32>().value;
        assert_abs_diff_eq!(v[0], 66.0_f32);
    }

    #[test]
    fn nodal_defect_with_unit_edges_matches_expected() {
        let n = 4usize;
        let edges = chain_edges_b1(n);
        let mut phi = vec![0.0_f32; n];
        phi[1] = 1.0;
        let nodal = Tensor::<B, 3>::from_data(Data::new(phi, Shape::new([1, n, 1])), &dev());
        let out = nodal_defect_tensor(nodal, Some(edges), None, 1.0_f32);
        let got: Vec<f32> = out.into_data().convert::<f32>().value;
        let want = [1.0_f32, 2.0, 0.5, 0.0];
        for i in 0..n {
            assert_abs_diff_eq!(got[i], want[i], epsilon = 1e-5);
        }
    }

    #[test]
    fn nodal_defect_scales_with_edge_length_from_positions() {
        let n = 3usize;
        let edges = chain_edges_b1(n);
        let mut coords = vec![0.0_f32; n * 3];
        for i in 0..n {
            coords[i * 3] = (i as f32) * 2.0;
        }
        let pos = Tensor::<B, 2>::from_data(Data::new(coords, Shape::new([n, 3])), &dev());
        let mut phi = vec![0.0_f32; n];
        phi[1] = 1.0;
        let nodal = Tensor::<B, 3>::from_data(Data::new(phi, Shape::new([1, n, 1])), &dev());
        let out = nodal_defect_tensor(nodal, Some(edges), Some(pos), 1.0_f32);
        let got: Vec<f32> = out.into_data().convert::<f32>().value;
        let want = [0.5_f32, 1.5, 0.5];
        for i in 0..n {
            assert_abs_diff_eq!(got[i], want[i], epsilon = 1e-4);
        }
    }

    #[test]
    fn emergence_hotspot_constant_sdf_equals_d_int() {
        let d_int = Tensor::<B, 4>::from_data(
            Data::new(vec![2.0_f32; 9], Shape::new([1, 1, 3, 3])),
            &dev(),
        );
        let sdf = Tensor::<B, 4>::zeros([1, 1, 3, 3], &dev());
        let monitor = EmergenceMonitor::<B>::new(0.5);
        let m = monitor.compute_dissipation_hotspots(d_int, sdf);
        let v: Vec<f32> = m.into_data().convert::<f32>().value;
        for x in v {
            assert_abs_diff_eq!(x, 2.0_f32, epsilon = 1e-5);
        }
    }

    #[test]
    fn emergence_monitor_default_lambda_matches_registry() {
        let monitor = EmergenceMonitor::<B>::with_default_lambda();
        assert_abs_diff_eq!(monitor.lambda, DEFAULT_EMERGENCE_LAMBDA);
    }

    #[test]
    fn emergence_posture_metadata_locked() {
        assert_eq!(W29_EMERGENCE_DEEPEN_CELL, "W29-023-EMERGENCE");
        assert_eq!(SLICE_ID, "phase-1-emergence-diagnostics");
        assert_eq!(CATALOG_ID, "umst.gate.emergence_diagnostics");
        assert_eq!(DEFAULT_MAX_EMERGENCE_VOXELS, 512);
        assert!(HONEST_FENCE.contains("production_wired=false"));
        assert!(HONEST_FENCE.contains("physics_green=false"));
        assert!(HONEST_FENCE.contains("master=false"));
    }

    #[test]
    fn emergence_posture_honest_prep_not_green() {
        let probe = emergence_posture_probe();
        assert!(emergence_posture_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master);
        assert!(!probe.ppo_trainer_hot_path_landed);
        assert!(!probe.msdf_emergence_grid_bridge_landed);
        assert_eq!(probe.fence_wired_count, EMERGENCE_FENCE_WIRED_COUNT);
        assert_eq!(emergence_fence_wired_count(), EMERGENCE_FENCE_WIRED_COUNT);
        validate_emergence_posture_honesty().expect("posture honesty");
    }

    #[test]
    fn emergence_fake_production_probe_fails_honesty() {
        let mut probe = emergence_posture_probe();
        probe.production_wired = true;
        assert!(!emergence_posture_honest(&probe));
        probe = emergence_posture_probe();
        probe.physics_green = true;
        assert!(!emergence_posture_honest(&probe));
        probe = emergence_posture_probe();
        probe.master = true;
        assert!(!emergence_posture_honest(&probe));
    }

    #[test]
    fn emergence_fence_facet_inventory_matches_wired_count() {
        assert_eq!(EMERGENCE_FENCE_FACETS.len(), EMERGENCE_FENCE_FACET_COUNT);
        assert_eq!(emergence_fence_wired_count(), 4);
        assert!(EMERGENCE_FENCE_FACETS.iter().filter(|f| f.wired).count() == 4);
    }
}
