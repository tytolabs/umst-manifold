#!/usr/bin/env python3
"""Patch test call sites for solve_equilibrium* / forward_and_loss Result migration (3b-1/3b-2)."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TESTS = ROOT / "tests"

FN_MARKERS = (
    "VectorMechanicsSolver::solve_equilibrium",
    "VectorMechanicsSolver::solve_equilibrium_with_pcg_report",
    "VectorMechanicsSolver::solve_equilibrium_typed",
    "AdjointComplianceQ1Hex::forward_and_loss",
    "AdjointCompliance::forward_and_loss",
    "strain_tensor_for_fracture_after_mechanics",
)

SKIP_SUFFIX = (".unwrap()", ".expect(", ".?;", ")?;")


def patch_calls(text: str) -> tuple[str, int]:
    lines = text.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    patches = 0
    while i < len(lines):
        line = lines[i]
        if any(m in line for m in FN_MARKERS):
            block = [line]
            j = i + 1
            depth = line.count("(") - line.count(")")
            while j < len(lines) and depth > 0:
                block.append(lines[j])
                depth += lines[j].count("(") - lines[j].count(")")
                j += 1
            last = block[-1]
            stripped = last.rstrip()
            if stripped.endswith(";") and not any(s in stripped for s in SKIP_SUFFIX):
                indent = last[: len(last) - len(last.lstrip())]
                block[-1] = f"{indent})\n{indent}.unwrap();\n"
                patches += 1
            out.extend(block)
            i = j
            continue
        out.append(line)
        i += 1
    return "".join(out), patches


def main() -> None:
    total = 0
    for path in sorted(TESTS.rglob("*.rs")):
        original = path.read_text()
        patched, n = patch_calls(original)
        if n:
            path.write_text(patched)
            print(f"{path.relative_to(ROOT)}: {n} site(s)")
            total += n
    print(f"patched {total} call site(s)")


if __name__ == "__main__":
    main()
