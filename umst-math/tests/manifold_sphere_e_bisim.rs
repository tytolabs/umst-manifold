//! M-0: S² / S^n marker + primitive sphere SDF (ε-bisim to second run).
#![cfg(test)]

use std::f64::consts::PI;
use umst_math::manifold::primitives::{sphere_sdf, volume_and_surface_of_sphere};
use umst_math::manifold::Manifold;

#[test]
fn aa_manifold_s2_carrier_label() {
    let s = umst_math::manifold!(S2);
    assert!(s.carrier_label().contains("S2"));
}

#[test]
fn aa_manifold_sn_label_includes_n() {
    let s = umst_math::manifold!(Sn, 3u8);
    assert_eq!(s.n, 3);
    assert!(s.carrier_label().contains("S^n"));
}

#[test]
fn aa_manifold_sphere_sdf_runs_bisim() {
    let c = [0.0, 0.0, 0.0];
    let p = [0.1, 0.0, 0.0];
    let a = sphere_sdf(p, c, 1.0);
    let b = sphere_sdf(p, c, 1.0);
    assert!((a - b).abs() < 1e-15);
}

#[test]
fn aa_manifold_volume_surface_unit_sphere() {
    let (v, s) = volume_and_surface_of_sphere(1.0);
    assert!((v - 4.0 * PI / 3.0).abs() < 1e-9);
    assert!((s - 4.0 * PI).abs() < 1e-9);
}

#[test]
fn aa_manifold_bounding_inradius_unit_cube() {
    use umst_math::manifold::primitives::bounding_inradius_from_aabb;
    let r = bounding_inradius_from_aabb(2.0, 2.0, 2.0);
    assert!((r - 1.0).abs() < 1e-9);
}
