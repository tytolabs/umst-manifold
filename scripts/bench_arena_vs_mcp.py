#!/usr/bin/env python3
"""Arena vs MCP round-trip benchmark — Phase 2 exit witness (≥5× in-process target).

Compares stdio MCP `umst_gate_check` wall time vs in-process `load_arena` hot loop (N=100).
Exit 0 when ratio >= UMST_BENCH_MIN_RATIO (default 5.0). Set UMST_BENCH_SKIP_RATIO=1 to log only.
"""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

MANIFOLD = Path(__file__).resolve().parents[1]
CONCRETE = MANIFOLD.parent / "umst-concrete-cartridge"
MIX = {
    "w_c": "9/20",
    "temperature_k": "29315/100",
    "aggregate_volume_fraction": "7/10",
}
N_CALLS = int(os.environ.get("UMST_BENCH_N", os.environ.get("UMST_BENCH_ITERATIONS", "100")))
MIN_RATIO = float(os.environ.get("UMST_BENCH_MIN_RATIO", "5.0"))
SKIP_RATIO = os.environ.get("UMST_BENCH_SKIP_RATIO", "") == "1"
ARENA_ONLY = "--arena-only" in sys.argv


def bench_mcp_gate(n: int) -> float:
    proc = subprocess.Popen(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "umst-mcp",
            "--features",
            "agent-layer",
            "--release",
        ],
        cwd=CONCRETE,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        text=True,
    )
    assert proc.stdin and proc.stdout

    def rpc(payload: dict) -> dict:
        proc.stdin.write(json.dumps(payload) + "\n")
        proc.stdin.flush()
        return json.loads(proc.stdout.readline())

    rpc(
        {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "bench", "version": "0.1"},
            },
        }
    )
    t0 = time.perf_counter()
    for i in range(n):
        rpc(
            {
                "jsonrpc": "2.0",
                "id": 2 + i,
                "method": "tools/call",
                "params": {"name": "umst_gate_check", "arguments": {"mix": MIX}},
            }
        )
    elapsed = time.perf_counter() - t0
    proc.terminate()
    proc.wait(timeout=120)
    return elapsed


def bench_arena_load_loop(n: int) -> float:
    """Parse inner arena hot-loop timing from release test output (excludes cargo startup)."""
    env = os.environ.copy()
    env["UMST_ARENA_HOT_ITERS"] = str(n)
    proc = subprocess.run(
        [
            "cargo",
            "test",
            "-p",
            "umst-runtime-arena",
            "--release",
            "bench_load_arena",
            "--",
            "--nocapture",
        ],
        cwd=MANIFOLD,
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )
    combined = proc.stdout + proc.stderr
    if proc.returncode != 0:
        sys.stderr.write(combined)
        raise SystemExit(proc.returncode)
    match = re.search(r"arena_100_loads_sec\s+([0-9.]+)", combined)
    if not match:
        match = re.search(r"arena_hot_loop_ok iters=\d+ sec=([0-9.]+)", combined)
    if not match:
        print("FAIL: bench_load_arena_hot_loop missing timing line", file=sys.stderr)
        raise SystemExit(1)
    if "arena_100_loads_sec" in combined:
        return float(match.group(1))
    return float(match.group(1)) * (n / 100.0)


def main() -> int:
    if not CONCRETE.is_dir():
        print(f"skip: concrete cartridge not found at {CONCRETE}", file=sys.stderr)
        return 0

    arena_sec = bench_arena_load_loop(N_CALLS)
    print(f"arena_{N_CALLS}_loads_sec {arena_sec:.6f}")

    if ARENA_ONLY:
        print("arena-only mode: skipping MCP ratio gate")
        return 0

    mcp_sec = bench_mcp_gate(N_CALLS)
    print(f"mcp_{N_CALLS}_gate_check_sec {mcp_sec:.6f}")

    ratio = mcp_sec / max(arena_sec, 1e-12)
    print(f"arena_vs_mcp_ratio {ratio:.2f} (min {MIN_RATIO})")

    if SKIP_RATIO:
        print("UMST_BENCH_SKIP_RATIO=1 — witness logged only")
        return 0

    if ratio < MIN_RATIO:
        print(f"FAIL: ratio {ratio:.2f} < required {MIN_RATIO}", file=sys.stderr)
        return 1

    print("bench_arena_vs_mcp: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
