SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Dependabot triage — `sha2` 0.10 → 0.11

**Status:** Closed with ignore rule — keep `sha2 = "0.10"` until build-script digest formatting is migrated.

**Reason:** `sha2` 0.11 changes `Digest::finalize()` to return `hybrid_array::Array<u8, _>` instead of a type implementing `LowerHex`. The `umst-manifold` catalog `build.rs` uses `format!("{digest:x}")` on finalize output; compile fails with `E0277` (verified 2026-06-10).

**Dependabot:** PR #13 closed; `.github/dependabot.yml` ignores `sha2` semver-major bumps.

**Action when revisiting:** migrate build script to `hex::encode(hasher.finalize())` or equivalent, run `verify_umst_stack.sh` + catalog-drift, remove ignore rule, merge bump.

## Deferred majors (2026-06-10 CI-green wave)

| Dependency | Reason |
|------------|--------|
| `petgraph` 0.8 | API churn; not in CI-green scope |
| `bincode` 3.x | workspace pinned `=2.0.0-rc.3` |
| `burn` / `burn-ndarray` 0.21 | ML stack major; separate upgrade wave |
| `rand` 0.10 | transitive API drift across workspace |

PRs closed with `.github/dependabot.yml` `semver-major` ignore rules.

## O6 triage (2026-06-24)

**Known alerts:** ~3 open Dependabot alerts on `umst-manifold`.

**Automation state:** `.github/dependabot.yml` enables `cargo` (weekly) + `github-actions`
(monthly) ecosystem updates. Non-major updates are now **grouped** (`cargo-minor-patch`,
`actions-minor-patch`) into a single PR each to cut review noise; known-incompatible
majors stay in the `ignore` list above.

**Version bumps deferred:** Actual `Cargo.toml` version bumps are **not** performed in this
pass. Applying and validating a bump requires building with the pinned `rustc 1.88`
toolchain (`time-core@0.1.9` needs 1.88), which is unavailable in the Ops/cold environment.
Bumps are deferred to a **build-capable worker** who can run `cargo build && cargo test` +
`verify_umst_stack.sh` + catalog-drift before merging each (grouped) Dependabot PR.

## W-63 cargo updater removed (2026-10-09)

Every Dependabot `cargo` run on `main` failed with `path_dependencies_not_reachable`
(runs 36925080411, 37842824020, 37842832452): `Cargo.toml` reaches `umst-foundations`,
`umst-cartridge-api` and `umst-cartridges` by `../` path, those repositories are private, and
Dependabot fetches this repository alone. `.github/dependabot.yml` therefore carries only the
`github-actions` ecosystem. Removed `cargo` ignore rules, for restoration with the entry:
`umst-math` (all), and semver-major for `sha2`, `petgraph`, `bincode`, `burn`, `burn-ndarray`,
`rand`, `thiserror`, `sha3`; group `cargo-minor-patch` (minor + patch). Cargo bumps run by hand
on a build-capable checkout with `cargo update`, `cargo test` and `verify_umst_stack.sh`.
Open PR #64 (the last grouped cargo bump) can no longer be recreated by Dependabot.

