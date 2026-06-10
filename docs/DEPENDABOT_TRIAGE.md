# Dependabot triage — `sha2` 0.10 → 0.11

**Status:** Closed with ignore rule — keep `sha2 = "0.10"` until build-script digest formatting is migrated.

**Reason:** `sha2` 0.11 changes `Digest::finalize()` to return `hybrid_array::Array<u8, _>` instead of a type implementing `LowerHex`. The `umst-manifold` catalog `build.rs` uses `format!("{digest:x}")` on finalize output; compile fails with `E0277` (verified 2026-06-10).

**Dependabot:** PR #13 closed; `.github/dependabot.yml` ignores `sha2` semver-major bumps.

**Action when revisiting:** migrate build script to `hex::encode(hasher.finalize())` or equivalent, run `verify_umst_stack.sh` + catalog-drift, remove ignore rule, merge bump.
