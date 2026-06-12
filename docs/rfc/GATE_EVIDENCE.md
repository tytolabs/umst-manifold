# RFC: Gate evidence bundle (integration-contracts D5 — spec only)

**Status:** Draft spec — **no Lean mechanization in this wave**  
**Audit:** finding **#10** (transient vector solid dynamics), ledger honesty (Wave 0)

## Question for formal (Lean)

> Should thermodynamic gate verdicts require a **machine-checkable solve witness** (`SolveReport` + residual bound),
> or is catalog + CBF margin sufficient for v0.4 acceptance?

**Flag:** unresolved — needs `umst-formal` fiber decision before wiring gate HTTP payloads.

## Evidence columns (F / E)

| Column | Meaning | This wave |
| --- | --- | --- |
| **F** (functional) | Rust `SolveReport::converged()` + `GroundedConst` SSOT | D1 + D3 |
| **E** (empirical) | `tests/verification/MANIFEST.toml` NEVER-RUN ledger | D4 skeleton |

## Target bundle (future)

```text
GateVerdict
├── catalog_witness_digest
├── cbf_margin_min
└── solve_witness: Option<SolveReport>   // optional until Wave 3 adoption
```

## Scope boundary

- Spec only — no `gate/` code changes.  
- No cartridge Striatus acceptance rewiring.  
- Wave 2 executes `#[ignore]` envelopes and fills MANIFEST `last_run` fields.

## Deliberately not done

- Lean proof of `converged()` ↔ discrete equilibrium.  
- Gate server JSON schema change.  
- Photonics / rheology / acoustics certification.
