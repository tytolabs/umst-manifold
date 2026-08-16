#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
"""
Validate `docs/CONSTANTS.md` ↔ `src/constants_registry.rs` registry invariants.

Rules:
- Every migrated `name` in the CONSTANTS.md table must appear in `constants_registry.rs`.
- Every `THMC_FLOATS_TODO` symbol must remain documented in CONSTANTS.md (TODO section).
- `GroundedConst` struct must exist (FP contract).

Exit status: 0 if OK, 1 on any violation.

Usage (from ``umst-manifold/``)::

    python3 scripts/check_constants.py
    python3 scripts/check_constants.py --check-thmc-todo
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

_MIGRATED_NAME_RE = re.compile(r"`([a-z][a-z0-9_]+)`")
_REGISTRY_ROW_RE = re.compile(
    r"^\|\s*`([a-z][a-z0-9_]+)`\s*\|", re.MULTILINE
)
_THMC_TODO_RE = re.compile(r'"([A-Z][A-Z0-9_]+)"')


def _read(path: Path) -> str:
    if not path.is_file():
        print(f"error: missing {path}", file=sys.stderr)
        sys.exit(1)
    return path.read_text(encoding="utf-8")


def migrated_names_from_docs(docs: str) -> list[str]:
    """Parse the ## Migrated rows table (backtick name in first column)."""
    start = docs.find("## Migrated rows")
    if start < 0:
        print("error: CONSTANTS.md missing '## Migrated rows'", file=sys.stderr)
        sys.exit(1)
    section = docs[start:]
    end = section.find("\n## ", 1)
    if end > 0:
        section = section[:end]
    names: list[str] = []
    for m in _REGISTRY_ROW_RE.finditer(section):
        name = m.group(1)
        if name in ("name", "value", "feature"):
            continue
        names.append(name)
    return names


def thmc_todo_from_registry(registry: str) -> list[str]:
    block_start = registry.find("THMC_FLOATS_TODO")
    if block_start < 0:
        return []
    block = registry[block_start : block_start + 4000]
    return _THMC_TODO_RE.findall(block)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    script_dir = Path(__file__).resolve().parent
    root = script_dir.parent
    ap.add_argument(
        "--constants-md",
        type=Path,
        default=root / "docs" / "CONSTANTS.md",
    )
    ap.add_argument(
        "--registry-rs",
        type=Path,
        default=root / "src" / "constants_registry.rs",
    )
    ap.add_argument(
        "--check-thmc-todo",
        action="store_true",
        help="Require THMC_FLOATS_TODO symbols to appear in CONSTANTS.md",
    )
    args = ap.parse_args()

    docs = _read(args.constants_md)
    registry = _read(args.registry_rs)

    if "struct GroundedConst" not in registry:
        print("error: constants_registry.rs missing GroundedConst", file=sys.stderr)
        sys.exit(1)

    errors = 0
    for name in migrated_names_from_docs(docs):
        if f'name: "{name}"' not in registry and f'name: \'{name}\'' not in registry:
            # also accept `.name` references via GroundedConst literals
            if f'"{name}"' not in registry:
                print(
                    f"error: migrated name {name!r} in CONSTANTS.md not found in constants_registry.rs",
                    file=sys.stderr,
                )
                errors += 1

    if args.check_thmc_todo:
        for sym in thmc_todo_from_registry(registry):
            if sym not in docs:
                print(
                    f"error: THMC_FLOATS_TODO symbol {sym!r} missing from CONSTANTS.md",
                    file=sys.stderr,
                )
                errors += 1

    if errors:
        sys.exit(1)
    n = len(migrated_names_from_docs(docs))
    print(f"OK: {args.constants_md.name} ↔ constants_registry.rs ({n} migrated name(s))")


if __name__ == "__main__":
    main()
