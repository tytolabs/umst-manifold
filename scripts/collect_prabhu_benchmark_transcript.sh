#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
#
# RW-FP-PRABHU PB-S3 — collect prabhu_benchmark_subset transcript.
# Runs PB-1..PB-3 timing harnesses + parity guard; writes JSON artifact.
set -euo pipefail

MANIFOLD="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${MANIFOLD}/artifacts/benchmarks"
OUT_JSON="${OUT_DIR}/prabhu_runtime_subset.json"
BASELINE_SHA="${PRABHU_BASELINE_SHA:-a7b01c5db651e8a5840d965ce1747d0f4b50163c}"
PARITY_SHA256="149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/umst-p-prabhu-s3}"

export PATH="${HOME}/.cargo/bin:${PATH}"
export CARGO_TARGET_DIR

cd "${MANIFOLD}"

mkdir -p "${OUT_DIR}"

echo "== PB parity guard =="
cargo test -p umst-manifold --test phase0d_gate_routing --quiet
cargo test -p umst-manifold --test gate_parity_fixture --quiet

echo "== PB-1 gate routing =="
PB1_LOG="$(mktemp)"
cargo test -p umst-manifold --test prabhu_gate_route_timing -- --nocapture 2>&1 | tee "${PB1_LOG}"
GATE_ROUTE_US="$(grep -Eo 'gate_route_us_per_call=[0-9.]+' "${PB1_LOG}" | tail -1 | cut -d= -f2)"
if [[ -z "${GATE_ROUTE_US}" ]]; then
  echo "FAIL: missing gate_route_us_per_call" >&2
  exit 1
fi

echo "== PB-2 THMC step =="
PB2_LOG="$(mktemp)"
cargo test -p umst-manifold --features thmc-coupled \
  --test prabhu_thmc_step_timing -- --nocapture 2>&1 | tee "${PB2_LOG}"
THMC_STEP_MS="$(grep -Eo 'thmc_step_ms_per_node=[0-9.]+' "${PB2_LOG}" | tail -1 | cut -d= -f2)"
if [[ -z "${THMC_STEP_MS}" ]]; then
  echo "FAIL: missing thmc_step_ms_per_node" >&2
  exit 1
fi

echo "== PB-3 arena alloc =="
PB3_LOG="$(mktemp)"
UMST_ARENA_HOT_ITERS="${UMST_ARENA_HOT_ITERS:-10000}" \
  cargo bench -p umst-runtime-arena --bench prabhu_arena_alloc 2>&1 | tee "${PB3_LOG}"
ARENA_100_SEC="$(grep -Eo 'arena_100_loads_sec=[0-9.]+' "${PB3_LOG}" | tail -1 | cut -d= -f2)"
if [[ -z "${ARENA_100_SEC}" ]]; then
  echo "FAIL: missing arena_100_loads_sec" >&2
  exit 1
fi

GIT_SHA="$(git -C "${MANIFOLD}" rev-parse HEAD)"
RUSTC_VER="$(rustc -V 2>/dev/null || echo unknown)"
HOST_UNAME="$(uname -mrs 2>/dev/null || echo unknown)"

cat > "${OUT_JSON}" <<EOF
{
  "schema_version": 0,
  "workload_id": "prabhu_benchmark_subset",
  "git_sha": "${GIT_SHA}",
  "baseline_sha": "${BASELINE_SHA}",
  "rustc": "${RUSTC_VER}",
  "host": "${HOST_UNAME}",
  "metrics": {
    "gate_route_us_per_call": ${GATE_ROUTE_US},
    "thmc_step_ms_per_node": ${THMC_STEP_MS},
    "arena_100_loads_sec": ${ARENA_100_SEC}
  },
  "parity": {
    "gate_parity_sha256": "${PARITY_SHA256}",
    "gate_parity_tests": "phase0d + fixture green"
  },
  "thresholds_ref": "artifacts/benchmarks/prabhu_thresholds.toml"
}
EOF

echo "Wrote ${OUT_JSON}"
echo "gate_route_us_per_call=${GATE_ROUTE_US}"
echo "thmc_step_ms_per_node=${THMC_STEP_MS}"
echo "arena_100_loads_sec=${ARENA_100_SEC}"
