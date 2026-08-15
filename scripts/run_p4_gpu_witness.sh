#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Local fallback for Phase 4 GPU rejection witness (macOS Metal via wgpu).
#
# Usage (from umst-manifold):
#   bash scripts/run_p4_gpu_witness.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$ROOT"

cargo test -p umst-manifold --features kleisli-ppo-hot-bind,wgpu \
  --test rejection_witness_gpu \
  -- --nocapture

echo "OK: p4 GPU witness baseline regenerated"
