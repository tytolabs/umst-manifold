# Arena vs MCP round-trip benchmark

**Exit witness (Phase 2):** in-process gate/arena ≥ **5×** vs one stdio MCP `tools/call` round-trip per gate check.

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

# Log ratios only (CI-friendly — no fail on slow runners)
UMST_BENCH_SKIP_RATIO=1 python3 scripts/bench_arena_vs_mcp.py

# Tune iterations / threshold
UMST_BENCH_ITERATIONS=30 UMST_BENCH_MIN_RATIO=5 python3 scripts/bench_arena_vs_mcp.py
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
| Published ≥5× ratio on CI hardware | **Partial** — run locally; use `UMST_BENCH_SKIP_RATIO=1` on slow CI |

Paste measured `ratio_mcp_over_inprocess` into [`IMPLEMENTATION_EVIDENCE.md`](../../../outputs/IMPLEMENTATION_EVIDENCE.md) P2 row after hardware runs.
