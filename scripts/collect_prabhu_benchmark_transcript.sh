#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
#
# RW-FP-PRABHU PB-S3 + INV4-S4 — collect prabhu_benchmark_subset transcript.
# Runs PB-1..PB-3 timing harnesses + parity guard; wires umst-bench energy probe.
set -euo pipefail

MANIFOLD="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MAOS_ROOT="$(cd "${MANIFOLD}/.." && pwd)"
BENCH_CRATE="${MAOS_ROOT}/crates/umst-bench"
OUT_DIR="${MANIFOLD}/artifacts/benchmarks"
OUT_JSON="${OUT_DIR}/prabhu_runtime_subset.json"
BASELINE_SHA="${PRABHU_BASELINE_SHA:-a7b01c5db651e8a5840d965ce1747d0f4b50163c}"
PARITY_SHA256="149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/umst-p-prabhu-s3}"
CARGO_ABORT_THRESHOLD="${PRABHU_CARGO_ABORT_THRESHOLD:-16}"

export PATH="${HOME}/.cargo/bin:${PATH}"
export CARGO_TARGET_DIR

cargo_live() {
  (pgrep -lf 'cargo ' 2>/dev/null; pgrep -lf rustc 2>/dev/null) | wc -l | tr -d ' '
}

LIVE="$(cargo_live)"
if [[ "${LIVE}" -gt "${CARGO_ABORT_THRESHOLD}" ]]; then
  echo "ABORT: cargo>${LIVE}>${CARGO_ABORT_THRESHOLD}" >&2
  exit 3
fi

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

echo "== INV4-S4 energy probe (measure_kj_per_result) =="
ENERGY_FRAGMENT="$(mktemp)"
cargo run --manifest-path "${BENCH_CRATE}/Cargo.toml" --bin prabhu_energy_probe --quiet \
  > "${ENERGY_FRAGMENT}"

GIT_SHA="$(git -C "${MANIFOLD}" rev-parse HEAD)"
RUSTC_VER="$(rustc -V 2>/dev/null || echo unknown)"
HOST_UNAME="$(uname -mrs 2>/dev/null || echo unknown)"

LATENCY_JSON="$(mktemp)"
cat > "${LATENCY_JSON}" <<EOF
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

python3 - "${LATENCY_JSON}" "${ENERGY_FRAGMENT}" "${OUT_JSON}" <<'PY'
import json
import sys

latency_path, energy_path, out_path = sys.argv[1:4]
with open(latency_path, encoding="utf-8") as f:
    doc = json.load(f)
with open(energy_path, encoding="utf-8") as f:
    energy = json.load(f)

doc["energy_backend"] = energy["energy_backend"]
doc["energy_domains"] = energy["energy_domains"]
doc["energy_n_iterations"] = energy["energy_n_iterations"]
doc["energy_warmup_discarded"] = energy["energy_warmup_discarded"]
doc["energy_uncertainty_kj"] = energy["energy_uncertainty_kj"]
doc["metrics"]["energy_kj_per_result"] = energy["metrics"]["energy_kj_per_result"]

with open(out_path, "w", encoding="utf-8") as f:
    json.dump(doc, f, indent=2)
    f.write("\n")
PY

echo "Wrote ${OUT_JSON}"
echo "gate_route_us_per_call=${GATE_ROUTE_US}"
echo "thmc_step_ms_per_node=${THMC_STEP_MS}"
echo "arena_100_loads_sec=${ARENA_100_SEC}"
python3 - "${OUT_JSON}" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as f:
    doc = json.load(f)
energy = doc.get("metrics", {}).get("energy_kj_per_result", {})
print(f"energy_backend={doc.get('energy_backend')}")
for key, value in energy.items():
    print(f"energy_kj_per_result.{key}={value}")
PY
