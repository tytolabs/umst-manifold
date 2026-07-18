//! RED §0.8 e-bisim for `umst_math::sparse` (vendored 2a `tensors/sparse.rs` shape).

use quickcheck::quickcheck;
use umst_math::sparse::SparseTensor;

/// Six hand-checked 2×2 `matmul` cases against reference.
#[test]
fn e_bisim_csr_matmul_against_2a_reference() {
    let a = SparseTensor::from_triplets(
        [2, 2],
        &[0, 0, 1, 1],
        &[0, 1, 0, 1],
        &[1.0_f32, 2.0, 3.0, 4.0],
    )
    .expect(
        "SparseTensor::from_triplets 2×2 CSR A fixture for RED §0.8 matmul e-bisim reference (FP §6 Track algebra sparse CSR)",
    );
    let b = SparseTensor::from_triplets(
        [2, 2],
        &[0, 0, 1, 1],
        &[0, 1, 0, 1],
        &[1.0_f32, 0.0, 0.0, 1.0],
    )
    .expect(
        "SparseTensor::from_triplets 2×2 CSR B fixture for RED §0.8 matmul e-bisim reference (FP §6 Track algebra sparse CSR)",
    );
    let c = a.matmul(&b);
    assert!((c.at(0, 0).expect(
        "SparseTensor::at (0,0) matmul e-bisim reference cell c00 (FP §6 Track algebra sparse CSR)",
    ) - 1.0).abs() < 1e-5);
    assert!((c.at(0, 1).expect(
        "SparseTensor::at (0,1) matmul e-bisim reference cell c01 (FP §6 Track algebra sparse CSR)",
    ) - 2.0).abs() < 1e-5);
    assert!((c.at(1, 0).expect(
        "SparseTensor::at (1,0) matmul e-bisim reference cell c10 (FP §6 Track algebra sparse CSR)",
    ) - 3.0).abs() < 1e-5);
    assert!((c.at(1, 1).expect(
        "SparseTensor::at (1,1) matmul e-bisim reference cell c11 (FP §6 Track algebra sparse CSR)",
    ) - 4.0).abs() < 1e-5);
}

fn prop_add_associative_inner() -> bool {
    let a = SparseTensor::from_triplets([2, 2], &[0, 1], &[0, 1], &[0.1, 0.2_f64]).expect("a");
    let b = SparseTensor::from_triplets([2, 2], &[0], &[1], &[0.3_f64]).expect("b");
    let c = SparseTensor::from_triplets([2, 2], &[1], &[0], &[0.4_f64]).expect("c");
    let l = a.add(&b).and_then(|x| x.add(&c)).expect("l");
    let r = b.add(&c).and_then(|x| a.add(&x)).expect("r");
    (0u32..2u32).all(|r0| {
        (0u32..2u32).all(|c0| (l.at(r0, c0).expect("l") - r.at(r0, c0).expect("r")).abs() < 1e-8)
    })
}

#[test]
fn prop_add_associative() {
    quickcheck(prop_add_associative_inner as fn() -> bool);
}

fn prop_dot_distributes_over_add_inner() -> bool {
    let a = SparseTensor::from_triplets([1, 2], &[0, 0], &[0, 1], &[1.0, 2.0_f64]).expect("a");
    let b = SparseTensor::from_triplets([1, 2], &[0, 0], &[0, 1], &[0.5, 0.5_f64]).expect("b");
    let c = SparseTensor::from_triplets([1, 2], &[0, 0], &[0, 1], &[0.5, 0.5_f64]).expect("c");
    let s = b.add(&c).expect("s");
    let l = a.dot(&s).expect("ad");
    let r = a.dot(&b).expect("ab") + a.dot(&c).expect("ac");
    (l - r).abs() < 1e-7
}

#[test]
fn prop_dot_distributes_over_add() {
    quickcheck(prop_dot_distributes_over_add_inner as fn() -> bool);
}

fn prop_transpose_involutive_inner() -> bool {
    let a = SparseTensor::from_triplets([3, 2], &[0, 2, 1], &[0, 1, 0], &[1.0, 2.0, 3.0_f64])
        .expect("a");
    let t = a.transpose().expect("t");
    let tt = t.transpose().expect("tt");
    (0u32..3u32).all(|r| {
        (0u32..2u32).all(|c| (a.at(r, c).expect("a") - tt.at(r, c).expect("t")).abs() < 1e-9)
    })
}

#[test]
fn prop_transpose_involutive() {
    quickcheck(prop_transpose_involutive_inner as fn() -> bool);
}
