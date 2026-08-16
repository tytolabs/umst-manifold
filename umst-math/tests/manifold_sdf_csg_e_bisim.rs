// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! CSG / SDFGate parity (Haskell `SDFGate.hs` discipline; no FFI).
#![cfg(test)]

use umst_math::manifold::canonicalize::canonicalize_voxelize;
use umst_math::manifold::csg::{
    clausius_duhem_sdf, default_smooth_k, gate_sdf, hard_intersection, hard_union,
    mass_conservation_sdf, smooth_min, ThermoGateState,
};
use umst_math::manifold::sdf::{GateSdf, Sdf};

#[test]
fn aa_manifold_hard_union_assoc() {
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let c: f64 = 0.15;
    let u1 = hard_union(a, hard_union(b, c));
    let u2 = hard_union(hard_union(a, b), c);
    assert!((u1 - u2).abs() < 1e-15);
}

#[test]
fn aa_manifold_hard_intersection_is_min() {
    let a: f64 = 0.3;
    let b: f64 = 0.1;
    assert!((hard_intersection(a, b) - a.min(b)).abs() < 1e-15);
}

#[test]
fn aa_manifold_smooth_min_to_min_k_small() {
    let a = 0.4_f64;
    let b = 0.1_f64;
    let s = smooth_min(a, b, 1e-5);
    assert!((s - a.min(b)).abs() < 0.15);
}

#[test]
fn aa_manifold_gate_sdf_on_fixture_matches_const_field() {
    let old = ThermoGateState {
        density: 2400.0,
        free_energy: -10.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let new = ThermoGateState {
        density: 2450.0,
        free_energy: -12.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let g = gate_sdf(&old, &new);
    let sdf = GateSdf::from_thermo_pair(&old, &new);
    assert!((g - sdf.dist([0.0, 0.0, 0.0])).abs() < 1e-9);
    let (h, _) = canonicalize_voxelize(&sdf, 3).expect("vox");
    let (h2, _) = canonicalize_voxelize(&sdf, 3).expect("vox");
    assert_eq!(h, h2);
}

#[test]
fn aa_manifold_sub_sdf_pieces() {
    let o = ThermoGateState {
        density: 2400.0,
        free_energy: 0.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let n = ThermoGateState {
        density: 2500.0,
        free_energy: 0.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let m = mass_conservation_sdf(&o, &n);
    let c = clausius_duhem_sdf(&o, &n);
    let g = hard_union(m, c);
    assert!((g - m.max(c)).abs() < 1e-9);
}

#[test]
fn aa_manifold_default_smooth_k_positive() {
    let k = default_smooth_k();
    assert!(k > 0.0);
}

#[test]
fn aa_manifold_smooth_converges() {
    let a = 1.0_f64;
    let b = 2.0_f64;
    for i in 2..7 {
        let k = 10.0f64.powi(-i);
        let s = smooth_min(a, b, k);
        if i >= 4 {
            assert!((s - a.min(b)).abs() < 0.1);
        }
    }
}
