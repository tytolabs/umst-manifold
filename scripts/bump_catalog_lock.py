#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
"""Bump artifacts/catalog.lock.json and artifacts/upstream_catalog.json to the current formal trees.

Every field is computed from the exports bidirectional_catalog_check.sh compares against: the double-slit
fiber, the umst-formal fiber and their merge (double-slit's committed artifacts/catalog.json, regenerated with
`make lean-catalog-export` there). Run after a formal release, then run bidirectional_catalog_check.sh.
"""
from __future__ import annotations

import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DS = ROOT.parent / "umst-formal-double-slit"
FM = ROOT.parent / "umst-formal"
EXPORT = DS / "tools/lean_export/export_catalog.py"
sys.path.insert(0, str(ROOT / "scripts"))
sys.dont_write_bytecode = True
import catalog_lock_verify as verify  # noqa: E402


def export(lean_root: Path) -> dict:
    with tempfile.TemporaryDirectory() as d:
        out = Path(d) / "catalog.json"
        subprocess.run([sys.executable, str(EXPORT), "--lean-root", str(lean_root), "--out", str(out)],
                       check=True, capture_output=True)
        return json.loads(out.read_text(encoding="utf-8"))


def short_head(repo: Path) -> str:
    return subprocess.run(["git", "-C", str(repo), "rev-parse", "--short", "HEAD"],
                          capture_output=True, text=True, check=True).stdout.strip()


def main() -> int:
    merged_path = DS / "artifacts/catalog.json"
    merged = json.loads(merged_path.read_text(encoding="utf-8"))
    fibers = {"umst-formal-double-slit": export(DS / "Lean"), "umst-formal": export(FM / "Lean")}
    lock_path = ROOT / "artifacts/catalog.lock.json"
    lock = json.loads(lock_path.read_text(encoding="utf-8"))
    for pin in lock.get("fiber_pins", []):
        fiber = fibers.get(pin.get("repo"))
        if fiber is not None:
            pin["catalog_digest_hex"] = fiber["digest"]
            pin["module_count"] = len(fiber["modules"])
    lock["upstream_catalog_digest_hex"] = merged["digest"]
    lock["composed_catalog_digest_hex"] = merged["digest"]
    lock["module_count"] = len(merged["modules"])
    lock["module_graph_edge_count"] = len(merged["module_graph_edges"])
    lock["composed_primary_fiber_fingerprint_hex"] = verify.primary_fiber_fingerprint(lock)
    lock["notes"] = (f"Catalog bump: umst-formal {short_head(FM)} + double-slit {short_head(DS)} exports; merged "
                     f"{len(merged['modules'])} modules / {len(merged['module_graph_edges'])} edges. "
                     "UCRS fiber pin unchanged (preview). scripts/bump_catalog_lock.py")
    lock_path.write_text(json.dumps(lock, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    shutil.copyfile(merged_path, ROOT / "artifacts/upstream_catalog.json")
    print(f"catalog lock: {lock['module_count']} modules, {lock['module_graph_edge_count']} edges, "
          f"composed {merged['digest'][:12]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
