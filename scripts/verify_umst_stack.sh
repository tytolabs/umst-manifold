#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
#
# Local / CI parity for UMST catalog drift: cargo check, Lean export digest vs lock,
# gate + formal witness integration tests, and optional prototype adversarial gate (E6).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERIFY_STEP_LOG="$(mktemp -t umst-verify-steps.XXXXXX)"
TMP=""
TMP_DS=""
TMP_FM=""

cleanup_on_exit() {
  rm -f "${VERIFY_STEP_LOG}"
  if [[ -n "${TMP}" ]]; then
    rm -f "${TMP}" "${TMP_DS}" "${TMP_FM}"
  fi
}
trap cleanup_on_exit EXIT

verify_step_echo() {
  echo "$1"
  echo "$1" >> "${VERIFY_STEP_LOG}"
}

if [[ ! -f Cargo.toml ]]; then
  echo "verify_umst_stack: expected Cargo.toml in ${ROOT}" >&2
  exit 1
fi

resolve_formal_root() {
  if [[ -n "${UMST_FORMAL_ROOT:-}" ]]; then
    echo "$(cd "${UMST_FORMAL_ROOT}" && pwd)"
    return 0
  fi
  local sibling
  sibling="$(cd "${ROOT}/.." && pwd)/umst-formal-double-slit"
  if [[ -d "${sibling}/Lean" && -f "${sibling}/tools/lean_export/export_catalog.py" ]]; then
    echo "${sibling}"
    return 0
  fi
  return 1
}

resolve_formal_classical_root() {
  if [[ -n "${UMST_FORMAL_CLASSICAL_ROOT:-}" ]]; then
    echo "$(cd "${UMST_FORMAL_CLASSICAL_ROOT}" && pwd)"
    return 0
  fi
  local sibling
  sibling="$(cd "${ROOT}/.." && pwd)/umst-formal"
  if [[ -d "${sibling}/Lean" ]]; then
    echo "${sibling}"
    return 0
  fi
  return 1
}

echo "==> cargo check (default features)"
cargo check --verbose

echo "==> Lean catalog export regen vs artifacts/catalog.lock.json"
LOCK="${ROOT}/artifacts/catalog.lock.json"
if [[ ! -f "${LOCK}" ]]; then
  echo "FAIL: missing ${LOCK}" >&2
  exit 1
fi

echo "==> catalog.lock R0 pin (module_count=119, digest 0697014f…)"
export ROOT
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
print(f"OK: catalog.lock module_count=119 digest={digest[:8]}…")
PYLOCK

FORMAL_ROOT=""
if FORMAL_ROOT="$(resolve_formal_root)"; then
  TMP="$(mktemp -t umst-catalog.XXXXXX.json)"
  TMP_DS="$(mktemp -t umst-catalog-ds.XXXXXX.json)"
  TMP_FM="$(mktemp -t umst-catalog-fm.XXXXXX.json)"
  EXPORT_TOOL="${FORMAL_ROOT}/tools/lean_export/export_catalog.py"
  VERIFY_TOOL="${ROOT}/scripts/catalog_lock_verify.py"

  python3 "${EXPORT_TOOL}" --lean-root "${FORMAL_ROOT}/Lean" --out "${TMP_DS}"

  FIBER_VERIFY_ARGS=()
  CLASSICAL_ROOT=""
  if CLASSICAL_ROOT="$(resolve_formal_classical_root)"; then
    python3 "${EXPORT_TOOL}" --lean-root "${CLASSICAL_ROOT}/Lean" --out "${TMP_FM}"
    FIBER_VERIFY_ARGS+=(umst-formal="${TMP_FM}")
  fi
  FIBER_VERIFY_ARGS+=(umst-formal-double-slit="${TMP_DS}")

  EXPORT_ARGS=(--lean-root "${FORMAL_ROOT}/Lean")
  ALSO_LEAN="${FORMAL_ROOT}/../umst-formal/Lean"
  if [[ -d "${ALSO_LEAN}" ]]; then
    EXPORT_ARGS+=(--also-lean-root "${ALSO_LEAN}" --also-lean-repo-tag umst-formal)
    export APPROVE_CROSS_REPO_MERGE=1
    EXPORT_ARGS+=(--cross-repo-preview-out "${FORMAL_ROOT}/artifacts/catalog-cross-repo-preview.json")
  fi
  python3 "${EXPORT_TOOL}" "${EXPORT_ARGS[@]}" --out "${TMP}"

  python3 "${VERIFY_TOOL}" "${LOCK}" "${TMP}" "${FIBER_VERIFY_ARGS[@]}"
else
  if [[ "${UMST_REQUIRE_FORMAL_EXPORT:-0}" == "1" ]]; then
    echo "FAIL: UMST_REQUIRE_FORMAL_EXPORT=1 but umst-formal-double-slit not found (set UMST_FORMAL_ROOT)" >&2
    exit 1
  fi
  echo "SKIP: umst-formal-double-slit not present (set UMST_REQUIRE_FORMAL_EXPORT=1 to enforce)"
fi

run_bidirectional_catalog_check_if_present() {
  local script
  for script in     "${ROOT}/scripts/bidirectional_catalog_check.sh"     "${FORMAL_ROOT:+$FORMAL_ROOT/scripts/bidirectional_catalog_check.sh}"; do
    [[ -n "${script}" && -f "${script}" ]] || continue
    echo "==> bidirectional catalog check (${script})"
    bash "${script}"
    return 0
  done
  return 0
}
run_bidirectional_catalog_check_if_present

echo "==> cargo test -p umst-manifold (default unit + integration)"
cargo test -p umst-manifold --verbose

echo "==> gate parity + Kleisli + dual-run integration tests"
cargo test -p umst-manifold --verbose \
  --test gate_parity_fixture --test gate_kleisli --test gate_cbf_parity \
  --test gate_dual_run_parity

echo "==> formal witness + release manifest profile + ROS contract (feature-gated)"
# Release lane (R5 v1): `manifest_strict_witness` exercises StrictCatalogMatch + digest reject.
# Skip only with UMST_RELEASE_MANIFEST_PROFILE=0 (dev iteration on witness plumbing).
MANIFEST_STRICT_ARGS=(--test manifest_strict_witness)
if [[ "${UMST_RELEASE_MANIFEST_PROFILE:-1}" == "0" ]]; then
  echo "SKIP: release manifest profile (UMST_RELEASE_MANIFEST_PROFILE=0)"
  MANIFEST_STRICT_ARGS=()
fi
cargo test -p umst-manifold --verbose \
  --features formal-witness,ros2-contract,serde \
  --test formal_witness --test ros_contract_serde_roundtrip \
  "${MANIFEST_STRICT_ARGS[@]}"

verify_step_echo "==> epistemic trace schema G.2 (ros2-contract, serde)"
cargo test -p umst-manifold --verbose \
  --features ros2-contract,serde \
  --test epistemic_trace_schema

verify_step_echo "==> trace calibration G.3 (trace-calibration)"
cargo test -p umst-manifold --verbose \
  --features trace-calibration \
  --test trace_calibration

resolve_prototype_root() {
  if [[ -n "${UMST_PROTOTYPE_ROOT:-}" ]]; then
    if [[ -f "${UMST_PROTOTYPE_ROOT}/scripts/test_gate_adversarial.py" ]]; then
      echo "$(cd "${UMST_PROTOTYPE_ROOT}" && pwd)"
      return 0
    fi
    return 1
  fi
  local parent sibling
  parent="$(cd "${ROOT}/.." && pwd)"
  for sibling in umst-prototype umst-prototype_2; do
    if [[ -f "${parent}/${sibling}/scripts/test_gate_adversarial.py" ]]; then
      echo "${parent}/${sibling}"
      return 0
    fi
  done
  return 1
}

run_adversarial_gate_if_present() {
  local proto script
  if ! proto="$(resolve_prototype_root)"; then
    if [[ "${UMST_REQUIRE_ADVERSARIAL_GATE:-0}" == "1" ]]; then
      echo "FAIL: UMST_REQUIRE_ADVERSARIAL_GATE=1 but prototype adversarial script not found (set UMST_PROTOTYPE_ROOT)" >&2
      exit 1
    fi
    echo "SKIP: umst-prototype adversarial gate (scripts/test_gate_adversarial.py not found; set UMST_PROTOTYPE_ROOT)"
    return 0
  fi
  script="${proto}/scripts/test_gate_adversarial.py"
  echo "==> adversarial gate parity (${script})"
  export UMST_ADVERSARIAL_PROTO="${proto}"
  python3 "${script}"
  python3 - << 'PYADV'
import json, sys
from pathlib import Path
out = Path(__import__("os").environ["UMST_ADVERSARIAL_PROTO"]) / "results" / "adversarial_gate_test.json"
if not out.is_file():
    print("FAIL: missing adversarial output", file=sys.stderr)
    sys.exit(1)
summary = json.loads(out.read_text()).get("summary", {})
fn = summary.get("false_negatives", -1)
if fn != 0:
    print(f"FAIL: adversarial gate false_negatives={fn} (must be 0)", file=sys.stderr)
    sys.exit(1)
total = summary.get("total_test_cases", "?")
print(f"OK: adversarial gate FNR=0 ({total} cases)")
PYADV
}

echo "==> gate-server HTTP integration"
cargo test -p umst-manifold --verbose \
  --features gate-server-bin --test gate_server_http

echo "==> gate Kleisli + reject catalog_id + adversarial golden (Rust)"
cargo test -p umst-manifold --verbose \
  --test gate_kleisli --test gate_reject_catalog_id --test gate_adversarial

run_adversarial_gate_if_present

echo "==> catalog partition + incremental graph pin (R0)"
cargo test -p umst-manifold --verbose \
  --test catalog_all_ids_registered --test catalog_incremental_graph_drift

echo "==> catalog_lock_119 (lock module_count vs upstream export)"
cargo test -p umst-manifold --verbose \
  --test catalog_all_ids_registered catalog_lock_module_count_matches_upstream_export_119

echo "==> god-grade CI profile (strict manifest lane default)"
cargo test -p umst-manifold --verbose --test ci_god_grade_profile

verify_step_echo "==> epistemic trace schema + calibration + regime honesty (R6)"
cargo test -p umst-manifold --verbose \
  --features ros2-contract,serde,trace-calibration \
  --test epistemic_trace_schema --test trace_calibration \
  --test regime_soundness_claims_allowlist

echo "==> adaptive witness priority queue (tests only; not hot path)"
cargo test -p umst-manifold --verbose --test witness_priority_queue

echo "==> guard: epistemic+trace steps in verify log"
EPISTEMIC_TRACE_MARKERS=(
  "==> epistemic trace schema G.2 (ros2-contract, serde)"
  "==> trace calibration G.3 (trace-calibration)"
  "==> epistemic trace schema + calibration + regime honesty (R6)"
)
missing_steps=()
for marker in "${EPISTEMIC_TRACE_MARKERS[@]}"; do
  if ! grep -qF "${marker}" "${VERIFY_STEP_LOG}" 2>/dev/null; then
    missing_steps+=("${marker}")
  fi
done
if [[ ${#missing_steps[@]} -gt 0 ]]; then
  echo "FAIL: verify log missing epistemic/trace steps: ${missing_steps[*]}" >&2
  exit 1
fi
echo "OK: epistemic+trace steps recorded in verify log"

echo "==> W8 publish prep (machine-only; no git push)"
UMST_W8_SKIP_FULL_STACK=1 bash "${ROOT}/scripts/w8_publish_readiness.sh"

echo "verify_umst_stack: OK"
