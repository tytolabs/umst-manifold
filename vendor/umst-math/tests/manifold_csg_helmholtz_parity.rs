//! 1D Helmholtz SDF gradient parity with `qHydration` (SDFGate.hs, UMST.hs).
#![cfg(test)]

use umst_math::manifold::csg;

#[test]
fn aa_manifold_helmholtz_grad_matches_numeric() {
    const Q: f64 = 450.0; // qHydration (Haskell)
                          // Step large enough to beat f64 cancellation (ε-bisim, not ulp-strict)
    const EPS: f64 = 1e-4;
    let a = 0.37f64;
    let psi0 = csg::umst_helmholtz_sdf(a);
    let psi1 = csg::umst_helmholtz_sdf(a + EPS);
    let d_num = (psi1 - psi0) / EPS;
    let d_theory = csg::helmholtz_gradient(Q);
    assert!(
        (d_num - d_theory).abs() < 1e-3,
        "I5 numeric {d_num} vs theory {d_theory}"
    );
}

#[test]
fn aa_manifold_helmholtz_linearity() {
    for a in [0.0_f64, 0.1, 0.99, 1.0] {
        let v = csg::umst_helmholtz_sdf(a);
        let w = csg::helmholtz_sdf_1d(a, 450.0);
        assert!((v - w).abs() < 1e-9);
    }
}
