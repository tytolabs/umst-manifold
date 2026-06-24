#!/usr/bin/env python3
"""Arena vs MCP round-trip benchmark — Phase 2 exit witness (≥5× in-process target)."""

from __future__ import annotations

import json
import os
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
ITERATIONS = int(os.environ.get("UMST_BENCH_ITERATIONS", "50"))
MIN_RATIO = float(os.environ.get("UMST_BENCH_MIN_RATIO", "5.0"))
SKIP_RATIO = os.environ.get("UMST_BENCH_SKIP_RATIO", "") == "1"


def bench_mcp_gate(iterations: int) -> float:
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
    for i in range(iterations):
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
    return elapsed


def bench_inprocess_gate(iterations: int) -> float:
    env = os.environ.copy()
    env["UMST_INPROCESS_GATE_ITERS"] = str(iterations)
    t0 = time.perf_counter()
    proc = subprocess.run(
        [
            "cargo",
            "test",
            "-p",
            "umst-concrete-cartridge",
            "--features",
            "agent-layer",
            "--release",
            "--test",
            "inprocess_gate_batch",
            "inprocess_gate_batch_hot_loop",
            "--",
            "--exact",
            "--nocapture",
        ],
        cwd=CONCRETE,
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        sys.stderr.write(proc.stdout)
        sys.stderr.write(proc.stderr)
        raise SystemExit(proc.returncode)
    return time.perf_counter() - t0


def bench_arena_hot_loop(iterations: int) -> float:
    env = os.environ.copy()
    env["UMST_ARENA_HOT_ITERS"] = str(iterations * 100)
    t0 = time.perf_counter()
    proc = subprocess.run(
        [
            "cargo",
            "test",
            "-p",
            "umst-runtime-arena",
            "--release",
            "bench_load_arena_hot_loop",
            "--",
            "--exact",
            "--nocapture",
        ],
        cwd=MANIFOLD,
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        sys.stderr.write(proc.stdout)
        sys.stderr.write(proc.stderr)
        raise SystemExit(proc.returncode)
    return time.perf_counter() - t0


def main() -> int:
    if not CONCRETE.is_dir():
        print(f"skip: concrete cartridge not found at {CONCRETE}", file=sys.stderr)
        return 0

    print(f"bench iterations={ITERATIONS} min_ratio={MIN_RATIO}")
    mcp_s = bench_mcp_gate(ITERATIONS)
    inproc_s = bench_inprocess_gate(ITERATIONS)
    arena_s = bench_arena_hot_loop(ITERATIONS)

    ratio_inproc = mcp_s / max(inproc_s, 1e-9)
    ratio_arena = mcp_s / max(arena_s, 1e-9)

    print(f"mcp_gate_{ITERATIONS}_sec={mcp_s:.4f}")
    print(f"inprocess_gate_{ITERATIONS}_sec={inproc_s:.4f}")
    print(f"arena_hot_{ITERATIONS}x100_sec={arena_s:.4f}")
    print(f"ratio_mcp_over_inprocess={ratio_inproc:.2f}x")
    print(f"ratio_mcp_over_arena={ratio_arena:.2f}x")

    if SKIP_RATIO:
        print("UMST_BENCH_SKIP_RATIO=1 — witness logged only")
        return 0

    if ratio_inproc < MIN_RATIO and ratio_arena < MIN_RATIO:
        print(
            f"FAIL: neither in-process ({ratio_inproc:.2f}x) nor arena ({ratio_arena:.2f}x) "
            f"met {MIN_RATIO}x",
            file=sys.stderr,
        )
        return 1
    print("PASS: Phase 2 benchmark ratio met")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
