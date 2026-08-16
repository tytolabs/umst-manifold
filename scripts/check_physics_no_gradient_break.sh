#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
# Fail CI if `into_data` / `into_scalar` appear under src/physics outside the audited allowlist.
# Tests may use these in #[cfg(test)] modules in the same files — allowlist is per-file for simplicity.
#
# Usage (from repo root): bash umst-manifold/scripts/check_physics_no_gradient_break.sh
# Usage (from umst-manifold): bash scripts/check_physics_no_gradient_break.sh
#
# Requires bash 3.2+ (macOS / Ubuntu); avoids mapfile for portability.
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ALLOWLIST="$SCRIPT_DIR/physics_gradient_escape_allowlist.txt"
PHYS_DIR="$ROOT/src/physics"

if [[ ! -d "$PHYS_DIR" ]]; then
	echo "error: expected physics directory at $PHYS_DIR" >&2
	exit 1
fi

if [[ ! -f "$ALLOWLIST" ]]; then
	echo "error: missing allowlist $ALLOWLIST" >&2
	exit 1
fi

ALLOWED=()
while IFS= read -r line || [[ -n "${line-}" ]]; do
	[[ -z "${line// /}" ]] && continue
	ALLOWED+=("$line")
done < <(grep -v '^#' "$ALLOWLIST" | sed '/^[[:space:]]*$/d')

is_allowed() {
	local rel="$1"
	local a
	for a in "${ALLOWED[@]}"; do
		if [[ "$rel" == "$a" ]]; then
			return 0
		fi
	done
	return 1
}

PATTERN='into_scalar|into_data'

hit_files=()
while IFS= read -r -d '' f; do
	if grep -qE "$PATTERN" "$f" 2>/dev/null; then
		hit_files+=("$f")
	fi
done < <(find "$PHYS_DIR" -name '*.rs' -print0)

if [[ ${#hit_files[@]} -eq 0 ]]; then
	echo "OK: no into_data / into_scalar under src/physics"
	exit 0
fi

status=0
for file in "${hit_files[@]}"; do
	rel="${file#"$ROOT"/}"
	if is_allowed "$rel"; then
		echo "NOTE (allowlisted): $rel"
		continue
	fi
	echo "error: forbidden gradient escape pattern(s) in $rel (not in physics_gradient_escape_allowlist.txt)" >&2
	grep -nE "$PATTERN" "$file" >&2 || true
	status=1
done

if [[ "$status" -ne 0 ]]; then
	echo "Fix: use Burn tensor ops, or after human audit add path to scripts/physics_gradient_escape_allowlist.txt with justification." >&2
	exit 1
fi

echo "OK: physics gradient escape check passed (allowlisted files only)."
exit 0
