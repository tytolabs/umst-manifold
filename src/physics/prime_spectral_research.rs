// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Pre-registered prime-spectral research harness (Tier-1 topology_solver testbed only).
//!
//! See `docs/PRIME_SPECTRAL_PROTOCOL.md`. Guidance only — not a thermodynamic gate conjunct.
//! Enabled with **`topology-density-evolution`**.

use std::time::Instant;

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

use crate::physics::prime_spectral_filter::PrimeSpectralFilter;
use crate::physics::solvers::topology_solver::{
    pre_filter_prime_spectral, TopologySolver, TopologySolverConfig,
};
use crate::physics::topology_filter::HelmholtzFilter;

pub const TAU: f32 = 1e-3;
pub const MAX_ITERS: usize = 2000;
pub const DT: f32 = 0.35;
pub const SCHEMA: &str = "prime_spectral_research_v1";
pub const MIN_ITER_REDUCTION: f32 = 0.10;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InitialCondition {
    UniformPerturb,
    Spike,
    Random,
}

impl InitialCondition {
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Self::UniformPerturb => "IC_uniform_perturb",
            Self::Spike => "IC_spike",
            Self::Random => "IC_random",
        }
    }

    #[must_use]
    pub fn all() -> &'static [Self] {
        &[Self::UniformPerturb, Self::Spike, Self::Random]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ResearchMode {
    A,
    B,
    C,
    D,
    /// Track 4: matched-random control (equal sparsity vs von Mangoldt).
    E_RandomMatched,
}

impl ResearchMode {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::A => "A_helmholtz",
            Self::B => "B_prime",
            Self::C => "C_compose",
            Self::D => "D_coprime_stride",
            Self::E_RandomMatched => "E_random_matched",
        }
    }

    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::A,
            Self::B,
            Self::C,
            Self::D,
            Self::E_RandomMatched,
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchParams {
    pub epsilon: f32,
    pub use_mangoldt: bool,
    pub coprime_stride: Option<u32>,
}

impl Default for ResearchParams {
    fn default() -> Self {
        Self {
            epsilon: 0.05,
            use_mangoldt: true,
            coprime_stride: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModeRunResult {
    pub mode: ResearchMode,
    pub iterations: usize,
    pub converged: bool,
    pub final_l2: f32,
    pub wall_ms: u64,
    pub iter_reduction_vs_a: f32,
    pub wall_reduction_vs_a: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleRunRecord {
    pub schema: String,
    pub ic: String,
    pub seed: u64,
    pub grid_nx: usize,
    pub grid_ny: usize,
    pub params: ResearchParams,
    pub modes: Vec<ModeRunResult>,
    pub verdict_computed: ComputedVerdict,
    pub apophenia_flags: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerdictLabel {
    PatternR2,
    PatternR3,
    KillZetaTrack,
    NoiseSingleIc,
    NoiseSingleSeed,
    NoiseSingleGrid,
    NoiseCheaperOperatorOnly,
    Inconclusive,
}

impl VerdictLabel {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PatternR2 => "pattern_r2",
            Self::PatternR3 => "pattern_r3",
            Self::KillZetaTrack => "kill_zeta_track",
            Self::NoiseSingleIc => "noise_single_ic",
            Self::NoiseSingleSeed => "noise_single_seed",
            Self::NoiseSingleGrid => "noise_single_grid",
            Self::NoiseCheaperOperatorOnly => "noise_cheaper_operator_only",
            Self::Inconclusive => "inconclusive",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputedVerdict {
    pub label: VerdictLabel,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacingRunRecord {
    pub graph_nx: usize,
    pub graph_ny: usize,
    pub n_nodes: usize,
    pub ks_gue: f32,
    pub ks_poisson: f32,
    pub spacing_ratio_gue_score: f32,
    pub spacing_ratio_poisson_score: f32,
    pub prime_weighted: bool,
}

pub fn grid_edges<B: Backend<FloatElem = f32>>(
    nx: usize,
    ny: usize,
    device: &B::Device,
) -> Tensor<B, 2, Int> {
    let mut pairs: Vec<(i64, i64)> = Vec::new();
    let id = |ix: usize, iy: usize| -> i64 { (ix + iy * nx) as i64 };
    for iy in 0..ny {
        for ix in 0..(nx - 1) {
            pairs.push((id(ix, iy), id(ix + 1, iy)));
        }
    }
    for iy in 0..(ny - 1) {
        for ix in 0..nx {
            pairs.push((id(ix, iy), id(ix, iy + 1)));
        }
    }
    let ne = pairs.len();
    let mut f = Vec::with_capacity(ne * 2);
    for (a, _) in &pairs {
        f.push(*a as f32);
    }
    for (_, b) in &pairs {
        f.push(*b as f32);
    }
    Tensor::<B, 1>::from_floats(f.as_slice(), device)
        .reshape([2, ne])
        .int()
}

fn seeded_uniform(seed: u64, n: usize, lo: f32, hi: f32) -> Vec<f32> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n).map(|_| rng.gen_range(lo..hi)).collect()
}

fn seeded_noise(seed: u64, n: usize) -> Vec<f32> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

fn build_initial_rho<B: Backend<FloatElem = f32>>(
    ic: InitialCondition,
    seed: u64,
    nx: usize,
    ny: usize,
    device: &B::Device,
) -> Tensor<B, 3> {
    let n = nx * ny;
    let vals: Vec<f32> = match ic {
        InitialCondition::UniformPerturb => seeded_noise(seed, n)
            .into_iter()
            .map(|z| 0.5 + 0.05 * z)
            .collect(),
        InitialCondition::Spike => {
            let mut v = vec![0.0_f32; n];
            v[n / 2] = 1.0;
            v
        }
        InitialCondition::Random => seeded_uniform(seed, n, 0.2, 0.8),
    };
    Tensor::<B, 3>::from_data(Data::new(vals, Shape::new([1, n, 1])), device)
}

fn l2_norm<B: Backend<FloatElem = f32>>(t: Tensor<B, 3>) -> f32 {
    t.powf_scalar(2.0).sum().into_data().value[0].sqrt()
}

fn l2_diff<B: Backend<FloatElem = f32>>(a: Tensor<B, 3>, b: Tensor<B, 3>) -> f32 {
    l2_norm(a.sub(b))
}

fn helmholtz_defaults() -> HelmholtzFilter {
    HelmholtzFilter::new(2.0, 240, 1e-7)
}

fn prime_filter_from_params(params: &ResearchParams, mode: ResearchMode) -> PrimeSpectralFilter {
    match mode {
        ResearchMode::D => PrimeSpectralFilter::new(
            params.epsilon,
            true,
            params.coprime_stride.or(Some(3)),
        ),
        _ if params.use_mangoldt => PrimeSpectralFilter::new(params.epsilon, false, None),
        _ => PrimeSpectralFilter::new(0.0, false, None),
    }
}

fn step_relative_metric<B: Backend<FloatElem = f32>>(
    rho: &Tensor<B, 3>,
    rho_prev: &Tensor<B, 3>,
) -> f32 {
    l2_diff(rho.clone(), rho_prev.clone()) / l2_norm(rho.clone()).max(1e-12)
}

fn convergence_metric<B: Backend<FloatElem = f32>>(
    ic: InitialCondition,
    rho: &Tensor<B, 3>,
    rho_prev: &Tensor<B, 3>,
    target: Option<&Tensor<B, 3>>,
) -> f32 {
    match ic {
        InitialCondition::Spike => {
            let tgt = target.expect("spike IC requires mode-A target");
            l2_diff(rho.clone(), tgt.clone())
        }
        InitialCondition::UniformPerturb => {
            let step = l2_diff(rho.clone(), rho_prev.clone());
            let rel = step / l2_norm(rho.clone()).max(1e-12);
            let to_half = l2_diff(
                rho.clone(),
                Tensor::<B, 3>::full(rho.dims(), 0.5, &rho.device()),
            );
            rel.min(to_half)
        }
        InitialCondition::Random => {
            l2_diff(rho.clone(), rho_prev.clone()) / l2_norm(rho.clone()).max(1e-12)
        }
    }
}

fn is_converged(metric: f32) -> bool {
    metric < TAU
}

pub fn run_mode_a_to_star<B: Backend<FloatElem = f32>>(
    ic: InitialCondition,
    seed: u64,
    nx: usize,
    ny: usize,
    device: &B::Device,
) -> Tensor<B, 3> {
    let edges = grid_edges::<B>(nx, ny, device);
    let n = nx * ny;
    let damage = Tensor::<B, 3>::zeros([1, n, 1], device);
    let boundary = Tensor::<B, 3>::ones([1, n, 3], device);
    let policy = Tensor::<B, 2>::ones([n, 1], device);
    let helm = helmholtz_defaults();
    let mut solver = TopologySolver::new(
        build_initial_rho::<B>(ic, seed, nx, ny, device),
        TopologySolverConfig::default(),
    );
    let mut rho_prev = solver.rho.clone();
    for _ in 0..MAX_ITERS {
        solver.step_density_diffusion_filtered(
            DT,
            edges.clone(),
            damage.clone(),
            boundary.clone(),
            policy.clone(),
            |t| t,
            |t| helm.apply(t, edges.clone(), 1.0),
        );
        let metric = step_relative_metric(&solver.rho, &rho_prev);
        if is_converged(metric) {
            break;
        }
        rho_prev = solver.rho.clone();
    }
    solver.rho
}

#[allow(clippy::too_many_arguments)]
pub fn run_single_mode<B: Backend<FloatElem = f32>>(
    mode: ResearchMode,
    params: &ResearchParams,
    seed: u64,
    ic: InitialCondition,
    rho0: Tensor<B, 3>,
    edges: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    boundary: Tensor<B, 3>,
    policy: Tensor<B, 2>,
    helm: &HelmholtzFilter,
    dx: f32,
    spike_target: Option<Tensor<B, 3>>,
) -> ModeRunResult {
    let t0 = Instant::now();
    let ps = prime_filter_from_params(params, mode);
    let mut solver = TopologySolver::new(rho0, TopologySolverConfig::default());
    let mut rho_prev = solver.rho.clone();
    let mut iterations = 0_usize;
    let mut final_l2 = f32::MAX;

    for _ in 0..MAX_ITERS {
        iterations += 1;
        match mode {
            ResearchMode::A => {
                solver.step_density_diffusion_filtered(
                    DT,
                    edges.clone(),
                    damage.clone(),
                    boundary.clone(),
                    policy.clone(),
                    |t| t,
                    |t| helm.apply(t, edges.clone(), dx),
                );
            }
            ResearchMode::B if params.use_mangoldt => {
                let ps = ps.clone();
                solver.step_density_diffusion_filtered(
                    DT,
                    edges.clone(),
                    damage.clone(),
                    boundary.clone(),
                    policy.clone(),
                    move |t| pre_filter_prime_spectral(&ps, t),
                    |t| t,
                );
            }
            ResearchMode::C if params.use_mangoldt => {
                let ps = ps.clone();
                solver.step_density_diffusion_filtered(
                    DT,
                    edges.clone(),
                    damage.clone(),
                    boundary.clone(),
                    policy.clone(),
                    move |t| pre_filter_prime_spectral(&ps, t),
                    |t| helm.apply(t, edges.clone(), dx),
                );
            }
            ResearchMode::D => {
                let ps = ps.clone();
                solver.step_density_diffusion_filtered(
                    DT,
                    edges.clone(),
                    damage.clone(),
                    boundary.clone(),
                    policy.clone(),
                    move |t| pre_filter_prime_spectral(&ps, t),
                    |t| t,
                );
            }
            ResearchMode::E_RandomMatched => {
                let n = rho_prev.dims()[1];
                let weights =
                    crate::physics::prime_spectral_filter::matched_random_weight_table(
                        seed,
                        n,
                        params.epsilon,
                    );
                solver.step_density_diffusion_filtered(
                    DT,
                    edges.clone(),
                    damage.clone(),
                    boundary.clone(),
                    policy.clone(),
                    move |t| PrimeSpectralFilter::apply_weights(t, &weights, n),
                    |t| t,
                );
            }
            _ => {
                solver.step_density_diffusion_filtered(
                    DT,
                    edges.clone(),
                    damage.clone(),
                    boundary.clone(),
                    policy.clone(),
                    |t| t,
                    |t| t,
                );
            }
        }
        final_l2 = convergence_metric(ic, &solver.rho, &rho_prev, spike_target.as_ref());
        if is_converged(final_l2) {
            break;
        }
        rho_prev = solver.rho.clone();
    }

    ModeRunResult {
        mode,
        iterations,
        converged: is_converged(final_l2),
        final_l2,
        wall_ms: t0.elapsed().as_millis() as u64,
        iter_reduction_vs_a: 0.0,
        wall_reduction_vs_a: 0.0,
    }
}

pub fn run_full_record<B: Backend<FloatElem = f32>>(
    ic: InitialCondition,
    seed: u64,
    nx: usize,
    ny: usize,
    params: ResearchParams,
    device: &B::Device,
) -> SingleRunRecord {
    let n = nx * ny;
    let edges = grid_edges::<B>(nx, ny, device);
    let rho0 = build_initial_rho::<B>(ic, seed, nx, ny, device);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], device);
    let boundary = Tensor::<B, 3>::ones([1, n, 3], device);
    let policy = Tensor::<B, 2>::ones([n, 1], device);
    let helm = helmholtz_defaults();
    let spike_target = if ic == InitialCondition::Spike {
        Some(run_mode_a_to_star::<B>(ic, seed, nx, ny, device))
    } else {
        None
    };

    let mut modes: Vec<ModeRunResult> = ResearchMode::all()
        .iter()
        .map(|&mode| {
            run_single_mode::<B>(
                mode,
                &params,
                seed,
                ic,
                rho0.clone(),
                edges.clone(),
                damage.clone(),
                boundary.clone(),
                policy.clone(),
                &helm,
                1.0,
                spike_target.clone(),
            )
        })
        .collect();

    let a_iters = modes
        .iter()
        .find(|m| m.mode == ResearchMode::A)
        .map(|m| m.iterations)
        .unwrap_or(1)
        .max(1) as f32;
    let a_wall = modes
        .iter()
        .find(|m| m.mode == ResearchMode::A)
        .map(|m| m.wall_ms)
        .unwrap_or(1)
        .max(1) as f32;

    for m in &mut modes {
        m.iter_reduction_vs_a = 1.0 - (m.iterations as f32 / a_iters);
        m.wall_reduction_vs_a = 1.0 - (m.wall_ms as f32 / a_wall);
    }

    let verdict_computed = compute_r2_verdict(&modes);
    let apophenia_flags = apophenia_flags_from_modes(&modes);

    SingleRunRecord {
        schema: SCHEMA.to_string(),
        ic: ic.id().to_string(),
        seed,
        grid_nx: nx,
        grid_ny: ny,
        params,
        modes,
        verdict_computed,
        apophenia_flags,
    }
}

fn apophenia_flags_from_modes(modes: &[ModeRunResult]) -> Vec<String> {
    let mut flags = Vec::new();
    let c = modes.iter().find(|m| m.mode == ResearchMode::C);
    let a = modes.iter().find(|m| m.mode == ResearchMode::A);
    if let (Some(c), Some(a)) = (c, a) {
        if c.wall_reduction_vs_a >= MIN_ITER_REDUCTION && c.iter_reduction_vs_a < MIN_ITER_REDUCTION
        {
            flags.push(VerdictLabel::NoiseCheaperOperatorOnly.as_str().to_string());
        }
        if c.iter_reduction_vs_a >= MIN_ITER_REDUCTION && !c.converged && a.converged {
            flags.push(VerdictLabel::NoiseCheaperOperatorOnly.as_str().to_string());
        }
    }
    flags
}

#[must_use]
pub fn compute_r2_sweep_verdict(records: &[SingleRunRecord]) -> ComputedVerdict {
    let mut wins: Vec<(String, u64, usize, usize)> = Vec::new();
    let mut apophenia = Vec::new();

    for rec in records {
        let a = rec.modes.iter().find(|m| m.mode == ResearchMode::A);
        let c = rec.modes.iter().find(|m| m.mode == ResearchMode::C);
        if let (Some(a), Some(c)) = (a, c) {
            if c.converged && a.converged && c.iter_reduction_vs_a >= MIN_ITER_REDUCTION {
                wins.push((
                    rec.ic.clone(),
                    rec.seed,
                    rec.grid_nx,
                    rec.grid_ny,
                ));
            }
        }
        apophenia.extend(rec.apophenia_flags.clone());
    }

    let ics: std::collections::BTreeSet<_> = wins.iter().map(|(ic, _, _, _)| ic.clone()).collect();
    let seeds: std::collections::BTreeSet<_> = wins.iter().map(|(_, s, _, _)| *s).collect();
    let grids: std::collections::BTreeSet<_> = wins
        .iter()
        .map(|(_, _, nx, ny)| (*nx, *ny))
        .collect();

    if ics.len() < 2 {
        apophenia.push(VerdictLabel::NoiseSingleIc.as_str().to_string());
    }
    if seeds.len() < 2 {
        apophenia.push(VerdictLabel::NoiseSingleSeed.as_str().to_string());
    }
    if grids.len() < 2 {
        apophenia.push(VerdictLabel::NoiseSingleGrid.as_str().to_string());
    }

    if ics.len() >= 2 && seeds.len() >= 2 && grids.len() >= 2 && !wins.is_empty() {
        return ComputedVerdict {
            label: VerdictLabel::PatternR2,
            summary: format!(
                "pattern_r2: {} wins across {} ICs, {} seeds, {} grids",
                wins.len(),
                ics.len(),
                seeds.len(),
                grids.len()
            ),
        };
    }

    ComputedVerdict {
        label: VerdictLabel::Inconclusive,
        summary: if records.is_empty() {
            "no sweep records".to_string()
        } else {
            format!(
                "R2 sweep: no replicated pattern (wins={}, ics={}, seeds={}, grids={})",
                wins.len(),
                ics.len(),
                seeds.len(),
                grids.len()
            )
        },
    }
}

#[must_use]
pub fn compute_r2_verdict(modes: &[ModeRunResult]) -> ComputedVerdict {
    let a = modes.iter().find(|m| m.mode == ResearchMode::A);
    let c = modes.iter().find(|m| m.mode == ResearchMode::C);
    match (a, c) {
        (Some(a), Some(c))
            if c.converged && a.converged && c.iter_reduction_vs_a >= MIN_ITER_REDUCTION =>
        {
            ComputedVerdict {
                label: VerdictLabel::PatternR2,
                summary: format!(
                    "mode C iter_reduction={:.3} >= {:.2} at equal tau",
                    c.iter_reduction_vs_a, MIN_ITER_REDUCTION
                ),
            }
        }
        _ => ComputedVerdict {
            label: VerdictLabel::Inconclusive,
            summary: "R2 pattern criteria not met on this run".to_string(),
        },
    }
}

#[must_use]
pub fn graph_laplacian_matrix(nx: usize, ny: usize) -> Vec<Vec<f64>> {
    let n = nx * ny;
    let mut lap = vec![vec![0.0_f64; n]; n];
    let id = |ix: usize, iy: usize| ix + iy * nx;
    for iy in 0..ny {
        for ix in 0..nx {
            let i = id(ix, iy);
            let mut deg = 0.0_f64;
            if ix + 1 < nx {
                let j = id(ix + 1, iy);
                lap[i][j] = -1.0;
                lap[j][i] = -1.0;
                deg += 1.0;
            }
            if iy + 1 < ny {
                let j = id(ix, iy + 1);
                lap[i][j] = -1.0;
                lap[j][i] = -1.0;
                deg += 1.0;
            }
            if ix > 0 {
                deg += 1.0;
            }
            if iy > 0 {
                deg += 1.0;
            }
            lap[i][i] = deg;
        }
    }
    lap
}

#[must_use]
pub fn symmetric_eigenvalues(matrix: &[Vec<f64>]) -> Vec<f64> {
    let n = matrix.len();
    if n == 0 {
        return Vec::new();
    }
    let mut a: Vec<Vec<f64>> = matrix.to_vec();
    const MAX_SWEEPS: usize = 100;
    const TOL: f64 = 1e-12;
    for _ in 0..MAX_SWEEPS {
        let mut off = 0.0_f64;
        for i in 0..n {
            for j in (i + 1)..n {
                off += a[i][j].abs();
            }
        }
        if off < TOL {
            break;
        }
        for p in 0..n {
            for q in (p + 1)..n {
                if a[p][q].abs() < TOL {
                    continue;
                }
                let theta = 0.5 * (a[q][q] - a[p][p]) / a[p][q];
                let t = theta.signum() / (theta.abs() + (1.0 + theta * theta).sqrt());
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                let app = a[p][p];
                let aqq = a[q][q];
                let apq = a[p][q];
                a[p][p] = c * c * app - 2.0 * s * c * apq + s * s * aqq;
                a[q][q] = s * s * app + 2.0 * s * c * apq + c * c * aqq;
                a[p][q] = 0.0;
                a[q][p] = 0.0;
                for r in 0..n {
                    if r == p || r == q {
                        continue;
                    }
                    let arp = a[r][p];
                    let arq = a[r][q];
                    a[r][p] = c * arp - s * arq;
                    a[p][r] = a[r][p];
                    a[r][q] = s * arp + c * arq;
                    a[q][r] = a[r][q];
                }
            }
        }
    }
    let mut evals: Vec<f64> = (0..n).map(|i| a[i][i]).collect();
    evals.sort_by(|x, y| x.partial_cmp(y).unwrap());
    evals
}

#[must_use]
pub fn unfolded_spacings(eigenvalues: &[f64]) -> Vec<f64> {
    if eigenvalues.len() < 2 {
        return Vec::new();
    }
    let lambda_max = eigenvalues.last().copied().unwrap_or(1.0).max(1e-12);
    let n = eigenvalues.len() as f64;
    let rho_bar = n / lambda_max;
    eigenvalues
        .windows(2)
        .map(|w| (w[1] - w[0]) * rho_bar)
        .collect()
}

fn ks_distance(spacings: &[f64], null_cdf: impl Fn(f64) -> f64) -> f32 {
    if spacings.is_empty() {
        return 1.0;
    }
    let mut sorted = spacings.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len() as f64;
    let mut d_max = 0.0_f64;
    for (i, &s) in sorted.iter().enumerate() {
        let emp = (i as f64 + 1.0) / n;
        let null = null_cdf(s);
        d_max = d_max.max((emp - null).abs());
        let emp_prev = i as f64 / n;
        d_max = d_max.max((emp_prev - null).abs());
    }
    d_max as f32
}

fn gue_surmise_pdf(s: f64) -> f64 {
    if s < 0.0 {
        return 0.0;
    }
    (32.0 / std::f64::consts::PI.powi(2)) * s * s * (-4.0 * s * s / std::f64::consts::PI).exp()
}

fn gue_surmise_cdf(s: f64) -> f64 {
    if s <= 0.0 {
        return 0.0;
    }
    let upper = s.min(6.0);
    let steps = 400;
    let ds = upper / steps as f64;
    let mut acc = 0.0_f64;
    for k in 0..steps {
        let x0 = k as f64 * ds;
        let x1 = (k + 1) as f64 * ds;
        acc += 0.5 * (gue_surmise_pdf(x0) + gue_surmise_pdf(x1)) * ds;
    }
    acc.clamp(0.0, 1.0)
}

/// Nearest-neighbour spacing ratio (Atas-style, on unfolded spacings).
#[must_use]
pub fn spacing_ratios(spacings: &[f64]) -> Vec<f64> {
    spacings
        .windows(2)
        .map(|w| {
            let (a, b) = (w[0], w[1]);
            if a.abs() + b.abs() < 1e-15 {
                0.0
            } else {
                a.min(b) / a.max(b).max(1e-15)
            }
        })
        .collect()
}

const POISSON_RATIO_MEAN: f64 = 0.429;
const GUE_RATIO_MEAN: f64 = 0.602;

#[must_use]
fn ratio_mean_score(ratios: &[f64], target: f64) -> f32 {
    if ratios.is_empty() {
        return 1.0;
    }
    let mean = ratios.iter().sum::<f64>() / ratios.len() as f64;
    (mean - target).abs() as f32
}

#[must_use]
pub fn run_spacing_study(nx: usize, ny: usize, prime_weighted: bool) -> SpacingRunRecord {
    let mut lap = graph_laplacian_matrix(nx, ny);
    if prime_weighted {
        let n = nx * ny;
        for i in 0..n {
            let w = crate::physics::prime_spectral_filter::von_mangoldt_weight((i + 1) as u32);
            let scale = if w > 0.0 {
                1.0 + 0.05 * (w / (i + 1) as f32)
            } else {
                1.0
            };
            for j in 0..n {
                lap[i][j] *= scale as f64;
            }
        }
    }
    let evals = symmetric_eigenvalues(&lap);
    let spacings = unfolded_spacings(&evals);
    let ratios = spacing_ratios(&spacings);
    SpacingRunRecord {
        graph_nx: nx,
        graph_ny: ny,
        n_nodes: nx * ny,
        ks_gue: ks_distance(&spacings, gue_surmise_cdf),
        ks_poisson: ks_distance(&spacings, |s| 1.0 - (-s).exp()),
        spacing_ratio_gue_score: ratio_mean_score(&ratios, GUE_RATIO_MEAN),
        spacing_ratio_poisson_score: ratio_mean_score(&ratios, POISSON_RATIO_MEAN),
        prime_weighted,
    }
}

const R3_KS_GUE_MAX: f32 = 0.15;
const R3_KS_MARGIN: f32 = 0.05;

#[must_use]
pub fn compute_r3_verdict(records: &[SpacingRunRecord]) -> ComputedVerdict {
    let ks_hits: Vec<_> = records
        .iter()
        .filter(|r| r.ks_gue <= R3_KS_GUE_MAX && (r.ks_poisson - r.ks_gue) >= R3_KS_MARGIN)
        .collect();
    let ratio_hits: Vec<_> = records
        .iter()
        .filter(|r| r.spacing_ratio_gue_score + 0.05 < r.spacing_ratio_poisson_score)
        .collect();
    if ks_hits.len() >= 2 || ratio_hits.len() >= 2 {
        ComputedVerdict {
            label: VerdictLabel::PatternR3,
            summary: format!(
                "R3 spacing hit: ks_hits={} ratio_hits={}",
                ks_hits.len(),
                ratio_hits.len()
            ),
        }
    } else {
        ComputedVerdict {
            label: VerdictLabel::Inconclusive,
            summary: "R3 spacing criteria not met".to_string(),
        }
    }
}

#[must_use]
pub fn combined_final_verdict(r2: &ComputedVerdict, r3: &ComputedVerdict) -> ComputedVerdict {
    if r2.label == VerdictLabel::PatternR2 || r3.label == VerdictLabel::PatternR3 {
        ComputedVerdict {
            label: if r2.label == VerdictLabel::PatternR2 {
                VerdictLabel::PatternR2
            } else {
                VerdictLabel::PatternR3
            },
            summary: format!("{}; {}", r2.summary, r3.summary),
        }
    } else {
        ComputedVerdict {
            label: VerdictLabel::KillZetaTrack,
            summary: format!(
                "kill_zeta_track: no pattern_r2 and no pattern_r3 (r2={}, r3={})",
                r2.label.as_str(),
                r3.label.as_str()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn uniform_perturb_finite() {
        let dev = Default::default();
        let record = run_full_record::<B>(
            InitialCondition::UniformPerturb,
            42,
            8,
            8,
            ResearchParams::default(),
            &dev,
        );
        assert_eq!(record.schema, SCHEMA);
        assert!(record.modes.iter().all(|m| m.final_l2.is_finite()));
        assert!(record.modes.iter().all(|m| m.iterations > 0));
    }
}
