// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Dissipation–geometry diagnostics: **structured grid** hotspots plus **sparse nodal** defect helpers.
//!
//! [`EmergenceMonitor`] builds \(m = D_{int} + \lambda|\nabla \mathrm{SDF}|^2\) on `[B,D,H,W]`.
//! [`nodal_defect_tensor`] uses the same `edges_b1` gather/scatter layout as
//! [`crate::physics::laplacian::TopologicalLaplacian::scalar_laplacian`] (primal edge increment, no damage mask).

#![allow(clippy::single_range_in_vec_init)] // Burn `Tensor::slice` uses `[Range<usize>; k]` per dimension; Clippy misreads single-row slices.

use burn::tensor::{backend::Backend, Int, Tensor};

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
}
