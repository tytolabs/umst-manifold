#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""SSOT check: Lean declaration counts must match committed snapshot (never hand-type in READMEs)."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

SNAPSHOT = Path(__file__).resolve().parent / "theorem_counts_snapshot.json"

FORMAL_STATS_REL = Path("umst-formal") / "scripts" / "lean_declaration_stats.py"
DOUBLE_SLIT_STATS_REL = (
    Path("umst-formal-double-slit") / "scripts" / "lean_declaration_stats.py"
)


def monorepo_workspace_root(start: Path) -> Path | None:
    """tyto-workspace root: ancestor directory containing formal Lean stats scripts."""
    for parent in start.parents:
        if (parent / FORMAL_STATS_REL).is_file() and (
            parent / DOUBLE_SLIT_STATS_REL
        ).is_file():
            return parent
    return None


def resolve_repos() -> tuple[Path | None, dict[str, Path]]:
    workspace = monorepo_workspace_root(Path(__file__).resolve())
    if workspace is None:
        return None, {}
    repos = {
        "umst-formal": workspace / FORMAL_STATS_REL,
        "umst-formal-double-slit": workspace / DOUBLE_SLIT_STATS_REL,
    }
    return workspace, repos


def run_stats(script: Path) -> dict:
    repo = script.parent.parent
    out = subprocess.check_output(
        ["python3", str(script), "--json"],
        cwd=repo,
        text=True,
    )
    return json.loads(out)


def main() -> int:
    if not SNAPSHOT.is_file():
        print(f"FAIL: missing snapshot {SNAPSHOT}", file=sys.stderr)
        return 1

    workspace, repos = resolve_repos()
    if workspace is None:
        print(
            "SKIP: formal siblings missing (no umst-formal + umst-formal-double-slit "
            "with lean_declaration_stats.py in ancestor workspace); "
            "run from tyto-workspace monorepo for SSOT check",
            file=sys.stderr,
        )
        return 0

    expected = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    errors: list[str] = []

    for name, script in repos.items():
        if not script.is_file():
            errors.append(f"{name}: stats script missing at {script}")
            continue
        got = run_stats(script)
        want = expected.get(name)
        if want is None:
            errors.append(f"{name}: no snapshot entry")
            continue
        mapping = {
            "lake_roots": got.get("lake_roots_count"),
            "theorem": got.get("roots_only", {}).get("theorem"),
            "lemma": got.get("roots_only", {}).get("lemma"),
        }
        for key in ("lake_roots", "theorem", "lemma"):
            if mapping[key] != want.get(key):
                errors.append(
                    f"{name}: {key} want={want.get(key)} got={mapping[key]} "
                    f"(re-run stats and update theorem_counts_snapshot.json)"
                )

    if errors:
        for e in errors:
            print(f"FAIL: {e}", file=sys.stderr)
        return 1

    print("OK: theorem counts match SSOT snapshot")
    for name in repos:
        w = expected[name]
        print(
            f"  {name}: {w['lake_roots']} roots, "
            f"{w['theorem']} theorem, {w['lemma']} lemma"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
