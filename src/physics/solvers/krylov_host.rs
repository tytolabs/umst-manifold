// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Host-side `f32` GMRES (no restart) for matrix-free solves on packed `f32` vectors.
//!
//! Shared by THMC JFNK (`thmc_jfnk` shim under `solver-experimental`) and acoustics Newmark linear
//! solves. Prefer
//! [`gmres_f32_try`] with a fallible matvec in production paths.
//!
//! # Honest boundary (W29-076)
//!
//! Host GMRES contracts (`gmres_f32` / [`gmres_f32_try`]) are exercised by
//! `cargo test -p umst-manifold krylov_host`. Solves packed `f32` systems for research / shim
//! callers. Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`, not OP-5.

use crate::physics::PhysicsError;

/// W29 deepen cell — host Krylov GMRES honest fence bundle.
pub const W29_KRYLOV_HOST_DEEPEN_CELL: &str = "W29-076-KRYLOV_HOST";

/// Honest posture tag — host GMRES landed; fleet production wiring refused.
pub const KRYLOV_HOST_POSTURE_TAG: &str = "honest-host-gmres-f32-research-lane";

/// Honest physics posture — unit contracts pass; does not certify fleet physics GREEN.
pub const KRYLOV_HOST_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by host GMRES alone.
pub const KRYLOV_HOST_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const KRYLOV_HOST_MASTER: bool = false;

/// OP-5 ceremony pin — not claimed by this module.
pub const KRYLOV_HOST_OP5: bool = false;

/// Whether host GMRES (`gmres_f32` / [`gmres_f32_try`]) contracts are landed in this module.
pub const KRYLOV_HOST_GMRES_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const KRYLOV_HOST_HONEST_FENCE: &str =
    "host_gmres_f32_landed=true|gmres_f32_try_wired=true|matvec_error_propagates=true|production_wired=false|physics_green=false|master=false|op5=false";

/// Typed probe for host Krylov GMRES posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KrylovHostPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub gmres_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for host Krylov GMRES.
#[must_use]
pub fn krylov_host_honest_posture_bundle() -> KrylovHostPostureProbe {
    KrylovHostPostureProbe {
        physics_green: KRYLOV_HOST_PHYSICS_GREEN,
        production_wired: KRYLOV_HOST_PRODUCTION_WIRED,
        master: KRYLOV_HOST_MASTER,
        op5: KRYLOV_HOST_OP5,
        gmres_landed: KRYLOV_HOST_GMRES_LANDED,
        honest_fence: KRYLOV_HOST_HONEST_FENCE,
        posture_tag: KRYLOV_HOST_POSTURE_TAG,
        deepen_cell: W29_KRYLOV_HOST_DEEPEN_CELL,
    }
}

/// Host GMRES landed with production / master / OP-5 composition honestly open.
#[must_use]
pub fn krylov_host_posture_honest(probe: &KrylovHostPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && probe.gmres_landed
        && probe.deepen_cell == W29_KRYLOV_HOST_DEEPEN_CELL
        && probe.posture_tag == KRYLOV_HOST_POSTURE_TAG
        && probe.honest_fence.contains("host_gmres_f32_landed=true")
        && probe.honest_fence.contains("gmres_f32_try_wired=true")
        && probe.honest_fence.contains("matvec_error_propagates=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
}

const GMRES_CTX: &str = "gmres_f32_try";

/// GMRES without restart: solve \(A x = b\) with matrix-free \(A\) via fallible `matvec`.
///
/// Any `Err` from `matvec` aborts the solve and is returned — no panics on residual assembly failure.
pub fn gmres_f32_try<F>(
    mut matvec: F,
    b: &[f32],
    n: usize,
    max_iter: usize,
    rel_tol: f32,
) -> Result<Vec<f32>, PhysicsError>
where
    F: FnMut(&[f32]) -> Result<Vec<f32>, PhysicsError>,
{
    if n == 0 {
        return Err(PhysicsError::InvariantViolation { context: GMRES_CTX });
    }
    if b.len() != n {
        return Err(PhysicsError::BufferLength {
            context: GMRES_CTX,
            expected: n,
            got: b.len(),
        });
    }
    if max_iter == 0 {
        return Err(PhysicsError::InvariantViolation {
            context: "gmres_f32_try: max_iter=0",
        });
    }
    if rel_tol <= 0.0_f32 {
        return Err(PhysicsError::InvariantViolation {
            context: "gmres_f32_try: rel_tol must be positive",
        });
    }

    let beta: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if beta < 1e-30_f32 {
        return Ok(vec![0.0_f32; n]);
    }

    let m_max = max_iter.min(n);
    let mut v: Vec<Vec<f32>> = Vec::with_capacity(m_max + 1);
    let mut v0 = vec![0.0_f32; n];
    let inv_beta = 1.0_f32 / beta;
    for i in 0..n {
        v0[i] = b[i] * inv_beta;
    }
    v.push(v0);

    let mut h_cols: Vec<Vec<f32>> = Vec::with_capacity(m_max);
    let mut cs: Vec<f32> = Vec::with_capacity(m_max);
    let mut sn: Vec<f32> = Vec::with_capacity(m_max);
    let mut g: Vec<f32> = vec![0.0_f32; m_max + 2];
    g[0] = beta;

    let tol_abs = rel_tol * beta;

    for j in 0..m_max {
        let av = matvec(&v[j])?;
        if av.len() != n {
            return Err(PhysicsError::BufferLength {
                context: GMRES_CTX,
                expected: n,
                got: av.len(),
            });
        }

        let mut w = av;
        let mut h_col = vec![0.0_f32; j + 2];
        for i in 0..=j {
            let hij: f32 = v[i].iter().zip(w.iter()).map(|(vi, wi)| vi * wi).sum();
            h_col[i] = hij;
            for (wk, vik) in w.iter_mut().zip(v[i].iter()) {
                *wk -= hij * vik;
            }
        }
        let h_next: f32 = w.iter().map(|x| x * x).sum::<f32>().sqrt();
        h_col[j + 1] = h_next;

        for i in 0..j {
            let temp = cs[i] * h_col[i] + sn[i] * h_col[i + 1];
            h_col[i + 1] = -sn[i] * h_col[i] + cs[i] * h_col[i + 1];
            h_col[i] = temp;
        }

        if h_next < 1e-30_f32 {
            h_cols.push(h_col);
            let y = solve_upper_hessenberg_triangular(&h_cols, &g, j + 1)?;
            return reconstruct_solution_try(&v, &y, b, &mut matvec, beta, rel_tol);
        }

        let mut v_next = vec![0.0_f32; n];
        let inv = 1.0_f32 / h_next;
        for k in 0..n {
            v_next[k] = w[k] * inv;
        }
        v.push(v_next);

        let a = h_col[j];
        let bval = h_col[j + 1];
        let r = (a * a + bval * bval).sqrt();
        if r < 1e-30_f32 {
            h_cols.push(h_col);
            let y = solve_upper_hessenberg_triangular(&h_cols, &g, j + 1)?;
            return reconstruct_solution_try(&v, &y, b, &mut matvec, beta, rel_tol);
        }
        let c = a / r;
        let s = bval / r;
        cs.push(c);
        sn.push(s);
        h_col[j] = r;
        h_col[j + 1] = 0.0_f32;

        let gj = g[j];
        let gj1 = g[j + 1];
        g[j] = c * gj + s * gj1;
        g[j + 1] = -s * gj + c * gj1;

        h_cols.push(h_col);

        let resid_est = g[j + 1].abs();
        if resid_est <= tol_abs {
            let y = solve_upper_hessenberg_triangular(&h_cols, &g, j + 1)?;
            return reconstruct_solution_try(&v, &y, b, &mut matvec, beta, rel_tol);
        }
    }

    let y = solve_upper_hessenberg_triangular(&h_cols, &g, m_max)?;
    reconstruct_solution_try(&v, &y, b, &mut matvec, beta, rel_tol)
}

/// GMRES without restart: infallible `matvec` adapter over [`gmres_f32_try`].
pub fn gmres_f32<F>(
    matvec: F,
    b: &[f32],
    n: usize,
    max_iter: usize,
    rel_tol: f32,
) -> Result<Vec<f32>, PhysicsError>
where
    F: FnMut(&[f32]) -> Vec<f32>,
{
    let mut matvec = matvec;
    gmres_f32_try(|v| Ok(matvec(v)), b, n, max_iter, rel_tol)
}

fn solve_upper_hessenberg_triangular(
    h_cols: &[Vec<f32>],
    g: &[f32],
    dim: usize,
) -> Result<Vec<f32>, PhysicsError> {
    if dim == 0 {
        return Ok(vec![]);
    }
    let mut r: Vec<f32> = vec![0.0_f32; dim * dim];
    for j in 0..dim {
        for i in 0..=j {
            r[i * dim + j] = h_cols[j][i];
        }
    }
    let mut rhs: Vec<f32> = g[..dim].to_vec();
    for i in (0..dim).rev() {
        let mut sum = rhs[i];
        for j in (i + 1)..dim {
            sum -= r[i * dim + j] * rhs[j];
        }
        let diag = r[i * dim + i];
        if diag.abs() < 1e-30_f32 {
            return Err(PhysicsError::KrylovDiverged {
                context: "gmres_f32_try: hessenberg triangular solve singular",
            });
        }
        rhs[i] = sum / diag;
    }
    Ok(rhs)
}

fn reconstruct_solution_try<F>(
    v: &[Vec<f32>],
    y: &[f32],
    b: &[f32],
    matvec: &mut F,
    beta: f32,
    rel_tol: f32,
) -> Result<Vec<f32>, PhysicsError>
where
    F: FnMut(&[f32]) -> Result<Vec<f32>, PhysicsError>,
{
    let n = b.len();
    let mut x = vec![0.0_f32; n];
    for (j, yj) in y.iter().enumerate() {
        for (xi, vji) in x.iter_mut().zip(v[j].iter()) {
            *xi += yj * vji;
        }
    }

    let ax = matvec(&x)?;
    if ax.len() != n {
        return Err(PhysicsError::BufferLength {
            context: GMRES_CTX,
            expected: n,
            got: ax.len(),
        });
    }
    let res: f32 = b
        .iter()
        .zip(ax.iter())
        .map(|(bi, axi)| {
            let d = bi - axi;
            d * d
        })
        .sum::<f32>()
        .sqrt();
    let tol = (rel_tol * 50.0_f32).max(1e-5_f32) * beta.max(1e-30_f32);
    if res > tol {
        return Err(PhysicsError::KrylovDiverged {
            context: "gmres_f32_try: final residual exceeds tolerance",
        });
    }
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::{
        gmres_f32, gmres_f32_try, krylov_host_honest_posture_bundle, krylov_host_posture_honest,
        KRYLOV_HOST_GMRES_LANDED, KRYLOV_HOST_HONEST_FENCE, KRYLOV_HOST_MASTER, KRYLOV_HOST_OP5,
        KRYLOV_HOST_PHYSICS_GREEN, KRYLOV_HOST_PRODUCTION_WIRED, W29_KRYLOV_HOST_DEEPEN_CELL,
    };
    use crate::physics::PhysicsError;

    #[test]
    fn krylov_host_honest_posture_refuses_green_production_master_op5() {
        let probe = krylov_host_honest_posture_bundle();
        assert!(krylov_host_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert_eq!(probe.deepen_cell, W29_KRYLOV_HOST_DEEPEN_CELL);
        assert!(KRYLOV_HOST_GMRES_LANDED);
        assert!(!KRYLOV_HOST_PHYSICS_GREEN);
        assert!(!KRYLOV_HOST_PRODUCTION_WIRED);
        assert!(!KRYLOV_HOST_MASTER);
        assert!(!KRYLOV_HOST_OP5);
        assert!(KRYLOV_HOST_HONEST_FENCE.contains("production_wired=false"));
        assert!(KRYLOV_HOST_HONEST_FENCE.contains("physics_green=false"));
        assert!(KRYLOV_HOST_HONEST_FENCE.contains("master=false"));
        assert!(KRYLOV_HOST_HONEST_FENCE.contains("op5=false"));
    }

    #[test]
    fn gmres_identity() {
        let n = 4usize;
        let b = vec![1.0_f32, 2.0_f32, -0.5_f32, 0.25_f32];
        let matvec = |v: &[f32]| v.to_vec();
        let x = gmres_f32(matvec, &b, n, n, 1e-5_f32).expect(
            "gmres_f32 on identity matvec n=4 must converge for Krylov host smoke witness (FP §6 Track G krylov)",
        );
        for i in 0..n {
            assert!(
                (x[i] - b[i]).abs() < 1e-4_f32,
                "i={i} x={} b={}",
                x[i],
                b[i]
            );
        }
    }

    #[test]
    fn gmres_zero_rhs_returns_zero() {
        let n = 3usize;
        let b = vec![0.0_f32; n];
        let matvec = |v: &[f32]| v.to_vec();
        let x = gmres_f32(matvec, &b, n, n, 1e-6_f32)
            .expect("zero RHS must short-circuit to zero without matvec");
        assert_eq!(x, vec![0.0_f32; n]);
    }

    #[test]
    fn gmres_rejects_n_zero_and_buffer_mismatch() {
        let err_n = gmres_f32(|v: &[f32]| v.to_vec(), &[], 0, 1, 1e-5_f32)
            .expect_err("n=0 must InvariantViolation");
        match err_n {
            PhysicsError::InvariantViolation { context } => {
                assert_eq!(context, "gmres_f32_try");
            }
            other => panic!("expected InvariantViolation, got {other:?}"),
        }

        let err_len = gmres_f32(|v: &[f32]| v.to_vec(), &[1.0_f32, 2.0_f32], 3, 3, 1e-5_f32)
            .expect_err("b.len()!=n must BufferLength");
        match err_len {
            PhysicsError::BufferLength {
                context,
                expected,
                got,
            } => {
                assert_eq!(context, "gmres_f32_try");
                assert_eq!(expected, 3);
                assert_eq!(got, 2);
            }
            other => panic!("expected BufferLength, got {other:?}"),
        }
    }

    #[test]
    fn gmres_rejects_max_iter_zero_and_nonpositive_tol() {
        let n = 2usize;
        let b = vec![1.0_f32, 0.0_f32];
        let err_iter = gmres_f32(|v: &[f32]| v.to_vec(), &b, n, 0, 1e-5_f32)
            .expect_err("max_iter=0 must InvariantViolation");
        assert!(
            matches!(
                err_iter,
                PhysicsError::InvariantViolation { context } if context.contains("max_iter=0")
            ),
            "{err_iter:?}"
        );
        let err_tol = gmres_f32(|v: &[f32]| v.to_vec(), &b, n, n, 0.0_f32)
            .expect_err("rel_tol<=0 must InvariantViolation");
        assert!(
            matches!(
                err_tol,
                PhysicsError::InvariantViolation { context } if context.contains("rel_tol")
            ),
            "{err_tol:?}"
        );
    }

    #[test]
    fn gmres_small_dense() {
        let a: [f32; 25] = [
            4.0, 1.0, 0.0, 0.0, 0.0, //
            1.0, 4.0, 1.0, 0.0, 0.0, //
            0.0, 1.0, 4.0, 1.0, 0.0, //
            0.0, 0.0, 1.0, 4.0, 1.0, //
            0.0, 0.0, 0.0, 1.0, 4.0,
        ];
        let n = 5usize;
        let b = vec![1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32];
        let matvec = |v: &[f32]| -> Vec<f32> {
            let mut out = vec![0.0_f32; n];
            for i in 0..n {
                let mut s = 0.0_f32;
                for j in 0..n {
                    s += a[i * n + j] * v[j];
                }
                out[i] = s;
            }
            out
        };
        let x = gmres_f32(matvec, &b, n, n + 5, 1e-4_f32).expect(
            "gmres_f32 on 5×5 tridiagonal SPD matvec must converge for Krylov host dense smoke witness (FP §6 Track G krylov)",
        );
        let ax = matvec(&x);
        let res: f32 = b
            .iter()
            .zip(ax.iter())
            .map(|(bi, axi)| {
                let d = bi - axi;
                d * d
            })
            .sum::<f32>()
            .sqrt();
        assert!(res < 1e-3_f32, "residual {res}");
    }

    #[test]
    fn gmres_try_propagates_matvec_error() {
        let n = 2usize;
        let b = vec![1.0_f32, 0.0_f32];
        let mut calls = 0usize;
        let matvec = |_v: &[f32]| -> Result<Vec<f32>, PhysicsError> {
            calls += 1;
            Err(PhysicsError::Domain {
                detail: "injected".into(),
            })
        };
        let err = gmres_f32_try(matvec, &b, n, n, 1e-5_f32).expect_err(
            "gmres_f32_try must propagate injected matvec PhysicsError without retry (FP §6 Track G krylov)",
        );
        assert!(err.to_string().contains("injected"), "{err}");
        assert_eq!(calls, 1, "should not retry after matvec Err");
    }

    #[test]
    fn gmres_try_rejects_matvec_wrong_length() {
        let n = 2usize;
        let b = vec![1.0_f32, 0.0_f32];
        let matvec = |_v: &[f32]| -> Result<Vec<f32>, PhysicsError> { Ok(vec![1.0_f32]) };
        let err = gmres_f32_try(matvec, &b, n, n, 1e-5_f32)
            .expect_err("matvec wrong length must BufferLength");
        match err {
            PhysicsError::BufferLength {
                context,
                expected,
                got,
            } => {
                assert_eq!(context, "gmres_f32_try");
                assert_eq!(expected, 2);
                assert_eq!(got, 1);
            }
            other => panic!("expected BufferLength, got {other:?}"),
        }
    }
}
