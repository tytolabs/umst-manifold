#!/usr/bin/env bash
# W9 T2.1 — kernel catalog_id grep vs cartridge registry parity (Goal C).
# Extended Track C: tyto-workspace verify_public_stack + rejection telemetry grep.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FAIL=0
WARN=0
WS="$(cd "${ROOT}/.." && pwd)"
MAOS_WS="${MAOS_WORKSPACE:-$WS}"

warn() {
  echo "WARN: $*"
  WARN=$((WARN + 1))
}

fail() {
  echo "FAIL: $*"
  FAIL=1
}

echo "=== W9 T2.1 catalog_id kernel grep ==="
REGISTRY_IDS="$(rg -o 'pub const [A-Z0-9_]+_CATALOG_ID: &str = "[^"]+"' src/runtime/catalog/traceability.rs | sed 's/.*"\(.*\)"/\1/' | sort -u)"
KERNEL_HITS="$(rg -o 'umst\.[a-z0-9._-]+|thermodynamic_mix' src --glob '*.rs' | sort -u)"

while IFS= read -r id; do
  [ -z "$id" ] && continue
  if ! echo "$KERNEL_HITS" | grep -qx "$id"; then
    warn "registry id not referenced in src kernel: $id"
  fi
done <<< "$REGISTRY_IDS"

echo "=== CD transition catalog lock ==="
if rg -n "CD_TRANSITION_CATALOG_ID" src/runtime/catalog/traceability.rs src/gate/ src/ai/constraint_loss.rs >/dev/null; then
  echo "OK: CD_TRANSITION_CATALOG_ID wired in traceability + gate + constraint_loss"
else
  fail "CD_TRANSITION_CATALOG_ID wiring incomplete"
fi

echo "=== Rejection telemetry (Wave 10 T3) ==="
if [ -f src/ai/rejection_telemetry.rs ]; then
  if rg -q 'fn rejection_rate' src/ai/rejection_telemetry.rs \
    && rg -q 'fn mean_slack_at_commit' src/ai/rejection_telemetry.rs \
    && rg -q 'rejection_telemetry' src/ai/ppo.rs; then
    echo "OK: rejection_rate + mean_slack_at_commit wired in ppo"
  else
    fail "rejection_telemetry API incomplete"
  fi
else
  fail "missing src/ai/rejection_telemetry.rs"
fi

echo "=== tyto-workspace verify_public_stack feature-gated lane ==="
VPS="${MAOS_WS}/scripts/verify_public_stack.sh"
if [ -f "$VPS" ]; then
  if rg -q 'Feature-gated physics' "$VPS" \
    && rg -q 'thmc-coupled' "$VPS" \
    && rg -q 'epistemic-ppo' "$VPS"; then
    echo "OK: verify_public_stack.sh documents feature-gated physics lane"
  else
    fail "verify_public_stack.sh missing feature-gated physics stanza"
  fi
else
  warn "tyto-workspace verify_public_stack.sh not found at $VPS"
fi

echo "=== Cartridge CD_TRANSITION cross-repo (optional) ==="
CC_ROOT="${MAOS_WS}/umst-concrete-cartridge"
if [ -d "$CC_ROOT/crates/umst-concrete-cartridge/src" ]; then
  if rg -q 'CD_TRANSITION_CATALOG_ID' "$CC_ROOT/crates/umst-concrete-cartridge/src/pipeline/dual_gate.rs"; then
    echo "OK: cartridge dual_gate.rs cites CD_TRANSITION_CATALOG_ID"
  else
    warn "cartridge dual_gate.rs missing CD_TRANSITION_CATALOG_ID"
  fi
else
  warn "umst-concrete-cartridge checkout not present for cross-repo grep"
fi

echo "=== Landauer slack (W5) ==="
if rg -q 'landauer_slack_violation' src/ai/constraint_loss.rs \
  && rg -q 'lambda_landauer' src/ai/ppo.rs; then
  echo "OK: landauer_slack_violation + lambda_landauer wired"
else
  fail "landauer_slack_violation wiring incomplete"
fi

echo "=== P4 training witness JSON ==="
if [ -f artifacts/training/p4_rejection_baseline.json ]; then
  echo "OK: artifacts/training/p4_rejection_baseline.json present"
else
  warn "missing artifacts/training/p4_rejection_baseline.json"
fi

echo "=== Arena mmap (P2) ==="
if rg -q 'mmap_arena_path' umst-runtime-arena/src/mmap.rs 2>/dev/null; then
  echo "OK: umst-runtime-arena mmap module present"
else
  warn "mmap module not found"
fi
echo "=== Solver never-run ledger (Track C) ==="
if [ -f docs/SOLVER_NEVER_RUN_LEDGER.md ]; then
  IGNORE_COUNT="$(rg -c '#\[ignore' tests src --glob '*.rs' 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')"
  echo "OK: SOLVER_NEVER_RUN_LEDGER.md present (manifold #[ignore] lines: ${IGNORE_COUNT})"
else
  fail "missing docs/SOLVER_NEVER_RUN_LEDGER.md"
fi

if [ "$FAIL" -eq 0 ]; then
  echo "W9 T2.1 audit: OK (${WARN} warning(s))"
  exit 0
fi
echo "W9 T2.1 audit: FAIL (${WARN} warning(s))"
exit 1
