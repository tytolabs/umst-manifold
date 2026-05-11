// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Γ-convergence / phase-field fracture verification.
//!
//! Specification: `composer_prompts/v0.4_solver_completion_no_namesakes.md`. Repo table + full
//! status: `docs/Solver-Status.md` → **Phase-field fracture: implemented vs deferred**.
//!
//! **Implemented (this harness + `fracture_field.rs`):** inner AT2 damage relaxation uses **red–black**
//! half-steps on the graph Laplacian (see solver module docs). With `--features fracture-at2`:
//! `update_damage_smoke_tiny_chain` (finite \(d\); `outer_iterations == 1` with a fixed-strain
//! provider matches [`PhaseFieldFractureSolver::update_damage`]); `at2_surface_energy_scale_matches_gc_order_of_magnitude` (order-of-magnitude
//! \(G_c/l\cdot\bar d\) on the tiny chain); `at2_gc_linear_scaling_smoke` (doubling \(G_c\) at fixed \((l,\varepsilon)\): \(\bar d\) stays same order and \(G_c/l\cdot\bar d\) tracks \(\Delta G_c\) loosely — explicit red–black sweep, not a converged Γ-limit).
//!
//! **Deferred:** **multi-\(l_0\) Γ-limit** harness not implemented (filename = intent); would need
//! systematic \(l_0,h\) refinement, reference solutions, and dissipation-to-\(G_c\) checks beyond
//! current smoke. **Full staggered elasticity–damage** (fresh \(\varepsilon\) each outer mechanics
//! solve) is not validated here — [`PhaseFieldFractureSolver::update_damage`] is fixed-strain
//! damage only; orchestration remains `ThmcSolver` / shell.

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

use umst_manifold::physics::solvers::PhaseFieldFractureSolver;
#[cfg(feature = "fracture-at2")]
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = NdArray<f32>;

#[test]
fn update_damage_smoke_tiny_chain() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

    // Uniform small uniaxial strain ε_xx = 1e−3 at every node (symmetric 3×3).
    let exx = 1e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9; // b=0
        strain_data[base] = exx; // (0,0)
        strain_data[base + 4] = 0.0; // (1,1)
        strain_data[base + 8] = 0.0; // (2,2)
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };

    let d_new = solver.update_damage(
        strain.clone(),
        damage.clone(),
        fracture_energy_gc.clone(),
        edges_b1.clone(),
    );

    assert_eq!(d_new.dims(), damage.dims());

    #[cfg(not(feature = "fracture-at2"))]
    {
        let unchanged = damage.into_data().value == d_new.into_data().value;
        assert!(unchanged);
    }

    #[cfg(feature = "fracture-at2")]
    {
        for &x in d_new.clone().into_data().value.iter() {
            assert!(x.is_finite(), "expected finite damage; got {}", x);
            assert!(
                (0.0..=1.0).contains(&x),
                "expected damage in [0,1]; got {}",
                x
            );
        }
        // `outer_iterations == 1` + fixed strain provider matches a single inner relaxation.
        let d_stagg = solver.update_damage_staggered(
            |_d| strain.clone(),
            Tensor::<B, 3>::zeros([batch, n, 1], &dev),
            fracture_energy_gc.clone(),
            edges_b1.clone(),
            1,
        );
        let v_new = d_new.clone().into_data().value;
        let v_stagg = d_stagg.into_data().value;
        assert_eq!(
            v_new, v_stagg,
            "outer_iterations==1 with constant strain_fn must match update_damage"
        );
        let sum_d: f32 = v_new.iter().sum();
        let max_d = v_new.iter().copied().fold(0.0_f32, f32::max);
        assert!(
            max_d > 1e-10_f32 && sum_d > 1e-10_f32,
            "smoke expected stable positive AT2 damage (max and total); max_d={max_d} sum_d={sum_d} vals={v_new:?}"
        );
    }
}

/// Multi–length-scale smoke: damage stays admissible for several \(l\) (not Γ-convergence).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_length_scale_sweep_non_regression() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);
    let exx = 4e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![120.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    for l in [0.06_f32, 0.09_f32, 0.12_f32] {
        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let solver = PhaseFieldFractureSolver { length_scale: l };
        let d_new = solver.update_damage(
            strain.clone(),
            damage,
            fracture_energy_gc.clone(),
            edges_b1.clone(),
        );
        for &x in d_new.clone().into_data().value.iter() {
            assert!(x.is_finite(), "l={l}: non-finite damage");
            assert!((0.0..=1.0).contains(&x), "l={l}: damage out of range");
        }
    }
}

/// Stronger tensile strain on the smoke **tiny chain**: AT2 drive yields finite positive damage and a `Gc/l`-consistent dissipation scale.
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_surface_energy_scale_matches_gc_order_of_magnitude() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

    let exx = 5e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let gc = 150.0_f32;
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![gc; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    let solver = PhaseFieldFractureSolver {
        length_scale: 0.08_f32,
    };
    let d_new = solver.update_damage(strain, damage, fracture_energy_gc, edges_b1);
    let vals = d_new.into_data().value;
    let mean_d: f32 = vals.iter().sum::<f32>() / vals.len() as f32;
    assert!(
        mean_d > 1e-6_f32,
        "expected AT2 mean damage under tensile strain; mean_d={mean_d}"
    );

    let l = 0.08_f32;
    let dissipation_scale = gc / l * mean_d;
    assert!(
        (1e-3_f32..1e7_f32).contains(&dissipation_scale),
        "Gc/l·d̄ scale {dissipation_scale} out of expected band"
    );
}

/// Doubling \(G_c\) at fixed \((l,\varepsilon)\): **dissipation-scale** proxy \(G_c/l\cdot\bar d\) rises
/// roughly with \(\Delta G_c\) (loose band); \(\bar d\) remains bounded and comparable order (explicit
/// relaxation — not a claim of pointwise monotone \(\bar d\) vs \(G_c\) on this tiny chain).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_gc_linear_scaling_smoke() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

    let exx = 5e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage0 = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let l = 0.08_f32;
    let gc_lo = 100.0_f32;
    let gc_hi = 200.0_f32;

    let gc_field_lo = Tensor::from_data(
        Data::new(vec![gc_lo; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    let gc_field_hi = Tensor::from_data(
        Data::new(vec![gc_hi; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    let solver = PhaseFieldFractureSolver { length_scale: l };

    let d_lo = solver.update_damage(
        strain.clone(),
        damage0.clone(),
        gc_field_lo,
        edges_b1.clone(),
    );
    let d_hi = solver.update_damage(strain, damage0, gc_field_hi, edges_b1.clone());

    let vals_lo = d_lo.into_data().value;
    let vals_hi = d_hi.into_data().value;
    let mean_lo: f32 = vals_lo.iter().sum::<f32>() / vals_lo.len() as f32;
    let mean_hi: f32 = vals_hi.iter().sum::<f32>() / vals_hi.len() as f32;

    assert!(
        mean_lo > 1e-8_f32 && mean_hi > 1e-8_f32,
        "expected positive mean damage at both Gc; mean_lo={mean_lo} mean_hi={mean_hi}"
    );

    let r_md = mean_hi / mean_lo;
    assert!(
        (0.55_f32..1.65_f32).contains(&r_md),
        "mean_d ratio (2Gc vs Gc) expected bounded O(1) smoke; mean_lo={mean_lo} mean_hi={mean_hi} r_md={r_md}"
    );

    let scale_lo = gc_lo / l * mean_lo;
    let scale_hi = gc_hi / l * mean_hi;
    assert!(
        scale_lo > 1e-6_f32 && scale_hi > 1e-6_f32,
        "dissipation-scale proxy should stay above noise; scale_lo={scale_lo} scale_hi={scale_hi}"
    );
    let r_scale = scale_hi / scale_lo;
    assert!(
        (1.05_f32..3.8_f32).contains(&r_scale),
        "Gc/l·d̄ should rise sub-quadratically when Gc doubles; scale_lo={scale_lo} scale_hi={scale_hi} r_scale={r_scale}"
    );
}

/// First outer pass uses **negligible** tensile strain `A`, second pass uses strong strain `B`.
/// Total damage after two passes must exceed a **single** `update_damage` call with only `A`
/// (irreversibility can make “`A` then stronger `B`” match “`B` only” when the first pass already
/// saturates damage — this harness uses a tiny first drive so the second pass is observable).
#[cfg(feature = "fracture-at2")]
#[test]
fn staggered_two_outer_strains_exceeds_single_pass_weak_strain_only() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

    fn uniaxial_strain(dev: &NdArrayDevice, batch: usize, n: usize, exx: f32) -> Tensor<B, 4> {
        let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
        for nod in 0..n {
            let base = (batch * nod) * 9;
            strain_data[base] = exx;
            strain_data[base + 4] = 0.0;
            strain_data[base + 8] = 0.0;
        }
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), dev)
    }

    let strain_weak = uniaxial_strain(&dev, batch, n, 1e-12_f32);
    let strain_strong = uniaxial_strain(&dev, batch, n, 5e-3_f32);

    let damage0 = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };

    let d_single_weak = solver.update_damage(
        strain_weak.clone(),
        damage0.clone(),
        fracture_energy_gc.clone(),
        edges_b1.clone(),
    );

    let mut outer_k = 0usize;
    let d_staggered = solver.update_damage_staggered(
        |_d: &Tensor<B, 3>| {
            let s = if outer_k == 0 {
                strain_weak.clone()
            } else {
                strain_strong.clone()
            };
            outer_k += 1;
            s
        },
        damage0,
        fracture_energy_gc,
        edges_b1,
        2,
    );

    let sum_w: f32 = d_single_weak.into_data().value.iter().sum();
    let sum_st: f32 = d_staggered.into_data().value.iter().sum();
    assert!(
        sum_st > sum_w + 1e-8_f32,
        "expected weak→strong staggered total damage to exceed single weak pass; sum_w={sum_w} sum_st={sum_st}"
    );
}

/// Γ-convergence harness (Phase 2.4): single 1-D pre-notched bar; refine `(l₀, h)` pairs together
/// keeping `h/l₀ = 1/4` and check that the discrete dissipation
/// `D_h = Σ_i [ d_i² · h / (2 l₀) + (l₀/2) (d_{i+1}-d_i)²/h ] · Gc`
/// approaches the analytic `Gc` limit, with monotonically decreasing error.
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_gamma_convergence_three_length_scales() {
    let dev = NdArrayDevice::Cpu;
    let length_l: f32 = 5.0;
    let gc_val: f32 = 1.0;
    // ψ⁺ ≡ 0: with **irreversibility** (`out.max_pair(damage_old)`) the relaxation cannot reduce the
    // pre-localised seed, so `d` stays at the optimal AT2 continuum profile `exp(-|x-L/2|/l₀)`.
    // The discrete functional `D_h` evaluated on this profile is the textbook Γ-convergence quantity
    // (Bourdin–Francfort–Marigo 2000 §3.2 / Miehe 2010 §4.1). Continuum value is exactly `Gc`.
    let psi_plus_drive: f32 = 0.0;

    let pairs: [(f32, f32); 3] = [(0.04, 0.01), (0.02, 0.005), (0.01, 0.0025)];
    let mut errors: Vec<f32> = Vec::with_capacity(pairs.len());
    let mut d_hs: Vec<f32> = Vec::with_capacity(pairs.len());

    for (l0, h) in pairs.iter().copied() {
        let n: usize = ((length_l / h).ceil() as usize) + 1;
        let e_ct: usize = n - 1;
        let batch: usize = 1;

        // Edges: 0–1, 1–2, …, (N−2)–(N−1) packed as `[2, E]` (row 0 = src, row 1 = tgt).
        let mut edge_data = Vec::with_capacity(2 * e_ct);
        for i in 0..e_ct {
            edge_data.push(i as i64);
        }
        for i in 0..e_ct {
            edge_data.push((i + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edge_data, Shape::new([2, e_ct])), &dev);

        // Initial damage: optimal AT2 profile exp(-|x − L/2| / l₀), the analytic minimiser of
        // the surface-energy density `γ(d) = d²/(2l₀) + (l₀/2)|∇d|²` for a single fully-developed
        // crack at `L/2`. Irreversibility preserves this seed across relaxation passes.
        let mut d_init = vec![0.0_f32; n];
        let centre = length_l * 0.5;
        for (i, slot) in d_init.iter_mut().enumerate().take(n) {
            let x = (i as f32) * h;
            *slot = (-((x - centre).abs()) / l0).exp();
        }
        let damage = Tensor::<B, 3>::from_data(Data::new(d_init, Shape::new([batch, n, 1])), &dev);

        // Uniform ψ⁺ encoded as diagonal strain with ε_xx s.t. ½·ε² = psi_plus_drive ⇒ ε = √(2·psi).
        let exx = (2.0_f32 * psi_plus_drive).sqrt();
        let mut strain_data = vec![0.0_f32; batch * n * 9];
        for nod in 0..n {
            let base = nod * 9;
            strain_data[base] = exx;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![gc_val; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: l0 };
        // 32 outer passes of the fixed-strain relaxation (each call already runs the inner red–black loop).
        let mut d_curr = damage.clone();
        for _ in 0..32 {
            d_curr = solver.update_damage(
                strain.clone(),
                d_curr,
                fracture_energy_gc.clone(),
                edges_b1.clone(),
            );
        }

        let d_vals: Vec<f32> = d_curr.into_data().value;
        let mut d_h: f32 = 0.0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            d_h += d_vals[i].powi(2) * h / (2.0 * l0);
        }
        for i in 0..(n - 1) {
            let grad = (d_vals[i + 1] - d_vals[i]) / h;
            d_h += (l0 / 2.0) * grad.powi(2) * h;
        }
        d_h *= gc_val;
        let err = (d_h - gc_val).abs() / gc_val;
        d_hs.push(d_h);
        errors.push(err);
        eprintln!("Γ-conv: l0={l0:.4} h={h:.4} N={n} D_h={d_h:.4} rel_err={err:.4}");
    }

    // Acceptance: bound the coarsest error generously (AT2 discrete factor ≈ 1.06 over `Gc`) and
    // check monotone decrease of the error as `l₀` shrinks (Γ-convergence signature).
    for (i, &err) in errors.iter().enumerate() {
        assert!(
            err < 0.30,
            "Γ-conv error too large at pair {i}: D_h={} rel_err={err}",
            d_hs[i]
        );
    }
    assert!(
        errors[1] <= errors[0] + 1e-3,
        "error must not increase between coarse→mid: {:?}",
        errors
    );
    assert!(
        errors[2] <= errors[1] + 1e-3,
        "error must not increase between mid→fine: {:?}",
        errors
    );
}

/// Phase 3.1 — staggered elasticity–damage loop owns the mechanics solve internally.
/// Tensile 1-D bar with a pre-localised damage seed; the staggered loop must increase compliance
/// monotonically and drive damage at the load-point above a high threshold.
#[cfg(feature = "fracture-at2")]
#[test]
fn staggered_fracture_compliance_monotone_increasing() {
    use umst_manifold::physics::solvers::fracture_field::StaggeredFractureConfig;
    use umst_manifold::physics::solvers::PhaseFieldFractureSolver;

    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 20usize;
    let e_ct = n - 1;
    let length_l: f32 = 1.0;
    let h = length_l / ((n - 1) as f32);

    let mut coords_data = Vec::with_capacity(n * 3);
    for i in 0..n {
        coords_data.push(i as f32 * h);
        coords_data.push(0.0);
        coords_data.push(0.0);
    }
    let coords: Tensor<B, 2> = Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);

    let mut edges = Vec::with_capacity(2 * e_ct);
    for eid in 0..e_ct {
        edges.push(eid as i64);
    }
    for eid in 0..e_ct {
        edges.push((eid + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    // Tensile force F at right tip, only x-DOF free everywhere except the pinned left node.
    let force: f32 = 0.1;
    let mut bf_data = vec![0.0_f32; n * 3];
    bf_data[(n - 1) * 3] = force;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &dev);

    // Pin x at node 0; lock y,z everywhere; free x elsewhere.
    let mut bm_data = vec![0.0_f32; n * 3];
    for i in 1..n {
        bm_data[i * 3] = 1.0;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &dev);

    // ρ = 1 everywhere → E_eff = E0.
    let rho_node = Tensor::<B, 3>::ones([batch, n, 1], &dev);

    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 400,
        cg_tolerance: 1e-8,
        pcg_tolerance: 1e-8,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let cross_section_area = 0.01_f32;
    let e0: f32 = 1.0;

    // Baseline compliance c_0 with damage = 0 — solve once via outer_iters = 1 and capture u tip.
    let cfg_one = StaggeredFractureConfig {
        outer_iters: 1,
        damage_relaxation_passes: 1,
        gc: 0.01,
        length_scale: 0.05,
        kappa_reg: 1e-6,
    };
    let (u0, _d0) = PhaseFieldFractureSolver::solve_staggered_with_mechanics::<B>(
        coords.clone(),
        edges_b1.clone(),
        body_force.clone(),
        boundary_mask.clone(),
        rho_node.clone(),
        e0,
        cross_section_area,
        &cg,
        cfg_one,
    );
    let u0_vals = u0.into_data().value;
    let c0 = force * u0_vals[(n - 1) * 3];

    // Capture compliance at increasing outer counts and check monotonicity.
    let mut compliances: Vec<f32> = vec![c0];
    let outer_schedule = [2usize, 5, 10, 20, 30];
    let mut d_last_max: f32 = 0.0;
    for &k in outer_schedule.iter() {
        let cfg_k = StaggeredFractureConfig {
            outer_iters: k,
            damage_relaxation_passes: 1,
            gc: 0.01,
            length_scale: 0.05,
            kappa_reg: 1e-6,
        };
        let (u_k, d_k) = PhaseFieldFractureSolver::solve_staggered_with_mechanics::<B>(
            coords.clone(),
            edges_b1.clone(),
            body_force.clone(),
            boundary_mask.clone(),
            rho_node.clone(),
            e0,
            cross_section_area,
            &cg,
            cfg_k,
        );
        let u_vals = u_k.into_data().value;
        let d_vals = d_k.into_data().value;
        let tip_u = u_vals[(n - 1) * 3];
        let c_k = force * tip_u;
        let max_d = d_vals.iter().copied().fold(0.0_f32, f32::max);
        d_last_max = max_d;
        eprintln!("staggered: k={k} c_k={c_k:.6} max_d={max_d:.4}");
        compliances.push(c_k);
    }

    let c_final = *compliances.last().unwrap();
    assert!(
        c_final > c0,
        "expected compliance to grow: c0={c0} c_final={c_final}"
    );
    assert!(
        d_last_max > 0.5,
        "expected significant damage growth; max_d_final={d_last_max}"
    );

    // Monotone non-decreasing (allow ε = 1e-4 slack).
    for w in compliances.windows(2) {
        assert!(
            w[1] >= w[0] - 1e-4,
            "compliance must be monotone non-decreasing; got sequence {compliances:?}"
        );
    }
}
