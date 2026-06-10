//! §0.8 — RED ε-bisim for Joseph scalar EKF (`1.0` ms per step; `update` = `update_with_step_ms(·, 1.0)`).
//!
//! ε: absolute **1e-9** (pinned in TUI-7 HANDBACK).

use umst_math::constants::registry::registry_f64_by_name;
use umst_math::smoothing::{EkfSmoother, MetricSmoother};

const EPS: f64 = 1e-9;

fn run_sequence(raw: &[f64]) -> Vec<f64> {
    let mut s = EkfSmoother::new(0.0);
    let mut v = Vec::with_capacity(raw.len());
    for &z in raw {
        v.push(s.update(z));
    }
    v
}

fn run_sequence_twice(raw: &[f64]) -> (Vec<f64>, Vec<f64>) {
    (run_sequence(raw), run_sequence(raw))
}

// Fixture sequences: deterministic; expected rows match Python reference at dt=1.0 ms, q=10, r=500, p0=1000, x0=0.
const SEQ0: [f64; 8] = [1.0, 2.0, 1.5, 0.0, 3.0, 1.0, 2.0, 0.5];
const EXP0: [f64; 8] = [
    0.6688741721854304,
    1.2118265234099286,
    1.2981822395976936,
    0.9837205034678143,
    1.4026082878626092,
    1.3279229140619873,
    1.4424927760906194,
    1.2916971767981256,
];

const SEQ1: [f64; 8] = [0.0, 0.0, 1.0, 1.0, 0.0, 2.0, 0.0, 1.0];
const EXP1: [f64; 8] = [
    0.0,
    0.0,
    0.2996657333270354,
    0.46930934934004986,
    0.3718090008621085,
    0.6738446445767217,
    0.5589734645094278,
    0.6295361917900014,
];

const SEQ2: [f64; 8] = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0];
const EXP2: [f64; 8] = [
    6.688741721854305,
    7.631479883930672,
    7.741912734758351,
    7.562197468663447,
    7.2376465135200405,
    6.8225545461313,
    6.3413899718812905,
    5.806779080358317,
];

const SEQ3: [f64; 8] = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
const EXP3: [f64; 8] = [
    0.06688741721854305,
    0.12118265234099287,
    0.17476808395882468,
    0.22932654053739907,
    0.2855597194809778,
    0.3438895914595436,
    0.40459619971489924,
    0.4678594464358359,
];

const SEQ4: [f64; 8] = [100.0, 200.0, 150.0, 0.0, 50.0, 10.0, 20.0, 5.0];
const EXP4: [f64; 8] = [
    66.88741721854305,
    121.18265234099286,
    129.81822395976937,
    98.37205034678144,
    88.32261968439246,
    73.79347484431166,
    64.62323065669489,
    55.08371902677611,
];

fn assert_seq_close(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len());
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        assert!(
            (x - y).abs() <= EPS,
            "i={i} {x} vs {y} diff={}",
            (x - y).abs()
        );
    }
}

#[test]
fn ekf_seq0_canonical() {
    let v = run_sequence(&SEQ0);
    assert_seq_close(&v, &EXP0);
}

#[test]
fn ekf_seq0_ebisim() {
    let (a, b) = run_sequence_twice(&SEQ0);
    assert_eq!(a, b);
}

#[test]
fn ekf_seq1_canonical() {
    let v = run_sequence(&SEQ1);
    assert_seq_close(&v, &EXP1);
}

#[test]
fn ekf_seq1_ebisim() {
    let (a, b) = run_sequence_twice(&SEQ1);
    assert_eq!(a, b);
}

#[test]
fn ekf_seq2_canonical() {
    let v = run_sequence(&SEQ2);
    assert_seq_close(&v, &EXP2);
}

#[test]
fn ekf_seq2_ebisim() {
    let (a, b) = run_sequence_twice(&SEQ2);
    assert_eq!(a, b);
}

#[test]
fn ekf_seq3_canonical() {
    let v = run_sequence(&SEQ3);
    assert_seq_close(&v, &EXP3);
}

#[test]
fn ekf_seq3_ebisim() {
    let (a, b) = run_sequence_twice(&SEQ3);
    assert_eq!(a, b);
}

#[test]
fn ekf_seq4_canonical() {
    let v = run_sequence(&SEQ4);
    assert_seq_close(&v, &EXP4);
}

#[test]
fn ekf_seq4_ebisim() {
    let (a, b) = run_sequence_twice(&SEQ4);
    assert_eq!(a, b);
}

// --- TUI-7b: G5 two-regime synthetic (100 pts), deterministic; per-metric amplitude = max|SEQk| in ε-bisim fixtures; §0.8 variance win vs (10,500)

const TUNING_IMPROV_FRAC: f64 = 0.05;
const G5_Q0: f64 = 10.0;
const G5_R0: f64 = 500.0;

/// MEASUREMENT: G5 (smooth ∥ noisy-step) on amplitude `scale` — TUI-7b RED witness
fn g5_tuning_synth_100(scale: f64) -> [f64; 100] {
    let mut z = [0.0f64; 100];
    for (i, slot) in z.iter_mut().enumerate() {
        *slot = scale
            * if i < 50 {
                0.5 * (0.1 * i as f64).sin() + 0.01 * (42.0 * i as f64).sin()
            } else {
                1.0 + 0.3 * (0.1 * i as f64).sin() + 0.2 * (42.0 * i as f64 + 3.0).sin()
            };
    }
    z
}

fn output_variance_ekf(z: &[f64; 100], q: f64, r: f64) -> f64 {
    let mut s = EkfSmoother::new_with_q_r(0.0, q, r);
    let mut out: Vec<f64> = Vec::with_capacity(100);
    for &zi in z {
        out.push(s.update_with_step_ms(zi, 1.0));
    }
    let m = out.iter().sum::<f64>() / out.len() as f64;
    out.iter().map(|a| (a - m) * (a - m)).sum::<f64>() / out.len() as f64
}

fn assert_tuning_helps_ekf(scale: f64, qn: &str, rn: &str) {
    let z = g5_tuning_synth_100(scale);
    let v_u = output_variance_ekf(&z, G5_Q0, G5_R0);
    let q = registry_f64_by_name(qn).expect("registry Q");
    let r = registry_f64_by_name(rn).expect("registry R");
    let v_t = output_variance_ekf(&z, q, r);
    assert!(v_u > 1.0e-20, "non-degenerate output variance (uniform)");
    let imp = (v_u - v_t) / v_u;
    assert!(
        imp + 1e-12 >= TUNING_IMPROV_FRAC,
        "TUI-7b EKF tuning: expected >= {TUNING_IMPROV_FRAC} var reduction, got {imp} (q={q} r={r})"
    );
}

#[test]
fn ekf_tuning_rcc_g5() {
    assert_tuning_helps_ekf(3.0, "egoff_smoother_q_rcc", "egoff_smoother_r_rcc");
}
#[test]
fn ekf_tuning_mi_g5() {
    assert_tuning_helps_ekf(2.0, "egoff_smoother_q_mi", "egoff_smoother_r_mi");
}
#[test]
fn ekf_tuning_eta_cog_g5() {
    assert_tuning_helps_ekf(10.0, "egoff_smoother_q_eta_cog", "egoff_smoother_r_eta_cog");
}
#[test]
fn ekf_tuning_dignity_g5() {
    assert_tuning_helps_ekf(0.8, "egoff_smoother_q_dignity", "egoff_smoother_r_dignity");
}
#[test]
fn ekf_tuning_landauer_g5() {
    assert_tuning_helps_ekf(
        200.0,
        "egoff_smoother_q_landauer_slack",
        "egoff_smoother_r_landauer_slack",
    );
}
