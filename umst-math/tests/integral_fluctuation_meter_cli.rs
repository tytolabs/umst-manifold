// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The `integral_fluctuation_meter` binary reports the Jarzynski estimate of a work file and raises the integral
//! fluctuation alarm against a known free-energy change; its exit code is 0 for a report, 1 for an alarm and 2 for
//! a refusal.

use std::path::PathBuf;
use std::process::{Command, Output};

fn works_file(name: &str, text: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "integral_fluctuation_meter_cli_{}_{name}",
        std::process::id()
    ));
    std::fs::write(&path, text).expect("write works file");
    path
}

fn meter(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_integral_fluctuation_meter"))
        .args(args)
        .output()
        .expect("run the meter")
}

fn field(out: &Output, key: &str) -> String {
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{key} ")).map(str::to_owned))
        .unwrap_or_else(|| {
            panic!(
                "no `{key}` line in {}",
                String::from_utf8_lossy(&out.stdout)
            )
        })
}

fn exact_delta_f(works: &[f64]) -> f64 {
    -(works.iter().map(|w| (-w).exp()).sum::<f64>() / works.len() as f64).ln()
}

#[test]
fn report_without_a_known_free_energy_change_carries_the_estimate_and_no_verdict() {
    let works = [0.5, 1.5, 1.0, 2.0];
    let file = works_file("plain", "# reduced works\n0.5\n1.5\n\n1.0\n2.0\n");
    let out = meter(&[file.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(field(&out, "samples"), "4");
    let estimate: f64 = field(&out, "delta_f_estimate").parse().unwrap();
    assert!((estimate - exact_delta_f(&works)).abs() < 1e-9);
    let mean: f64 = field(&out, "mean_work").parse().unwrap();
    assert!((mean - 1.25).abs() < 1e-9);
    assert!(!String::from_utf8_lossy(&out.stdout).contains("verdict"));
}

#[test]
fn a_free_energy_change_inside_the_interval_is_consistent_and_one_outside_alarms() {
    let file = works_file("known", "1.0\n1.1\n1.2\n1.3\n");
    let path = file.to_str().unwrap();
    // Interval of the four works at δ = 1/10 with support [0.5, 2]: wide, around the estimate.
    let consistent = meter(&[path, "1.0", "0", "0.5", "2.0", "1", "10"]);
    assert_eq!(consistent.status.code(), Some(0));
    assert_eq!(field(&consistent, "verdict"), "consistent");
    let alarm = meter(&[path, "9.0", "0", "0.5", "2.0", "1", "10"]);
    assert_eq!(alarm.status.code(), Some(1));
    assert_eq!(field(&alarm, "verdict"), "alarm");
    let interval = field(&alarm, "interval");
    let ends: Vec<f64> = interval
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert!(
        ends[0] < ends[1] && 9.0 > ends[1],
        "interval {interval} must exclude 9.0 from above"
    );
}

#[test]
fn refusals_exit_with_code_two_and_name_the_cause() {
    let good = works_file("good", "1.0\n2.0\n");
    let bad = works_file("bad", "1.0\nnot-a-number\n");
    let empty = works_file("empty", "# nothing\n");
    let outside = works_file("outside", "1.0\n5.0\n");
    let cases: [(Vec<&str>, &str); 6] = [
        (vec!["/nonexistent/works"], "/nonexistent/works"),
        (vec![bad.to_str().unwrap()], "Unparsed { line: 2 }"),
        (vec![empty.to_str().unwrap()], "Empty"),
        (vec![good.to_str().unwrap(), "1.0"], "arguments"),
        (
            vec![good.to_str().unwrap(), "1.0", "0", "0.5", "2.5", "0", "5"],
            "level refused",
        ),
        (
            vec![
                outside.to_str().unwrap(),
                "1.0",
                "0",
                "0.5",
                "2.5",
                "1",
                "10",
            ],
            "Support",
        ),
    ];
    for (args, cause) in cases {
        let out = meter(&args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains(cause),
            "{args:?}: stderr `{stderr}` lacks `{cause}`"
        );
    }
}
