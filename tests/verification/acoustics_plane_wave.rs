// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::needless_range_loop)]

//! **1-D periodic bar** (`ρ u_tt = E u_xx`): implicit Newmark via [`AcousticNewmarkBar1dPeriodic`]
//! — return map after one lumped eigenperiod \(T=2\pi/\Omega\) (with \(m=\rho\Delta x\)), \(h\)-refinement band,
//! dense Newmark-acceleration checks, and an undamped energy-drift harness at **n=128**.
//!
//! **Return-map timing:** [`semi_discrete_omega`] supplies **Ω** with \(\Omega^2=\lambda_K/m\) on the periodic
//! stencil (not the stencil dispersion \(\omega_{\mathrm{disp}}=(2c/\Delta x)|\sin(k\Delta x/2)|\) alone — that
//! omits the lumped \(1/\Delta x\) from \(m=\rho\Delta x\) and mis-times \(T\) by \(\mathcal O(1/\sqrt{\Delta x})\)).
//! **[`AcousticWaveSolver`](umst_manifold::physics::solvers::AcousticWaveSolver)** stays graph-free nodal
//! contraction (Track D in `composer_prompts/v0.4_solver_completion_no_namesakes.md`). See
//! **`docs/Solver-Status.md`** (Acoustics).

use std::f32::consts::PI;

use umst_manifold::physics::solvers::{AcousticNewmarkBar1dPeriodic, AcousticNewmarkBar1dWork};

fn init_plane_wave(
    bar: &AcousticNewmarkBar1dPeriodic,
    k: f32,
    u: &mut [f32],
    v: &mut [f32],
    a: &mut [f32],
) {
    let dx = bar.dx();
    let m = bar.density * dx;
    for i in 0..bar.n {
        let x = i as f32 * dx;
        u[i] = (k * x).sin();
        v[i] = 0.0_f32;
    }
    let mut ku = vec![0.0_f32; bar.n];
    apply_k_periodic_1d_local(u, bar.youngs_modulus, dx, bar.n, &mut ku);
    for i in 0..bar.n {
        a[i] = -ku[i] / m;
    }
}

fn apply_k_periodic_1d_local(u: &[f32], e: f32, dx: f32, n: usize, out: &mut [f32]) {
    let c = e / (dx * dx);
    for i in 0..n {
        let im = if i == 0 { n - 1 } else { i - 1 };
        let ip = if i + 1 == n { 0 } else { i + 1 };
        out[i] = c * (2.0_f32 * u[i] - u[im] - u[ip]);
    }
}

/// Finite-difference dispersion relation for the **central-difference** Laplacian on a uniform grid:
/// `ω_disp = (2c/Δx) |sin(k Δx / 2)|` (same algebraic factor as [`semi_discrete_omega`] before the lumped
/// `√Δx` correction). Used only where we intentionally compare against spatial-stencil targets — **not**
/// for [`semi_discrete_omega`] return-map timing (`m = ρ Δx` ⇒ eigenfrequency `Ω = ω_disp / √Δx`).
fn dispersion_omega_fd(bar: &AcousticNewmarkBar1dPeriodic, k: f32) -> f32 {
    let dx = bar.dx();
    let c = bar.wave_speed();
    (2.0_f32 * c / dx) * (k * dx * 0.5_f32).sin().abs()
}

/// Angular frequency Ω of the **lumped** semi-discrete mode `sin(k x)` on the periodic
/// central-difference stencil with `m = ρ Δx`:
///
/// `λ_K = (E/Δx²)·4 sin²(k Δx / 2)` for `Ku`, so `Ω² = λ_K/m = ((2c/Δx) sin(k Δx / 2))² / Δx`.
///
/// The factor `1/Δx` (vs continuum dispersion `ω_disp = (2c/Δx)|sin(k Δx / 2)|`) is required so
/// `ü + Ω² u` matches `m ü + Ku = 0`. Using `ω_disp` alone for `T = 2π/ω` mis-times the return map
/// by `𝒪(1/√Δx)` on a fixed-length bar — see [`Solver-Status`](../../docs/Solver-Status.md) acoustics lane.
fn semi_discrete_omega(bar: &AcousticNewmarkBar1dPeriodic, k: f32) -> f32 {
    let dx = bar.dx();
    dispersion_omega_fd(bar, k) / dx.sqrt()
}

fn l2_error_vs_analytic(
    bar: &AcousticNewmarkBar1dPeriodic,
    u: &[f32],
    t: f32,
    k: f32,
    omega: f32,
) -> f32 {
    let dx = bar.dx();
    let mut err = 0.0_f32;
    let mut norm = 0.0_f32;
    let ct = (omega * t).cos();
    for i in 0..bar.n {
        let x = i as f32 * dx;
        let ua = (k * x).sin() * ct;
        let d = u[i] - ua;
        err += d * d;
        norm += ua * ua;
    }
    (err * dx).sqrt() / (norm * dx).sqrt().max(1e-12_f32)
}

/// Relative discrete L² error `‖u − u₀‖ / ‖u₀‖` (same mesh, periodic inner product `Σ · Δx`).
fn rel_l2_to_reference(u: &[f32], u_ref: &[f32], dx: f32) -> f32 {
    debug_assert_eq!(u.len(), u_ref.len());
    let mut err = 0.0_f32;
    let mut norm = 0.0_f32;
    for i in 0..u.len() {
        let d = u[i] - u_ref[i];
        err += d * d;
        norm += u_ref[i] * u_ref[i];
    }
    (err * dx).sqrt() / (norm * dx).sqrt().max(1e-12_f32)
}

/// Integrate to `t_end` using exactly `n_steps` equal substeps (`Δt = t_end / n_steps`).
/// This avoids a terminal partial step with a different `Δt` (and recomputed Cholesky factor).
fn run_fixed_substeps(
    bar: &AcousticNewmarkBar1dPeriodic,
    ws: &mut AcousticNewmarkBar1dWork,
    n_steps: usize,
    t_end: f32,
    u: &mut [f32],
    v: &mut [f32],
    a: &mut [f32],
) {
    assert!(n_steps > 0);
    let dt = t_end / n_steps as f32;
    for _ in 0..n_steps {
        bar.step(ws, dt, u, v, a);
    }
}

/// Reference acceleration solve for one Newmark step (dense Gaussian elimination in **f64**).
fn newmark_acceleration_dense_reference(
    n: usize,
    dt: f32,
    beta: f32,
    e: f32,
    rho: f32,
    dx: f32,
    u_tilde: &[f32],
) -> Vec<f32> {
    let m_node = rho as f64 * dx as f64;
    let alpha = beta as f64 * (dt as f64).powi(2) * e as f64 / (dx as f64).powi(2);
    let mut s = vec![0.0_f64; n * n];
    for i in 0..n {
        s[i * n + i] = m_node + 2.0_f64 * alpha;
        let ip = (i + 1) % n;
        let im = (i + n - 1) % n;
        s[i * n + ip] -= alpha;
        s[i * n + im] -= alpha;
    }
    let mut rhs = vec![0.0_f64; n];
    let mut ku = vec![0.0_f32; n];
    apply_k_periodic_1d_local(u_tilde, e, dx, n, &mut ku);
    for i in 0..n {
        rhs[i] = -ku[i] as f64;
    }
    dense_solve_row_major_f64(&mut s, &mut rhs, n);
    rhs.into_iter().map(|z| z as f32).collect()
}

fn dense_solve_row_major_f64(a: &mut [f64], b: &mut [f64], n: usize) {
    for col in 0..n {
        let mut piv = col;
        let mut best = (a[col * n + col]).abs();
        for r in (col + 1)..n {
            let v = (a[r * n + col]).abs();
            if v > best {
                best = v;
                piv = r;
            }
        }
        if piv != col {
            for j in 0..n {
                a.swap(col * n + j, piv * n + j);
            }
            b.swap(col, piv);
        }
        let diag = a[col * n + col];
        assert!(diag.abs() > 1e-18_f64, "singular pivot");
        let inv = 1.0_f64 / diag;
        for j in col..n {
            a[col * n + j] *= inv;
        }
        b[col] *= inv;
        for r in 0..n {
            if r == col {
                continue;
            }
            let f = a[r * n + col];
            if f == 0.0_f64 {
                continue;
            }
            for j in col..n {
                a[r * n + j] -= f * a[col * n + j];
            }
            b[r] -= f * b[col];
        }
    }
}

#[test]
fn newmark_acceleration_matches_dense_reference_n8() {
    let n = 8_usize;
    let l = 1.0_f32;
    let e = 1.0_f32;
    let rho = 1.0_f32;
    let bar = AcousticNewmarkBar1dPeriodic {
        n,
        length: l,
        youngs_modulus: e,
        density: rho,
        newmark_beta: 0.25_f32,
        newmark_gamma: 0.5_f32,
    };
    let mut ws = bar.workspace();
    let dt = 0.01_f32;
    let beta = bar.newmark_beta;
    let _gamma = bar.newmark_gamma;
    let dx = bar.dx();
    let dt2 = dt * dt;
    let half_minus_beta = 0.5_f32 - beta;

    let mut u: Vec<f32> = (0..n).map(|i| (i as f32 * 0.23).sin()).collect();
    let mut v: Vec<f32> = (0..n).map(|i| (i as f32 * 0.11).cos() * 0.1).collect();
    let mut a: Vec<f32> = (0..n).map(|i| (i as f32) * 0.02).collect();

    let acc_n = a.clone();
    let mut u_tilde = vec![0.0_f32; n];
    for i in 0..n {
        u_tilde[i] = u[i] + dt * v[i] + dt2 * half_minus_beta * acc_n[i];
    }
    let a_dense = newmark_acceleration_dense_reference(n, dt, beta, e, rho, dx, &u_tilde);

    bar.step(&mut ws, dt, &mut u, &mut v, &mut a);

    let tol = 5e-4_f32;
    for i in 0..n {
        assert!(
            (a[i] - a_dense[i]).abs() < tol,
            "accel mismatch at {i}: bar={} dense={}",
            a[i],
            a_dense[i]
        );
    }
}

#[test]
fn newmark_acceleration_matches_dense_reference_n128() {
    let n = 128_usize;
    let l = 1.0_f32;
    let e = 1.0_f32;
    let rho = 1.0_f32;
    let bar = AcousticNewmarkBar1dPeriodic {
        n,
        length: l,
        youngs_modulus: e,
        density: rho,
        newmark_beta: 0.25_f32,
        newmark_gamma: 0.5_f32,
    };
    let mut ws = bar.workspace();
    let dt = bar.dx() / 1000.0_f32;
    let beta = bar.newmark_beta;
    let dx = bar.dx();
    let dt2 = dt * dt;
    let half_minus_beta = 0.5_f32 - beta;

    let mut u: Vec<f32> = (0..n).map(|i| (i as f32 * 0.23).sin()).collect();
    let mut v: Vec<f32> = (0..n).map(|i| (i as f32 * 0.11).cos() * 0.1).collect();
    let mut a: Vec<f32> = (0..n).map(|i| (i as f32) * 0.02).collect();

    let acc_n = a.clone();
    let mut u_tilde = vec![0.0_f32; n];
    for i in 0..n {
        u_tilde[i] = u[i] + dt * v[i] + dt2 * half_minus_beta * acc_n[i];
    }
    let a_dense = newmark_acceleration_dense_reference(n, dt, beta, e, rho, dx, &u_tilde);

    bar.step(&mut ws, dt, &mut u, &mut v, &mut a);

    let tol = 1e-4_f32;
    let mut max_d = 0.0_f32;
    for i in 0..n {
        max_d = max_d.max((a[i] - a_dense[i]).abs());
    }
    assert!(max_d < tol, "max accel mismatch n=128: {max_d}");
}

/// Relative discrete \(L^2\) to `u₀` after one lumped eigenperiod \(T=2\pi/\Omega\) for the
/// fundamental mode `sin(kx)`, \(k=2\pi/L\), using the same CFL-scaled substep recipe as the n=100
/// CI gate (`dt_cfl = 0.01·dx/c`, `n_steps = ceil(T/dt_cfl).max(512)`).
fn plane_wave_return_map_rel_l2_to_u0_after_one_period(n: usize) -> f32 {
    let l = 1.0_f32;
    let e = 1.0_f32;
    let rho = 1.0_f32;
    let k = 2.0_f32 * PI / l;
    let bar = AcousticNewmarkBar1dPeriodic {
        n,
        length: l,
        youngs_modulus: e,
        density: rho,
        newmark_beta: 0.25_f32,
        newmark_gamma: 0.5_f32,
    };
    let omega = semi_discrete_omega(&bar, k);
    let t_period = 2.0_f32 * PI / omega;
    let mut ws = bar.workspace();
    let mut u = vec![0.0_f32; bar.n];
    let mut v = vec![0.0_f32; bar.n];
    let mut a = vec![0.0_f32; bar.n];
    init_plane_wave(&bar, k, &mut u, &mut v, &mut a);
    let u0 = u.clone();

    let dx = bar.dx();
    // Implicit Newmark is stable for large dt, but accuracy needs ~mesh-aware stepping: hold
    // `dt · c / dx` (CFL-like) similar across resolutions when comparing return maps.
    let dt_cfl = 0.01_f32 * dx / (e / rho).sqrt();
    let n_steps = (t_period / dt_cfl).ceil().max(512.0_f32) as usize;
    run_fixed_substeps(&bar, &mut ws, n_steps, t_period, &mut u, &mut v, &mut a);
    assert!(
        u.iter()
            .chain(v.iter())
            .chain(a.iter())
            .all(|x| x.is_finite()),
        "non-finite state after plane-wave integration (n={n})"
    );

    rel_l2_to_reference(&u, &u0, dx)
}

/// Return map after one lumped period for `sin(kx)` on a **100-node** periodic bar.
#[test]
fn plane_wave_return_map_n100_l2_within_two_percent() {
    let rel = plane_wave_return_map_rel_l2_to_u0_after_one_period(100);
    assert!(
        rel < 0.02_f32,
        "expected return-map L2 relative error < 2% after one Ω period; got {rel}"
    );
}

/// Same return-map recipe as [`plane_wave_return_map_n100_l2_within_two_percent`] at **n=64**.
#[test]
fn plane_wave_return_map_n64_l2_within_two_percent() {
    let rel = plane_wave_return_map_rel_l2_to_u0_after_one_period(64);
    assert!(
        rel < 0.02_f32,
        "expected return-map L2 relative error < 2% after one Ω period at n=64; got {rel}"
    );
}

/// Same return-map recipe at **n=128** (brief mesh count).
#[test]
fn plane_wave_return_map_n128_l2_within_two_percent() {
    let rel = plane_wave_return_map_rel_l2_to_u0_after_one_period(128);
    assert!(
        rel < 0.02_f32,
        "expected return-map L2 relative error < 2% after one Ω period at n=128; got {rel}"
    );
}
#[test]
fn plane_wave_h_refinement_second_order_band() {
    let l = 1.0_f32;
    let e = 1.0_f32;
    let rho = 1.0_f32;
    let c = (e / rho).sqrt();
    let k = 2.0_f32 * PI / l;

    let run_n = |n: usize| -> f32 {
        let bar = AcousticNewmarkBar1dPeriodic {
            n,
            length: l,
            youngs_modulus: e,
            density: rho,
            newmark_beta: 0.25_f32,
            newmark_gamma: 0.5_f32,
        };
        // Semi-discrete spatial dispersion `ω_disp` for the stencil (not lumped Ω): snapshot at 0.1× the
        // corresponding temporal period so phase tracks FD dispersion while spatial truncation error
        // still scales like `𝒪(h²)` between `n=32` and `n=64`.
        let omega_disp = dispersion_omega_fd(&bar, k);
        let t_snap = 0.1_f32 * (2.0_f32 * PI / omega_disp);
        let mut ws = bar.workspace();
        let mut u = vec![0.0_f32; bar.n];
        let mut v = vec![0.0_f32; bar.n];
        let mut a = vec![0.0_f32; bar.n];
        init_plane_wave(&bar, k, &mut u, &mut v, &mut a);
        let dx = bar.dx();
        let dt_cfl = 0.01_f32 * dx / c;
        let n_steps = (t_snap / dt_cfl).ceil().max(512.0_f32) as usize;
        run_fixed_substeps(&bar, &mut ws, n_steps, t_snap, &mut u, &mut v, &mut a);
        l2_error_vs_analytic(&bar, &u, t_snap, k, omega_disp)
    };

    let e_coarse = run_n(32);
    let e_fine = run_n(64);
    let ratio = e_coarse / e_fine.max(1e-12_f32);
    assert!(
        (2.5_f32..=5.5_f32).contains(&ratio),
        "expected O(h²) error ratio ~4 between n=32 and n=64; got {ratio} (e32={e_coarse}, e64={e_fine})"
    );
}

/// Undamped discrete energy drift (n=128) — companion to **`plane_wave_return_map_n128_l2_within_two_percent`**; see module rustdoc and **`docs/Solver-Status.md`** (Acoustics).
#[test]
fn undamped_energy_drift_under_half_percent_over_1000_steps() {
    let l = 1.0_f32;
    let e = 1.0_f32;
    let rho = 1.0_f32;
    let c = (e / rho).sqrt();

    let bar = AcousticNewmarkBar1dPeriodic {
        n: 128,
        length: l,
        youngs_modulus: e,
        density: rho,
        newmark_beta: 0.25_f32,
        newmark_gamma: 0.5_f32,
    };
    let mut ws = bar.workspace();
    let mut u = vec![0.0_f32; bar.n];
    let mut v = vec![0.0_f32; bar.n];
    let mut a = vec![0.0_f32; bar.n];
    let k = 2.0_f32 * PI / l;
    init_plane_wave(&bar, k, &mut u, &mut v, &mut a);

    let dx = bar.dx();
    let dt = 0.5_f32 * dx / c;

    let e0 = bar.mechanical_energy(&u, &v);
    let mut emax = e0;
    let mut emin = e0;
    for _ in 0..1000 {
        bar.step(&mut ws, dt, &mut u, &mut v, &mut a);
        let en = bar.mechanical_energy(&u, &v);
        emax = emax.max(en);
        emin = emin.min(en);
    }
    let denom = e0.abs().max(1e-12_f32);
    let drift = (emax - emin) / denom;
    assert!(
        drift < 0.005_f32,
        "expected peak-to-peak energy drift < 0.5% over 1000 steps; got {drift} (e0={e0})"
    );
}
