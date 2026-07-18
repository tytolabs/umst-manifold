// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Graph DEC adjoint witness on a toy 2-node / 1-edge mesh.
//!
//! Uses [`B1Incidence`] from [`umst_manifold::core::dec_typestate`] as the typed carrier for
//! oriented primal **B₁** incidence (`edges_b1` shape `[2, E]`). Proves the unweighted Frobenius
//! pairing identity
//!
//! ```text
//! ⟨B₁ᵀ ω, u⟩ = ⟨ω, B₁ u⟩
//! ```
//!
//! where `B₁ᵀ` is [`primal_scalar_edge_increment`] (0-cochain gradient) and `B₁` is
//! [`primal_divergence_from_edge_flux_topo`] (weak divergence). Also witnesses `B₁ᵀ B₁` as the
//! graph Laplacian on the same toy mesh.

use approx::assert_abs_diff_eq;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::core::dec_typestate::B1Incidence;
use umst_manifold::physics::dec_primal::{
    primal_divergence_from_edge_flux_topo, primal_scalar_edge_increment,
};

type B = NdArray;

fn tensor_inner(a: Tensor<B, 3>, b: Tensor<B, 3>) -> f32 {
    a.mul(b).sum().into_scalar()
}

/// Single oriented edge `0 → 1` on two vertices, wrapped as [`B1Incidence`].
fn toy_two_node_one_edge_b1() -> B1Incidence<B> {
    let device = Default::default();
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1], Shape::new([2, 1])), &device);
    B1Incidence::try_new(edges_b1).expect(
        "B1Incidence::try_new on toy 2-node 1-edge DEC graph mesh (FP §6 Track DEC mesh FEM harness)",
    )
}

#[test]
fn dec_graph_adjoint_b1_transpose_composition_matches_pairing() {
    let b1 = toy_two_node_one_edge_b1();
    assert_eq!(b1.n_edges(), 1);
    let topo = b1.to_edge_topology();
    let device = Default::default();

    let omega = Tensor::from_data(
        Data::new(vec![1.2_f32, -0.45], Shape::new([1, 2, 1])),
        &device,
    );
    let edge_flux = Tensor::from_data(Data::new(vec![0.7_f32], Shape::new([1, 1, 1])), &device);
    let nodal_template = Tensor::zeros([1, 2, 1], &device);

    // Sign convention: primal_divergence scatters +flux at src, −flux at tgt; adjoint pairs with −B₁ᵀ.
    let b1t_omega = primal_scalar_edge_increment(omega.clone(), &topo).neg();
    // B₁ u: edge 1-cochain → nodal 0-cochain (divergence).
    let b1_u = primal_divergence_from_edge_flux_topo(edge_flux.clone(), &topo, &nodal_template);

    let lhs = tensor_inner(b1t_omega, edge_flux);
    let rhs = tensor_inner(omega, b1_u);
    assert_abs_diff_eq!(lhs, rhs, epsilon = 1.0e-5);
}

#[test]
fn dec_graph_adjoint_b1_transpose_b1_is_graph_laplacian_on_toy_mesh() {
    let b1 = toy_two_node_one_edge_b1();
    let topo = b1.to_edge_topology();
    let device = Default::default();

    let omega = Tensor::from_data(
        Data::new(vec![2.0_f32, 5.0], Shape::new([1, 2, 1])),
        &device,
    );
    let nodal_template = Tensor::zeros([1, 2, 1], &device);

    let grad = primal_scalar_edge_increment(omega.clone(), &topo);
    let lap_omega = primal_divergence_from_edge_flux_topo(grad, &topo, &nodal_template);
    let v: Vec<f32> = lap_omega.into_data().value;
    assert_eq!(v.len(), 2);
    let increment = 5.0_f32 - 2.0_f32;
    assert_abs_diff_eq!(v[0], increment, epsilon = 1.0e-5);
    assert_abs_diff_eq!(v[1], -increment, epsilon = 1.0e-5);
    assert_abs_diff_eq!(v[0] + v[1], 0.0, epsilon = 1.0e-5);
}
