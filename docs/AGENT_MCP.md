# Agent MCP — DesignQuery extension (R3)

**Audience:** Cursor agents, Hermes/OpenCode orchestration, in-process hot-path optimizers  
**Manifold modules:** `src/design/query.rs`, `src/physics/compliance_functional.rs`, `src/runtime/gate/admissibility_margin.rs`, `src/core/traits.rs` (`DesignRepresentation`)  
**Cartridge:** B6 harness migrates env flags → `DesignQueryContext` (follow-up PR)

---

## DesignQuery v0 (read-only)

Stateless hot-path query:

```text
query_v0(ctx, latent, coords) → {
  geometry: ρ field from DesignRepresentation::decode,
  metrics: { compliance_optimizer, compliance_gate, penalization_p_* },
  margin: AdmissibilityMargin (signed CD headroom),
  witness: { seed, repr_id }
}
```

**Penalization contract (R1):**

| Field | Mode | Typical value |
|-------|------|----------------|
| `penalization_optimizer` | `CompliancePenalization::Schedule { outer, total }` | running `p_act` |
| `penalization_gate` | `CompliancePenalization::Gate(3.0)` | fixed p=3 §9 anchor |

Invariant: **value = gradient = gate** at the declared anchor — optimizer and gate may differ in `p`, but each calls the same `Q1HexComplianceFunctional` kernel.

**Feature:** `cargo test -p umst-manifold --features design-query`

---

## DesignQuery v1 (gradients)

Adds:

- `d_metric_dz` — sensitivity of compliance surrogate w.r.t. latent `z`
- `d_margin_dz` — sensitivity of signed margin penalty w.r.t. `z`

Use **in-process** `StructuralDesignQuery::query_v1` for optimization loops (≥5× throughput vs MCP stdio per `benchmarks/arena_vs_mcp.md`).

---

## Transport tiers

| Tier | Surface | DesignQuery |
|------|---------|-------------|
| **Hot** | `StructuralDesignQuery` in manifold | Primary — autodiff + PCG off-tape |
| **Warm** | Arena session + problem digest | `DesignQueryContext { seed, compliance_ctx, … }` |
| **Cold** | Future `umst_design_query` MCP tool | Serde wrapper only — no physics in MCP crate |

---

## Related docs

- MaOS workspace plan: `outputs/.plans/umst-agent-facing-refactor.md`
- `umst-concrete-cartridge/docs/AGENT_MCP.md` (stdio MCP base contract)
- [`benchmarks/arena_vs_mcp.md`](benchmarks/arena_vs_mcp.md)
