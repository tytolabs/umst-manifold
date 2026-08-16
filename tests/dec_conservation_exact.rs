// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! R14-5 — exact `d₁∘d₀ = 0` on canonical tet boundary DEC complex.

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::dec_primal::{
    canonical_tetrahedron_boundary_dec_coo, dec_primal_max_abs_d1_of_scalar_gradient,
    DEC_DD_ZERO_EXACT_MEASURED,
};
use umst_manifold::physics::topology::EdgeTopology;

type B = NdArray<f32>;

fn tet_topology(
    device: &NdArrayDevice,
) -> (EdgeTopology<B>, Tensor<B, 2, Int>, Vec<(usize, usize)>) {
    let coo = canonical_tetrahedron_boundary_dec_coo();
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(coo.edges_b1_flat.to_vec(), Shape::new([2, 6])),
        device,
    );
    let faces_b2: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(coo.faces_b2_flat.to_vec(), Shape::new([2, 12])),
        device,
    );
    let ranges: Vec<(usize, usize)> = coo.face_column_ranges.to_vec();
    let topo = EdgeTopology::new(edges_b1);
    (topo, faces_b2, ranges)
}

#[test]
fn dec_dd_zero_exact_on_tetrahedron_boundary() {
    assert!(DEC_DD_ZERO_EXACT_MEASURED);
    let device = NdArrayDevice::default();
    let (topo, faces_b2, ranges) = tet_topology(&device);
    let nodal: Tensor<B, 3> = Tensor::from_data(
        Data::new(vec![0.0_f32, 1.0, 2.0, 3.0], Shape::new([1, 4, 1])),
        &device,
    );
    let max_abs = dec_primal_max_abs_d1_of_scalar_gradient(nodal, &topo, faces_b2, &ranges);
    assert_eq!(max_abs, 0.0, "d₁∘d₀ must be exactly zero, got {max_abs}");
}

#[test]
fn dec_dd_zero_exact_on_quadratic_potential() {
    let device = NdArrayDevice::default();
    let (topo, faces_b2, ranges) = tet_topology(&device);
    let nodal: Tensor<B, 3> = Tensor::from_data(
        Data::new(vec![0.0_f32, 1.0, 4.0, 9.0], Shape::new([1, 4, 1])),
        &device,
    );
    let max_abs = dec_primal_max_abs_d1_of_scalar_gradient(nodal, &topo, faces_b2, &ranges);
    assert_eq!(
        max_abs, 0.0,
        "d₁∘d₀ must be exactly zero on closed chain, got {max_abs}"
    );
}
