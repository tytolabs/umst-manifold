#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
# SPDX-License-Identifier: MIT
"""Regenerate artifacts/training/p4_rejection_baseline.json from measured witness test."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    proc = subprocess.run(
        [
            "cargo",
            "test",
            "-p",
            "umst-manifold",
            "--features",
            "kleisli-ppo-hot-bind",
            "--test",
            "rejection_witness",
            "p4_rejection_baseline_measured_witness",
            "--",
            "--exact",
        ],
        cwd=ROOT,
        check=False,
    )
    if proc.returncode != 0:
        return proc.returncode
    out = ROOT / "artifacts" / "training" / "p4_rejection_baseline.json"
    print(f"regenerated {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

