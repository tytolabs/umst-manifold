// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Pure graph / DEC operators: no clocks, no material cartridges, no Krylov loops.
//!
//! ## Functorial / natural reading (primal vs dual discrete spaces)
//!
//! Fix an oriented graph \(G=(V,E)\). Nodal tensors live on **primal 0-cells** \(C^0\); edge tensors
//! on **primal 1-cells** \(C^1\). The incidence gather/scatter used by [`super::topology::EdgeTopology`]
//! is the literal Burn realisation of the chain maps \(B_1^\top: C^1\to C^0\) (weak divergence) and
//! \(d_0: C^0\to C^1\) (coboundary / edge increment). Changing only the **cochain values** while
//! holding `edges_b1` fixed is a linear natural transformation between finite-dimensional spaces of
//! sections; metric/Hodge weights (when present in callers) post-compose on \(C^k\) before duality
//! pairings. [`super::dec_primal::primal_d1_edge_flux_to_faces`] and its transpose extend the same
//! pattern to 2-cells via `faces_b2` column ranges.
//!
//! Re-exports live modules for the `physics::operators::*` path; legacy `physics::laplacian`
//! and `physics::dec_operators` remain for stable imports.
//!
//! # Honest boundary (W29-062)
//!
//! This module is the **pure DEC / graph operator facade** (re-exports of
//! [`super::dec_operators`], [`super::dec_primal`], [`super::laplacian`]). It does **not** own
//! clocks, material cartridges, or Krylov host loops (those live under [`super::operator`] /
//! solvers). Unit contracts: `cargo test -p umst-manifold operators`. Not physics GREEN, not
//! `PRODUCTION_WIRED`, not `MASTER`, not OP-5.

/// W29 deepen cell — pure operators facade honest fence bundle.
pub const W29_OPERATORS_DEEPEN_CELL: &str = "W29-062-OPERATORS";

/// Honest posture tag — DEC/graph facade re-exports landed; fleet production wiring refused.
pub const OPERATORS_POSTURE_TAG: &str = "honest-pure-dec-graph-operators-facade-research-lane";

/// Honest physics posture — facade unit contracts pass; does not certify fleet physics GREEN.
pub const OPERATORS_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by the pure-operator facade alone.
pub const OPERATORS_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const OPERATORS_MASTER: bool = false;

/// OP-5 ceremony pin — not claimed by this module.
pub const OPERATORS_OP5: bool = false;

/// Whether dec_operators / dec_primal / laplacian re-exports are landed on this facade.
pub const OPERATORS_FACADE_REEXPORTS_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const OPERATORS_HONEST_FENCE: &str =
    "operators_facade_reexports_landed=true|dec_operators_reexport=true|dec_primal_reexport=true|laplacian_reexport=true|production_wired=false|physics_green=false|master=false|op5=false";

const _: () = assert!(!OPERATORS_PHYSICS_GREEN);
const _: () = assert!(!OPERATORS_PRODUCTION_WIRED);
const _: () = assert!(!OPERATORS_MASTER);
const _: () = assert!(!OPERATORS_OP5);
const _: () = assert!(OPERATORS_FACADE_REEXPORTS_LANDED);

/// Typed probe for pure operators facade posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorsFacadePostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub facade_reexports_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the pure operators facade.
#[must_use]
pub fn operators_facade_honest_posture_bundle() -> OperatorsFacadePostureProbe {
    OperatorsFacadePostureProbe {
        physics_green: OPERATORS_PHYSICS_GREEN,
        production_wired: OPERATORS_PRODUCTION_WIRED,
        master: OPERATORS_MASTER,
        op5: OPERATORS_OP5,
        facade_reexports_landed: OPERATORS_FACADE_REEXPORTS_LANDED,
        honest_fence: OPERATORS_HONEST_FENCE,
        posture_tag: OPERATORS_POSTURE_TAG,
        deepen_cell: W29_OPERATORS_DEEPEN_CELL,
    }
}

/// Facade re-exports landed with production / master / OP-5 / GREEN composition honestly open.
#[must_use]
pub fn operators_facade_posture_honest(probe: &OperatorsFacadePostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && probe.facade_reexports_landed
        && probe.deepen_cell == W29_OPERATORS_DEEPEN_CELL
        && probe.posture_tag == OPERATORS_POSTURE_TAG
        && probe
            .honest_fence
            .contains("operators_facade_reexports_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER / OP-5 claims on the operators facade.
#[must_use]
pub fn operators_facade_refuse_overclaim(
    probe: &OperatorsFacadePostureProbe,
) -> Result<(), &'static str> {
    if probe.physics_green {
        return Err("OPERATORS_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired {
        return Err("OPERATORS_PRODUCTION_WIRED must stay false until embodied loop closes");
    }
    if probe.master {
        return Err("OPERATORS_MASTER must stay false — not claimed by facade re-exports alone");
    }
    if probe.op5 {
        return Err("OPERATORS_OP5 must stay false — not claimed by facade re-exports alone");
    }
    if !operators_facade_posture_honest(probe) {
        return Err("operators facade posture fence inconsistent");
    }
    Ok(())
}

pub use super::dec_operators::*;
pub use super::dec_primal::*;
pub use super::laplacian::*;

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Data;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    fn chain_graph_edges(
        n_nodes: usize,
        device: &<B as burn::tensor::backend::Backend>::Device,
    ) -> burn::tensor::Tensor<B, 2, burn::tensor::Int> {
        let ne = n_nodes.saturating_sub(1);
        let mut e = Vec::with_capacity(ne * 2);
        for i in 0..ne {
            e.push(i as i64);
        }
        for i in 0..ne {
            e.push((i + 1) as i64);
        }
        let flat: Vec<f32> = e.iter().map(|&x| x as f32).collect();
        burn::tensor::Tensor::<B, 1>::from_data(Data::new(flat, [e.len()].into()), device)
            .reshape([2, ne])
            .int()
    }

    #[test]
    fn operators_honest_posture_refuses_green_production_master_op5() {
        let probe = operators_facade_honest_posture_bundle();
        assert!(operators_facade_posture_honest(&probe));
        assert!(operators_facade_refuse_overclaim(&probe).is_ok());
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert!(probe.facade_reexports_landed);
        assert_eq!(probe.deepen_cell, W29_OPERATORS_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, OPERATORS_POSTURE_TAG);
        assert!(!OPERATORS_PHYSICS_GREEN);
        assert!(!OPERATORS_PRODUCTION_WIRED);
        assert!(!OPERATORS_MASTER);
        assert!(!OPERATORS_OP5);
    }

    #[test]
    fn operators_facade_reexports_dec_edge_arithmetic_mean() {
        let device = Default::default();
        let nodal = burn::tensor::Tensor::<B, 3>::from_data(
            Data::new(vec![2.0_f32, 4.0, 6.0], [1, 3, 1].into()),
            &device,
        );
        let edges = chain_graph_edges(3, &device);
        // Via facade re-export of DecEdgeOperators (not direct dec_operators path).
        let edge_vals = DecEdgeOperators::arithmetic_mean_on_edges(nodal, edges);
        let v = edge_vals.into_data().value;
        assert_eq!(v.len(), 2);
        assert!((v[0] - 3.0).abs() < 1e-5, "edge 0→1 mean = {}", v[0]);
        assert!((v[1] - 5.0).abs() < 1e-5, "edge 1→2 mean = {}", v[1]);
    }

    #[test]
    fn operators_facade_reexports_primal_edge_increment_on_chain() {
        use crate::physics::topology::EdgeTopology;

        let device = Default::default();
        let nodal = burn::tensor::Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 3.0, 6.0], [1, 3, 1].into()),
            &device,
        );
        let edges = chain_graph_edges(3, &device);
        let topo = EdgeTopology::new(edges);
        // Via facade re-export of primal_scalar_edge_increment.
        let d0 = primal_scalar_edge_increment(nodal, &topo);
        let v = d0.into_data().value;
        assert_eq!(v.len(), 2);
        assert!((v[0] - 2.0).abs() < 1e-5, "d0 edge0 = {}", v[0]);
        assert!((v[1] - 3.0).abs() < 1e-5, "d0 edge1 = {}", v[1]);
    }

    #[test]
    fn operators_facade_reexports_topological_laplacian_constant_field() {
        let device = Default::default();
        let x = burn::tensor::Tensor::<B, 3>::from_data(
            Data::new(vec![5.0_f32; 3], [1, 3, 1].into()),
            &device,
        );
        let damage = burn::tensor::Tensor::<B, 3>::zeros([1, 3, 1], &device);
        let edges = chain_graph_edges(3, &device);
        let lap = TopologicalLaplacian::scalar_laplacian(x, edges, damage);
        let v = lap.into_data().value;
        assert_eq!(v.len(), 3);
        for (i, &vi) in v.iter().enumerate() {
            assert!(vi.abs() < 1e-5, "constant field Laplacian[{i}] = {vi}");
        }
    }

    #[test]
    fn operators_facade_downstream_postures_remain_honest() {
        // Facade must not invent GREEN on surfaces it re-exports.
        assert!(dec_edge_operators_posture_honest(
            &dec_edge_operators_honest_posture_bundle()
        ));
        assert!(dec_primal_posture_honest(
            &dec_primal_honest_posture_bundle()
        ));
        assert!(laplacian_posture_honest(&laplacian_honest_posture_bundle()));
        assert!(validate_laplacian_honesty().is_ok());
    }
}
