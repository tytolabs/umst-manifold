// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
#![allow(clippy::single_range_in_vec_init)]

//! DEC maps between nodal samples and edge values (Refinement #3).
//!
//! **Naming:** edge-wise reductions are grouped on [`DecEdgeOperators`]; there is no `DecOperator`
//! type in this crate (see repository markdown `docs/FP_CATEGORICAL_DEC.md`).
//!
//! These reductions are **nonlinear** maps \(C^0\to C^1\) on the same primal skeleton as
//! [`super::dec_primal`]’s linear \(d_0\): they average intensive data at endpoints for edge-wise
//! material models. Functorially: they post-compose with gather \((\mathrm{id}\otimes\mathrm{id})^*: C^0\times C^0\to\) edge slots;
//! naturality is with respect to **relabeling / restriction of vertices** that commute with `edges_b1`.
//!
//! - **Arithmetic mean** on edges: intensive mechanical moduli (Young’s modulus) on the primal 1-skeleton.
//! - **Harmonic mean** on edges: flux-consistent reduction for positive transport coefficients.
//!
//! # Honest boundary (W29-049)
//!
//! Edge-wise **nonlinear** cochain reductions are **topology-gated** intensive means on the primal
//! 1-skeleton via [`super::topology::EdgeTopology`]. Linear incidence (`d_0`, divergence) stays in
//! [`super::dec_primal`]. Mean contracts are exercised by `cargo test -p umst-manifold dec_operators`.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`.

/// W29 deepen cell — DEC edge-operator honest fence bundle.
pub const W29_DEC_OPERATORS_DEEPEN_CELL: &str = "W29-049-DEC_OPERATORS";

/// Honest posture tag — edge reductions landed; fleet production wiring refused.
pub const DEC_OPERATORS_POSTURE_TAG: &str = "honest-dec-edge-reductions-research-lane";

/// Honest physics posture — mean contracts pass unit tests; does not certify fleet physics GREEN.
pub const DEC_OPERATORS_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by edge-mean reductions alone.
pub const DEC_OPERATORS_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const DEC_OPERATORS_MASTER: bool = false;

/// Whether arithmetic and harmonic mean Burn contracts are landed in this module.
pub const DEC_OPERATORS_MEAN_CONTRACTS_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const DEC_OPERATORS_HONEST_FENCE: &str =
    "dec_edge_means_landed=true arithmetic_mean_wired=true harmonic_mean_wired=true production_wired=false master_composition_wired=false";

/// Typed probe for DEC edge-operator posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecEdgeOperatorsPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub mean_contracts_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for DEC edge operators.
#[must_use]
pub fn dec_edge_operators_honest_posture_bundle() -> DecEdgeOperatorsPostureProbe {
    DecEdgeOperatorsPostureProbe {
        physics_green: DEC_OPERATORS_PHYSICS_GREEN,
        production_wired: DEC_OPERATORS_PRODUCTION_WIRED,
        master: DEC_OPERATORS_MASTER,
        mean_contracts_landed: DEC_OPERATORS_MEAN_CONTRACTS_LANDED,
        honest_fence: DEC_OPERATORS_HONEST_FENCE,
        posture_tag: DEC_OPERATORS_POSTURE_TAG,
        deepen_cell: W29_DEC_OPERATORS_DEEPEN_CELL,
    }
}

/// Edge-mean SSOT landed with production/master composition honestly open.
#[must_use]
pub fn dec_edge_operators_posture_honest(probe: &DecEdgeOperatorsPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.mean_contracts_landed
        && probe.honest_fence.contains("dec_edge_means_landed=true")
        && probe.honest_fence.contains("production_wired=false")
}

use burn::tensor::{backend::Backend, Int, Tensor};

use super::topology::EdgeTopology;

pub struct DecEdgeOperators;

impl DecEdgeOperators {
    /// Arithmetic mean at edge endpoints. `[B, N, C]` → `[B, E, C]`.
    pub fn arithmetic_mean_on_edges<B: Backend>(
        nodal: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
    ) -> Tensor<B, 3> {
        let topo = EdgeTopology::new(edges_b1);
        let (srcv, tgtv) = topo.gather_endpoints(nodal);
        srcv.add(tgtv).mul_scalar(0.5_f32)
    }

    /// Harmonic mean with epsilon floor: `2(a+\epsilon)(b+\epsilon) / (a+b+2\epsilon)`.
    pub fn harmonic_mean_on_edges<B: Backend>(
        nodal: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        eps: f32,
    ) -> Tensor<B, 3> {
        let topo = EdgeTopology::new(edges_b1);
        let (srcv, tgtv) = topo.gather_endpoints(nodal);
        let sa = srcv.clone().add_scalar(eps);
        let sb = tgtv.clone().add_scalar(eps);
        sa.clone()
            .mul(sb.clone())
            .mul_scalar(2.0_f32)
            .div(sa.add(sb))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Data;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    fn chain_graph_edges(
        n_nodes: usize,
        device: &<B as burn::tensor::backend::Backend>::Device,
    ) -> Tensor<B, 2, Int> {
        let ne = n_nodes.saturating_sub(1);
        let mut e = Vec::with_capacity(ne * 2);
        for i in 0..ne {
            e.push(i as i64);
        }
        for i in 0..ne {
            e.push((i + 1) as i64);
        }
        Tensor::<B, 2, Int>::from_data(Data::new(e, [2, ne].into()), device)
    }

    #[test]
    fn dec_operators_honest_posture_refuses_green_and_production() {
        let probe = dec_edge_operators_honest_posture_bundle();
        assert!(dec_edge_operators_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert_eq!(probe.deepen_cell, W29_DEC_OPERATORS_DEEPEN_CELL);
    }

    #[test]
    fn dec_operators_arithmetic_mean_on_chain() {
        let device = Default::default();
        let nodal = Tensor::<B, 3>::from_data(
            Data::new(vec![2.0_f32, 4.0, 6.0], [1, 3, 1].into()),
            &device,
        );
        let edges = chain_graph_edges(3, &device);
        let edge_vals = DecEdgeOperators::arithmetic_mean_on_edges(nodal, edges);
        let v = edge_vals.into_data().value;
        assert_eq!(v.len(), 2);
        assert!((v[0] - 3.0).abs() < 1e-5, "edge 0→1 mean = {}", v[0]);
        assert!((v[1] - 5.0).abs() < 1e-5, "edge 1→2 mean = {}", v[1]);
    }

    #[test]
    fn dec_operators_arithmetic_mean_constant_field() {
        let device = Default::default();
        let nodal =
            Tensor::<B, 3>::from_data(Data::new(vec![7.5_f32; 4], [1, 4, 1].into()), &device);
        let edges = chain_graph_edges(4, &device);
        let edge_vals = DecEdgeOperators::arithmetic_mean_on_edges(nodal, edges);
        for x in edge_vals.into_data().value {
            assert!(
                (x - 7.5).abs() < 1e-5,
                "constant nodal field → constant edge mean"
            );
        }
    }

    #[test]
    fn dec_operators_harmonic_mean_matches_closed_form() {
        let device = Default::default();
        let nodal =
            Tensor::<B, 3>::from_data(Data::new(vec![2.0_f32, 6.0], [1, 2, 1].into()), &device);
        let edges = chain_graph_edges(2, &device);
        let eps = 0.0_f32;
        let edge_vals = DecEdgeOperators::harmonic_mean_on_edges(nodal, edges, eps);
        let h = edge_vals.into_data().value[0];
        let expected = 2.0 * 2.0 * 6.0 / (2.0 + 6.0);
        assert!(
            (h - expected).abs() < 1e-5,
            "harmonic mean 2↔6 = {h}, expected {expected}"
        );
    }

    #[test]
    fn dec_operators_harmonic_mean_eps_floor_positive() {
        let device = Default::default();
        let nodal =
            Tensor::<B, 3>::from_data(Data::new(vec![0.0_f32, 0.0], [1, 2, 1].into()), &device);
        let edges = chain_graph_edges(2, &device);
        let eps = 1.0_f32;
        let edge_vals = DecEdgeOperators::harmonic_mean_on_edges(nodal, edges, eps);
        let h = edge_vals.into_data().value[0];
        // 2(eps)(eps) / (2*eps) = eps
        assert!(
            (h - eps).abs() < 1e-5,
            "zero nodal + eps floor → {h}, expected {eps}"
        );
    }

    #[test]
    fn dec_operators_means_symmetric_under_endpoint_swap() {
        let device = Default::default();
        let nodal =
            Tensor::<B, 3>::from_data(Data::new(vec![3.0_f32, 9.0], [1, 2, 1].into()), &device);
        let fwd = Tensor::<B, 2, Int>::from_data(Data::new(vec![0i64, 1], [2, 1].into()), &device);
        let rev = Tensor::<B, 2, Int>::from_data(Data::new(vec![1i64, 0], [2, 1].into()), &device);
        let arith_fwd = DecEdgeOperators::arithmetic_mean_on_edges(nodal.clone(), fwd.clone());
        let arith_rev = DecEdgeOperators::arithmetic_mean_on_edges(nodal.clone(), rev.clone());
        let harm_fwd = DecEdgeOperators::harmonic_mean_on_edges(nodal.clone(), fwd, 0.0);
        let harm_rev = DecEdgeOperators::harmonic_mean_on_edges(nodal, rev, 0.0);
        assert!((arith_fwd.into_data().value[0] - arith_rev.into_data().value[0]).abs() < 1e-5);
        assert!((harm_fwd.into_data().value[0] - harm_rev.into_data().value[0]).abs() < 1e-5);
    }
}
