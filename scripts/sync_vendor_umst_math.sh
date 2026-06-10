#!/usr/bin/env bash
# Refresh vendor/umst-math from sibling egoff (local workspace layout).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="${EGOFF_ROOT:-$ROOT/../egoff}/umst-math"
DST="$ROOT/vendor/umst-math"
if [[ ! -f "$SRC/Cargo.toml" ]]; then
  echo "error: umst-math not found at $SRC" >&2
  exit 1
fi
rsync -a --delete --exclude target "$SRC/" "$DST/"
echo "synced $SRC -> $DST"
