#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
"""Verify manifold catalog.lock.json against regenerated Lean catalog exports."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any


def module_count(doc: dict[str, Any]) -> int:
    if "modules" in doc:
        return len(doc["modules"])
    if "entries" in doc:
        return len(doc["entries"])
    return 0


def composed_digest(lock: dict[str, Any]) -> str:
    if lock.get("version", 1) >= 2:
        composed = lock.get("composed_catalog_digest_hex") or ""
        upstream = lock.get("upstream_catalog_digest_hex") or ""
        if composed and upstream and composed != upstream:
            print(
                f"FAIL: composed_catalog_digest_hex != upstream_catalog_digest_hex "
                f"({composed[:12]}… vs {upstream[:12]}…)",
                file=sys.stderr,
            )
            sys.exit(1)
        return composed or upstream
    return lock.get("upstream_catalog_digest_hex", "")


def verify_incremental_module_graph(export: dict[str, Any], *, label: str) -> None:
    """Incremental catalog drift: every import edge must originate from an exported module."""
    modules = export.get("modules")
    if not isinstance(modules, list):
        print(f"FAIL: {label} export missing modules[]", file=sys.stderr)
        sys.exit(1)
    module_names = {m.get("module") for m in modules if isinstance(m, dict) and m.get("module")}
    edges = export.get("module_graph_edges")
    if not isinstance(edges, list) or not edges:
        print(f"FAIL: {label} export missing module_graph_edges[]", file=sys.stderr)
        sys.exit(1)
    for edge in edges:
        if not isinstance(edge, dict):
            print(f"FAIL: {label} malformed module_graph_edges row", file=sys.stderr)
            sys.exit(1)
        src = edge.get("from")
        if not src or src not in module_names:
            print(
                f"FAIL: {label} module_graph edge from unknown module {src!r}",
                file=sys.stderr,
            )
            sys.exit(1)
    print(f"OK: {label} module_graph_edges ({len(edges)} edges, incremental DAG)")


def verify_composed_export(
    lock: dict[str, Any], export: dict[str, Any], *, label: str
) -> None:
    want = composed_digest(lock)
    got = export.get("digest", "")
    if not want or not got:
        print("FAIL: missing composed digest fields", file=sys.stderr)
        sys.exit(1)
    if got != want:
        print(
            f"FAIL: {label} composed drift want={want} got={got}",
            file=sys.stderr,
        )
        sys.exit(1)
    lock_n = lock.get("module_count")
    exp_n = module_count(export)
    if lock_n is not None and int(lock_n) != exp_n:
        print(f"FAIL: lock module_count={lock_n} export={exp_n}", file=sys.stderr)
        sys.exit(1)
    lock_edges = lock.get("module_graph_edge_count")
    edge_n = len(export.get("module_graph_edges") or [])
    if lock_edges is not None and int(lock_edges) != edge_n:
        print(
            f"FAIL: {label} module_graph_edge_count={lock_edges} export={edge_n}",
            file=sys.stderr,
        )
        sys.exit(1)
    verify_incremental_module_graph(export, label=label)
    print(f"OK: {label} composed digest ({got[:12]}…, {exp_n} modules)")


def verify_fiber_pin(
    pin: dict[str, Any], export: dict[str, Any], *, label: str
) -> None:
    want = pin.get("catalog_digest_hex", "")
    got = export.get("digest", "")
    if not want or not got:
        print(f"FAIL: missing fiber digest for {label}", file=sys.stderr)
        sys.exit(1)
    if got != want:
        print(
            f"FAIL: {label} fiber drift want={want} got={got}",
            file=sys.stderr,
        )
        sys.exit(1)
    pin_n = pin.get("module_count")
    exp_n = module_count(export)
    if pin_n is not None and int(pin_n) != exp_n:
        print(f"FAIL: {label} module_count={pin_n} export={exp_n}", file=sys.stderr)
        sys.exit(1)
    print(f"OK: {label} fiber pin ({got[:12]}…, {exp_n} modules)")


def verify_lock_exports(
    lock_path: Path,
    composed_export_path: Path,
    fiber_exports: dict[str, Path],
) -> None:
    import os

    lock = json.loads(lock_path.read_text())
    pins = lock.get("fiber_pins")
    if pins and fiber_exports and (
        composed_export_path.resolve() == lock_path.resolve()
        or os.environ.get("UMST_VERIFY_FIBERS_ONLY") == "1"
    ):
        by_repo = {p.get("repo"): p for p in pins if p.get("repo")}
        for repo, export_path in fiber_exports.items():
            pin = by_repo.get(repo)
            if pin is None:
                print(f"FAIL: lock missing fiber_pins entry for {repo}", file=sys.stderr)
                sys.exit(1)
            export = json.loads(export_path.read_text())
            verify_fiber_pin(pin, export, label=repo)
            verify_incremental_module_graph(export, label=f"{repo} fiber")
        print("OK: dual-pin fiber verification (composed R0 checked via catalog_lock_119 tests)")
        return

    composed = json.loads(composed_export_path.read_text())

    verify_composed_export(lock, composed, label="merged export")

    pins = lock.get("fiber_pins")
    if not pins:
        return

    by_repo = {p.get("repo"): p for p in pins if p.get("repo")}
    for repo, export_path in fiber_exports.items():
        pin = by_repo.get(repo)
        if pin is None:
            print(f"FAIL: lock missing fiber_pins entry for {repo}", file=sys.stderr)
            sys.exit(1)
        export = json.loads(export_path.read_text())
        verify_fiber_pin(pin, export, label=repo)
        verify_incremental_module_graph(export, label=f"{repo} fiber")


def main() -> None:
    if len(sys.argv) < 3:
        print(
            "usage: catalog_lock_verify.py <lock.json> <composed-export.json> "
            "[repo=export.json ...]",
            file=sys.stderr,
        )
        sys.exit(2)

    lock_path = Path(sys.argv[1])
    composed_path = Path(sys.argv[2])
    fiber_exports: dict[str, Path] = {}
    for arg in sys.argv[3:]:
        if "=" not in arg:
            print(f"FAIL: expected repo=path, got {arg!r}", file=sys.stderr)
            sys.exit(2)
        repo, path = arg.split("=", 1)
        fiber_exports[repo] = Path(path)

    verify_lock_exports(lock_path, composed_path, fiber_exports)


if __name__ == "__main__":
    main()
