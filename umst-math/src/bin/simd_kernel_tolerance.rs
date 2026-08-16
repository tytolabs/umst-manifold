// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Emit NDJSON lines for `.umst-ci/simd/<timestamp>/kernel_tolerance.jsonl`.
//! Built only with `--features simd` (see `Cargo.toml` `required-features`).

use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let target = args.next().unwrap_or_else(|| {
        eprintln!("usage: simd_kernel_tolerance <target-triple>");
        std::process::exit(2);
    });
    let rows = umst_math::kernels::kernel_tolerance_rows(target);
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for row in rows {
        serde_json::to_writer(&mut out, &row)?;
        writeln!(out)?;
    }
    Ok(())
}
