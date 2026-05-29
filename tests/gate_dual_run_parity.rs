// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Dual-run parity: manifold **mix-proposal gate** ([`ThermodynamicMixFilter`] / [`MixProposalScalars`],
//! the wasm-free port of prototype `thermodynamic_filter`) vs prototype [`ThermodynamicFilter`].
//!
//! Fixture JSON: `tests/data/gate_dual_run_fixtures.json` (golden from `umst-prototype` unit tests).
//! When the prototype helper binary is available, also runs `gate_dual_fixture` via subprocess and
//! reports live agreement rate.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

use serde::Deserialize;
use umst_manifold::gate::{
    mix_proposal::{
        evaluate_mix_transition, MixProposalScalars, ThermodynamicStateSnapshot,
        Q_HYDRATION_J_PER_KG,
    },
    ThermodynamicMixFilter,
};

const FIXTURE_REL: &str = "tests/data/gate_dual_run_fixtures.json";

static FIXTURE_PATH: OnceLock<PathBuf> = OnceLock::new();

fn fixture_path() -> &'static Path {
    FIXTURE_PATH.get_or_init(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_REL))
}

#[derive(Debug, Deserialize)]
struct MixInput {
    w_c: f64,
    alpha: f64,
    temp_k: f64,
    #[serde(default)]
    s_intrinsic_mpa: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct SnapshotInput {
    density: f64,
    temperature: f64,
    free_energy: f64,
    entropy: f64,
    hydration_degree: f64,
    strength: f64,
}

#[derive(Debug, Deserialize)]
struct PrototypeGolden {
    accepted: bool,
    mass_conserved: bool,
    energy_positive: bool,
    #[serde(default)]
    dissipation_sign: Option<String>,
    #[serde(default)]
    dissipation_relative_tolerance: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
enum FixtureCase {
    FromMix {
        id: String,
        old: MixInput,
        new: MixInput,
        dt_seconds: f64,
        prototype_golden: PrototypeGolden,
    },
    ExplicitSnapshot {
        id: String,
        old: SnapshotInput,
        new: SnapshotInput,
        dt_seconds: f64,
        prototype_golden: PrototypeGolden,
    },
}

#[derive(Debug, Deserialize)]
struct FixtureFile {
    schema_version: u32,
    cases: Vec<FixtureCase>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TransitionOutcome {
    accepted: bool,
    dissipation: f64,
    mass_conserved: bool,
    energy_positive: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SubprocessTransitionResult {
    accepted: bool,
    dissipation: f64,
    mass_conserved: bool,
    energy_positive: bool,
}

#[derive(Debug, Deserialize)]
struct SubprocessOutput {
    results: Vec<SubprocessTransitionResult>,
}

fn mix_to_proposal(m: &MixInput) -> MixProposalScalars {
    MixProposalScalars {
        water_cement_ratio: m.w_c,
        hydration_degree: m.alpha,
        temperature_k: m.temp_k,
        s_intrinsic_mpa: m.s_intrinsic_mpa,
    }
}

fn snapshot_to_state(s: &SnapshotInput) -> ThermodynamicStateSnapshot {
    ThermodynamicStateSnapshot {
        density: s.density,
        temperature: s.temperature,
        free_energy: s.free_energy,
        entropy: s.entropy,
        hydration_degree: s.hydration_degree,
        strength: s.strength,
    }
}

fn run_manifold_mix_gate(case: &FixtureCase) -> TransitionOutcome {
    let mut filter = ThermodynamicMixFilter::new();
    match case {
        FixtureCase::FromMix {
            old,
            new,
            dt_seconds,
            ..
        } => {
            let old_p = mix_to_proposal(old);
            let new_p = mix_to_proposal(new);
            let r = evaluate_mix_transition(&mut filter, &old_p, &new_p, *dt_seconds);
            TransitionOutcome {
                accepted: r.accepted,
                dissipation: r.dissipation,
                mass_conserved: r.mass_conserved,
                energy_positive: r.energy_positive,
            }
        }
        FixtureCase::ExplicitSnapshot {
            old,
            new,
            dt_seconds,
            ..
        } => {
            let old_s = snapshot_to_state(old);
            let new_s = snapshot_to_state(new);
            let r = filter.check_transition(&old_s, &new_s, *dt_seconds);
            TransitionOutcome {
                accepted: r.accepted,
                dissipation: r.dissipation,
                mass_conserved: r.mass_conserved,
                energy_positive: r.energy_positive,
            }
        }
    }
}

fn golden_to_outcome(g: &PrototypeGolden, manifold: &TransitionOutcome) -> TransitionOutcome {
    let _ = manifold;
    TransitionOutcome {
        accepted: g.accepted,
        dissipation: 0.0,
        mass_conserved: g.mass_conserved,
        energy_positive: g.energy_positive,
    }
}

fn check_dissipation_sign(sign: &str, d: f64) {
    match sign {
        "positive" => assert!(d > 0.0, "expected positive dissipation, got {d}"),
        "negative" => assert!(d < 0.0, "expected negative dissipation, got {d}"),
        "nonnegative" => assert!(d >= -1e-6, "expected nonnegative dissipation, got {d}"),
        "any" => {}
        other => panic!("unknown dissipation_sign: {other}"),
    }
}

fn assert_matches_golden(
    case_id: &str,
    got: &TransitionOutcome,
    golden: &PrototypeGolden,
    old_mix: Option<(&MixInput, &MixInput, f64)>,
) {
    assert_eq!(
        got.accepted, golden.accepted,
        "{case_id}: accepted mismatch (manifold {})",
        got.accepted
    );
    assert_eq!(
        got.mass_conserved, golden.mass_conserved,
        "{case_id}: mass_conserved mismatch"
    );
    assert_eq!(
        got.energy_positive, golden.energy_positive,
        "{case_id}: energy_positive mismatch"
    );

    if let Some(sign) = golden.dissipation_sign.as_deref() {
        check_dissipation_sign(sign, got.dissipation);
    }

    if let Some(tol) = golden.dissipation_relative_tolerance {
        if let Some((old, new, dt)) = old_mix {
            let rho = (2400.0 - 400.0 * old.w_c + 2400.0 - 400.0 * new.w_c) / 2.0;
            let alpha_dot = (new.alpha - old.alpha) / dt;
            let expected = rho * Q_HYDRATION_J_PER_KG * alpha_dot;
            let rel_err = ((got.dissipation - expected) / expected).abs();
            assert!(
                rel_err < tol,
                "{case_id}: D_int rel err {rel_err} (got {}, expected {})",
                got.dissipation,
                expected
            );
        }
    }
}

fn prototype_core_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../umst-prototype/src/rust/core/Cargo.toml")
}

fn run_prototype_subprocess(fixture_json: &str) -> Option<Vec<SubprocessTransitionResult>> {
    let manifest = prototype_core_manifest();
    if !manifest.exists() {
        return None;
    }

    let mut child = Command::new("cargo");
    child
        .args([
            "run",
            "--quiet",
            "--manifest-path",
            manifest.to_str()?,
            "--bin",
            "gate_dual_fixture",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = child.spawn().ok()?;
    use std::io::Write;
    child
        .stdin
        .as_mut()?
        .write_all(fixture_json.as_bytes())
        .ok()?;
    let out = child.wait_with_output().ok()?;
    if !out.status.success() {
        eprintln!(
            "gate_dual_fixture subprocess failed: status={:?} stderr={}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
        return None;
    }
    let parsed: SubprocessOutput = serde_json::from_slice(&out.stdout).ok()?;
    Some(parsed.results)
}

fn load_fixtures() -> FixtureFile {
    let raw = std::fs::read_to_string(fixture_path()).expect("read fixture JSON");
    serde_json::from_str(&raw).expect("parse fixture JSON")
}

#[test]
fn mix_proposal_gate_matches_prototype_golden_vectors() {
    let fixtures = load_fixtures();
    assert_eq!(fixtures.schema_version, 1);
    assert!(
        !fixtures.cases.is_empty(),
        "fixture file must contain at least one case"
    );

    let mut golden_agree = 0usize;
    let total = fixtures.cases.len();

    for case in &fixtures.cases {
        let (id, golden, mix_pair) = match case {
            FixtureCase::FromMix {
                id,
                old,
                new,
                dt_seconds,
                prototype_golden,
            } => (id.as_str(), prototype_golden, Some((old, new, *dt_seconds))),
            FixtureCase::ExplicitSnapshot {
                id,
                prototype_golden,
                ..
            } => (id.as_str(), prototype_golden, None),
        };

        let manifold = run_manifold_mix_gate(case);
        assert_matches_golden(id, &manifold, golden, mix_pair);

        let g_out = golden_to_outcome(golden, &manifold);
        if manifold.accepted == g_out.accepted
            && manifold.mass_conserved == g_out.mass_conserved
            && manifold.energy_positive == g_out.energy_positive
        {
            golden_agree += 1;
        }
    }

    let golden_rate = golden_agree as f64 / total as f64;
    eprintln!(
        "gate_dual_run_parity: manifold vs prototype_golden agreement {golden_agree}/{total} ({:.1}%)",
        golden_rate * 100.0
    );
    assert_eq!(
        golden_agree, total,
        "manifold mix gate must match all prototype golden vectors"
    );
}

#[test]
fn mix_proposal_gate_live_subprocess_matches_manifold_when_available() {
    let fixtures = load_fixtures();
    let bundle_json =
        std::fs::read_to_string(fixture_path()).expect("read fixtures for subprocess");

    let Some(live) = run_prototype_subprocess(&bundle_json) else {
        eprintln!(
            "gate_dual_run_parity: skipping live subprocess (prototype manifest or gate_dual_fixture unavailable)"
        );
        return;
    };

    assert_eq!(
        live.len(),
        fixtures.cases.len(),
        "subprocess must return one result per fixture case"
    );

    let mut live_agree = 0usize;
    let total = fixtures.cases.len();

    for (case, proto) in fixtures.cases.iter().zip(live.iter()) {
        let id = match case {
            FixtureCase::FromMix { id, .. } | FixtureCase::ExplicitSnapshot { id, .. } => id,
        };
        let manifold = run_manifold_mix_gate(case);

        assert_eq!(
            manifold.accepted, proto.accepted,
            "{id}: live prototype accepted mismatch"
        );
        assert!(
            (manifold.dissipation - proto.dissipation).abs() < 1e-9,
            "{id}: dissipation drift manifold={} prototype={}",
            manifold.dissipation,
            proto.dissipation
        );
        assert_eq!(
            manifold.mass_conserved, proto.mass_conserved,
            "{id}: mass_conserved live mismatch"
        );
        assert_eq!(
            manifold.energy_positive, proto.energy_positive,
            "{id}: energy_positive live mismatch"
        );
        live_agree += 1;
    }

    let live_rate = live_agree as f64 / total as f64;
    eprintln!(
        "gate_dual_run_parity: manifold vs live prototype subprocess agreement {live_agree}/{total} ({:.1}%)",
        live_rate * 100.0
    );
    assert_eq!(
        live_agree, total,
        "live prototype thermodynamic_filter must agree with manifold MixProposal gate on all fixtures"
    );
}
