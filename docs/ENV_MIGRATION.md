# Environment variable migration (EGOFF → UMST)

**Status:** In progress (batch H4a — memory family)  
**Breaking:** Operators must update shell exports and deployment configs.

## Memory family (2026-06-19)

| Legacy (`EGOFF_*`) | New (`UMST_*`) | Registry row |
|--------------------|----------------|--------------|
| `EGOFF_MEMORY_PROMOTION_REQUIRE_THEOREM` | `UMST_MEMORY_PROMOTION_REQUIRE_THEOREM` | `egoff_memory_m2_promotion_requires_theorem_default` |
| `EGOFF_MEMORY_EPHEMERAL_TTL_HOURS` | `UMST_MEMORY_EPHEMERAL_TTL_HOURS` | `egoff_memory_ephemeral_ttl_hours_typical` |
| `EGOFF_MEMORY_RETENTION_ALPHA` | `UMST_MEMORY_RETENTION_ALPHA` | retention policy |
| `EGOFF_MEMORY_RETENTION_EVICT` | `UMST_MEMORY_RETENTION_EVICT` | retention policy |
| `EGOFF_MEMORY_RETENTION_DEGRADE_FIRST` | `UMST_MEMORY_RETENTION_DEGRADE_FIRST` | retention policy |
| `EGOFF_MEMORY_HILBERT_BITS` | `UMST_MEMORY_HILBERT_BITS` | `egoff_memory_hilbert_bits`, `manifold_hilbert_bits_default` |

Subsequent batches (cockpit, manifold, GPU, LLM) are tracked in [`PENDING_RELEASE_WITNESS_ROADMAP.md`](PENDING_RELEASE_WITNESS_ROADMAP.md) item **0e**.

**Verify after H4a:** `rg 'EGOFF_MEMORY_' umst-manifold/umst-math` → empty.
