#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
"""
Validate `docs/Solver-Status.md` lane / verification invariants.

Rules:
- For each table row whose **Lane** cell is ``stable`` (case-insensitive), the
  **Verification** cell must be non-empty after stripping.

Optional:
- ``--check-paths``: every ``tests/....rs`` path mentioned in **Verification**
  must exist under the manifold repository root (typically next to ``tests/``).
- ``--check-memo-links``: markdown ``[...](research/....md)`` links (resolved next to
  ``Solver-Status.md``) and backticked ``docs/research/....md`` paths must exist under ``--root``.

Exit status: 0 if OK, 1 on any violation.

Usage (from ``umst-manifold/``)::

    python3 scripts/check_solver_status.py
    python3 scripts/check_solver_status.py --check-paths
    python3 scripts/check_solver_status.py --check-paths --check-memo-links

With a custom doc path::

    python3 scripts/check_solver_status.py --status-md path/to/Solver-Status.md --root path/to/umst-manifold
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

# Paths inside backticks or bare `tests/....rs` in the Verification column.
_VERIF_PATH_RE = re.compile(r"`(tests/[^`]+\.rs)`|(tests/[A-Za-z0-9_./-]+\.rs)")
# Track memos: `[title](research/foo.md)` relative to docs/ (parent of Solver-Status.md).
_MEMO_MARKDOWN_LINK_RE = re.compile(r"\]\((research/[A-Za-z0-9_./-]+\.md)\)")
# Inline `` `docs/research/foo.md` `` references anywhere in the status doc.
_DOCS_RESEARCH_BACKTICK_RE = re.compile(r"`(docs/research/[A-Za-z0-9_./-]+\.md)`")


def _split_table_row(line: str) -> list[str]:
    s = line.strip()
    if not s.startswith("|"):
        return []
    parts = [p.strip() for p in s.split("|")]
    # leading/trailing pipes produce empty strings at ends
    if parts and parts[0] == "":
        parts = parts[1:]
    if parts and parts[-1] == "":
        parts = parts[:-1]
    return parts


def _is_separator_row(cells: list[str]) -> bool:
    if not cells:
        return False
    return all(re.fullmatch(r":?-{3,}:?", c.strip() or "-") for c in cells)


def _find_solver_table(lines: list[str]) -> tuple[list[str], int] | None:
    """Return (header_cells, header_line_index) for the Lane/Verification table."""
    for i, line in enumerate(lines):
        cells = _split_table_row(line)
        if len(cells) < 3:
            continue
        joined = " ".join(c.lower() for c in cells)
        if "solver" in joined and "lane" in joined and "verification" in joined:
            return cells, i
    return None


def parse_solver_rows(status_md: Path) -> list[tuple[str, str, str, str]]:
    """
    Rows as (solver, lane, verification, notes) for the primary status table.
    Skips header and markdown separator rows.
    """
    text = status_md.read_text(encoding="utf-8")
    lines = text.splitlines()
    found = _find_solver_table(lines)
    if not found:
        print(f"error: no Solver/Lane/Verification table header in {status_md}", file=sys.stderr)
        sys.exit(1)
    _, header_i = found
    # Expect separator on next line
    rows: list[tuple[str, str, str, str]] = []
    for line in lines[header_i + 2 :]:
        if not line.strip().startswith("|"):
            break
        cells = _split_table_row(line)
        if _is_separator_row(cells):
            continue
        if len(cells) < 4:
            print(f"error: expected ≥4 columns, got {len(cells)}: {line!r}", file=sys.stderr)
            sys.exit(1)
        solver, lane, verification, notes = cells[0], cells[1], cells[2], cells[3]
        rows.append((solver, lane, verification, notes))
    return rows


def _lane_is_stable(lane: str) -> bool:
    t = lane.strip()
    if t.startswith("`") and t.endswith("`"):
        t = t[1:-1].strip()
    return t.lower() == "stable"


def _snippet(text: str, max_len: int) -> str:
    """Truncate for stderr messages without a misleading ellipsis."""
    t = text.strip()
    if len(t) <= max_len:
        return t
    return t[:max_len] + "..."


def _verification_paths(verification: str) -> list[str]:
    out: list[str] = []
    for m in _VERIF_PATH_RE.finditer(verification):
        p = m.group(1) or m.group(2)
        if p:
            out.append(p)
    return out


def _memo_link_targets(status_md: Path, root: Path, body: str) -> list[tuple[str, Path]]:
    """
    Return (display_ref, absolute_path) for each memo reference to verify.
    Markdown links are relative to docs/ (Solver-Status.md directory).
    Backticked paths are resolved under ``root`` (repository root).
    """
    doc_dir = status_md.parent.resolve()
    root_resolved = root.resolve()
    seen: set[tuple[str, str]] = set()
    out: list[tuple[str, Path]] = []

    for m in _MEMO_MARKDOWN_LINK_RE.finditer(body):
        rel = m.group(1)
        key = ("md_link", rel)
        if key in seen:
            continue
        seen.add(key)
        target = (doc_dir / rel).resolve()
        out.append((rel, target))

    for m in _DOCS_RESEARCH_BACKTICK_RE.finditer(body):
        rel = m.group(1)
        key = ("backtick", rel)
        if key in seen:
            continue
        seen.add(key)
        target = (root_resolved / rel).resolve()
        out.append((rel, target))

    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    script_dir = Path(__file__).resolve().parent
    default_root = script_dir.parent
    ap.add_argument(
        "--status-md",
        type=Path,
        default=default_root / "docs" / "Solver-Status.md",
        help="Path to Solver-Status.md (default: <repo>/docs/Solver-Status.md)",
    )
    ap.add_argument(
        "--root",
        type=Path,
        default=default_root,
        help="umst-manifold repository root (default: parent of scripts/)",
    )
    ap.add_argument(
        "--check-paths",
        action="store_true",
        help="Require Verification-referenced tests/*.rs paths to exist under --root",
    )
    ap.add_argument(
        "--check-memo-links",
        action="store_true",
        help="Require research memo markdown links and docs/research/*.md backticks to exist",
    )
    args = ap.parse_args()
    status_md: Path = args.status_md
    root: Path = args.root
    if not status_md.is_file():
        print(f"error: missing {status_md}", file=sys.stderr)
        sys.exit(1)

    body = status_md.read_text(encoding="utf-8")
    rows = parse_solver_rows(status_md)
    errors = 0
    for solver, lane, verification, _notes in rows:
        if _lane_is_stable(lane) and not verification.strip():
            print(
                "error: stable lane row must have non-empty Verification "
                f"(solver={_snippet(solver, 80)!r}, lane={_snippet(lane, 40)!r})",
                file=sys.stderr,
            )
            errors += 1

        if args.check_paths:
            for rel in _verification_paths(verification):
                target = root / rel
                if not target.is_file():
                    print(
                        f"error: Verification references missing file {rel!r} "
                        f"(expected {target}) for solver row {_snippet(solver, 60)!r}",
                        file=sys.stderr,
                    )
                    errors += 1

    if args.check_memo_links:
        for ref, target in _memo_link_targets(status_md, root, body):
            if not target.is_file():
                print(
                    f"error: memo reference {ref!r} missing (expected {target})",
                    file=sys.stderr,
                )
                errors += 1

    if errors:
        sys.exit(1)
    parts = ["stable lane"]
    if args.check_paths:
        parts.append("verification test paths")
    if args.check_memo_links:
        parts.append("research memo links")
    extra = " + ".join(parts)
    print(f"OK: {status_md} ({len(rows)} table row(s); {extra})")


if __name__ == "__main__":
    main()
