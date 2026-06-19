# VERIFY_TRANSCRIPT — machine run record

**Date:** 2026-06-19 (Composer backlog T2 — catalog bundle pin bump)  
**Host:** multi-repo workspace monorepo (local)  
**Exit code:** 0

## Toolchain

| Item | Value |
|------|-------|
| `rustc` | 1.88.0 (rustup; `$HOME/.cargo/bin` prepended) |
| `umst-manifold` | `f70c411a3e56bb38c0086ab6b1a81f19e5d701d9` |
| `umst-formal-double-slit` | `72a6fe93d44e471cd74e1d3f513396d3da7261c5` |
| `umst-formal` | `b09d4a0a99190b49c166833a360b96a464452bd0` |
| `umst-concrete-cartridge` | `4fe621556bc43b02d0a3208d7eb78f0b86f3e0e0` |

## Command (exact)

```bash
cd umst-manifold
export PATH="$HOME/.cargo/bin:$PATH"
UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh 2>&1 | tee /tmp/verify_composer_t2.log
```

## Catalog pin at run time (R0)

| Field | Value |
|-------|-------|
| `module_count` | **122** |
| `upstream_catalog_digest_hex` | `c61b1befdec77a82bbb9f6c3f7562e754218ef635f0e3b9990752138df5f4bb5` |
| `composed_primary_fiber_fingerprint_hex` | `c712aa93443c12a0043ee54ee895166609d0c1e104d8e2e61ed4e1c4390bb3f8` |
| `catalog_lock_bundle_sha256_hex` | `904f01b18d939d72ea63de27f639f94885b761ebad92b96082a602a620ace46c` |
| `module_graph_edge_count` | 329 |

## W9 acceptance (additional)

```text
check_domain_lexicon.sh          → domain_lexicon_tier1_hits=0
check_agnostic_on_fork.sh        → 0 files
check_theorem_counts_ssot.py     → formal 53/261/24, double-slit 59/540/34
cargo test --lib                 → 81/81
cargo test gate_cartridge_only_stub → 1/1
cargo test thmc_drying_shrinkage --features thmc-coupled --release → 32/32
umst-formal: lake build UMST     → Build completed successfully
```

## Transcript tail

```
OK: manifest_bridge_catalog_grounding
OK: formal_anchors manifest-bridge
w8_publish_readiness: PASS=21 FAIL=0 SKIP=2
verify_umst_stack: OK
```

Full log: `/tmp/verify_w9_followup.log`

## Research fence (prime-spectral)

Branch `prime-spectral-research` remains **CLOSED / AMBER** ([`outputs/prime-spectral-research/FINAL_FINDING.md`](../../outputs/prime-spectral-research/FINAL_FINDING.md)). R2/R3/Track 4–5 killed; Track 1 NTT conservation lead parked. Not merged to `main`.
