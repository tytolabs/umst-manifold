// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Verification ladder of the finite-cell hexahedral solver (`physics::solvers::finite_cell`).
//!
//! References:
//! - Free-free square plate, ν = 0.3, `λ = ω a² √(ρh/D)`: 13.47, 19.60, 24.27, 34.80 (Narita, improved Ritz, as
//!   recorded in `workspace/ops/lanes/refs/SOLVER_NUMERICS_BRIEF.md` §4; Leissa 1969 lies about 1 % above).
//! - Sandwich flexural rigidity per unit width `D = E_f t_f d²/2 + E_f t_f³/6 + E_c t_c³/12`, `d = t_c + t_f`
//!   (Allen 1969; Zenkert 1995), beam form with ν = 0.
//! - Patch test: a linear displacement field has zero nodal forces at interior nodes (Irons & Razzaque 1972).

use std::f64::consts::PI;

use umst_manifold::physics::solvers::finite_cell::analysis::{
    frequency_hz, modes, point_load, rigid_body_modes, static_displacement, surface_load,
    InertiaRelief, ModeRequest,
};
use umst_manifold::physics::solvers::finite_cell::assembly::{
    assemble, discretise, Discretisation, DiscretisationSpec, System,
};
use umst_manifold::physics::solvers::finite_cell::element::ElementKind;
use umst_manifold::physics::solvers::finite_cell::{
    isotropic_stiffness, MaterialTable, OccupancyField, Voigt6,
};
use umst_math::generalized_eigen::{Completeness, GeneralizedEigenSolution};
use umst_math::solve_combinator::{EnergyBudget, FixedJouleMeter, ProblemTolerance, SolveOutcome};

const E_AL: f64 = 70.0e9;
const NU_AL: f64 = 0.3;
const RHO_AL: f64 = 2700.0;

fn free(_: [f64; 3]) -> [bool; 3] {
    [false; 3]
}

/// A box `[lo, hi]` of material 0, or a layered box whose material follows `layers` along z.
struct Slab {
    lo: [f64; 3],
    hi: [f64; 3],
    /// `(z_top, material)` ascending; empty means one material 0.
    layers: Vec<(f64, u16)>,
    /// Whether the grid is told the layer planes (the negative control withholds them).
    conforming: bool,
}

impl OccupancyField for Slab {
    fn bounds(&self) -> ([f64; 3], [f64; 3]) {
        (self.lo, self.hi)
    }
    fn signed_distance(&self, p: [f64; 3]) -> f64 {
        (0..3)
            .map(|i| (self.lo[i] - p[i]).max(p[i] - self.hi[i]))
            .fold(f64::NEG_INFINITY, f64::max)
    }
    fn material_at(&self, p: [f64; 3]) -> Option<u16> {
        if self.signed_distance(p) > 0.0 {
            return None;
        }
        Some(
            self.layers
                .iter()
                .find(|(top, _)| p[2] <= *top)
                .map_or(0, |&(_, m)| m),
        )
    }
    fn interface_planes(&self, axis: usize) -> Vec<f64> {
        if axis == 2 && self.conforming {
            self.layers.iter().map(|&(top, _)| top).collect()
        } else {
            Vec::new()
        }
    }
}

/// A circular plate of radius `r` centred at `(cx, cy)`, thickness `[z0, z1]`, inside a square background box.
struct Disc {
    c: [f64; 2],
    r: f64,
    z: [f64; 2],
    half_box: f64,
}

impl OccupancyField for Disc {
    fn bounds(&self) -> ([f64; 3], [f64; 3]) {
        (
            [
                self.c[0] - self.half_box,
                self.c[1] - self.half_box,
                self.z[0],
            ],
            [
                self.c[0] + self.half_box,
                self.c[1] + self.half_box,
                self.z[1],
            ],
        )
    }
    fn signed_distance(&self, p: [f64; 3]) -> f64 {
        let radial = ((p[0] - self.c[0]).powi(2) + (p[1] - self.c[1]).powi(2)).sqrt() - self.r;
        radial.max(self.z[0] - p[2]).max(p[2] - self.z[1])
    }
    fn material_at(&self, p: [f64; 3]) -> Option<u16> {
        (self.signed_distance(p) <= 0.0).then_some(0)
    }
    fn interface_planes(&self, _: usize) -> Vec<f64> {
        Vec::new()
    }
}

fn table(entries: &[(Voigt6, f64)]) -> MaterialTable {
    MaterialTable::try_new(
        entries.iter().map(|e| e.0).collect(),
        entries.iter().map(|e| e.1).collect(),
    )
    .expect("table")
}

fn aluminium() -> MaterialTable {
    table(&[(isotropic_stiffness(E_AL, NU_AL), RHO_AL)])
}

fn spec(spacing: [f64; 3], theta: f64, kind: ElementKind) -> DiscretisationSpec {
    DiscretisationSpec {
        max_spacing: spacing,
        tolerance: spacing[0] * 1e-2,
        theta,
        kind,
    }
}

fn solve_modes(disc: &Discretisation, sys: &System, wanted: usize) -> GeneralizedEigenSolution {
    let request = ModeRequest {
        wanted,
        shift: -1.0,
        tolerance: ProblemTolerance::from_problem(1.0, 1e-9).expect("tol"),
    };
    let budget = EnergyBudget::from_joules_at(1.0e9, 300.0).expect("budget");
    let meter = FixedJouleMeter::from_joules(1.0e-3).expect("meter");
    match modes(disc, sys, &request, budget, &meter).expect("modes") {
        SolveOutcome::Converged { x, .. } => x,
        other => panic!("not converged: {other:?}"),
    }
}

/// Nodal forces `K u` for the linear field `u(x) = A x`.
fn forces_of_linear_field(
    disc: &Discretisation,
    sys: &System,
    a: [[f64; 3]; 3],
) -> (Vec<f64>, Vec<(usize, [f64; 3])>) {
    let nodes: Vec<(usize, [f64; 3])> =
        disc.free_nodes().map(|n| (n, disc.node_point(n))).collect();
    let mut u = vec![0.0; sys.n()];
    for &(n, p) in &nodes {
        for c in 0..3 {
            if let Some(d) = sys.dof(n, c) {
                u[d] = (0..3).map(|j| a[c][j] * p[j]).sum();
            }
        }
    }
    (sys.stiffness().mul(&u).expect("Ku"), nodes)
}

#[test]
fn patch_test_linear_field_balances_at_interior_nodes() {
    let a = [
        [1.0e-3, 2.0e-4, -3.0e-4],
        [5.0e-4, -2.0e-3, 1.0e-4],
        [-1.0e-4, 3.0e-4, 1.5e-3],
    ];
    for kind in [ElementKind::Q1, ElementKind::Q1E9] {
        // Whole cells.
        let slab = Slab {
            lo: [0.0; 3],
            hi: [0.04, 0.03, 0.02],
            layers: vec![],
            conforming: true,
        };
        let disc =
            discretise(&slab, &aluminium(), &spec([0.01, 0.01, 0.01], 0.0, kind)).expect("disc");
        let sys = assemble(&disc, &free).expect("sys");
        let (f, nodes) = forces_of_linear_field(&disc, &sys, a);
        let scale = f.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        for (n, p) in &nodes {
            let interior = (0..3).all(|i| p[i] > slab.lo[i] + 1e-9 && p[i] < slab.hi[i] - 1e-9);
            if interior {
                (0..3)
                    .filter_map(|c| sys.dof(*n, c))
                    .for_each(|d| assert!(f[d].abs() <= 1e-10 * scale, "{kind:?} {}", f[d]));
            }
        }
        // Cut cells: a disc whose interior nodes see clipped cells around them.
        let disc_field = Disc {
            c: [0.0, 0.0],
            r: 0.037,
            z: [0.0, 0.01],
            half_box: 0.04,
        };
        let disc = discretise(
            &disc_field,
            &aluminium(),
            &spec([0.01, 0.01, 0.005], 0.0, kind),
        )
        .expect("disc");
        let sys = assemble(&disc, &free).expect("sys");
        let (f, nodes) = forces_of_linear_field(&disc, &sys, a);
        let scale = f.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        for (n, p) in &nodes {
            // A node whose eight surrounding cells lie wholly inside the solid is interior.
            let inside = (0..8).all(|k| {
                let q = [
                    p[0] + if k & 1 == 0 { -0.0101 } else { 0.0101 },
                    p[1] + if k & 2 == 0 { -0.0101 } else { 0.0101 },
                    0.005,
                ];
                disc_field.signed_distance(q) < 0.0
            });
            if inside && p[2] > 1e-9 && p[2] < 0.01 - 1e-9 {
                (0..3)
                    .filter_map(|c| sys.dof(*n, c))
                    .for_each(|d| assert!(f[d].abs() <= 1e-10 * scale, "{kind:?} cut {}", f[d]));
            }
        }
    }
}

#[test]
fn rigid_body_modes_lie_in_the_stiffness_nullspace_on_a_cut_body() {
    let disc_field = Disc {
        c: [0.003, -0.002],
        r: 0.05,
        z: [0.0, 0.008],
        half_box: 0.055,
    };
    for theta in [0.0, 0.1] {
        let disc = discretise(
            &disc_field,
            &aluminium(),
            &spec([0.01, 0.01, 0.004], theta, ElementKind::Q1E9),
        )
        .expect("disc");
        let sys = assemble(&disc, &free).expect("sys");
        let k_scale = sys
            .stiffness()
            .diagonal()
            .iter()
            .fold(0.0_f64, |m, v| m.max(v.abs()));
        for (r, v) in rigid_body_modes(&disc, &sys).iter().enumerate() {
            let kv = sys.stiffness().mul(v).expect("Kv");
            let v_scale = v.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
            let worst = kv.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
            assert!(
                worst <= 1e-9 * k_scale * v_scale,
                "theta {theta} rigid {r}: |Kv| = {worst}"
            );
        }
    }
}

#[test]
fn mass_properties_are_exact_on_a_box_and_converge_on_a_disc() {
    let (a, b, c) = (0.06, 0.04, 0.01);
    let slab = Slab {
        lo: [0.0; 3],
        hi: [a, b, c],
        layers: vec![],
        conforming: true,
    };
    let disc = discretise(
        &slab,
        &aluminium(),
        &spec([0.01, 0.01, 0.005], 0.0, ElementKind::Q1),
    )
    .expect("disc");
    let mp = disc.mass_properties();
    let m = RHO_AL * a * b * c;
    assert!((mp.mass - m).abs() <= 1e-12 * m);
    assert!((mp.centroid[0] - a / 2.0).abs() < 1e-12 && (mp.centroid[1] - b / 2.0).abs() < 1e-12);
    let izz = m * (a * a + b * b) / 12.0;
    assert!(
        (mp.inertia[2][2] - izz).abs() <= 1e-10 * izz,
        "Izz {} vs {izz}",
        mp.inertia[2][2]
    );
    // Lumped row sums keep the total mass: the z-components sum to m.
    let sys = assemble(&disc, &free).expect("sys");
    let lumped = sys.lumped_mass();
    let mz: f64 = disc
        .free_nodes()
        .filter_map(|n| sys.dof(n, 2))
        .map(|d| lumped[d])
        .sum();
    assert!((mz - m).abs() <= 1e-12 * m, "lumped {mz} vs {m}");
    // A disc's area converges to πR² as the cells shrink.
    let r = 0.05;
    let errors: Vec<f64> = [0.02, 0.01, 0.005]
        .iter()
        .map(|&h| {
            let f = Disc {
                c: [0.0013, 0.0007],
                r,
                z: [0.0, 0.004],
                half_box: 0.06,
            };
            let disc = discretise(&f, &aluminium(), &spec([h, h, 0.004], 0.0, ElementKind::Q1))
                .expect("disc");
            (disc.mass_properties().mass - RHO_AL * PI * r * r * 0.004).abs()
                / (RHO_AL * PI * r * r * 0.004)
        })
        .collect();
    assert!(errors[2] < 1e-3, "disc mass error {errors:?}");
    assert!(
        errors[2] <= errors[0],
        "disc mass error does not fall: {errors:?}"
    );
}

fn narita_lambdas(kind: ElementKind, n_plane: usize) -> Vec<f64> {
    let (a, h) = (0.5, 0.01);
    let slab = Slab {
        lo: [0.0; 3],
        hi: [a, a, h],
        layers: vec![],
        conforming: true,
    };
    let s = a / n_plane as f64;
    let disc = discretise(&slab, &aluminium(), &spec([s, s, h / 2.0], 0.0, kind)).expect("disc");
    let sys = assemble(&disc, &free).expect("sys");
    let sol = solve_modes(&disc, &sys, 5);
    assert!(
        matches!(sol.completeness, Completeness::Certified { below: 11, .. }),
        "{:?}",
        sol.completeness
    );
    let d = E_AL * h.powi(3) / (12.0 * (1.0 - NU_AL * NU_AL));
    sol.pairs
        .iter()
        .map(|p| p.lambda.sqrt() * a * a * (RHO_AL * h / d).sqrt())
        .collect()
}

#[test]
fn free_square_plate_matches_narita_and_q1_bounds_from_above() {
    let reference = [13.47, 19.60, 24.27, 34.80];
    let q1e9 = narita_lambdas(ElementKind::Q1E9, 20);
    let q1 = narita_lambdas(ElementKind::Q1, 20);
    eprintln!("Q1E9 {q1e9:?}\nQ1   {q1:?}");
    for (i, r) in reference.iter().enumerate() {
        assert!(
            (q1e9[i] - r).abs() <= 0.02 * r,
            "Q1E9 mode {i}: {} vs {r}",
            q1e9[i]
        );
        assert!(
            q1[i] >= q1e9[i],
            "Q1 mode {i} must lie above Q1E9 (locking stiffens)"
        );
    }
}

#[test]
fn disc_modes_do_not_depend_on_the_grid_offset() {
    let base = |offset: f64| -> Vec<f64> {
        let f = Disc {
            c: [offset, 0.37 * offset],
            r: 0.1,
            z: [0.0, 0.004],
            half_box: 0.11,
        };
        let disc = discretise(
            &f,
            &aluminium(),
            &spec([0.01, 0.01, 0.002], 0.1, ElementKind::Q1E9),
        )
        .expect("disc");
        let sys = assemble(&disc, &free).expect("sys");
        solve_modes(&disc, &sys, 4)
            .pairs
            .iter()
            .map(|p| frequency_hz(p.lambda))
            .collect()
    };
    let (f0, f1) = (base(0.0), base(0.0037));
    eprintln!("disc f0 {f0:?}\ndisc f1 {f1:?}");
    for i in 0..4 {
        assert!(
            (f0[i] - f1[i]).abs() <= 0.01 * f0[i],
            "mode {i}: {} vs {}",
            f0[i],
            f1[i]
        );
    }
}

#[test]
fn slivers_make_no_spurious_low_modes_with_aggregation() {
    // The plate edge sits a hair inside a cell, leaving a sliver column of solid fraction 1e-4.
    let s = 0.01;
    let slab = Slab {
        lo: [0.0; 3],
        hi: [0.2 + s * 1e-4, 0.1, 0.004],
        layers: vec![],
        conforming: true,
    };
    let field = SliverBox {
        inner: slab,
        grid_hi: 0.21,
    };
    let disc = discretise(
        &field,
        &aluminium(),
        &spec([s, s, 0.002], 0.1, ElementKind::Q1E9),
    )
    .expect("disc");
    assert!(
        disc.diagnostics().aggregated_nodes > 0,
        "{:?}",
        disc.diagnostics()
    );
    let sys = assemble(&disc, &free).expect("sys");
    let sol = solve_modes(&disc, &sys, 3);
    // Reference without the sliver.
    let clean = Slab {
        lo: [0.0; 3],
        hi: [0.2, 0.1, 0.004],
        layers: vec![],
        conforming: true,
    };
    let disc_c = discretise(
        &clean,
        &aluminium(),
        &spec([s, s, 0.002], 0.1, ElementKind::Q1E9),
    )
    .expect("disc");
    let sys_c = assemble(&disc_c, &free).expect("sys");
    let sol_c = solve_modes(&disc_c, &sys_c, 3);
    for i in 0..3 {
        let (a, b) = (sol.pairs[i].lambda, sol_c.pairs[i].lambda);
        assert!(
            (a - b).abs() <= 0.005 * b,
            "mode {i}: sliver {a} vs clean {b}"
        );
    }
}

/// A box inside a larger grid extent along x, so the solid ends inside the last cell.
struct SliverBox {
    inner: Slab,
    grid_hi: f64,
}

impl OccupancyField for SliverBox {
    fn bounds(&self) -> ([f64; 3], [f64; 3]) {
        let (lo, mut hi) = self.inner.bounds();
        hi[0] = self.grid_hi;
        (lo, hi)
    }
    fn signed_distance(&self, p: [f64; 3]) -> f64 {
        self.inner.signed_distance(p)
    }
    fn material_at(&self, p: [f64; 3]) -> Option<u16> {
        self.inner.material_at(p)
    }
    fn interface_planes(&self, axis: usize) -> Vec<f64> {
        self.inner.interface_planes(axis)
    }
}

/// Four-point bending of a sandwich strip: `D b = M / κ` in the constant-moment span.
fn sandwich_rigidity(conforming: bool, t_f: f64, dz: f64) -> (f64, f64) {
    let (len, b, t_c) = (0.9, 0.02, 0.014);
    let (e_f, e_c, g_c) = (50.0e9, 0.2e9, 0.05e9);
    let face = isotropic_stiffness(e_f, 0.0);
    // Core: isotropic stiffness with a separate shear modulus (ν = 0 keeps beam theory exact).
    let mut core = isotropic_stiffness(e_c, 0.0);
    (3..6).for_each(|i| core[i][i] = g_c);
    let mats = table(&[(face, 1600.0), (core, 80.0)]);
    let z0 = 0.0;
    let layers = vec![
        (z0 + t_f, 0),
        (z0 + t_f + t_c, 1),
        (z0 + 2.0 * t_f + t_c, 0),
    ];
    let slab = Slab {
        lo: [0.0, 0.0, z0],
        hi: [len, b, z0 + 2.0 * t_f + t_c],
        layers,
        conforming,
    };
    let disc = discretise(
        &slab,
        &mats,
        &spec([0.005, 0.005, dz], 0.0, ElementKind::Q1E9),
    )
    .expect("disc");
    let (lo, hi) = slab.bounds();
    // Supports: the bottom lines at x = 0 and x = L carry z; the corner (0, 0) fixes x and y, and the corner
    // (L, 0) fixes y, which removes the rotation about z without restraining the span.
    let sup = move |p: [f64; 3]| -> [bool; 3] {
        let bottom = (p[2] - lo[2]).abs() < 1e-12;
        let end0 = p[0].abs() < 1e-12;
        let end1 = (p[0] - hi[0]).abs() < 1e-12;
        let edge = bottom && p[1].abs() < 1e-12;
        [
            edge && end0,
            edge && (end0 || end1),
            bottom && (end0 || end1),
        ]
    };
    let sys = assemble(&disc, &sup).expect("sys");
    // Loads P/2 at x = L/3 and 2L/3 on the top face, spread over the width.
    let p_total = 100.0;
    let width_points = 5;
    let f = [len / 3.0, 2.0 * len / 3.0]
        .iter()
        .fold(vec![0.0; sys.n()], |f, &x| {
            (0..width_points).fold(f, |mut f, k| {
                let y = b * (k as f64 + 0.5) / width_points as f64;
                let fl = point_load(
                    &disc,
                    &sys,
                    [x, y, hi[2]],
                    [0.0, 0.0, -p_total / 2.0 / width_points as f64],
                )
                .expect("load");
                f.iter_mut().zip(fl).for_each(|(a, v)| *a += v);
                f
            })
        });
    let u = static_displacement(&sys, &f).expect("u");
    let w = |x: f64| -> f64 {
        (0..width_points)
            .map(|k| {
                disc.value_at(
                    &sys,
                    &u,
                    [x, b * (k as f64 + 0.5) / width_points as f64, lo[2]],
                )
                .expect("w")[2]
            })
            .sum::<f64>()
            / width_points as f64
    };
    let s = 0.04;
    let kappa = (w(len / 2.0 - s) - 2.0 * w(len / 2.0) + w(len / 2.0 + s)) / (s * s);
    let moment = p_total / 2.0 * len / 3.0;
    let d_fe = moment / kappa.abs() / b;
    let d = t_c + t_f;
    let d_ref = e_f * t_f * d * d / 2.0 + e_f * t_f.powi(3) / 6.0 + e_c * t_c.powi(3) / 12.0;
    (d_fe, d_ref)
}

#[test]
fn sandwich_rigidity_from_conforming_layers_and_a_negative_control() {
    // Q1E9 is exact in pure bending on whole cells, so conforming layers recover D to rounding.
    let conforming: Vec<f64> = [0.0005, 0.0006, 0.001]
        .iter()
        .map(|&t_f| {
            let (d_fe, d_ref) = sandwich_rigidity(true, t_f, 0.004);
            let err = (d_fe - d_ref) / d_ref;
            eprintln!("t_f {t_f}: D_fe {d_fe:.6e} D_ref {d_ref:.6e} err {err:.3e}");
            assert!(err.abs() <= 1e-6, "t_f {t_f}: {d_fe} vs {d_ref}");
            err.abs()
        })
        .collect();
    // The same 4 mm cells without the layer planes: faces straddle cells. The material-aware octree and the
    // incompatible modes keep pure bending within about 1 % (8 Oct 2026: 0.89 %), but the error rises by four
    // orders of magnitude over the conforming grid; transverse-shear-dominated responses lose more (research brief
    // of 8 Oct 2026: 1 to 25 % in modes). The conforming planes are what make the answer exact.
    let (d_fe, d_ref) = sandwich_rigidity(false, 0.0006, 0.004);
    let err = ((d_fe - d_ref) / d_ref).abs();
    eprintln!("negative control: D_fe {d_fe:.6e} D_ref {d_ref:.6e} err {err:.3e}");
    assert!(
        err > 1e-3 && err > 1000.0 * conforming[1],
        "straddled faces must miss D by three orders more: {err}"
    );
}

#[test]
fn orthotropic_shear_slots_follow_the_voigt_order() {
    let (g12, g23, g13) = (3.0e9, 1.0e9, 2.0e9);
    let mut d = isotropic_stiffness(10.0e9, 0.2);
    d[3][3] = g12;
    d[4][4] = g23;
    d[5][5] = g13;
    let mats = table(&[(d, 1500.0)]);
    let slab = Slab {
        lo: [0.0; 3],
        hi: [0.02, 0.02, 0.02],
        layers: vec![],
        conforming: true,
    };
    let disc = discretise(
        &slab,
        &mats,
        &spec([0.01, 0.01, 0.01], 0.0, ElementKind::Q1),
    )
    .expect("disc");
    let sys = assemble(&disc, &free).expect("sys");
    let vol = 0.02_f64.powi(3);
    let gamma = 1e-3;
    // u = γ y e_x (γ_xy), v = γ z e_y (γ_yz), w = γ x e_z (γ_xz): energy ½ G γ² V.
    for (a, g) in [
        ([[0.0, gamma, 0.0], [0.0; 3], [0.0; 3]], g12),
        ([[0.0; 3], [0.0, 0.0, gamma], [0.0; 3]], g23),
        ([[0.0; 3], [0.0; 3], [gamma, 0.0, 0.0]], g13),
    ] {
        let (ku, nodes) = forces_of_linear_field(&disc, &sys, a);
        let u: Vec<f64> = {
            let mut u = vec![0.0; sys.n()];
            for &(n, p) in &nodes {
                for c in 0..3 {
                    if let Some(dd) = sys.dof(n, c) {
                        u[dd] = (0..3).map(|j| a[c][j] * p[j]).sum();
                    }
                }
            }
            u
        };
        let energy: f64 = u.iter().zip(&ku).map(|(x, y)| x * y).sum::<f64>() / 2.0;
        let expected = g * gamma * gamma * vol / 2.0;
        assert!(
            (energy - expected).abs() <= 1e-10 * expected,
            "G {g}: energy {energy} vs {expected}"
        );
    }
}

#[test]
fn apparent_mass_tends_to_the_rigid_mass_at_the_centre() {
    let slab = Slab {
        lo: [0.0; 3],
        hi: [0.2, 0.1, 0.006],
        layers: vec![],
        conforming: true,
    };
    let disc = discretise(
        &slab,
        &aluminium(),
        &spec([0.01, 0.01, 0.003], 0.0, ElementKind::Q1E9),
    )
    .expect("disc");
    let sys = assemble(&disc, &free).expect("sys");
    let sol = solve_modes(&disc, &sys, 4);
    let relief = InertiaRelief::new(&disc, &sys).expect("relief");
    let m = disc.mass_properties().mass;
    let centre = disc.mass_properties().centroid;
    let low = relief
        .apparent_mass(&sol, centre, [0.0, 0.0, 1.0], 1.0)
        .expect("am");
    assert!(
        (low.rigid_mass - m).abs() <= 1e-9 * m,
        "rigid mass at centre {} vs {m}",
        low.rigid_mass
    );
    assert!(
        (low.apparent_mass - m).abs() <= 1e-3 * m,
        "apparent mass at 1 rad/s {} vs {m}",
        low.apparent_mass
    );
    assert!(
        low.compliance > 0.0,
        "elastic compliance must be positive: {}",
        low.compliance
    );
    // Off centre the rigid mass falls (rotary inertia adds to the inverse).
    let corner = relief
        .apparent_mass(&sol, [0.19, 0.09, 0.006], [0.0, 0.0, 1.0], 1.0)
        .expect("am");
    assert!(
        corner.rigid_mass < m / 2.0,
        "corner rigid mass {}",
        corner.rigid_mass
    );
}

/// Hard simple support on the lateral faces of a plate `[0, a]² × [0, h]`: `w = 0` there; `u, v` fixed at the
/// origin corner and `v` at `(a, 0)` to remove the in-plane rigid motions.
fn ssss(a: f64) -> impl Fn([f64; 3]) -> [bool; 3] {
    move |p: [f64; 3]| {
        let on = |x: f64, t: f64| (x - t).abs() < 1e-12;
        let lateral = on(p[0], 0.0) || on(p[0], a) || on(p[1], 0.0) || on(p[1], a);
        let origin = on(p[0], 0.0) && on(p[1], 0.0) && on(p[2], 0.0);
        let far = on(p[0], a) && on(p[1], 0.0) && on(p[2], 0.0);
        [origin, origin || far, lateral]
    }
}

/// Centre deflection of a simply supported square Mindlin plate under uniform load: Navier (Kirchhoff) plus the
/// Marcus moment over the transverse shear stiffness `κ G h`, `κ = 5/6` (Timoshenko & Woinowsky-Krieger 1959;
/// Reddy, *Theory and Analysis of Elastic Plates*, 2007, §10.2).
fn ssss_centre_mindlin(q: f64, a: f64, h: f64, e: f64, nu: f64) -> f64 {
    let d = e * h.powi(3) / (12.0 * (1.0 - nu * nu));
    let kgh = 5.0 / 6.0 * e / (2.0 * (1.0 + nu)) * h;
    (0..100)
        .flat_map(|i| (0..100).map(move |j| (2 * i + 1, 2 * j + 1)))
        .map(|(m, n)| {
            let (mf, nf) = (m as f64, n as f64);
            let sign = if ((m - 1) / 2 + (n - 1) / 2) % 2 == 0 {
                1.0
            } else {
                -1.0
            };
            let s = mf * mf + nf * nf;
            let kirchhoff = 16.0 * q * a.powi(4) / (PI.powi(6) * d * mf * nf * s * s);
            let shear = 16.0 * q * a * a / (PI.powi(4) * mf * nf * s) / kgh;
            sign * (kirchhoff + shear)
        })
        .sum()
}

fn ssss_plate(n: usize, kind: ElementKind) -> (f64, f64) {
    let (a, h, q) = (0.5, 0.01, 1.0e3);
    let slab = Slab {
        lo: [0.0; 3],
        hi: [a, a, h],
        layers: vec![],
        conforming: true,
    };
    let s = a / n as f64;
    let disc = discretise(&slab, &aluminium(), &spec([s, s, h / 2.0], 0.0, kind)).expect("disc");
    let sup = ssss(a);
    let sys = assemble(&disc, &sup).expect("sys");
    let f = surface_load(&disc, &sys, &slab, h, None, [0.0, 0.0, -q], 1e-9).expect("load");
    let u = static_displacement(&sys, &f).expect("u");
    let w = -disc
        .value_at(&sys, &u, [a / 2.0, a / 2.0, h / 2.0])
        .expect("w")[2];
    (w, ssss_centre_mindlin(q, a, h, E_AL, NU_AL))
}

#[test]
fn surface_load_totals_the_traction_on_the_face_and_on_a_patch() {
    let slab = Slab {
        lo: [0.0; 3],
        hi: [0.2, 0.1, 0.01],
        layers: vec![],
        conforming: true,
    };
    let disc = discretise(
        &slab,
        &aluminium(),
        &spec([0.01, 0.01, 0.005], 0.0, ElementKind::Q1),
    )
    .expect("disc");
    let sys = assemble(&disc, &free).expect("sys");
    let fz = |f: &[f64]| -> f64 {
        disc.free_nodes()
            .filter_map(|n| sys.dof(n, 2))
            .map(|d| f[d])
            .sum()
    };
    let q = 2.5e3;
    let whole = surface_load(&disc, &sys, &slab, 0.01, None, [0.0, 0.0, q], 1e-9).expect("load");
    assert!(
        (fz(&whole) - q * 0.2 * 0.1).abs() <= 1e-12 * q * 0.02,
        "face total {}",
        fz(&whole)
    );
    let r = 0.023;
    let pad = move |p: [f64; 2]| ((p[0] - 0.0731).powi(2) + (p[1] - 0.0517).powi(2)).sqrt() - r;
    let patch =
        surface_load(&disc, &sys, &slab, 0.01, Some(&pad), [0.0, 0.0, q], 1e-4).expect("load");
    let exact = q * PI * r * r;
    assert!(
        (fz(&patch) - exact).abs() <= 2e-3 * exact,
        "patch total {} vs {exact}",
        fz(&patch)
    );
}

#[test]
fn kirchhoff_gate_simply_supported_plate_centre_deflection() {
    let (w, w_ref) = ssss_plate(20, ElementKind::Q1E9);
    let (w_q1, _) = ssss_plate(20, ElementKind::Q1);
    eprintln!("SSSS a/h = 50: Q1E9 {w:.6e}, Q1 {w_q1:.6e}, Navier–Mindlin {w_ref:.6e}");
    assert!(
        (w - w_ref).abs() <= 0.02 * w_ref,
        "Q1E9 centre deflection {w} vs {w_ref}"
    );
    assert!(w_q1 < w, "Q1 locks: it must deflect less than Q1E9");
}

#[test]
fn simply_supported_plate_frequencies_match_leissa() {
    // Two grids, h and h/2, against Leissa's Kirchhoff λ = π²(m² + n²) with the first-order Mindlin shear and rotary-inertia factor
    // 1/√(1 + k²(D/(κGh) + h²/12)), k² = π²(m² + n²)/a², κ = 5/6.
    let (a, h): (f64, f64) = (0.5, 0.005);
    let d = E_AL * h.powi(3) / (12.0 * (1.0 - NU_AL * NU_AL));
    let kgh = 5.0 / 6.0 * E_AL / (2.0 * (1.0 + NU_AL)) * h;
    let lambdas = |n: usize| -> Vec<f64> {
        let slab = Slab {
            lo: [0.0; 3],
            hi: [a, a, h],
            layers: vec![],
            conforming: true,
        };
        let s = a / n as f64;
        let disc = discretise(
            &slab,
            &aluminium(),
            &spec([s, s, h / 2.0], 0.0, ElementKind::Q1E9),
        )
        .expect("disc");
        let sup = ssss(a);
        let sys = assemble(&disc, &sup).expect("sys");
        let sol = solve_modes(&disc, &sys, 4);
        assert!(
            matches!(sol.completeness, Completeness::Certified { below: 4, .. }),
            "{:?}",
            sol.completeness
        );
        sol.pairs
            .iter()
            .map(|p| p.lambda.sqrt() * a * a * (RHO_AL * h / d).sqrt())
            .collect()
    };
    let (coarse, fine) = (lambdas(20), lambdas(40));
    for (i, (m, n)) in [(1_u32, 1_u32), (1, 2), (2, 1), (2, 2)]
        .into_iter()
        .enumerate()
    {
        let s = f64::from(m * m + n * n);
        let k2 = PI * PI * s / (a * a);
        let reference = PI * PI * s / (1.0 + k2 * (d / kgh + h * h / 12.0)).sqrt();
        eprintln!(
            "SSSS ({m},{n}): h {:.4} h/2 {:.4} reference {reference:.4} (fine err {:+.3} %)",
            coarse[i],
            fine[i],
            100.0 * (fine[i] - reference) / reference
        );
        // The 20-to-40 step converges faster than second order here (8 Oct 2026: about order 3.5 on (2,2)), so a
        // p = 2 extrapolation would overshoot; the gate is the fine-grid error and its fall from the coarse grid.
        assert!(
            (fine[i] - reference).abs() <= 0.005 * reference,
            "mode ({m},{n}): {} vs {reference}",
            fine[i]
        );
        assert!(
            (fine[i] - reference).abs() < (coarse[i] - reference).abs(),
            "mode ({m},{n}) must converge"
        );
    }
}

#[test]
fn simply_supported_deflection_converges_at_second_order() {
    let errors: Vec<f64> = [6, 12, 24]
        .iter()
        .map(|&n| {
            let (w, w_ref) = ssss_plate(n, ElementKind::Q1E9);
            ((w - w_ref) / w_ref).abs()
        })
        .collect();
    eprintln!("SSSS errors {errors:?}");
    let order = (errors[1] / errors[2]).log2();
    assert!(
        errors[2] < errors[1] && errors[1] < errors[0],
        "errors must fall: {errors:?}"
    );
    assert!(order > 1.5, "observed order {order}");
}
