#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
# W9 S2 verifier: Tier-1 domain lexicon in kernel src must be ZERO (no baseline ratchet).
#
# Usage (from umst-manifold): bash scripts/check_domain_lexicon.sh
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TOML="$ROOT/lexicon/domain_terms.toml"

if [[ ! -f "$TOML" ]]; then
  echo "error: missing $TOML" >&2
  exit 1
fi

count=$(python3 - "$ROOT" "$TOML" <<'PY'
import re, sys, tomllib
from pathlib import Path

root = Path(sys.argv[1])
with open(sys.argv[2], "rb") as f:
    terms = tomllib.load(f)["tier1"]["terms"]

count = 0
for path in sorted(root.joinpath("src").rglob("*.rs")):
    if path.name in ("w9_migration.rs", "traceability.rs"):
        continue
    lines = path.read_text().splitlines()
    for line in lines:
        if "serde(rename" in line:
            continue
        for term in terms:
            if re.search(rf"\b{re.escape(term)}\b", line):
                count += 1
print(count)
PY
)

echo "domain_lexicon_tier1_hits=$count"

if (( count != 0 )); then
  echo "FAIL: Tier-1 domain lexicon must be 0 (got $count)" >&2
  exit 1
fi

echo "OK: domain lexicon zero"
