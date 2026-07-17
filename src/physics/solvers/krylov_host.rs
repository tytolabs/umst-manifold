// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host-side `f32` GMRES (no restart) for matrix-free solves on packed `f32` vectors.
//!
//! Shared by THMC JFNK (`thmc_jfnk` shim under `solver-experimental`) and acoustics Newmark linear
//! solves. Prefer
//! [`gmres_f32_try`] with a fallible matvec in production paths.

use crate::physics::PhysicsError;

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
    use super::{gmres_f32, gmres_f32_try};
    use crate::physics::PhysicsError;

    #[test]
    fn gmres_identity() {
        let n = 4usize;
        let b = vec![1.0_f32, 2.0_f32, -0.5_f32, 0.25_f32];
        let matvec = |v: &[f32]| v.to_vec();
        let x = gmres_f32(matvec, &b, n, n, 1e-5_f32).expect("identity GMRES");
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
        let x = gmres_f32(matvec, &b, n, n + 5, 1e-4_f32).expect("dense GMRES");
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
        let err = gmres_f32_try(matvec, &b, n, n, 1e-5_f32).expect_err("should fail");
        assert!(err.to_string().contains("injected"), "{err}");
        assert_eq!(calls, 1, "should not retry after matvec Err");
    }
}
