#!/usr/bin/env bash
# Count Tier-1 domain identifier hits under src/ and ratchet against lexicon/baseline_count.txt.
#
# Usage (from umst-manifold): bash scripts/check_domain_lexicon.sh
#
# SPDX-License-Identifier: MIT

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TOML="$ROOT/lexicon/domain_terms.toml"
BASELINE="$ROOT/lexicon/baseline_count.txt"
SCAN_ROOT="$ROOT/src"

if [[ ! -f "$TOML" ]]; then
  echo "error: missing $TOML" >&2
  exit 1
fi

TERMS=()
while IFS= read -r term; do
  [[ -n "$term" ]] && TERMS+=("$term")
done < <(python3 - "$TOML" <<'PY'
import sys, tomllib
with open(sys.argv[1], "rb") as f:
    data = tomllib.load(f)
for t in data["tier1"]["terms"]:
    print(t)
PY
)

count=0
for term in "${TERMS[@]}"; do
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    n="${line##*:}"
    count=$((count + n))
  done < <(rg -c "\\b${term}\\b" "$SCAN_ROOT" --glob '*.rs' 2>/dev/null || true)
done

baseline=0
if [[ -f "$BASELINE" ]]; then
  baseline=$(tr -d '[:space:]' < "$BASELINE")
fi

echo "domain_lexicon_tier1_hits=$count baseline=$baseline"

if (( count > baseline )); then
  echo "FAIL: Tier-1 domain lexicon hits ($count) exceed baseline ($baseline)" >&2
  exit 1
fi

echo "OK: domain lexicon within baseline"
