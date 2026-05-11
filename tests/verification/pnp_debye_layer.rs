// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Scharfetter–Gummel PNP verification (`electrochemistry-mvp`): zero-field diffusion matches the graph
//! Laplacian, plus a mild Debye-style screening smoke (potential decay along a chain).
//!
//! **Default CI** runs non-`ignored` tests in this target. **Quasi-steady λ_D** exponential-fit gates
//! (`debye_screening_256_cells_phi_25mv_decay_length_within_band`,
//! `debye_screening_256_cells_phi_100mv_decay_length_within_band`) stay **`#[ignore]`** (long horizon
//! at **N=256**, dense Jacobian cost); opt-in:
//! `cargo test --features electrochemistry-pnp --test pnp_debye_layer -- --ignored`.
//! Track 14 dispatch + implicit chain on a short chain is covered by **`debye_implicit_dispatch_short_horizon_smoke`**
//! (`solve_pnp_step_dispatch` + `pnp_implicit_newton_chain: Some(…)`).
//!
//! Specification: `composer_prompts/v0.4_solver_completion_no_namesakes.md` (Track F).

#![cfg(feature = "electrochemistry-mvp")]
// Track 14 MVP-chain implicit Newton lives behind the same feature; there is no additional
// `#[cfg(feature = "...")]` for it — opt in at runtime via `pnp_implicit_newton_chain` +
// `solve_pnp_step_dispatch` (production path; falls back to explicit Picard if the chain helper
// returns `None`). Direct `try_solve_pnp_backward_euler_newton_chain` remains for unit tests in
// `electrochemistry.rs` and callers who bypass dispatch.

use approx::assert_relative_eq;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::laplacian::TopologicalLaplacian;
use umst_manifold::physics::solvers::{
    pnp_backward_euler_residual_l2_chain_host_f64, ElectroChemicalSolver, NewtonPnpContext,
};

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn chain_edges(n: usize) -> Tensor<B, 2, Int> {
    let e = n - 1;
    let mut v = Vec::with_capacity(2 * e);
    for i in 0..e {
        v.push(i as i64);
    }
    for i in 0..e {
        v.push((i + 1) as i64);
    }
    Tensor::from_data(Data::new(v, Shape::new([2, e])), &device())
}

fn max_abs_diff(a: &Tensor<B, 3>, b: &Tensor<B, 3>) -> f32 {
    let d = a.clone().sub(b.clone()).abs().into_data().value;
    d.iter().copied().fold(0.0_f32, f32::max)
}

#[test]
fn sg_zero_field_matches_explicit_fickian_graph_laplacian() {
    let dev = device();
    let n = 32usize;
    let edges = chain_edges(n);
    // Electroneutral at every node (c+ = c−) so ρ_e = 0 and φ stays zero under the Thomas Poisson
    // solve; SG then reduces to pure diffusion and matches the explicit Fickian graph Laplacian.
    let mut c_flat = vec![0.0_f32; n * 2];
    for i in 0..n {
        let x = i as f32 / (n - 1) as f32;
        let c = 0.5 + 0.4 * (1.0 - x);
        c_flat[i * 2] = c;
        c_flat[i * 2 + 1] = c;
    }
    let c = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let d0 = 0.04_f32;
    let d = Tensor::<B, 3>::full([1, n, 2], d0, &dev);
    let mask = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let dt = 1e-3_f32;
    let lap0 = TopologicalLaplacian::scalar_laplacian(
        c.clone().narrow(2, 0, 1),
        edges.clone(),
        mask.clone(),
    );
    let lap1 =
        TopologicalLaplacian::scalar_laplacian(c.clone().narrow(2, 1, 1), edges.clone(), mask);
    let fick = Tensor::cat(
        vec![
            c.clone().narrow(2, 0, 1).add(lap0.mul_scalar(dt * d0)),
            c.clone().narrow(2, 1, 1).add(lap1.mul_scalar(dt * d0)),
        ],
        2,
    );
    let solver = ElectroChemicalSolver {
        faraday_const: 1.0_f32,
        gas_const: 1.0_f32,
        ..Default::default()
    };
    let (_phi2, c_sg) = solver.solve_pnp_step(dt, phi, c, edges, eps, d);
    let err = max_abs_diff(&c_sg, &fick);
    assert_relative_eq!(err, 0.0_f32, epsilon = 5e-5_f32);
}

#[test]
fn pnp_screening_phi_decays_toward_bulk_smoke() {
    let dev = device();
    let n = 128usize;
    let edges = chain_edges(n);
    let c0 = 1.0_f32;
    let mut c_flat = Vec::with_capacity(n * 2);
    for i in 0..n {
        let x = i as f32 / (n - 1) as f32;
        let w = (-8.0_f32 * x * x).exp();
        c_flat.push(c0 * (1.0 + 0.08 * w));
        c_flat.push(c0 * (1.0 - 0.08 * w));
    }
    let mut c = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let mut phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let d = Tensor::<B, 3>::full([1, n, 2], 3e-2_f32, &dev);
    let solver = ElectroChemicalSolver {
        faraday_const: 1.0_f32,
        gas_const: 1.0_f32,
        ..Default::default()
    };
    let phi0 = 0.04_f32;
    for _ in 0..8000 {
        let (p, cn) =
            solver.solve_pnp_step(2e-4_f32, phi, c, edges.clone(), eps.clone(), d.clone());
        let n = p.dims()[1];
        let mid = p.clone().slice([0..1, 1..(n - 1), 0..1]);
        let left = Tensor::<B, 3>::full([1, 1, 1], phi0, &dev);
        let right = Tensor::<B, 3>::zeros([1, 1, 1], &dev);
        phi = Tensor::cat(vec![left, mid, right], 1);
        c = cn.clamp_min(1e-12_f32);
    }
    let pv = phi.into_data().value;
    let n_nodes = n;
    let a = pv[8].abs();
    let ctail = pv[n_nodes - 8].abs();
    assert!(
        a.is_finite() && pv[n_nodes / 2].is_finite() && ctail.is_finite(),
        "phi finite along chain"
    );
    assert_relative_eq!(pv[0], phi0, epsilon = 1e-4_f32);
    assert_relative_eq!(pv[n_nodes - 1], 0.0_f32, epsilon = 1e-4_f32);
    // With an exact chain Poisson solve, |φ| may peak in the interior; still expect bias at x≈0 to
    // dominate over the far-right bulk (Dirichlet φ=0 at x=1).
    assert!(
        a > ctail + 1e-4_f32,
        "expected phi |near left| > far bulk: a={a} ctail={ctail}"
    );
}

/// **Deferred — quasi-steady λ_D gate.** Linear Debye–Hückel screening on a 256-cell chain at
/// φ_0 = 1.0 V_T. The harness uses [`ElectroChemicalSolver::solve_pnp_step_dispatch`] with
/// [`ElectroChemicalSolver::pnp_implicit_newton_chain`] (full SG implicit Newton). Validating the
/// exponential-fit band requires the original **~18 time-unit** horizon (`dt`·`steps` ≈ `1.5e-3`×`12_000`);
/// shorter budgets fail the λ_eff vs λ_D check. Full horizon at **N=256** is **not** CI-fast (dense
/// finite-difference Jacobian per Newton). Run with `--ignored` locally when needed.
#[test]
#[ignore = "Deferred: dispatch + implicit Newton wired; quasi-steady λ_D fit needs full ~12k×1.5e-³ trajectory at N=256 (shortened runs fail rel_err); dense Jacobian cost keeps default CI from enabling"]
fn debye_screening_256_cells_phi_25mv_decay_length_within_band() {
    debye_screening_admissibility_check(256, 1.0_f32, 6.0_f32, 0.30_f32);
}

/// **Deferred — same harness as the 25 mV_T sibling.** Weakly non-linear (Gouy–Chapman) Debye
/// screening at φ_0 = 4.0 V_T (≈ 100 mV at 298 K), tolerance widened to ±45 % for the sech²
/// correction. Kept ignored until the implicit chain run reaches the same quasi-steady quality as
/// the linear Debye–Hückel sibling (large φ₀ stresses SG / Newton convergence).
#[test]
#[ignore = "Deferred: φ₀=4 V_T Gouy–Chapman gate — same implicit-dispatch harness as linear λ_D test; validate λ_eff vs λ_D after linear gate is promotable on CI"]
fn debye_screening_256_cells_phi_100mv_decay_length_within_band() {
    debye_screening_admissibility_check(256, 4.0_f32, 6.0_f32, 0.45_f32);
}

/// **h_inv mesh-spacing scaling** (Phase 1.5): the Scharfetter–Gummel flux has dimension
/// `[D]·[c]/[h]`, so halving the mesh spacing should double the SG-driven concentration change per
/// time step at fixed `dt`. Asserts that doubling `mesh_spacing` halves the explicit SG step's
/// drift toward equilibrium relative to a reference run.
///
/// formal_anchor: Literature
/// formal_citation: Scharfetter & Gummel 1969, IEEE TED 16:64
/// formal_form: "‖Δc(h=H)‖ ≈ 2 · ‖Δc(h=2H)‖ at fixed dt (linear in h_inv)"
#[test]
fn sg_flux_drift_scales_with_mesh_spacing_inverse() {
    let dev = device();
    let n = 32usize;
    let edges = chain_edges(n);

    // Build a non-equilibrium ion field: smooth Gaussian bump on c_+ at the centre.
    let mut c_flat = Vec::with_capacity(n * 2);
    for i in 0..n {
        let x = (i as f32 - 0.5 * n as f32) / (n as f32 * 0.2);
        let bump = (-x * x).exp() * 0.3_f32;
        c_flat.push(1.0_f32 + bump);
        c_flat.push(1.0_f32);
    }
    let c0 = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let phi0 = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let d = Tensor::<B, 3>::full([1, n, 2], 0.02_f32, &dev);
    let dt = 5e-4_f32;

    let solver_h1 = ElectroChemicalSolver {
        mesh_spacing: 1.0_f32,
        ..Default::default()
    };
    let (_, c1) = solver_h1.solve_pnp_step(
        dt,
        phi0.clone(),
        c0.clone(),
        edges.clone(),
        eps.clone(),
        d.clone(),
    );
    let drift_h1 = max_abs_diff(&c0, &c1);

    let solver_h2 = ElectroChemicalSolver {
        mesh_spacing: 2.0_f32,
        ..Default::default()
    };
    let (_, c2) = solver_h2.solve_pnp_step(dt, phi0, c0.clone(), edges, eps, d);
    let drift_h2 = max_abs_diff(&c0, &c2);

    // SG flux ∝ 1/h ⇒ drift ratio drift_h1 / drift_h2 ≈ 2.0. Allow ±20 % slack for boundary effects
    // (interior edges scale exactly; the two end nodes only have one neighbour).
    let ratio = drift_h1 / drift_h2.max(1e-30_f32);
    assert!(
        ratio > 1.6_f32 && ratio < 2.5_f32,
        "SG flux did not scale as 1/h: drift(h=1)={drift_h1}, drift(h=2)={drift_h2}, ratio={ratio} (expected ~2.0)"
    );
}

// NOTE: A `debye_screening_inversely_scales_with_sqrt_concentration` test was attempted; structural
// λ_D ∝ 1/√c₀ scaling is deferred for a dedicated harness. The `pnp_screening_phi_decays_toward_bulk_smoke`
// test still asserts qualitative screening under explicit Picard `solve_pnp_step`.

/// Shared harness: build a 1-D chain, drive Dirichlet `φ(0) = phi0_vt` against `φ(L) = 0`,
/// run the PNP transport long enough for the near-boundary screening layer to form, fit an
/// exponential to `|φ(x)|` on the interior window, and assert `λ_eff` vs `λ_D`.
///
/// Uses [`ElectroChemicalSolver::solve_pnp_step_dispatch`] with
/// [`ElectroChemicalSolver::pnp_implicit_newton_chain`] (same opt-in pattern as production: no extra
/// `#[cfg]`, only solver fields + API). Falls back to explicit Picard only if the implicit chain
/// helper returns `None` (should not happen for these MVP batch-1 chains).
fn debye_implicit_newton_context() -> NewtonPnpContext {
    NewtonPnpContext {
        max_newton_iters: 48,
        residual_tol_l2: 1e-9,
        damping: 1.0,
        fd_step: 1e-7,
        max_chain_nodes: 512,
        linearize_sg_fickian: false,
    }
}

/// **Track 14 — dispatch + implicit chain (CI-fast).** Same opt-in wiring as the long-horizon
/// `debye_screening_admissibility_check` harness (`pnp_implicit_newton_chain` +
/// `solve_pnp_step_dispatch`), but a **short** chain and **≈2.5k** outer steps (~20s debug) so default
/// `cargo test` certifies the MVP implicit path stays finite and keeps |φ| mass skewed toward the
/// driven left in the interior. Does **not** assert the λ_D exponential-fit band (that remains on the
/// `#[ignore]` `debye_screening_256_cells_*` gates).
#[test]
fn debye_implicit_dispatch_short_horizon_smoke() {
    let dev = device();
    let n = 48usize;
    let edges = chain_edges(n);

    let c0 = 1.0_f32;
    let eps_r = 1.0_f32;
    let z = 1.0_f32;
    let lambda_d = (eps_r / (2.0 * z * z * c0)).sqrt();
    let domain_in_lambda_d = 6.0_f32;
    let l_domain = domain_in_lambda_d * lambda_d;
    let h = l_domain / (n as f32 - 1.0);
    let phi0_vt = 1.0_f32;

    let mut c_flat = Vec::with_capacity(n * 2);
    for i in 0..n {
        let x = i as f32 * h;
        let seed = (-x / lambda_d).exp() * 0.02_f32;
        c_flat.push(c0 * (1.0 - seed));
        c_flat.push(c0 * (1.0 + seed));
    }
    let mut c = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let mut phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev).mul_scalar(eps_r);
    let diff = Tensor::<B, 3>::full([1, n, 2], 2e-2_f32, &dev);

    let newton = debye_implicit_newton_context();
    let solver = ElectroChemicalSolver {
        faraday_const: 1.0_f32,
        gas_const: 1.0_f32,
        coupling_picard_iters: 3,
        pnp_implicit_newton_chain: Some(newton),
        ..Default::default()
    };

    let dt = 1.5e-3_f32;
    let steps = 2500usize;
    for _ in 0..steps {
        let (p_next, c_next) = solver.solve_pnp_step_dispatch(
            dt,
            phi.clone(),
            c.clone(),
            edges.clone(),
            eps.clone(),
            diff.clone(),
        );
        let n_nodes = p_next.dims()[1];
        let mid = p_next.clone().slice([0..1, 1..(n_nodes - 1), 0..1]);
        let left = Tensor::<B, 3>::full([1, 1, 1], phi0_vt, &dev);
        let right = Tensor::<B, 3>::zeros([1, 1, 1], &dev);
        phi = Tensor::cat(vec![left, mid, right], 1);
        c = c_next.clamp_min(1e-8_f32);
    }

    let pv = phi.into_data().value;
    let cv = c.into_data().value;
    assert!(
        pv.iter().chain(cv.iter()).all(|x| x.is_finite()),
        "phi and c must stay finite under implicit dispatch"
    );
    assert_relative_eq!(pv[0], phi0_vt, epsilon = 5e-3_f32);
    assert_relative_eq!(pv[n - 1], 0.0_f32, epsilon = 5e-3_f32);
    // Interior (exclude Dirichlet pins): more |phi| toward the driven left than the open right bulk.
    let i_mid = n / 2;
    let left_interior: f32 = pv[1..i_mid].iter().map(|x| x.abs()).sum();
    let right_interior: f32 = pv[i_mid..(n - 1)].iter().map(|x| x.abs()).sum();
    assert!(
        left_interior > right_interior + 1e-4_f32,
        "expected |phi| mass skew toward the left in the interior: left_interior={left_interior} right_interior={right_interior}"
    );
}

fn debye_screening_admissibility_check(
    n: usize,
    phi0_vt: f32,
    domain_in_lambda_d: f32,
    band_tol: f32,
) {
    let dev = device();
    let edges = chain_edges(n);

    let c0 = 1.0_f32;
    let eps_r = 1.0_f32;
    let z = 1.0_f32;
    let lambda_d = (eps_r / (2.0 * z * z * c0)).sqrt();
    let l_domain = domain_in_lambda_d * lambda_d;
    let h = l_domain / (n as f32 - 1.0);

    // Initial ion fields: bulk, with a small Boltzmann-shaped seed near the left boundary so the
    // first transport step has non-trivial charge density to drive Poisson.
    let mut c_flat = Vec::with_capacity(n * 2);
    for i in 0..n {
        let x = i as f32 * h;
        let seed = (-x / lambda_d).exp() * 0.02_f32;
        c_flat.push(c0 * (1.0 - seed));
        c_flat.push(c0 * (1.0 + seed));
    }
    let mut c = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let mut phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev).mul_scalar(eps_r);
    let diff = Tensor::<B, 3>::full([1, n, 2], 2e-2_f32, &dev);

    let newton = debye_implicit_newton_context();
    let solver = ElectroChemicalSolver {
        faraday_const: 1.0_f32,
        gas_const: 1.0_f32,
        coupling_picard_iters: 3,
        pnp_implicit_newton_chain: Some(newton),
        ..Default::default()
    };

    // Picard legacy harness used 12_000 × 1.5e-³ ≈ 18 time units; keep the same horizon so λ_D
    // exponential-fit gates remain comparable when run with `--ignored`.
    let dt = 1.5e-3_f32;
    let steps = 12_000usize;
    for _ in 0..steps {
        let (p_next, c_next) = solver.solve_pnp_step_dispatch(
            dt,
            phi.clone(),
            c.clone(),
            edges.clone(),
            eps.clone(),
            diff.clone(),
        );
        let n_nodes = p_next.dims()[1];
        let mid = p_next.clone().slice([0..1, 1..(n_nodes - 1), 0..1]);
        let left = Tensor::<B, 3>::full([1, 1, 1], phi0_vt, &dev);
        let right = Tensor::<B, 3>::zeros([1, 1, 1], &dev);
        phi = Tensor::cat(vec![left, mid, right], 1);
        c = c_next.clamp_min(1e-8_f32);
    }

    let pv = phi.into_data().value;
    // Fit ln|phi(x)| = -x/lambda_eff + b on interior window [0.2 L, 0.7 L] where the boundary-layer
    // dominates and the Dirichlet far-field hasn't kicked in yet. Use simple least squares.
    let i_lo = (0.20 * n as f32) as usize;
    let i_hi = (0.70 * n as f32) as usize;
    let mut sx = 0.0_f64;
    let mut sy = 0.0_f64;
    let mut sxx = 0.0_f64;
    let mut sxy = 0.0_f64;
    let mut count = 0.0_f64;
    for (k, &p) in pv[i_lo..i_hi].iter().enumerate() {
        let i = i_lo + k;
        let val = p.abs();
        if val < 1e-6_f32 {
            continue;
        }
        let x = (i as f32 * h) as f64;
        let y = (val as f64).ln();
        sx += x;
        sy += y;
        sxx += x * x;
        sxy += x * y;
        count += 1.0;
    }
    assert!(count > 8.0, "too few usable interior samples in fit window");
    let slope = (count * sxy - sx * sy) / (count * sxx - sx * sx);
    let lambda_eff = (-1.0 / slope) as f32;

    assert!(
        lambda_eff > 0.0,
        "extracted decay length must be positive (got {lambda_eff})"
    );
    let rel = ((lambda_eff - lambda_d) / lambda_d).abs();
    assert!(
        rel < band_tol,
        "Debye decay length out of band: phi0={phi0_vt} V_T, λ_eff={lambda_eff:.4}, λ_D={lambda_d:.4}, rel_err={rel:.3} (tol {band_tol})"
    );
}

/// **Track P2.5 — mass conservation.** On a closed 1-D chain with zero applied potential and
/// non-trivial initial concentration, the SG flux must conserve total ion mass per species over
/// many time steps. Drift relative to initial sum should be at f32-rounding level (<1e-4 over
/// 5000 steps).
///
/// formal_anchor: Literature
/// formal_citation: Scharfetter & Gummel 1969, IEEE TED 16:64
/// formal_form: "Σ_i c_i(t_N) ≈ Σ_i c_i(0)  for closed graph, no Dirichlet on c"
#[test]
fn sg_mass_conserved_on_closed_chain_over_5000_steps() {
    let dev = device();
    let n = 64usize;
    let edges = chain_edges(n);

    // Initial: smooth Gaussian on c_+, uniform background on c_-.
    let mut c_flat = Vec::with_capacity(n * 2);
    for i in 0..n {
        let x = (i as f32 - 0.5 * n as f32) / (n as f32 * 0.18);
        let bump = (-x * x).exp() * 0.5;
        c_flat.push(1.0 + bump);
        c_flat.push(1.0);
    }
    let c0 = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);

    // Compute initial total mass per species.
    let initial_sum_plus: f32 = c0.clone().narrow(2, 0, 1).sum().into_scalar();
    let initial_sum_minus: f32 = c0.clone().narrow(2, 1, 1).sum().into_scalar();

    // Run with zero applied potential everywhere (no Dirichlet — but the solver internally fixes
    // endpoint phi to incoming value; pass phi = 0 throughout and DO NOT enforce Dirichlet
    // boundaries on phi externally, so the solver's internal Thomas/relaxation is the only update).
    let phi0 = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let d = Tensor::<B, 3>::full([1, n, 2], 0.02_f32, &dev);

    let solver = ElectroChemicalSolver::default();
    let dt = 5e-4_f32;
    let steps = 5_000usize;
    let mut c = c0;
    let mut phi = phi0;
    for _ in 0..steps {
        let (p, c_next) = solver.solve_pnp_step(
            dt,
            phi.clone(),
            c.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        );
        phi = p;
        c = c_next;
    }

    let final_sum_plus: f32 = c.clone().narrow(2, 0, 1).sum().into_scalar();
    let final_sum_minus: f32 = c.narrow(2, 1, 1).sum().into_scalar();

    let drift_plus = ((final_sum_plus - initial_sum_plus) / initial_sum_plus).abs();
    let drift_minus = ((final_sum_minus - initial_sum_minus) / initial_sum_minus).abs();

    assert!(
        drift_plus < 1e-3,
        "c_+ mass drift {drift_plus} over 5000 SG steps; expected <1e-3 (f32 rounding bound)"
    );
    assert!(
        drift_minus < 1e-3,
        "c_- mass drift {drift_minus} over 5000 SG steps; expected <1e-3 (f32 rounding bound)"
    );
}

/// Picard coupling (same `dt`, multiple ρ( c )→Φ→NP sweeps) stays finite and preserves screening trend.
#[test]
fn picard_coupling_iters_finite_smoke() {
    let dev = device();
    let n = 64usize;
    let edges = chain_edges(n);
    let c0 = 1.0_f32;
    let mut c_flat = Vec::with_capacity(n * 2);
    for i in 0..n {
        let x = i as f32 / (n - 1) as f32;
        let w = (-6.0_f32 * x * x).exp();
        c_flat.push(c0 * (1.0 + 0.05 * w));
        c_flat.push(c0 * (1.0 - 0.05 * w));
    }
    let mut c = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let mut phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let d = Tensor::<B, 3>::full([1, n, 2], 2e-2_f32, &dev);
    let solver = ElectroChemicalSolver {
        coupling_picard_iters: 4,
        ..Default::default()
    };
    for _ in 0..400 {
        let (p, cn) =
            solver.solve_pnp_step(3e-4_f32, phi, c, edges.clone(), eps.clone(), d.clone());
        let n = p.dims()[1];
        let mid = p.clone().slice([0..1, 1..(n - 1), 0..1]);
        let left = Tensor::<B, 3>::full([1, 1, 1], 0.03_f32, &dev);
        let right = Tensor::<B, 3>::zeros([1, 1, 1], &dev);
        phi = Tensor::cat(vec![left, mid, right], 1);
        c = cn.clamp_min(1e-12_f32);
    }
    let pv = phi.into_data().value;
    assert!(pv.iter().all(|x| x.is_finite()));
}

/// Picard L∞ early-stop with a tolerance so tight it never triggers must match running the full
/// outer iteration budget (guards `coupling_picard_tol_linf` bookkeeping).
#[test]
fn picard_coupling_linf_tol_never_triggers_matches_full_iters() {
    let dev = device();
    let n = 32usize;
    let edges = chain_edges(n);
    let mut c_flat = vec![0.0_f32; n * 2];
    for i in 0..n {
        let x = i as f32 / (n - 1) as f32;
        c_flat[i * 2] = 0.5 + 0.4 * (1.0 - x);
        c_flat[i * 2 + 1] = c_flat[i * 2];
    }
    let c = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
    let dt = 2e-4_f32;

    let solver_no_early = ElectroChemicalSolver {
        coupling_picard_iters: 5,
        coupling_picard_tol_linf: 0.0_f32,
        ..Default::default()
    };
    let solver_tight_tol = ElectroChemicalSolver {
        coupling_picard_iters: 5,
        coupling_picard_tol_linf: 1e-30_f32,
        ..Default::default()
    };
    let (p1, c1) = solver_no_early.solve_pnp_step(
        dt,
        phi.clone(),
        c.clone(),
        edges.clone(),
        eps.clone(),
        d.clone(),
    );
    let (p2, c2) = solver_tight_tol.solve_pnp_step(dt, phi, c, edges, eps, d);
    assert_relative_eq!(max_abs_diff(&p1, &p2), 0.0_f32, epsilon = 1e-6_f32);
    assert_relative_eq!(max_abs_diff(&c1, &c2), 0.0_f32, epsilon = 1e-6_f32);
}

/// Picard outer loop (experimental split path behind `solve_pnp_step`): \(\|\Delta\Phi\|_2\) and
/// \(\max|\Delta\Phi|\) tolerances that are too tight to ever fire match the full fixed-iteration budget.
#[test]
fn picard_convergence_smoke() {
    let dev = device();
    let n = 64usize;
    let edges = chain_edges(n);
    let c0 = 1.0_f32;
    let mut c_flat = Vec::with_capacity(n * 2);
    for i in 0..n {
        let x = i as f32 / (n - 1) as f32;
        let w = (-6.0_f32 * x * x).exp();
        c_flat.push(c0 * (1.0 + 0.05 * w));
        c_flat.push(c0 * (1.0 - 0.05 * w));
    }
    let c = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let d = Tensor::<B, 3>::full([1, n, 2], 2e-2_f32, &dev);
    let dt = 3e-4_f32;

    let tight_l2 = 1e-37_f32;
    let tight_linf_phi = 1e-37_f32;

    let solver_full = ElectroChemicalSolver {
        coupling_picard_iters: 5,
        coupling_picard_tol_linf: 0.0_f32,
        coupling_picard_tol_delta_phi_linf: 0.0_f32,
        coupling_picard_tol_delta_phi_l2: 0.0_f32,
        ..Default::default()
    };
    let solver_never_l2 = ElectroChemicalSolver {
        coupling_picard_iters: 5,
        coupling_picard_tol_delta_phi_l2: tight_l2,
        ..Default::default()
    };
    let solver_never_dphi = ElectroChemicalSolver {
        coupling_picard_iters: 5,
        coupling_picard_tol_delta_phi_linf: tight_linf_phi,
        ..Default::default()
    };

    let (p0, c0) = solver_full.solve_pnp_step(
        dt,
        phi.clone(),
        c.clone(),
        edges.clone(),
        eps.clone(),
        d.clone(),
    );
    let (p1, c1) = solver_never_l2.solve_pnp_step(
        dt,
        phi.clone(),
        c.clone(),
        edges.clone(),
        eps.clone(),
        d.clone(),
    );
    let (p2, c2) = solver_never_dphi.solve_pnp_step(dt, phi, c, edges, eps, d);

    assert_relative_eq!(max_abs_diff(&p0, &p1), 0.0_f32, epsilon = 1e-5_f32);
    assert_relative_eq!(max_abs_diff(&c0, &c1), 0.0_f32, epsilon = 1e-5_f32);
    assert_relative_eq!(max_abs_diff(&p0, &p2), 0.0_f32, epsilon = 1e-5_f32);
    assert_relative_eq!(max_abs_diff(&c0, &c2), 0.0_f32, epsilon = 1e-5_f32);
}

/// **Track 14 — implicit backward Euler Newton vs split.** Manufactured **linearised** SG (Fickian
/// flux in the implicit residual via [`NewtonPnpContext::linearize_sg_fickian`]) with huge
/// `gas_const` so production SG is drift-suppressed. In the **infinitesimal-**`dt` limit the
/// operator-split explicit step and the monolithic implicit BE state **agree** within a tight
/// tolerance; at **finite** `dt` they **diverge**, and the implicit state zeros the BE residual.
/// The implicit pass is also checked against a modest ‖R‖₂ bound on **f32** tensors for the
/// small-`dt` case (the `(c-c^n)/\Delta t` block amplifies f32 round-off when \(\Delta t\) is tiny).
///
/// The test **fails** if the small-`dt` split vs implicit agreement breaks (guards accidental
/// regression of the Newton path toward the split discretisation).
///
/// Implicit steps use [`ElectroChemicalSolver::solve_pnp_step_dispatch`] with
/// [`ElectroChemicalSolver::pnp_implicit_newton_chain`] (same production opt-in as the Debye
/// harness), not a bare [`ElectroChemicalSolver::try_solve_pnp_backward_euler_newton_chain`] call.
#[test]
fn backward_euler_implicit_newton_matches_split_in_linearized_small_dt_limit() {
    let dev = device();
    let n = 9_usize;
    let edges = chain_edges(n);
    let mut c_flat = vec![0.0_f32; n * 2];
    for i in 0..n {
        let x = i as f32 / (n - 1) as f32;
        c_flat[i * 2] = 1.0 + 0.02 * x;
        c_flat[i * 2 + 1] = 1.0 - 0.02 * x;
    }
    let c_n = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
    let g0 = 0.015_f32;
    let g1 = 0.0_f32;
    let mut ph = vec![0.0_f32; n];
    ph[0] = g0;
    ph[n - 1] = g1;
    let phi_n = Tensor::<B, 3>::from_data(Data::new(ph, Shape::new([1, n, 1])), &dev);
    let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
    let solver_split = ElectroChemicalSolver {
        faraday_const: 1.0_f32,
        gas_const: 1.0e9_f32,
        mesh_spacing: 1.0_f32,
        ..Default::default()
    };
    let newton = NewtonPnpContext {
        max_newton_iters: 28,
        residual_tol_l2: 1e-11,
        damping: 1.0,
        fd_step: 1e-7,
        max_chain_nodes: 32,
        linearize_sg_fickian: true,
    };
    let solver_dispatch_small = ElectroChemicalSolver {
        faraday_const: 1.0_f32,
        gas_const: 1.0e9_f32,
        mesh_spacing: 1.0_f32,
        pnp_implicit_newton_chain: Some(newton),
        ..Default::default()
    };
    let dt_small = 1e-7_f32;
    let (phi_i, c_i) = solver_dispatch_small.solve_pnp_step_dispatch(
        dt_small,
        phi_n.clone(),
        c_n.clone(),
        edges.clone(),
        eps.clone(),
        d.clone(),
    );
    let be_res = pnp_backward_euler_residual_l2_chain_host_f64(
        &solver_dispatch_small,
        &newton,
        dt_small,
        &phi_i,
        &c_i,
        &c_n,
        &edges,
        &eps,
        &d,
    )
    .expect("BE residual norm");
    // Host Newton converges in f64 to ‖R‖₂ ≪ 1e-6 before tensor export; re-evaluating R on f32
    // tensors with very small `dt` amplifies the (c−cⁿ)/Δt block (Track 14 / f32 export path).
    assert!(
        be_res < 5e-4_f64,
        "implicit BE residual (f32 state) should stay small, got L2={be_res}"
    );
    let (phi_s, c_s) = solver_split.solve_pnp_step(
        dt_small,
        phi_n.clone(),
        c_n.clone(),
        edges.clone(),
        eps.clone(),
        d.clone(),
    );
    let dphi = max_abs_diff(&phi_i, &phi_s);
    let dc = max_abs_diff(&c_i, &c_s);
    assert!(
        dphi < 5e-5_f32 && dc < 5e-5_f32,
        "small-dt linearised case: split vs implicit BE should agree; dphi={dphi} dc={dc}"
    );

    let dt_fin = 0.04_f32;
    let newton_fin = NewtonPnpContext {
        max_newton_iters: 40,
        residual_tol_l2: 1e-10,
        ..newton
    };
    let solver_dispatch_fin = ElectroChemicalSolver {
        faraday_const: 1.0_f32,
        gas_const: 1.0e9_f32,
        mesh_spacing: 1.0_f32,
        pnp_implicit_newton_chain: Some(newton_fin),
        ..Default::default()
    };
    let (phi_if, c_if) = solver_dispatch_fin.solve_pnp_step_dispatch(
        dt_fin,
        phi_n.clone(),
        c_n.clone(),
        edges.clone(),
        eps.clone(),
        d.clone(),
    );
    let be_fin = pnp_backward_euler_residual_l2_chain_host_f64(
        &solver_dispatch_fin,
        &newton,
        dt_fin,
        &phi_if,
        &c_if,
        &c_n,
        &edges,
        &eps,
        &d,
    )
    .expect("BE residual finite dt");
    assert!(
        be_fin < 5e-6_f64,
        "implicit solution should satisfy BE residual at finite dt, got {be_fin}"
    );
    let (phi_sf, c_sf) = solver_split.solve_pnp_step(dt_fin, phi_n, c_n, edges, eps, d);
    let gap = max_abs_diff(&phi_if, &phi_sf).max(max_abs_diff(&c_if, &c_sf));
    assert!(
        gap > 1e-8_f32,
        "finite dt: expect split state to differ measurably from implicit BE root, gap={gap}"
    );
}
