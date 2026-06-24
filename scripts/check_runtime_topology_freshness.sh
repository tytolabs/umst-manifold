#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Fail when RUNTIME_TOPOLOGY.md labels shipped arena/warm-path items as planned/skeleton.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOC="${ROOT}/docs/RUNTIME_TOPOLOGY.md"
FAIL=0

if [[ ! -f "${DOC}" ]]; then
  echo "FAIL: missing ${DOC}" >&2
  exit 1
fi

# Shipped table rows must not also claim planned/skeleton on the same line.
while IFS= read -r line; do
  if echo "${line}" | grep -qiE '\*\*shipped\*\*'; then
    if echo "${line}" | grep -qiE '\b(planned|skeleton)\b'; then
      echo "FAIL: shipped row contains planned/skeleton: ${line}" >&2
      FAIL=$((FAIL + 1))
    fi
  fi
done < "${DOC}"

# Known shipped symbols must not be tagged planned/skeleton anywhere in the doc.
SHIPPED_SYMBOLS=(
  load_arena
  mmap_arena_path
  seal_arena_commit
  umst-runtime-arena
  UmstArenaView
)
for sym in "${SHIPPED_SYMBOLS[@]}"; do
  while IFS= read -r line; do
    if ! echo "${line}" | grep -qiE '\b(planned|skeleton)\b'; then
      continue
    fi
    if echo "${line}" | grep -qiE "${sym}[^|\"]*shipped|shipped[^|\"]*${sym}"; then
      continue
    fi
    echo "FAIL: shipped symbol '${sym}' appears on planned/skeleton line: ${line}" >&2
    FAIL=$((FAIL + 1))
  done < <(grep -n "${sym}" "${DOC}" || true)
done

# Doc-wide skeleton is forbidden in RUNTIME_TOPOLOGY (other docs may use DEC skeleton).
if grep -qi 'skeleton' "${DOC}"; then
  echo "FAIL: RUNTIME_TOPOLOGY.md must not use 'skeleton' (use shipped/planned/live)" >&2
  grep -ni 'skeleton' "${DOC}" >&2 || true
  FAIL=$((FAIL + 1))
fi

if [[ "${FAIL}" -gt 0 ]]; then
  exit 1
fi

echo "check_runtime_topology_freshness: OK"
