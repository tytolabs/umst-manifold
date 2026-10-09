#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
"""SSOT check: Lean declaration counts of the pinned formal siblings match the committed snapshot.

The snapshot records, per formal sibling, the commit it was measured at. That commit must equal
the sibling's pin in `.umst-pins.toml`, so a pin bump without a fresh snapshot fails here. Counts are
measured on the pinned commit itself (`git archive <sha>` into a temporary directory), so the check
gives the same answer in CI (siblings cloned at the pin) and in the monorepo (siblings at any HEAD).

Usage:
  python3 scripts/check_theorem_counts_ssot.py           # verify
  python3 scripts/check_theorem_counts_ssot.py --write   # re-measure at the pins and rewrite the snapshot
"""

from __future__ import annotations

import argparse
import io
import json
import subprocess
import sys
import tarfile
import tempfile
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
SNAPSHOT = HERE / "theorem_counts_snapshot.json"
PINS = REPO / ".umst-pins.toml"
SIBLINGS = ("umst-formal", "umst-formal-double-slit")
STATS_REL = Path("scripts") / "lean_declaration_stats.py"


def sibling_root(name: str) -> Path | None:
    """Nearest ancestor directory holding `<name>/scripts/lean_declaration_stats.py`."""
    for parent in REPO.parents:
        if (parent / name / STATS_REL).is_file():
            return parent / name
    return None


def pinned_sha(name: str) -> str:
    pins = tomllib.loads(PINS.read_text(encoding="utf-8"))
    return pins[name]["sha"]


def counts_at(repo: Path, sha: str) -> dict:
    """Run the sibling's own stats script on the tree of commit `sha`."""
    archive = subprocess.run(
        ["git", "-C", str(repo), "archive", "--format=tar", sha],
        check=True,
        capture_output=True,
    ).stdout
    with tempfile.TemporaryDirectory(prefix="theorem-counts-") as tmp:
        with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
            tar.extractall(tmp, filter="data")
        out = subprocess.check_output(
            [sys.executable, str(Path(tmp) / STATS_REL), "--json"], cwd=tmp, text=True
        )
    got = json.loads(out)
    return {
        "sha": sha,
        "lake_roots": got["lake_roots_count"],
        "theorem": got["roots_only"]["theorem"],
        "lemma": got["roots_only"]["lemma"],
        "all_lean_theorem": got["all_lean_glob"]["theorem"],
        "all_lean_lemma": got["all_lean_glob"]["lemma"],
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--write", action="store_true", help="rewrite the snapshot at the pins")
    args = ap.parse_args()

    roots = {name: sibling_root(name) for name in SIBLINGS}
    missing = [name for name, root in roots.items() if root is None]
    if missing:
        print(
            f"SKIP: formal siblings missing ({', '.join(missing)}); clone them next to this "
            "repository at their pins (CI does) or run from the tyto-workspace monorepo",
            file=sys.stderr,
        )
        return 0

    measured: dict[str, dict] = {}
    errors: list[str] = []
    for name, root in roots.items():
        sha = pinned_sha(name)
        try:
            measured[name] = counts_at(root, sha)
        except subprocess.CalledProcessError as exc:
            detail = (exc.stderr or b"").decode(errors="replace").strip()
            errors.append(f"{name}: cannot read pinned commit {sha[:12]} in {root} ({detail})")

    if errors:
        for e in errors:
            print(f"FAIL: {e}", file=sys.stderr)
        return 1

    if args.write:
        SNAPSHOT.write_text(json.dumps(measured, indent=2) + "\n", encoding="utf-8")
        print(f"wrote {SNAPSHOT.relative_to(REPO)} at the pins")
        return 0

    if not SNAPSHOT.is_file():
        print(f"FAIL: missing snapshot {SNAPSHOT} (run with --write)", file=sys.stderr)
        return 1
    expected = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    for name, got in measured.items():
        want = expected.get(name, {})
        for key in ("sha", "lake_roots", "theorem", "lemma", "all_lean_theorem", "all_lean_lemma"):
            if want.get(key) != got[key]:
                errors.append(
                    f"{name}: {key} snapshot={want.get(key)} pinned={got[key]} "
                    "(run scripts/check_theorem_counts_ssot.py --write after a pin bump)"
                )

    if errors:
        for e in errors:
            print(f"FAIL: {e}", file=sys.stderr)
        return 1

    print("OK: theorem counts at the pins match the SSOT snapshot")
    for name, w in measured.items():
        print(
            f"  {name} @ {w['sha'][:7]}: {w['lake_roots']} roots, "
            f"{w['theorem']} theorem, {w['lemma']} lemma"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
