#!/usr/bin/env bash
# W9 S1 headline verifier: kernel src/ must be agnostic-on-fork (zero domain identifiers).
#
# Usage (from umst-manifold): bash scripts/check_agnostic_on_fork.sh
#
# SPDX-License-Identifier: MIT

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

PATTERN='\b(concrete|cement|hydration|powers|ConcreteCartridge|MixTensor|mix_proposal)\b'

HITS=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    *W9_MIGRATION*|*agnostic_on_fork_allowlist*) continue ;;
  esac
  if grep -vE 'serde\(rename\s*=' "$file" | grep -qE "$PATTERN"; then
    HITS+=("$file")
  fi
done < <(git -C "$ROOT" ls-files src)

if ((${#HITS[@]} > 0)); then
  echo "FAIL: agnostic-on-fork violations (${#HITS[@]} files):" >&2
  printf '  %s\n' "${HITS[@]}" >&2
  exit 1
fi

echo "OK: agnostic-on-fork grep clean (0 files)"
