#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
#
# Bidirectional UMST catalog drift guard (see docs/VERIFY.md §5.1):
#   (1) Regenerate Lean export from UMST_FORMAL_ROOT (export_catalog.py).
#   (2) Export digest + module_count match catalog.lock.json and committed catalog.json.
#   (3) Gate catalog_id implementations + GATE_REGISTRY ⊆ formal catalog.json (anchors).
#   (4) cargo test --test catalog_all_ids_registered
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

LOCK="${ROOT}/artifacts/catalog.lock.json"
GATE_SRC="${ROOT}/src/gate"
TRACEABILITY="${ROOT}/src/runtime/catalog/traceability.rs"

if [[ ! -f "${LOCK}" ]]; then
  echo "bidirectional_catalog_check: missing ${LOCK}" >&2
  exit 1
fi

resolve_formal_root() {
  if [[ -n "${UMST_FORMAL_ROOT:-}" && -d "${UMST_FORMAL_ROOT}/Lean" ]]; then
    echo "$(cd "${UMST_FORMAL_ROOT}" && pwd)"
    return 0
  fi
  local sibling
  sibling="$(cd "${ROOT}/.." && pwd)/umst-formal-double-slit"
  if [[ -d "${sibling}/Lean" ]]; then
    echo "${sibling}"
    return 0
  fi
  if [[ -d "${ROOT}/umst-formal-double-slit/Lean" ]]; then
    echo "${ROOT}/umst-formal-double-slit"
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

if ! FORMAL_ROOT="$(resolve_formal_root)"; then
  if [[ "${UMST_REQUIRE_FORMAL_EXPORT:-0}" == "1" ]]; then
    echo "bidirectional_catalog_check: set UMST_FORMAL_ROOT to umst-formal-double-slit" >&2
    exit 1
  fi
  echo "SKIP: umst-formal-double-slit not present (UMST_REQUIRE_FORMAL_EXPORT unset)"
  echo "==> (4) catalog_all_ids_registered (Lean catalog path from DEFAULT_UPSTREAM only)"
  cargo test --test catalog_all_ids_registered
  echo "bidirectional_catalog_check: OK (formal export skipped)"
  exit 0
fi

PINNED_CATALOG="${ROOT}/artifacts/upstream_catalog.json"
EXPORT_TOOL="${FORMAL_ROOT}/tools/lean_export/export_catalog.py"
LEAN_ROOT="${FORMAL_ROOT}/Lean"
CATALOG_JSON="${FORMAL_ROOT}/artifacts/catalog.json"
VERIFY_TOOL="${ROOT}/scripts/catalog_lock_verify.py"

if [[ ! -d "${LEAN_ROOT}" ]]; then
  echo "bidirectional_catalog_check: invalid formal tree at ${FORMAL_ROOT}" >&2
  exit 1
fi

if [[ ! -f "${EXPORT_TOOL}" ]]; then
  if [[ -f "${CATALOG_JSON}" ]]; then
    echo "==> (1–2) lock vs committed formal catalog.json (export tool absent)"
    python3 "${VERIFY_TOOL}" "${LOCK}" "${CATALOG_JSON}"
  elif [[ -f "${PINNED_CATALOG}" ]]; then
    echo "==> (1–2) lock vs pinned artifacts/upstream_catalog.json (export tool absent)"
    python3 "${VERIFY_TOOL}" "${LOCK}" "${PINNED_CATALOG}"
  else
    echo "bidirectional_catalog_check: no export tool and no catalog.json at ${FORMAL_ROOT} or ${PINNED_CATALOG}" >&2
    exit 1
  fi
  echo "==> (4) catalog_all_ids_registered"
  cargo test --test catalog_all_ids_registered
  echo "bidirectional_catalog_check: OK (pinned/committed catalog; live export skipped)"
  exit 0
fi

if [[ ! -f "${CATALOG_JSON}" ]]; then
  echo "bidirectional_catalog_check: missing ${CATALOG_JSON} (run make lean-catalog-export)" >&2
  exit 1
fi

TMP="$(mktemp -t umst-catalog-bidir.XXXXXX.json)"
TMP_DS="$(mktemp -t umst-catalog-bidir-ds.XXXXXX.json)"
TMP_FM="$(mktemp -t umst-catalog-bidir-fm.XXXXXX.json)"
trap 'rm -f "${TMP}" "${TMP_DS}" "${TMP_FM}"' EXIT
VERIFY_TOOL="${ROOT}/scripts/catalog_lock_verify.py"

echo "==> (1) Regenerate catalog export from ${FORMAL_ROOT}"
python3 "${EXPORT_TOOL}" --lean-root "${LEAN_ROOT}" --out "${TMP_DS}"

FIBER_VERIFY_ARGS=()
CLASSICAL_ROOT=""
if CLASSICAL_ROOT="$(resolve_formal_classical_root)"; then
  python3 "${EXPORT_TOOL}" --lean-root "${CLASSICAL_ROOT}/Lean" --out "${TMP_FM}"
  FIBER_VERIFY_ARGS+=(umst-formal="${TMP_FM}")
fi
FIBER_VERIFY_ARGS+=(umst-formal-double-slit="${TMP_DS}")

EXPORT_ARGS=(--lean-root "${LEAN_ROOT}")
ALSO_LEAN="${FORMAL_ROOT}/../umst-formal/Lean"
if [[ -d "${ALSO_LEAN}" ]]; then
  EXPORT_ARGS+=(--also-lean-root "${ALSO_LEAN}" --also-lean-repo-tag umst-formal)
  export APPROVE_CROSS_REPO_MERGE=1
fi
python3 "${EXPORT_TOOL}" "${EXPORT_ARGS[@]}" --out "${TMP}"

echo "==> (2) Compare export digest to manifold lock, fiber pins, and committed catalog.json"
python3 "${VERIFY_TOOL}" "${LOCK}" "${TMP}" "${FIBER_VERIFY_ARGS[@]}"

export LOCK TMP CATALOG_JSON
python3 - << 'PY'
import json
import os
import sys
from pathlib import Path

lock = json.loads(Path(os.environ["LOCK"]).read_text())
export = json.loads(Path(os.environ["TMP"]).read_text())
committed = json.loads(Path(os.environ["CATALOG_JSON"]).read_text())

digest = export.get("digest", "")
committed_digest = committed.get("digest", "")
want = lock.get("composed_catalog_digest_hex") or lock.get("upstream_catalog_digest_hex", "")

if not want or not digest:
    print("FAIL: missing digest fields", file=sys.stderr)
    sys.exit(1)

if committed_digest != digest:
    print(
        f"FAIL: committed catalog.json drift want={digest} got={committed_digest}",
        file=sys.stderr,
    )
    print(
        "Re-run: make lean-catalog-export in umst-formal-double-slit, "
        "commit artifacts/catalog.json, update catalog.lock.json fiber_pins / composed digest",
        file=sys.stderr,
    )
    sys.exit(1)

def module_count(doc: dict) -> int:
    if "modules" in doc:
        return len(doc["modules"])
    if "entries" in doc:
        return len(doc["entries"])
    return 0

exp_n = module_count(export)
com_n = module_count(committed)
if com_n != exp_n:
    print(f"FAIL: committed catalog module_count={com_n} export={exp_n}", file=sys.stderr)
    sys.exit(1)

print(f"OK: committed catalog.json matches regen ({digest[:12]}…, {exp_n} modules)")
PY

echo "==> (3) Gate catalog_id ⊆ formal catalog.json"
export ROOT LOCK GATE_SRC CATALOG_JSON TRACEABILITY
python3 - << 'PY'
import json
import re
import sys
from pathlib import Path

import os

root = Path(os.environ["ROOT"])
catalog_path = Path(os.environ["CATALOG_JSON"])
gate_dir = Path(os.environ["GATE_SRC"])
trace_path = Path(os.environ["TRACEABILITY"])

catalog_text = catalog_path.read_text(encoding="utf-8")
catalog = json.loads(catalog_text)

module_names: set[str] = set()
if "modules" in catalog:
    module_names = {m.get("module", "") for m in catalog["modules"]}
elif "entries" in catalog:
    for e in catalog["entries"]:
        module_names.add(e.get("name", ""))
        mid = e.get("id", "")
        if mid:
            module_names.add(mid.split(".")[-1])

ANCHOR = {
    "umst.gate.cd_transition": "Compat.Gate",
    "umst.gate.http_shim": "GateCompat",
    "umst.gate.kleisli_unit": "ProbeOptimization",
    "thermodynamic_mix": "GateCompat",
    "umst.cartridge.concrete.policy": None,
}

trace_body = trace_path.read_text(encoding="utf-8")


def rust_pub_const_str_ids(source: str, const_name: str) -> list[str]:
    """Parse `pub const NAME: &[&str] = &[ ... ];` without matching doc/backtick mentions."""
    marker = f"pub const {const_name}"
    idx = source.find(marker)
    if idx < 0:
        return []
    chunk = source[idx:].split("];", 1)[0]
    return re.findall(r'"([^"]+)"', chunk)


gate_reg = rust_pub_const_str_ids(trace_body, "GATE_REGISTRY_CATALOG_IDS")
gate_allow = set(rust_pub_const_str_ids(trace_body, "ALLOW_UNUSED_GATE_CATALOG_IDS"))

impl_pat = re.compile(
    r'fn catalog_id\(&self\) -> &\'static str \{\s*\n\s*"([^"]+)"',
    re.MULTILINE,
)
impl_ids: set[str] = set()
for path in sorted(gate_dir.glob("*.rs")):
    impl_ids.update(impl_pat.findall(path.read_text(encoding="utf-8")))

check_ids = sorted(set(gate_reg) | impl_ids)
if not check_ids:
    print("FAIL: no gate catalog_id sources", file=sys.stderr)
    sys.exit(1)


def anchored(cid: str) -> bool:
    if cid in catalog_text:
        return True
    if cid in gate_allow:
        return True
    mod = ANCHOR.get(cid)
    if mod is None:
        return False
    if mod not in module_names:
        return False
    return mod in catalog_text


fail = False
for cid in check_ids:
    if anchored(cid):
        how = "literal in catalog.json" if cid in catalog_text else (
            "ALLOW_UNUSED_GATE" if cid in gate_allow else f"Lean module {ANCHOR.get(cid)!r}"
        )
        print(f"OK: {cid} ({how})")
        continue
    print(f"FAIL: {cid} not in catalog.json and not allowlisted/anchored", file=sys.stderr)
    fail = True

# GATE_REGISTRY must be subset of catalog (literal, allowlist, or Lean wiring — same as Rust test intent)
for cid in gate_reg:
    if cid in gate_allow:
        continue
    if cid in catalog_text:
        continue
    mod = ANCHOR.get(cid)
    if mod and mod in module_names and mod in catalog_text:
        continue
    print(f"FAIL: GATE_REGISTRY {cid} not subset of catalog.json", file=sys.stderr)
    fail = True

if fail:
    sys.exit(1)
PY

echo "==> (4) cargo test --test catalog_all_ids_registered"
export UMST_LEAN_CATALOG_JSON="${CATALOG_JSON}"
cargo test --test catalog_all_ids_registered

echo "bidirectional_catalog_check: OK"
