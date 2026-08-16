#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
# Tier 0 doc link SSOT (M0-4) — manifold-local paths only.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL=0
for pair in \
  "src/physics/solvers/thmc.rs:docs/FP_CATEGORICAL_BURN.md" \
  "src/core/traits.rs:docs/Category-of-Material-Updates.md"; do
  src="${pair%%:*}"
  doc="${pair##*:}"
  if ! grep -q "${doc##*/}" "${ROOT}/${src}" 2>/dev/null; then
    echo "SKIP: no reference in ${src}"
    continue
  fi
  if [[ ! -f "${ROOT}/${doc}" ]]; then
    echo "FAIL: missing ${ROOT}/${doc}" >&2
    FAIL=$((FAIL + 1))
  fi
done
if [[ "${FAIL}" -gt 0 ]]; then exit 1; fi
echo "check_doc_links_tier0: OK"
