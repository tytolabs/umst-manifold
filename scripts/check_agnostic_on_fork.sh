#!/usr/bin/env bash
# W9 S1 grep gate: kernel src/ must not reference retired cartridge gate stubs.
#
# Usage (from umst-manifold): bash scripts/check_agnostic_on_fork.sh
#
# SPDX-License-Identifier: MIT

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SRC="$ROOT/src"

PATTERNS=(
  'gate::concrete_cartridge'
  'mod concrete_cartridge'
)

fail=0
for pat in "${PATTERNS[@]}"; do
  if rg -n "$pat" "$SRC" --glob '*.rs' 2>/dev/null; then
    echo "FAIL: agnostic-on-fork violation: $pat" >&2
    fail=1
  fi
done

if (( fail != 0 )); then
  exit 1
fi

echo "OK: agnostic-on-fork grep clean"
