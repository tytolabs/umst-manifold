#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
# Phase 4 multi-seed witness stub — runs 3 deterministic seeds and aggregates JSON.
#
# Usage (from umst-manifold):
#   bash scripts/run_p4_multiseed_witness.sh
#
# Writes: artifacts/training/p4_rejection_multiseed.json
# Per-seed single-run baselines: artifacts/training/rejection_baseline_gpu.json (last seed wins)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUT="${ROOT}/artifacts/training/p4_rejection_multiseed.json"
BASELINE="${ROOT}/artifacts/training/rejection_baseline_gpu.json"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT
SEEDS=(42 43 44)

cd "$ROOT"
mkdir -p "$(dirname "$OUT")"

idx=0
for seed in "${SEEDS[@]}"; do
  echo "== P4 witness seed=${seed} =="
  P4_WITNESS_SEED="${seed}" cargo test -p umst-manifold --features kleisli-ppo-hot-bind,wgpu \
    --test rejection_witness_gpu rejection_baseline_gpu_measured_witness -- --exact --nocapture
  if [[ ! -f "${BASELINE}" ]]; then
    echo "FAIL: expected ${BASELINE} after seed ${seed}" >&2
    exit 1
  fi
  cp "${BASELINE}" "${TMP_DIR}/run_${idx}.json"
  idx=$((idx + 1))
done

python3 - "${OUT}" "${SEEDS[*]}" "${TMP_DIR}" <<'PY'
import json
import sys
from datetime import date
from pathlib import Path

out_path = Path(sys.argv[1])
seeds = [int(s) for s in sys.argv[2].split()]
tmp_dir = Path(sys.argv[3])
runs = [json.loads((tmp_dir / f"run_{i}.json").read_text(encoding="utf-8")) for i in range(len(seeds))]

reductions = [r["delta"]["rejection_rate_reduction"] for r in runs]
targets = [r["delta"]["target_met"] for r in runs]

doc = {
    "schema_version": "p4_rejection_multiseed.v1",
    "generated_at": date.today().isoformat(),
    "stub_disclaimer": (
        "Aggregate of 3 single deterministic witness runs; not a full statistical study. "
        "Use for regression smoke only."
    ),
    "seeds": seeds,
    "runs": runs,
    "aggregate": {
        "n_seeds": len(seeds),
        "rejection_rate_reduction_mean": sum(reductions) / len(reductions),
        "rejection_rate_reduction_min": min(reductions),
        "rejection_rate_reduction_max": max(reductions),
        "all_target_met": all(targets),
    },
    "regenerate": "bash scripts/run_p4_multiseed_witness.sh",
}

out_path.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
print(f"wrote {out_path}")
PY

echo "OK: p4 multiseed witness stub (${#SEEDS[@]} seeds)"
