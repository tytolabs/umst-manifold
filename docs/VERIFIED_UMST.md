SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Verified UMST DEC typestate staging

**Status:** Witness spike (`math-verified-umst-typestate`, Wave 5 slot 4).  
**Scope:** `core::dec_typestate` staging only — no hot-path or gateway migration.

## Purpose

Phase 1 §1C introduced [`B1Incidence`](../../src/core/dec_typestate.rs) and
[`ScalarChannelIdx`](../../src/core/dec_typestate.rs) so invalid DEC layout and out-of-range scalar
indices are rejectable via [`Result`]. This spike **composes** those witnesses into a single staging
bundle before any full [`UnifiedMaterialStateTensor`](../../src/core/tensors.rs) migration.

| Witness | Role |
|---------|------|
| [`B1Incidence<B>`](../../src/core/dec_typestate.rs) | `edges_b1` must be shape `[2, E]` |
| [`ScalarChannelIdx`](../../src/core/dec_typestate.rs) | Runtime channel index ∈ `0 .. CHANNEL_COUNT` |
| [`ScalarChannel<const N>`](../../src/core/dec_typestate.rs) | Compile-time index witness vs pinned layout |
| [`VerifiedUMST<B>`](../../src/core/dec_typestate.rs) | Staging product of the above |

**Not** the proof-carrying gateway type [`core::tensors::VerifiedUMST<B, P>`](../../src/core/tensors.rs)
(Clausius–Duhem witness). The staging struct lives only in `dec_typestate` until a later wire pass
lifts validated topology into the full UMST carrier.

## API (`core::dec_typestate::VerifiedUMST`)

| Symbol | Role |
|--------|------|
| `VerifiedUMST::<B>::CHANNEL_COUNT` | Compile-time pin = [`UMST_SCALAR_CHANNEL_COUNT`](../../src/core/umst_schema.rs) |
| `VerifiedUMST::try_assemble(edges_b1, scalar_cols, channel)` | Total constructor; returns [`DecTypestateError`](../../src/core/dec_typestate.rs) |
| `b1()` / `channel()` / `into_b1()` | Accessors |

`try_assemble` checks, in order:

1. `scalar_cols == CHANNEL_COUNT` ([`ScalarWidthMismatch`](../../src/core/dec_typestate.rs))
2. `B1Incidence::try_new` layout `[2, E]`
3. `ScalarChannelIdx::try_new` channel range

## Tests

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo test verified_umst_typestate
```

- [`tests/verified_umst_typestate_witness.rs`](../tests/verified_umst_typestate_witness.rs) — assemble + reject paths.
- Unit tests in [`src/core/dec_typestate.rs`](../src/core/dec_typestate.rs) — per-component witnesses.

## Related

- [`CATALOG_FUNCTOR.md`](CATALOG_FUNCTOR.md) — fiber id → scalar channel count functor
- [`umst_schema.rs`](../src/core/umst_schema.rs) — pinned `SCALAR_*` layout SSOT
- [`Mathematical-Foundations.md`](Mathematical-Foundations.md) — proof-carrying `VerifiedUMST<P>`
