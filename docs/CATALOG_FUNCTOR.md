SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Catalog functor witness

**Status:** Witness spike (`math-catalog-functor`, Wave 3 slot 10).  
**Scope:** Pure math/tests only — no solver or hot-path changes.

## Purpose

Dual-pin catalog locks (`artifacts/catalog.lock.json`) and nodal scalar layout
(`artifacts/scalar_layout.lock.json`) live in different JSON artifacts. The **catalog functor**
is the pure map that assigns each catalog **fiber id** its expected nodal scalar channel count,
grounded in the layout sidecar.

| Domain | Artifact | Objects |
|--------|----------|---------|
| **Catalog** | `catalog.lock.json` | Fiber ids (`repo` slugs in `fiber_pins`) |
| **Scalar layout** | `scalar_layout.lock.json` | `scalar_channel_count`, `channel_ids[]` |

Lean catalog fibers (`lean_catalog_lock`) and preview / Track F pins carry **zero** nodal scalars.
Only the manifold runtime bearer (`umst-manifold`) projects to the sidecar width.

## API (`umst-math::catalog_functor`)

| Function | Role |
|----------|------|
| `expected_scalar_channel_count(fiber_id, lock_role, sidecar)` | Fiber id → `usize` channel count |
| `runtime_scalar_channel_count(sidecar)` | Manifold runtime image (= sidecar count) |
| `composed_digest_guard_holds(lock)` | T1 digest guard invariants (v2 locks) |
| `non_preview_fiber_fingerprint_hex(lock)` | Recomputed `composed_primary_fiber_fingerprint_hex` |

Implementation mirrors `build.rs` (`emit_catalog_digest_guard`) and
`scripts/catalog_lock_verify.py` (`--composed-digest-guard`) so CI and runtime witnesses share
one algebraic definition.

## Invariants (T1 composed digest guard)

For `version >= 2` locks with non-preview `fiber_pins`:

1. `composed_primary_fiber_fingerprint_hex` = SHA-256 of sorted `repo:catalog_digest_hex` pairs
   (preview / `track_f` roles excluded).
2. `composed_catalog_digest_hex` is 64-char lowercase hex.
3. `composed_catalog_digest_hex == upstream_catalog_digest_hex` when upstream is set.

Preview fibers (e.g. `umst-ucrs` Track F) do **not** affect the fingerprint.

## Tests

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo test catalog_functor
```

- `umst-math/tests/catalog_functor_witness.rs` — fixture lock JSON + scalar sidecar.
- Property: `composed_digest_guard_idempotent` on the pinned lock clone.

## Related

- [`DUAL_PIN_ARCHITECTURE.md`](DUAL_PIN_ARCHITECTURE.md) — lock schema v2
- [`CATALOG_UPDATE_PROTOCOL.md`](CATALOG_UPDATE_PROTOCOL.md) — fingerprint bump protocol
- `umst-layout-codegen` — scalar sidecar parse/emit functor
- `src/runtime/catalog/mod.rs` — runtime `catalog_lock_quickcheck` (structural, not fingerprint)
