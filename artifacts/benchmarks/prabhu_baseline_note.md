# PRABHU baseline lock (PB-S4)

| Field | Value |
|-------|-------|
| **Step** | PB-S4 — baseline capture |
| **Capture HEAD** | `ff61cd51086a8bf7e7829e1f2b6b85a5e3c4e1d7` |
| **Harness ancestry** | `64cf197` (PB-S0/S3 collector + benches) |
| **Schedule anchor** | `a7b01c5db651e8a5840d965ce1747d0f4b50163c` |
| **Host** | Darwin 25.5.0 arm64 |
| **Rustc** | rustc 1.88.0 (6b00bc388 2025-06-23) |
| **Transcript** | `prabhu_runtime_subset.json` |
| **Parity digest** | `d5608148e29eeabd83935988699d08ce1233c3e87f2cd217d658e0c71c7a841e` |

## Metrics @ capture

| ID | Key | Value |
|----|-----|------:|
| PB-1 | `gate_route_us_per_call` | 0.043 |
| PB-2 | `thmc_step_ms_per_node` | 3.280875 |
| PB-3 | `arena_100_loads_sec` | 0.000000 |

PB-3 release bench rounds to zero (borrow-only hot loop); debug `bench_load_arena_hot_loop` remains alternate witness.

## Operator follow-up (not invented here)

- **PB-S5:** `artifacts/benchmarks/prabhu_thresholds.toml` — Prabhu sets `max_regression_pct` per PB-id.
- **PB-S6:** CI probe `prabhu-benchmark-subset` — prep stub `scripts/verify_prabhu_benchmark_subset.sh` + design `prabhu_ci_probe_design.md` (enforce post-S5).

Regenerate transcript: `bash scripts/collect_prabhu_benchmark_transcript.sh`  
Dry-run probe: `bash scripts/verify_prabhu_benchmark_subset.sh`
