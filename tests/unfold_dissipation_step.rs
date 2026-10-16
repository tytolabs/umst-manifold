// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The unfold step on the passive route (cell UNFOLD-DISSIPATION-STEP).
//!
//! The model is `ψ(x) = k·x²/2`, `φ(v) = η·v²/2` (a [`PassivePhi`] quadratic form),
//! `w(x) = f·x`; its proximal step is `x₊ = (f + η·x₀/τ)/(k + η/τ)`. The statements checked
//! are those of `UMST.UnfoldProxStepLaws` (L3) and `UMST.UnfoldKleisli.bind_assoc` (L1),
//! whose four-language parity rows are read from umst-formal.

use std::path::Path;

use ordered_float::NotNan;
use umst_cartridge_api::convex_phi::PassivePhi;
use umst_manifold::solve::{
    admit, admitted_step, dissipative_unfold, Admitted, DissipationStepRefuse, DissipativeStep,
    StepKind,
};
use umst_math::solve_combinator::{
    CombinatorRefuse, EnergyBudget, FixedJouleMeter, ProblemProgressWindow, ProblemTolerance,
    UnfoldStop,
};

/// Rounding bound of the gate for these binary64 balances (Policy).
const TOLERANCE: f64 = 1e-12;

struct Quadratic {
    stiffness: f64,
    viscosity: f64,
    force: f64,
    phi: PassivePhi,
    kind: StepKind,
    /// Added to the proximal proposal; zero for the minimiser.
    overshoot: f64,
}

impl Quadratic {
    fn new(stiffness: f64, viscosity: f64, force: f64) -> Self {
        Self {
            stiffness,
            viscosity,
            force,
            phi: PassivePhi::quad(1, &[viscosity / 2.0]).expect("psd"),
            kind: StepKind::Proximal,
            overshoot: 0.0,
        }
    }
}

impl DissipativeStep for Quadratic {
    fn free_energy(&self, x: &[f64]) -> f64 {
        self.stiffness * x[0] * x[0] / 2.0
    }
    fn dissipation(&self) -> &PassivePhi {
        &self.phi
    }
    fn work(&self, x: &[f64]) -> f64 {
        self.force * x[0]
    }
    fn kind(&self) -> StepKind {
        self.kind
    }
    fn propose(&self, x0: &[f64], tau: f64) -> Result<Vec<f64>, CombinatorRefuse> {
        let rate = self.viscosity / tau;
        Ok(vec![
            (self.force + rate * x0[0]) / (self.stiffness + rate) + self.overshoot,
        ])
    }
}

fn stop() -> UnfoldStop {
    UnfoldStop::new(
        ProblemTolerance::from_problem(1.0, 1e-9).expect("tolerance"),
        EnergyBudget::from_joules_at(1e-12, 293.15).expect("budget"),
        ProblemProgressWindow::from_krylov_restart(1).expect("window"),
        1.0,
    )
    .expect("stop")
}

fn meter() -> FixedJouleMeter {
    FixedJouleMeter::from_joules(1e-18).expect("meter")
}

#[test]
fn a_proximal_step_satisfies_the_discrete_inequality() {
    // prox_step_le over a grid of models, start points and step lengths.
    for k in [0.5, 1.0, 3.0] {
        for eta in [0.25, 1.0, 4.0] {
            for f in [-2.0, 0.0, 1.5] {
                for x0 in [-1.0, 0.0, 2.0] {
                    for tau in [0.1, 1.0, 10.0] {
                        let m = Quadratic::new(k, eta, f);
                        let step = admitted_step(&m, vec![x0], tau, TOLERANCE).expect("admitted");
                        let b = step.steps()[0];
                        assert!(b.dissipated() >= 0.0);
                        assert!(b.delta_psi() + b.dissipated() <= b.work_in() + TOLERANCE * tau);
                    }
                }
            }
        }
    }
}

#[test]
fn an_anti_dissipative_candidate_is_refused_by_the_core_gate() {
    // prox_step_refused: an overshoot past the minimiser climbs ψ − w.
    let mut m = Quadratic::new(1.0, 1.0, 0.0);
    m.overshoot = 5.0;
    match admitted_step(&m, vec![1.0], 1.0, TOLERANCE) {
        Err(DissipationStepRefuse::Rejected { outcome, balance }) => {
            assert!(!outcome.is_accepted());
            assert!(balance.delta_psi() + balance.dissipated() > balance.work_in());
        }
        other => panic!("expected the gate's refusal, got {other:?}"),
    }
    // The same candidate checked directly.
    assert!(matches!(
        admit(&m, &[1.0], &[6.0], 1.0, TOLERANCE),
        Err(DissipationStepRefuse::Rejected { .. })
    ));
}

#[test]
fn a_jko_step_of_a_dirac_mass_is_the_proximal_step_with_unit_viscosity() {
    let mut m = Quadratic::new(2.0, 1.0, 1.0);
    m.kind = StepKind::Jko;
    let step = admitted_step(&m, vec![3.0], 0.5, TOLERANCE).expect("admitted");
    assert_eq!(step.steps()[0].kind(), StepKind::Jko);
    // τ·φ((x₊ − x₀)/τ) = |x₊ − x₀|²/(2τ), the squared Wasserstein distance over 2τ.
    let dx = step.value()[0] - 3.0;
    assert!((step.steps()[0].dissipated() - dx * dx / (2.0 * 0.5)).abs() <= 1e-12);
}

#[test]
fn bind_is_associative_with_unit_identity() {
    let a = Quadratic::new(1.0, 1.0, 1.0);
    let b = Quadratic::new(2.0, 0.5, -1.0);
    let c = Quadratic::new(0.5, 2.0, 0.5);
    let f = |x: Vec<f64>| admitted_step(&a, x, 0.5, TOLERANCE);
    let g = |x: Vec<f64>| admitted_step(&b, x, 0.5, TOLERANCE);
    let h = |x: Vec<f64>| admitted_step(&c, x, 0.5, TOLERANCE);
    let m = || Admitted::unit(vec![0.25]);
    let left = m()
        .and_then(f)
        .and_then(|y| y.and_then(g))
        .and_then(|z| z.and_then(h));
    let right = m().and_then(|x| f(x)?.and_then(|y| g(y)?.and_then(h)));
    assert_eq!(left, right);
    assert_eq!(m().and_then(f), f(vec![0.25]));
    assert_eq!(
        f(vec![0.25]).and_then(|s| s.and_then(|x| Ok(Admitted::unit(x)))),
        f(vec![0.25])
    );
}

#[test]
fn the_unfold_converges_with_every_part_admitted_and_stops_on_a_refused_part() {
    let loading = Quadratic::new(0.0, 1.0, 1.0);
    let spring = Quadratic::new(1.0, 1.0, 0.0);
    let residual = |x: &[f64]| {
        NotNan::new((x[0] - 1.0).abs()).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
    };
    let plain = Quadratic::new(1.0, 1.0, 1.0);
    let done = dissipative_unfold(
        &[&plain],
        residual,
        vec![0.0],
        1.0,
        TOLERANCE,
        stop(),
        &meter(),
    )
    .expect("converges to f/k");
    assert!((done.value().value()[0] - 1.0).abs() <= 1e-9);
    assert!(done.chain().meets());
    let s = done.value().steps();
    let (dpsi, diss, work) = s.iter().fold((0.0, 0.0, 0.0), |(a, b, c), x| {
        (a + x.delta_psi(), b + x.dissipated(), c + x.work_in())
    });
    assert!(dpsi + diss <= work + TOLERANCE * s.len() as f64);

    // Splitting: the force is applied by one part and the stiffness by the other.
    let split = dissipative_unfold(
        &[&loading, &spring],
        residual,
        vec![0.0],
        1.0,
        TOLERANCE,
        stop(),
        &meter(),
    );
    // With τ = 1 the loading part moves x by +1 and the spring part halves it, so the
    // split map x ↦ (x + 1)/2 contracts to f/k = 1 with every part admitted.
    let split = split.expect("split scheme converges");
    assert!((split.value().value()[0] - 1.0).abs() <= 1e-9);
    assert_eq!(split.value().steps().len() % 2, 0);
    assert!(split
        .value()
        .steps()
        .chunks(2)
        .all(|pair| pair[0].work_in() >= 0.0 && pair[1].work_in() == 0.0));

    let mut bad = Quadratic::new(1.0, 1.0, 1.0);
    bad.overshoot = 5.0;
    assert!(matches!(
        dissipative_unfold(
            &[&bad],
            residual,
            vec![0.0],
            1.0,
            TOLERANCE,
            stop(),
            &meter()
        ),
        Err(DissipationStepRefuse::Rejected { .. })
    ));
}

#[test]
fn the_cited_laws_have_four_language_parity_rows() {
    let formal = Path::new(env!("CARGO_MANIFEST_DIR")).join("../umst-formal");
    let parity: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(formal.join("formal_parity.json")).expect("formal_parity.json"),
    )
    .expect("json");
    let rows = parity["statements"].as_array().expect("statements");
    for (id, lean) in [
        ("prox.prox_step_le", "UMST.UnfoldProxStepLaws.prox_step_le"),
        (
            "prox.prox_step_refused",
            "UMST.UnfoldProxStepLaws.prox_step_refused",
        ),
        ("unfold.bind_assoc", "UMST.UnfoldKleisli.bind_assoc"),
    ] {
        let row = rows
            .iter()
            .find(|r| r["id"] == id)
            .unwrap_or_else(|| panic!("no parity row {id}"));
        assert_eq!(row["lean"]["name"], lean);
        for lang in ["coq", "agda", "haskell"] {
            assert!(row[lang]["name"].is_string(), "{id} lacks {lang}");
        }
        let file = row["lean"]["file"].as_str().expect("lean file");
        let short = lean.rsplit('.').next().expect("short name");
        let text = std::fs::read_to_string(formal.join(file)).expect("lean source");
        assert!(
            text.contains(&format!("theorem {short}")),
            "{file} lacks {short}"
        );
    }
}
