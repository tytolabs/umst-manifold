#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
"""public_workflow_guard — W-63/W-68: public default-branch workflows stay green without private siblings."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

RUST_WF = ".github/workflows/rust.yml"
CATALOG_WF = ".github/workflows/umst-catalog-drift.yml"
SETUP_CI = ".github/actions/setup-ci/action.yml"
SIBLING_SCRIPT = ".github/scripts/checkout-umst-siblings.sh"

# Jobs that call setup-ci with private path deps; must not fail the workflow on public runners.
RUST_PRIVATE_JOBS = (
    "build-test:",
    "verify-umst-stack:",
    "kleisli-ppo-hot-bind:",
    "arena-vs-mcp:",
    "lint:",
    "research-stack:",
)


def repo_root() -> Path:
    return Path(__file__).resolve().parent.parent


def job_block_has_continue_on_error(workflow: str, job_key: str) -> bool:
    """Return true when the job block contains continue-on-error: true before the next top-level job."""
    start = workflow.find(job_key)
    if start < 0:
        return False
    rest = workflow[start:]
    end = rest.find("\n  ", 1)
    if end < 0:
        block = rest
    else:
        nxt = re.search(r"\n  [a-z0-9][a-z0-9_-]*:", rest[end + 1 :])
        block = rest[: end + 1 + (nxt.start() if nxt else len(rest))]
    return "continue-on-error: true" in block


def check_workflows(root: Path) -> list[str]:
    errors: list[str] = []
    rust = (root / RUST_WF).read_text(encoding="utf-8")
    catalog = (root / CATALOG_WF).read_text(encoding="utf-8")
    setup = (root / SETUP_CI).read_text(encoding="utf-8")

    if "w63-public-sibling-boundary:" not in rust:
        errors.append(f"{RUST_WF} missing w63-public-sibling-boundary job")
    if "w63-catalog-public-boundary:" not in catalog:
        errors.append(f"{CATALOG_WF} missing w63-catalog-public-boundary job")
    if SIBLING_SCRIPT not in rust and "checkout-umst-siblings.sh" not in rust:
        errors.append(f"{RUST_WF} missing W-63 sibling script reference")
    if not (root / SIBLING_SCRIPT).is_file():
        errors.append(f"missing {SIBLING_SCRIPT}")

    for job in RUST_PRIVATE_JOBS:
        if job not in rust:
            errors.append(f"{RUST_WF} missing job {job[:-1]}")
        elif not job_block_has_continue_on_error(rust, job):
            errors.append(f"{RUST_WF} job {job[:-1]} must set continue-on-error: true (W-63)")

    if "verify-umst-stack:" not in catalog:
        errors.append(f"{CATALOG_WF} missing verify-umst-stack job")
    elif not job_block_has_continue_on_error(catalog, "verify-umst-stack:"):
        errors.append(f"{CATALOG_WF} verify-umst-stack must set continue-on-error: true")

    if "checkout_private_siblings" not in setup:
        errors.append(f"{SETUP_CI} missing checkout_private_siblings input")
    if 'checkout_private_siblings: "false"' not in catalog:
        errors.append(f"{CATALOG_WF} verify job must skip private siblings via setup-ci")

    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="exit 1 when workflow guard wiring drifts")
    _ = parser.parse_args()
    errors = check_workflows(repo_root())
    if errors:
        for e in errors:
            print(e, file=sys.stderr)
        return 1
    print("public_workflow_guard: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
