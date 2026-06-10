//! Resolution-level hash monotonicity + byte determinism.
#![cfg(test)]

use umst_math::manifold::canonicalize::{canonicalize_voxelize, fnv1a_64, stack_refinement_h8};
use umst_math::manifold::csg::ThermoGateState;
use umst_math::manifold::sdf::GateSdf;

#[test]
fn aa_manifold_hash_prefix_monotone() {
    let t = ThermoGateState {
        density: 2400.0,
        free_energy: -1.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let t2 = ThermoGateState {
        density: 2401.0,
        free_energy: -1.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let s = GateSdf::from_thermo_pair(&t, &t2);
    let (h8, b8) = canonicalize_voxelize(&s, 3).expect("c");
    let (h9, b9) = canonicalize_voxelize(&s, 4).expect("c");
    let tail = fnv1a_64(&b9);
    let stack = stack_refinement_h8(h8, tail);
    assert_eq!(&stack[..8], h8.as_slice());
    assert_ne!(h8, h9);
    let _ = (b8, b9, stack);
}

#[test]
fn aa_manifold_canonicalize_determinism() {
    let t = ThermoGateState {
        density: 2400.0,
        free_energy: 0.0,
        hydration: 0.2,
        strength: 0.0,
        max_strength: 1.0,
    };
    let t2 = ThermoGateState {
        density: 2400.0,
        free_energy: 0.0,
        hydration: 0.3,
        strength: 0.0,
        max_strength: 1.0,
    };
    let s = GateSdf::from_thermo_pair(&t, &t2);
    let (a, b) = (
        canonicalize_voxelize(&s, 4).expect("a").0,
        canonicalize_voxelize(&s, 4).expect("b").0,
    );
    assert_eq!(a, b);
}

#[test]
fn aa_manifold_refinement_bytes_distinct() {
    let t = ThermoGateState {
        density: 2400.0,
        free_energy: 0.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let t2 = ThermoGateState {
        density: 2500.0,
        free_energy: 0.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let s = GateSdf::from_thermo_pair(&t, &t2);
    let (h1, v1) = canonicalize_voxelize(&s, 3).expect("c");
    let (h2, v2) = canonicalize_voxelize(&s, 4).expect("c");
    assert_ne!(v1.len(), v2.len());
    // prefix property of composite 16-byte stack
    let st = stack_refinement_h8(h1, fnv1a_64(&v2));
    assert_eq!(&st[..8], h1);
    assert_ne!(h1, h2);
}

#[test]
fn aa_manifold_fnv_idempotent() {
    let a = b"test";
    assert_eq!(fnv1a_64(a), fnv1a_64(a));
}

#[test]
fn aa_manifold_voxelize_grid_size() {
    let t = ThermoGateState {
        density: 2400.0,
        free_energy: 0.0,
        hydration: 0.0,
        strength: 0.0,
        max_strength: 1.0,
    };
    let t2 = t;
    let s = GateSdf::from_thermo_pair(&t, &t2);
    let (_, v) = canonicalize_voxelize(&s, 2).expect("c");
    let n = 1u32 << 2;
    assert_eq!(v.len(), 8 * (n * n * n) as usize);
}
