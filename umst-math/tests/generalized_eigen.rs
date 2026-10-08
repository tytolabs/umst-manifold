// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Profile `L D Lᵀ` and certified generalised eigenpairs against closed-form spectra.
//!
//! References: the spring chain `tridiag(−1, 2, −1)` has `λ_j = 2 − 2 cos(jπ/(n+1))`; the linear two-node bar
//! with consistent mass has the discrete spectrum `λ_j = (6/h²)(1 − cos θ_j)/(2 + cos θ_j)` with `θ_j = jπ/N`
//! (fixed–fixed, `j = 1…N−1`; free–free, `j = 0…N`), per unit `E/ρ` (Hughes, *The Finite Element Method*, 2000,
//! §9.2, dispersion of the consistent-mass linear element).

use std::f64::consts::PI;

use umst_math::generalized_eigen::{
    lowest_eigenpairs, tridiagonal_eigen, Completeness, Deflation, EigenBound, EigenRefuse,
    EigenRequest, Pencil,
};
use umst_math::profile_ldlt::{
    ldlt, spd_factor, Clique, ProfilePattern, ProfileRefuse, SymmetricProfile,
};
use umst_math::solve_combinator::{EnergyBudget, FixedJouleMeter, ProblemTolerance, SolveOutcome};

fn budget() -> EnergyBudget {
    EnergyBudget::from_joules_at(1.0e6, 300.0).expect("budget")
}

fn meter() -> FixedJouleMeter {
    FixedJouleMeter::from_joules(1.0e-3).expect("meter")
}

/// Two-node bar elements on `nodes` nodes; `fixed_ends` removes both end nodes.
fn bar(n_el: usize, fixed_ends: bool) -> (SymmetricProfile, SymmetricProfile) {
    let h = 1.0 / n_el as f64;
    let ke = [1.0 / h, -1.0 / h, -1.0 / h, 1.0 / h];
    let me = [2.0 * h / 6.0, h / 6.0, h / 6.0, 2.0 * h / 6.0];
    let dof = |node: usize| -> Option<usize> {
        if fixed_ends {
            (node > 0 && node < n_el).then(|| node - 1)
        } else {
            Some(node)
        }
    };
    let n = if fixed_ends { n_el - 1 } else { n_el + 1 };
    // Keep only the free rows and columns of each element block.
    let cliques: Vec<(Vec<usize>, Vec<f64>, Vec<f64>)> = (0..n_el)
        .map(|e| {
            let local: Vec<(usize, usize)> = [e, e + 1]
                .iter()
                .enumerate()
                .filter_map(|(a, &node)| dof(node).map(|d| (a, d)))
                .collect();
            let dofs: Vec<usize> = local.iter().map(|&(_, d)| d).collect();
            let pick = |blk: &[f64; 4]| -> Vec<f64> {
                local
                    .iter()
                    .flat_map(|&(a, _)| local.iter().map(move |&(b, _)| blk[a * 2 + b]))
                    .collect()
            };
            (dofs, pick(&ke), pick(&me))
        })
        .collect();
    let pattern =
        ProfilePattern::from_cliques(n, cliques.iter().map(|c| c.0.as_slice())).expect("pattern");
    let k = pattern
        .assemble(cliques.iter().map(|c| Clique {
            dofs: &c.0,
            block: &c.1,
        }))
        .expect("K");
    let m = pattern
        .assemble(cliques.iter().map(|c| Clique {
            dofs: &c.0,
            block: &c.2,
        }))
        .expect("M");
    (k, m)
}

fn bar_exact(n_el: usize, j: usize) -> f64 {
    let h = 1.0 / n_el as f64;
    let t = j as f64 * PI / n_el as f64;
    (6.0 / (h * h)) * (1.0 - t.cos()) / (2.0 + t.cos())
}

fn converged<X>(o: SolveOutcome<X>) -> X {
    match o {
        SolveOutcome::Converged { x, .. } => x,
        SolveOutcome::Stalled { .. } => panic!("stalled"),
        SolveOutcome::BudgetSpent { .. } => panic!("budget spent"),
    }
}

#[test]
fn profile_matches_dense_and_ldlt_solves() {
    // Random-looking symmetric positive definite matrix from overlapping 3-cliques with a diagonal boost.
    let n = 9;
    let cliques: Vec<(Vec<usize>, Vec<f64>)> = (0..n - 2)
        .map(|i| {
            let dofs = vec![i, i + 1, i + 2];
            let a = 1.0 + i as f64 * 0.37;
            let block = vec![
                4.0 * a,
                -a,
                0.5,
                -a,
                4.0 * a,
                -0.3 * a,
                0.5,
                -0.3 * a,
                4.0 * a,
            ];
            (dofs, block)
        })
        .collect();
    let pattern =
        ProfilePattern::from_cliques(n, cliques.iter().map(|c| c.0.as_slice())).expect("pattern");
    let a = pattern
        .assemble(cliques.iter().map(|c| Clique {
            dofs: &c.0,
            block: &c.1,
        }))
        .expect("A");
    let dense = |i: usize, j: usize| -> f64 {
        cliques
            .iter()
            .filter_map(|(d, b)| {
                let p = d.iter().position(|&x| x == i)?;
                let q = d.iter().position(|&x| x == j)?;
                Some(b[p * 3 + q])
            })
            .sum()
    };
    (0..n).for_each(|i| (0..n).for_each(|j| assert!((a.get(i, j) - dense(i, j)).abs() < 1e-14)));
    let x_true: Vec<f64> = (0..n).map(|i| (i as f64 * 0.7).sin() + 0.2).collect();
    let b = a.mul(&x_true).expect("b");
    let x = spd_factor(&a).expect("spd").solve(&b).expect("solve");
    let err = x
        .iter()
        .zip(&x_true)
        .map(|(p, q)| (p - q).abs())
        .fold(0.0, f64::max);
    assert!(err < 1e-12, "solve error {err}");
}

#[test]
fn inertia_counts_eigenvalues_below_a_shift() {
    let (k, m) = bar(20, true);
    // Between the third and fourth eigenvalues exactly three lie below.
    let tau = 0.5 * (bar_exact(20, 3) + bar_exact(20, 4));
    let f = ldlt(&k.combine(1.0, &m, -tau).expect("pencil")).expect("factor");
    assert_eq!(f.inertia().negative, 3);
    assert_eq!(f.inertia().positive, 16);
}

#[test]
fn singular_and_indefinite_matrices_refuse() {
    let pattern = ProfilePattern::from_cliques(2, [&[0_usize, 1][..]]).expect("pattern");
    let singular = pattern
        .assemble([Clique {
            dofs: &[0, 1],
            block: &[1.0, 1.0, 1.0, 1.0],
        }])
        .expect("A");
    assert_eq!(
        ldlt(&singular),
        Err(ProfileRefuse::NearZeroPivot { row: 1 })
    );
    let indefinite = pattern
        .assemble([Clique {
            dofs: &[0, 1],
            block: &[1.0, 2.0, 2.0, 1.0],
        }])
        .expect("A");
    assert_eq!(
        spd_factor(&indefinite),
        Err(ProfileRefuse::NotPositiveDefinite { row: 1 })
    );
    assert_eq!(ldlt(&indefinite).expect("ldlt").inertia().negative, 1);
    assert_eq!(
        ProfilePattern::from_cliques(2, [&[0_usize, 2][..]]),
        Err(ProfileRefuse::IndexOutOfRange { index: 2 })
    );
}

#[test]
fn tridiagonal_ql_matches_the_spring_chain() {
    let n = 7;
    let (values, vectors) = tridiagonal_eigen(&vec![2.0; n], &vec![-1.0; n - 1]).expect("ql");
    for (j, v) in values.iter().enumerate() {
        let exact = 2.0 - 2.0 * ((j + 1) as f64 * PI / (n + 1) as f64).cos();
        assert!((v - exact).abs() < 1e-13, "λ_{j} = {v}, exact {exact}");
    }
    // Columns are orthonormal.
    for a in 0..n {
        for b in 0..n {
            let d: f64 = (0..n).map(|r| vectors[r][a] * vectors[r][b]).sum();
            assert!((d - if a == b { 1.0 } else { 0.0 }).abs() < 1e-12);
        }
    }
}

#[test]
fn fixed_bar_lowest_modes_are_certified_and_complete() {
    let n_el = 60;
    let (k, m) = bar(n_el, true);
    let request = EigenRequest {
        wanted: 5,
        shift: -1.0,
        deflation: Vec::new(),
        tolerance: ProblemTolerance::from_problem(1.0, 1e-10).expect("tol"),
    };
    let sol = converged(
        lowest_eigenpairs(Pencil { k: &k, m: &m }, &request, budget(), &meter()).expect("solve"),
    );
    assert_eq!(sol.pairs.len(), 5);
    assert!(
        matches!(sol.completeness, Completeness::Certified { below: 5, .. }),
        "{:?}",
        sol.completeness
    );
    for (j, p) in sol.pairs.iter().enumerate() {
        let exact = bar_exact(n_el, j + 1);
        assert!(
            p.lower <= exact && exact <= p.upper,
            "mode {j}: [{}, {}] misses {exact}",
            p.lower,
            p.upper
        );
        assert!(
            (p.lambda - exact).abs() <= 1e-9 * exact,
            "mode {j}: {} vs {exact}",
            p.lambda
        );
        assert_eq!(p.bound, EigenBound::KatoTemple);
    }
}

#[test]
fn free_bar_deflates_the_rigid_mode_and_tightens_with_kato_temple() {
    let n_el = 40;
    let (k, m) = bar(n_el, false);
    let rigid = Deflation {
        vector: vec![1.0; n_el + 1],
    };
    let request = EigenRequest {
        wanted: 4,
        shift: -1.0,
        deflation: vec![rigid],
        tolerance: ProblemTolerance::from_problem(1.0, 1e-10).expect("tol"),
    };
    let sol = converged(
        lowest_eigenpairs(Pencil { k: &k, m: &m }, &request, budget(), &meter()).expect("solve"),
    );
    assert_eq!(sol.deflated.len(), 1);
    assert!(
        sol.deflated[0].lambda.abs() < 1e-10,
        "rigid mode Rayleigh quotient {}",
        sol.deflated[0].lambda
    );
    assert!(
        matches!(sol.completeness, Completeness::Certified { below: 5, .. }),
        "{:?}",
        sol.completeness
    );
    for (j, p) in sol.pairs.iter().enumerate() {
        let exact = bar_exact(n_el, j + 1);
        assert!(p.lower <= exact && exact <= p.upper, "mode {j}");
        assert_eq!(
            p.bound,
            EigenBound::KatoTemple,
            "mode {j} must be tightened"
        );
        assert!(
            p.upper - p.lower < 2.0 * (p.residual + p.rounding),
            "mode {j}: Kato–Temple must narrow the Weinstein width"
        );
    }
}

#[test]
fn shift_above_the_spectrum_refuses() {
    let (k, m) = bar(10, true);
    let request = EigenRequest {
        wanted: 2,
        shift: bar_exact(10, 1) + 1.0,
        deflation: Vec::new(),
        tolerance: ProblemTolerance::from_problem(1.0, 1e-8).expect("tol"),
    };
    assert_eq!(
        lowest_eigenpairs(Pencil { k: &k, m: &m }, &request, budget(), &meter()).err(),
        Some(EigenRefuse::ShiftNotBelowSpectrum { below: 1 })
    );
}

#[test]
fn malformed_requests_refuse() {
    let (k, m) = bar(10, true);
    let tol = ProblemTolerance::from_problem(1.0, 1e-8).expect("tol");
    let none = EigenRequest {
        wanted: 0,
        shift: -1.0,
        deflation: Vec::new(),
        tolerance: tol,
    };
    assert_eq!(
        lowest_eigenpairs(Pencil { k: &k, m: &m }, &none, budget(), &meter()).err(),
        Some(EigenRefuse::WantedOutOfRange)
    );
    let bad = EigenRequest {
        wanted: 1,
        shift: -1.0,
        deflation: vec![Deflation {
            vector: vec![1.0; 3],
        }],
        tolerance: tol,
    };
    assert_eq!(
        lowest_eigenpairs(Pencil { k: &k, m: &m }, &bad, budget(), &meter()).err(),
        Some(EigenRefuse::DeflationInvalid)
    );
}

#[test]
fn owned_and_borrowed_cliques_assemble_the_same_matrix() {
    let cliques: Vec<(Vec<usize>, Vec<f64>)> = (0..5)
        .map(|i| (vec![i, i + 2], vec![3.0 + i as f64, -1.0, -1.0, 2.0]))
        .collect();
    let pattern =
        ProfilePattern::from_cliques(7, cliques.iter().map(|c| c.0.clone())).expect("pattern");
    let borrowed = pattern
        .assemble(cliques.iter().map(|c| Clique {
            dofs: &c.0,
            block: &c.1,
        }))
        .expect("A");
    let owned = pattern.assemble_owned(cliques).expect("A");
    assert_eq!(borrowed, owned);
}

/// Two uncoupled spring chains of `n` masses each; the second chain's springs scaled by `1 + split`.
fn twin_chains(n: usize, split: f64) -> (SymmetricProfile, SymmetricProfile) {
    let mut cliques: Vec<(Vec<usize>, Vec<f64>, Vec<f64>)> = Vec::new();
    for (block, k) in [(0, 1.0), (1, 1.0 + split)] {
        let base = block * n;
        // Fixed–fixed chain: diagonal 2k, off-diagonal −k, as element pairs plus end springs.
        for i in 0..n - 1 {
            cliques.push((
                vec![base + i, base + i + 1],
                vec![k, -k, -k, k],
                vec![0.5, 0.0, 0.0, 0.5],
            ));
        }
        cliques.push((vec![base], vec![k], vec![0.5]));
        cliques.push((vec![base + n - 1], vec![k], vec![0.5]));
    }
    let pattern = ProfilePattern::from_cliques(2 * n, cliques.iter().map(|c| c.0.as_slice()))
        .expect("pattern");
    let k = pattern
        .assemble(cliques.iter().map(|c| Clique {
            dofs: &c.0,
            block: &c.1,
        }))
        .expect("K");
    let m = pattern
        .assemble(cliques.iter().map(|c| Clique {
            dofs: &c.0,
            block: &c.2,
        }))
        .expect("M");
    (k, m)
}

#[test]
fn near_double_eigenvalues_are_bounded_as_a_cluster() {
    let n = 30;
    let split = 1e-13;
    let (k, m) = twin_chains(n, split);
    let request = EigenRequest {
        wanted: 4,
        shift: -1.0,
        deflation: Vec::new(),
        tolerance: ProblemTolerance::from_problem(1.0, 1e-12).expect("tol"),
    };
    let sol = converged(
        lowest_eigenpairs(Pencil { k: &k, m: &m }, &request, budget(), &meter()).expect("solve"),
    );
    let exact = |j: usize, s: f64| (1.0 + s) * (2.0 - 2.0 * (j as f64 * PI / (n + 1) as f64).cos());
    assert!(
        matches!(sol.completeness, Completeness::Certified { below: 4, .. }),
        "{:?}",
        sol.completeness
    );
    assert_eq!(sol.shortfall, 0);
    for p in &sol.pairs {
        let j = if p.lambda < 0.5 * (exact(1, 0.0) + exact(2, 0.0)) {
            1
        } else {
            2
        };
        assert!(
            p.lower <= exact(j, 0.0) && exact(j, split) <= p.upper,
            "{p:?} misses chain {j}"
        );
    }
    assert!(
        sol.pairs.iter().any(|p| p.bound == EigenBound::Cluster),
        "overlapping pairs must form a cluster"
    );
    eprintln!(
        "near-double: {:?}, {} pairs, shortfall {}",
        sol.completeness,
        sol.pairs.len(),
        sol.shortfall
    );
}

#[test]
fn a_missed_copy_of_a_double_eigenvalue_is_never_certified() {
    let (k, m) = twin_chains(20, 0.0);
    let request = EigenRequest {
        wanted: 1,
        shift: -1.0,
        deflation: Vec::new(),
        tolerance: ProblemTolerance::from_problem(1.0, 1e-10).expect("tol"),
    };
    let sol = converged(
        lowest_eigenpairs(Pencil { k: &k, m: &m }, &request, budget(), &meter()).expect("solve"),
    );
    // λ₁ has multiplicity two and one pair was asked for: the count below any τ above λ₁ is at least two.
    assert!(
        !matches!(sol.completeness, Completeness::Certified { .. }),
        "{:?}",
        sol.completeness
    );
}
