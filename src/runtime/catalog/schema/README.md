# Witness catalog JSON (schema v1)

Two related artifacts coexist in **`runtime::catalog`**:

| Artifact | Role | Typical path |
|---------|------|--------------|
| **Lock bundle** (`catalog.lock.json`) | Lean exporter / pinning — digest in `UMST_CATALOG_LOCK_SHA256_HEX` | `artifacts/catalog.lock.json` |
| **Witness envelope** ([`WitnessCatalog`]) | Small JSON list of bounded witness checkpoints | Embedded via `build.rs` → `OUT_DIR/catalog_constants.rs` |

---

## Witness envelope (`WitnessCatalog`)

```json
{
  "version": 1,
  "witnesses": [
    {
      "id": "namespace.hook.name",
      "description": "optional note"
    }
  ]
}
```

| Field       | Required | Notes |
|------------|-----------|-------|
| `version`  | yes       | `1` for this revision. |
| `witnesses`| no        | Omitted ⇒ empty list — valid. |

| Record field   | Required | Notes |
|----------------|----------|-------|
| `id`           | yes      | Stable identifier (dot-separated namespaces encouraged). |
| `description`  | no       | Omit when unset. |

### Build-time bytes (witness envelope)

`build.rs` selects JSON bytes (**first hit wins**):

1. **`UMST_CATALOG_BUILD_JSON`** — absolute path to JSON (must exist if set).
2. **`witness_catalog.json`** beside `Cargo.toml`.
3. **Fallback** — minimal built-in envelope in **`build.rs`** so fresh clones compile without extra files.

It writes **`catalog_constants.rs`** into `OUT_DIR` with:

- `WITNESS_CATALOG_EMBEDDED_SHA256_HEX`
- `WITNESS_CATALOG_EMBEDDED_LEN`
- `WITNESS_CATALOG_EMBEDDED_BYTES`

### Runtime override (`WitnessCatalog::load_default`)

When **`UMST_CATALOG_PATH`** is set (UTF-8 path), that file replaces the embedded default; otherwise callers use [`WitnessCatalog::from_embedded`].

---

## Lock bundle (`catalog.lock.json`)

Digest is **SHA-256 (hex lowercase)** over the verbatim lock-file bytes (`UMST_CATALOG` selects the source path).

See **`docs/TCB.md`**, **`docs/RUNTIME_TOPOLOGY.md`** (fiber pins + `commit_stamp`), and `artifacts/catalog.lock.json` layout.

### v2 `fiber_pins[]` (per-fiber Lean catalog pin)

| Field | Required | Notes |
|-------|----------|-------|
| `repo` | yes | Sibling repo id |
| `catalog_digest_hex` | yes | 64-char lowercase SHA-256 of that fiber's catalog export |
| `module_count` | yes | Entry count for audit |
| `lock_role` | recommended | `lean_catalog_lock` for primary fibers; `preview` / `track_f` marks tertiary preview pins (e.g. `umst-ucrs`) excluded from `composed_catalog_digest_hex` |
| `catalog_path` | recommended | Relative catalog artifact path |
| `commit_stamp` | no | Optional UCRS witness stamp at last witnessed commit; absent at cold pin time |
