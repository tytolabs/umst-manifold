#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
#
# W8 publish *prep* gate (machine-verified, no git push).
# Checks: lock 119, digest 0697014f, 16/16 checklist evidence, manifest-bridge
# (git-pinned cartridge G-02 OR workspace [patch]), no dirty secrets, Phase-0 preflight.
# Operator push/clone/GHA remain human-only.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORKSPACE="$(cd "${ROOT}/.." && pwd)"
MANIFOLD="${ROOT}"
CONCRETE="${WORKSPACE}/umst-concrete-cartridge"
FORMAL_DS="${WORKSPACE}/umst-formal-double-slit"

PASS=0
FAIL=0
SKIP=0

step() {
  echo ""
  echo "==> $*"
}

ok() {
  echo "OK: $*"
  PASS=$((PASS + 1))
}

fail() {
  echo "FAIL: $*" >&2
  FAIL=$((FAIL + 1))
}

skip() {
  echo "SKIP: $*"
  SKIP=$((SKIP + 1))
}

require_file() {
  if [[ ! -f "$1" ]]; then
    fail "missing file: $1"
    return 1
  fi
  ok "present $(basename "$1")"
}

grep_q() {
  grep -q "$@" 2>/dev/null
}

# --- 0. Workspace layout ---
step "workspace layout"
require_file "${MANIFOLD}/Cargo.toml"
require_file "${MANIFOLD}/artifacts/catalog.lock.json"
if [[ -d "${CONCRETE}" ]]; then
  ok "umst-concrete-cartridge sibling"
else
  skip "umst-concrete-cartridge not in workspace (manifest-bridge tests skipped)"
fi

# --- 1. W8 API surface (manifold) ---
step "W8 manifold API surface"
require_file "${MANIFOLD}/src/lib.rs"
if grep_q 'pub mod manifest' "${MANIFOLD}/src/lib.rs"; then
  ok "pub mod manifest in lib.rs"
else
  fail "lib.rs missing pub mod manifest"
fi
if grep_q '^manifest-bridge = \[\]' "${MANIFOLD}/Cargo.toml" \
  && grep_q '^manifold-manifest = \[\]' "${MANIFOLD}/Cargo.toml"; then
  ok "manifest-bridge + manifold-manifest features"
else
  fail "Cargo.toml missing manifest-bridge / manifold-manifest features"
fi

# --- 2. Catalog lock R0 pin (119 / 0697014f) ---
step "catalog.lock R0 pin"
export ROOT="${MANIFOLD}"
python3 - << 'PYLOCK'
import json, os, sys
from pathlib import Path
lock_path = Path(os.environ["ROOT"]) / "artifacts" / "catalog.lock.json"
lock = json.loads(lock_path.read_text())
count = lock.get("module_count")
digest = lock.get("upstream_catalog_digest_hex") or lock.get("composed_catalog_digest_hex") or ""
if count != 119:
    print(f"FAIL: catalog.lock module_count={count!r} (expected 119)", file=sys.stderr)
    sys.exit(1)
if not str(digest).startswith("0697014f"):
    print(f"FAIL: catalog.lock digest prefix (got {digest!r})", file=sys.stderr)
    sys.exit(1)
print(f"OK: catalog.lock module_count=119 digest={digest[:16]}…")
PYLOCK
ok "catalog.lock digest prefix 0697014f"

# --- 3. No dirty secrets ---
step "secrets hygiene (no .env / credentials in git index)"
cd "${MANIFOLD}"
SECRET_PATTERN='(AKIA[0-9A-Z]{16}|ghp_[a-zA-Z0-9]{20,}|github_pat_[a-zA-Z0-9_]{20,}|sk-[a-zA-Z0-9]{20,}|BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY)'
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  BAD="$(git ls-files 2>/dev/null | grep -iE '\.env$|credentials\.json|\.pem$|id_rsa|secrets\.ya?ml' || true)"
  if [[ -n "${BAD}" ]]; then
    fail "tracked secret-like paths in umst-manifold: ${BAD}"
  else
    ok "no tracked secret-like paths in umst-manifold"
  fi
  if git diff 2>/dev/null | grep -qE "${SECRET_PATTERN}"; then
    fail "secret-like pattern in working tree diff"
  fi
  if git diff --cached 2>/dev/null | grep -qE "${SECRET_PATTERN}"; then
    fail "secret-like pattern in staged diff"
  fi
  STAGED="$(git diff --cached --name-only 2>/dev/null || true)"
  STAGED_BAD="$(printf '%s\n' "${STAGED}" | grep -iE '\.env$|credentials\.json|\.pem$|id_rsa|secrets\.ya?ml' || true)"
  if [[ -n "${STAGED_BAD}" ]]; then
    fail "staged secret-like paths in umst-manifold: ${STAGED_BAD}"
  else
    ok "no .env or credential paths in staged files"
  fi
  REV="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
  ok "manifold HEAD ${REV}"
else
  skip "umst-manifold not a git repo"
fi

# --- 4. God-grade checklist 16/16 evidence (verify_umst_stack wiring) ---
step "god-grade checklist 16/16 evidence (verify_umst_stack.sh)"
STACK="${MANIFOLD}/scripts/verify_umst_stack.sh"
require_file "${STACK}"
REQUIRED_MARKERS=(
  'module_count=119'
  'catalog_all_ids_registered'
  'gate_cbf_parity'
  'catalog.lock.json'
  'gate_parity_fixture'
  'gate_kleisli'
  'gate_dual_run_parity'
  'gate_reject_catalog_id'
  'gate_adversarial'
  'manifest_strict_witness'
  'epistemic_trace_schema'
  'trace_calibration'
  'regime_soundness_claims_allowlist'
  'witness_priority_queue'
  'catalog_incremental_graph_drift'
  'ci_god_grade_profile'
)
missing=()
for m in "${REQUIRED_MARKERS[@]}"; do
  if ! grep_q "${m}" "${STACK}"; then
    missing+=("${m}")
  fi
done
if [[ ${#missing[@]} -gt 0 ]]; then
  fail "verify_umst_stack.sh missing 16/16 markers: ${missing[*]}"
else
  ok "16/16 automation evidence markers in verify_umst_stack.sh"
fi

# --- 5. Manifold preflight (Phase 0) ---
step "manifold cargo check + test"
cd "${MANIFOLD}"
cargo check -p umst-manifold --quiet
ok "cargo check umst-manifold"
# Library tests only (integration test `w8_publish_readiness` invokes this script).
UMST_RELEASE_MANIFEST_PROFILE="${UMST_RELEASE_MANIFEST_PROFILE:-0}" \
  cargo test -p umst-manifold --lib --quiet
ok "cargo test umst-manifold --lib"

step "manifest in public API (cargo doc)"
cargo doc --no-deps -p umst-manifold --quiet
if [[ -f "${MANIFOLD}/target/doc/umst_manifold/manifest/index.html" ]]; then
  ok "cargo doc published manifest module"
else
  fail "cargo doc missing public manifest/index.html"
fi

# --- 6. TCB unchanged (single physicalSecondLaw) ---
step "TCB LandauerLaw axiom count"
if [[ -f "${FORMAL_DS}/Lean/LandauerLaw.lean" ]]; then
  AXIOM_CT="$(grep -c '^axiom ' "${FORMAL_DS}/Lean/LandauerLaw.lean" 2>/dev/null || echo 0)"
  if [[ "${AXIOM_CT}" == "1" ]] && grep_q 'physicalSecondLaw' "${FORMAL_DS}/Lean/LandauerLaw.lean"; then
    ok "LandauerLaw.lean single axiom physicalSecondLaw"
  else
    fail "LandauerLaw.lean axiom count=${AXIOM_CT} (expected 1 physicalSecondLaw)"
  fi
else
  skip "umst-formal-double-slit absent (TCB check skipped)"
fi

# --- 7. Stack verify + bidirectional (optional; prep default skips recursion) ---
if [[ "${UMST_W8_RUN_FULL_STACK:-0}" != "1" && "${UMST_W8_SKIP_FULL_STACK:-1}" == "1" ]]; then
  skip "verify_umst_stack.sh (set UMST_W8_RUN_FULL_STACK=1 for full stack)"
  skip "bidirectional_catalog_check.sh (set UMST_W8_RUN_FULL_STACK=1 for full stack)"
else
  step "verify_umst_stack.sh (UMST_REQUIRE_FORMAL_EXPORT=1 when formal sibling present)"
  cd "${MANIFOLD}"
  export UMST_REQUIRE_FORMAL_EXPORT=1
  if [[ -d "${FORMAL_DS}/Lean" ]]; then
    export UMST_FORMAL_ROOT="${FORMAL_DS}"
  fi
  bash scripts/verify_umst_stack.sh
  ok "verify_umst_stack.sh exit 0"

  step "bidirectional_catalog_check.sh"
  bash scripts/bidirectional_catalog_check.sh
  ok "bidirectional_catalog_check.sh exit 0"
fi

# --- 8. Concrete cartridge manifest-bridge (git pin or local [patch]) ---
if [[ -d "${CONCRETE}" ]]; then
  step "concrete manifest-bridge (git-pinned or workspace patch)"
  cd "${CONCRETE}"
  CARTRIDGE_CRATE="${CONCRETE}/crates/umst-concrete-cartridge/Cargo.toml"
  if grep_q '\[patch\."https://github.com/tytolabs/umst-manifold.git"\]' Cargo.toml \
    && grep_q 'path = "../umst-manifold"' Cargo.toml; then
    ok "workspace [patch] to sibling umst-manifold (local W8 dev)"
  elif [[ -f "${CARTRIDGE_CRATE}" ]] \
    && grep_q 'umst-manifold = { git = "https://github.com/tytolabs/umst-manifold.git"' "${CARTRIDGE_CRATE}" \
    && grep -qE 'rev = "[0-9a-f]{7,40}"' "${CARTRIDGE_CRATE}" 2>/dev/null; then
    MANIFOLD_REV="$(
      grep -E 'umst-manifold = \{ git' "${CARTRIDGE_CRATE}" \
        | sed -n 's/.*rev = "\([^"]*\)".*/\1/p' | head -1
    )"
    ok "git-pinned umst-manifold rev=${MANIFOLD_REV:0:12}… (G-02; no workspace [patch])"
  else
    fail "concrete needs workspace [patch] to ../umst-manifold OR git rev pin on umst-manifold"
  fi
  cargo test -p umst-concrete-cartridge --features manifest-bridge --lib --quiet
  ok "cargo test manifest-bridge (lib)"
  if [[ -f crates/umst-concrete-cartridge/tests/manifest_bridge_catalog_grounding.rs ]]; then
    cargo test -p umst-concrete-cartridge --features manifest-bridge \
      --test manifest_bridge_catalog_grounding --quiet
    ok "manifest_bridge_catalog_grounding"
  fi
  if [[ -f crates/umst-concrete-cartridge/tests/formal_anchors.rs ]]; then
    cargo test -p umst-concrete-cartridge --features manifest-bridge \
      --test formal_anchors --quiet
    ok "formal_anchors manifest-bridge"
  fi
else
  skip "concrete cartridge tests (repo absent)"
fi

# --- 9. Remote publish NOT attempted (policy) ---
step "publish policy (no git push / gh / cargo publish)"
ok "script does not invoke git push, gh, or cargo publish"

# --- Summary ---
echo ""
echo "w8_publish_readiness: PASS=${PASS} FAIL=${FAIL} SKIP=${SKIP}"
if [[ "${FAIL}" -gt 0 ]]; then
  echo "w8_publish_readiness: FAILED (${FAIL} checks)" >&2
  exit 1
fi
echo "w8_publish_readiness: READY (prep automated; publish remains human-only)"
echo "w8_publish_readiness: OK"
exit 0
