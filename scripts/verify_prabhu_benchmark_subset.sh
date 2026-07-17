#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
#
# RW-FP-PRABHU PB-S6 — CI probe for prabhu_benchmark_subset (PB-1..3).
# SSOT: outputs/.tmp/fp_prabhu_benchmark_schedule.md (§ PB-S6)
#
# Validates the committed PB-S4 baseline transcript and, once PB-S5 lands,
# compares a fresh witness against operator-signed regression thresholds.
# Agent scope: probe script + docs only — never invent prabhu_thresholds.toml.
#
# Inputs:
#   artifacts/benchmarks/prabhu_runtime_subset.json   PB-S4 baseline (committed)
#   artifacts/benchmarks/prabhu_thresholds.toml         PB-S5 operator (optional pre-S5)
#
# Related:
#   scripts/collect_prabhu_benchmark_transcript.sh      PB-S3 collector (--collect)
#   artifacts/benchmarks/prabhu_ci_probe_design.md      CI wiring notes (rust.yml post-S5)
#
# Modes:
#   dry-run (default)  schema + parity check; log witnesses; exit 0 if thresholds absent
#   enforce            regression gate — current <= baseline * (1 + max_pct/100) per metric
#
# Usage:
#   bash scripts/verify_prabhu_benchmark_subset.sh                 # dry-run (default)
#   bash scripts/verify_prabhu_benchmark_subset.sh --dry-run
#   bash scripts/verify_prabhu_benchmark_subset.sh --dry-run --collect
#   bash scripts/verify_prabhu_benchmark_subset.sh --enforce       # post-PB-S5 only
#   bash scripts/verify_prabhu_benchmark_subset.sh --enforce --collect
#
# Flags:
#   --dry-run   validate baseline schema + parity; non-blocking without thresholds
#   --enforce   fail on regression over max_regression_pct (needs operator thresholds)
#   --collect   run collect_prabhu_benchmark_transcript.sh before compare
#   -h, --help  print this header and exit
#
# Env:
#   PRABHU_BASELINE_JSON    override baseline transcript path
#   PRABHU_THRESHOLDS_TOML  override thresholds path
#   CARGO_TARGET_DIR        forwarded to collector when --collect
#
# Exit codes:
#   0  dry-run OK, or enforce gate green
#   1  missing baseline, schema/parity failure, enforce without thresholds, or regression
#   2  unknown argument
set -euo pipefail

MANIFOLD="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASELINE_JSON="${PRABHU_BASELINE_JSON:-${MANIFOLD}/artifacts/benchmarks/prabhu_runtime_subset.json}"
THRESHOLDS_TOML="${PRABHU_THRESHOLDS_TOML:-${MANIFOLD}/artifacts/benchmarks/prabhu_thresholds.toml}"
# Locked by PB-S3 collector parity guard (phase0d + gate_parity_fixture).
PARITY_SHA256="149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3"

MODE="dry-run"
COLLECT=0

for arg in "$@"; do
  case "${arg}" in
    --dry-run) MODE="dry-run" ;;
    --enforce) MODE="enforce" ;;
    --collect) COLLECT=1 ;;
    -h|--help)
      awk '/^set -euo pipefail/{exit} NR>1{sub(/^# ?/,""); print}' "$0"
      exit 0
      ;;
    *)
      echo "Unknown arg: ${arg} (try --help)" >&2
      exit 2
      ;;
  esac
done

export PATH="${HOME}/.cargo/bin:${PATH}"

echo "== prabhu-benchmark-subset probe (mode=${MODE}, collect=${COLLECT}) =="

if [[ ! -f "${BASELINE_JSON}" ]]; then
  echo "FAIL: baseline transcript missing: ${BASELINE_JSON}" >&2
  echo "Run: bash scripts/collect_prabhu_benchmark_transcript.sh (PB-S3/S4)" >&2
  exit 1
fi

if [[ "${COLLECT}" -eq 1 ]]; then
  echo "== collecting fresh transcript (PB-S3 collector) =="
  bash "${MANIFOLD}/scripts/collect_prabhu_benchmark_transcript.sh"
fi

CURRENT_JSON="${BASELINE_JSON}"
if [[ "${COLLECT}" -eq 1 ]]; then
  CURRENT_JSON="${MANIFOLD}/artifacts/benchmarks/prabhu_runtime_subset.json"
fi

python3 - "${MODE}" "${BASELINE_JSON}" "${CURRENT_JSON}" "${THRESHOLDS_TOML}" "${PARITY_SHA256}" <<'PY'
"""PB-S6 probe: validate transcript schema/parity; optional regression gate (PB-S5)."""
from __future__ import annotations

import json
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:  # pragma: no cover
    import tomli as tomllib  # type: ignore

mode, baseline_path, current_path, thresholds_path, parity_sha = sys.argv[1:6]

REQUIRED_METRICS = (
    "gate_route_us_per_call",
    "thmc_step_ms_per_node",
    "arena_100_loads_sec",
)

THRESHOLD_SECTIONS = {
    # Transcript key -> TOML section (operator sets max_regression_pct in PB-S5).
    "gate_route_us_per_call": "pb1_gate_route",
    "thmc_step_ms_per_node": "pb2_thmc_step",
    "arena_100_loads_sec": "pb3_arena_alloc",
}


def load_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as fh:
        return json.load(fh)


def validate_transcript(doc: dict, label: str) -> None:
    if doc.get("schema_version") != 0:
        raise SystemExit(f"FAIL: {label} schema_version must be 0")
    if doc.get("workload_id") != "prabhu_benchmark_subset":
        raise SystemExit(f"FAIL: {label} workload_id must be prabhu_benchmark_subset")
    metrics = doc.get("metrics")
    if not isinstance(metrics, dict):
        raise SystemExit(f"FAIL: {label} missing metrics object")
    for key in REQUIRED_METRICS:
        if key not in metrics:
            raise SystemExit(f"FAIL: {label} metrics missing {key}")
        if metrics[key] is None:
            raise SystemExit(f"FAIL: {label} metrics.{key} is null")
    parity = doc.get("parity") or {}
    if parity.get("gate_parity_sha256") != parity_sha:
        raise SystemExit(f"FAIL: {label} gate_parity_sha256 mismatch (digest drift)")
    ref = doc.get("thresholds_ref")
    if ref != "artifacts/benchmarks/prabhu_thresholds.toml":
        raise SystemExit(f"FAIL: {label} thresholds_ref must point at prabhu_thresholds.toml")
    print(f"OK: {label} schema + parity digest")


baseline = load_json(Path(baseline_path))
current = load_json(Path(current_path))
validate_transcript(baseline, "baseline transcript")
if current_path != baseline_path:
    validate_transcript(current, "current transcript")

thresholds_file = Path(thresholds_path)
if not thresholds_file.is_file():
    # Pre-S5: witness-only dry-run; enforce must fail closed.
    print(f"BLOCKED-OPERATOR: missing {thresholds_file} (PB-S5)")
    if mode == "enforce":
        raise SystemExit("FAIL: --enforce requires operator-signed prabhu_thresholds.toml")
    print("DRY-RUN: probe would log transcript only; no regression gate until PB-S5 lands")
    print("CI wiring: job prabhu-benchmark-subset, continue-on-error: true until PB-S5")
    for key in REQUIRED_METRICS:
        b = float(baseline["metrics"][key])
        c = float(current["metrics"][key])
        print(f"  witness {key}: baseline={b} current={c} (no threshold — skipped)")
    raise SystemExit(0)

with thresholds_file.open("rb") as fh:
    thresholds = tomllib.load(fh)

print(f"OK: thresholds present at {thresholds_file}")

regressions: list[str] = []
for metric_key, section in THRESHOLD_SECTIONS.items():
    section_doc = thresholds.get(section)
    if not isinstance(section_doc, dict):
        raise SystemExit(f"FAIL: thresholds missing [{section}]")
    if "max_regression_pct" not in section_doc:
        raise SystemExit(f"FAIL: thresholds [{section}] missing max_regression_pct")
    max_pct = float(section_doc["max_regression_pct"])
    baseline_val = float(baseline["metrics"][metric_key])
    current_val = float(current["metrics"][metric_key])
    # Higher is worse for all three transcript keys (latency / wall time).
    allowed = baseline_val * (1.0 + max_pct / 100.0)
    delta_pct = ((current_val - baseline_val) / baseline_val * 100.0) if baseline_val else 0.0
    status = "PASS" if current_val <= allowed else "FAIL"
    print(
        f"  {metric_key}: baseline={baseline_val} current={current_val} "
        f"max_regression_pct={max_pct} allowed<={allowed:.6g} delta={delta_pct:+.2f}% -> {status}"
    )
    if status == "FAIL":
        regressions.append(metric_key)

if regressions and mode == "enforce":
    raise SystemExit(f"FAIL: regression over threshold for: {', '.join(regressions)}")

if mode == "dry-run":
    print("DRY-RUN: thresholds read; no fail on regression (use --enforce post-S5)")
else:
    print("OK: prabhu-benchmark-subset enforce gate green")
PY
