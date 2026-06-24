#!/usr/bin/env bash
# W9 T2.1 — kernel catalog_id grep vs cartridge registry parity (Goal C).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FAIL=0

echo "=== W9 T2.1 catalog_id kernel grep ==="
REGISTRY_IDS="$(rg -o 'pub const [A-Z0-9_]+_CATALOG_ID: &str = "[^"]+"' src/runtime/catalog/traceability.rs | sed 's/.*"\(.*\)"/\1/' | sort -u)"
KERNEL_HITS="$(rg -o 'umst\.[a-z0-9._-]+|thermodynamic_mix' src --glob '*.rs' | sort -u)"

while IFS= read -r id; do
  if ! echo "$KERNEL_HITS" | grep -qx "$id"; then
    echo "WARN: registry id not referenced in src kernel: $id"
  fi
done <<< "$REGISTRY_IDS"

echo "=== CD transition catalog lock ==="
rg -n "CD_TRANSITION_CATALOG_ID" src/runtime/catalog/traceability.rs src/gate/ src/ai/constraint_loss.rs >/dev/null \
  || { echo "FAIL: CD_TRANSITION_CATALOG_ID wiring"; FAIL=1; }

if [ "$FAIL" -eq 0 ]; then
  echo "W9 T2.1 audit: OK (warnings are informational)"
  exit 0
fi
echo "W9 T2.1 audit: FAIL"
exit 1
