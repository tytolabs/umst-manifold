// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Meter for a measured work ensemble: the Jarzynski estimate of the free-energy change, its mean-work upper
//! bound and bias estimate, and, when the free-energy change is known, the alarm of the integral fluctuation
//! check ([`umst_math::jarzynski_meter`]).
//!
//! Usage: `integral_fluctuation_meter WORKS_FILE [DELTA_F ERROR_BOUND SUPPORT_LO SUPPORT_HI LEVEL_NUM LEVEL_DEN]`
//!
//! `WORKS_FILE` holds one reduced work `w = βW` per line. The alarm needs all six trailing arguments: the known
//! ΔF and its stated error bound, the interval every work lies in, and the level δ as `LEVEL_NUM / LEVEL_DEN`.
//! Exit code 0 when the report is written and no alarm is raised, 1 on an alarm, 2 on a refusal.

use std::process::ExitCode;

use umst_math::jarzynski_meter::{report, KnownDeltaF, Level};

fn number(args: &[String], index: usize, what: &str) -> Result<f64, String> {
    args[index]
        .parse::<f64>()
        .map_err(|_| format!("{what} is not a number: {}", args[index]))
}

fn integer(args: &[String], index: usize, what: &str) -> Result<u64, String> {
    args[index]
        .parse::<u64>()
        .map_err(|_| format!("{what} is not a non-negative integer: {}", args[index]))
}

fn known_delta_f(args: &[String]) -> Result<Option<KnownDeltaF>, String> {
    match args.len() {
        2 => Ok(None),
        8 => {
            let level = Level::new(
                integer(args, 6, "LEVEL_NUM")?,
                integer(args, 7, "LEVEL_DEN")?,
            )
            .map_err(|r| format!("level refused: {r:?}"))?;
            Ok(Some(KnownDeltaF {
                delta_f: number(args, 2, "DELTA_F")?,
                error_bound: number(args, 3, "ERROR_BOUND")?,
                support: (
                    number(args, 4, "SUPPORT_LO")?,
                    number(args, 5, "SUPPORT_HI")?,
                ),
                level,
            }))
        }
        n => Err(format!(
            "expected 2 or 8 arguments after the program name, found {}",
            n - 1
        )),
    }
}

fn run(args: &[String]) -> Result<bool, String> {
    let known = known_delta_f(args)?;
    let text = std::fs::read_to_string(&args[1]).map_err(|e| format!("{}: {e}", args[1]))?;
    let r = report(&text, known).map_err(|r| format!("works refused: {r:?}"))?;
    println!("samples {}", r.samples);
    println!("delta_f_estimate {:.12}", r.delta_f_estimate);
    println!("mean_work {:.12}", r.mean_work);
    println!("delta_method_bias {:.12}", r.delta_method_bias);
    match r.verdict {
        None => Ok(false),
        Some(v) => {
            let ci = v.interval();
            println!("interval {:.12} {:.12}", ci.lower, ci.upper);
            println!(
                "verdict {}",
                if v.is_alarm() { "alarm" } else { "consistent" }
            );
            Ok(v.is_alarm())
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: integral_fluctuation_meter WORKS_FILE [DELTA_F ERROR_BOUND SUPPORT_LO SUPPORT_HI LEVEL_NUM LEVEL_DEN]");
        return ExitCode::from(2);
    }
    match run(&args) {
        Ok(false) => ExitCode::SUCCESS,
        Ok(true) => ExitCode::from(1),
        Err(msg) => {
            eprintln!("{msg}");
            ExitCode::from(2)
        }
    }
}
