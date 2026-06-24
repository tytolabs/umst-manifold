//! Hilbert 2D d=4: 16² = 256 indices, locality, C ≤ 6 witness.
#![cfg(test)]

use umst_math::manifold::hilbert::{d2xy, manhattan, xy2d, HilbertCurve};

#[test]
fn aa_manifold_hilbert_d4_covers_256() {
    let n = 4u8;
    let side = 1u32 << n;
    assert_eq!(side * side, 256);
    use std::collections::BTreeSet;
    let mut s = BTreeSet::new();
    for d in 0u32..256 {
        let p = d2xy(n, d);
        assert!(s.insert(p), "dupe at {d} -> {p:?}");
    }
    assert_eq!(s.len(), 256, "cover all cells");
    let _ = side;
}

#[test]
fn aa_manifold_hilbert_indices_in_range() {
    for d in 0u32..256 {
        let (x, y) = d2xy(4, d);
        assert!(x < 16 && y < 16);
    }
}

#[test]
fn aa_manifold_hilbert_adjacent_on_curve() {
    for d in 0u32..255 {
        let a = d2xy(4, d);
        let b = d2xy(4, d + 1);
        let m = manhattan(a, b);
        assert!(m <= 1, "Hilbert step d={d} manhattan {m}");
    }
}

#[test]
fn aa_manifold_hilbert_locality_c_bound() {
    // Policy row `manifold_hilbert_locality_constant` = 6: empirical on order-4 grid,
    // for pairs with curve distance ≤ 8, manhattan in grid is ≤ 6 * curve_distance^0.5 + ε.
    let n = 4u8;
    const C: f64 = 6.0;
    for d0 in 0u32..248u32 {
        for w in 1u32..=8u32 {
            let d1 = d0 + w;
            if d1 >= 256 {
                break;
            }
            let p0 = d2xy(n, d0);
            let p1 = d2xy(n, d1);
            let m = manhattan(p0, p1) as f64;
            assert!(
                m <= C * (w as f64).sqrt() + 0.1,
                "C witness d0={d0} w={w} m={m}"
            );
        }
    }
}

#[test]
fn aa_manifold_xy2d_roundtrip() {
    for d in 0u32..256u32 {
        let (x, y) = d2xy(4, d);
        let e = xy2d(4, x, y).expect("ok");
        assert_eq!(d, e);
    }
}

#[test]
fn aa_manifold_hilbert_curve_new_2d() {
    let h = HilbertCurve::new_2d(4).expect("ok");
    assert_eq!(h.bits, 4);
    assert_eq!(h.dim, 2);
    let t = h.d2xy(5).expect("in range");
    assert_eq!(d2xy(4, 5), t);
}
