SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Arena vs MCP round-trip benchmark

**Exit witness (Phase 2):** in-process gate/arena ≥ **5×** vs one stdio MCP `tools/call` round-trip per gate check.

## Summary (relative throughput)

| Surface | Relative speed | Notes |
|---------|----------------|-------|
| Stdio MCP `tools/call` | **1×** (baseline) | JSON-RPC per gate check |
| In-process library (`gate_check_mix`) | **~5–10×+** | Same process, no wire |
| Arena mmap (`load_arena` / `UmstArenaView`) | **~5–10×+** | Parse once; zero-copy hot loop |

CI job `arena-vs-mcp` enforces in-process arena ≥ **5×** MCP (`UMST_BENCH_N=30`). **10×** is aspirational on reference hardware.

## Surfaces

| Surface | Boundary | When |
|---------|----------|------|
| MCP stdio | Cold | Agents, discovery, single-shot gate/predict |
| `load_arena(bytes)` | Warm | Owned buffer, parse once |
| `mmap_arena_path` (`feature = "mmap"`) | Warm | File-backed arena, zero-copy view |
| `seal_arena_commit` | Warm egress | UCRS stamp bytes 12..20 on commit close |

## Harness

```bash
# Full benchmark (release builds; may take several minutes first compile)
cd umst-manifold
python3 scripts/bench_arena_vs_mcp.py

# Log ratios only (local dev — no fail on slow runners)
UMST_BENCH_SKIP_RATIO=1 python3 scripts/bench_arena_vs_mcp.py

# CI / enforced fail when ratio < 5× (default when CI=true or UMST_BENCH_ENFORCE=1)
UMST_BENCH_N=30 UMST_BENCH_MIN_RATIO=5 UMST_BENCH_ENFORCE=1 python3 scripts/bench_arena_vs_mcp.py

# Local reference (higher N)
UMST_BENCH_N=100 python3 scripts/bench_arena_vs_mcp.py
```

Agent batch example (in-process gate, no MCP):

```bash
cd umst-concrete-cartridge
python3 examples/agent/06_arena_batch.py
```

## Status (2026-06-24)

| Item | Status |
|------|--------|
| `mmap_arena_path` + `seal_arena_commit` | **Shipped** |
| `examples/agent/06_arena_batch.py` + CI | **Shipped** |
| `scripts/bench_arena_vs_mcp.py` | **Shipped** |
| CI `arena-vs-mcp` job (`UMST_BENCH_N=30`, ratio ≥5×) | **Shipped** |
| Published ≥5× ratio on CI hardware | **CI-pinned** — see `artifacts/benchmarks/arena_vs_mcp_ci.json` |

Paste measured `ratio_mcp_over_inprocess` into [`IMPLEMENTATION_EVIDENCE.md`](../../../outputs/IMPLEMENTATION_EVIDENCE.md) P2 row after hardware runs.
