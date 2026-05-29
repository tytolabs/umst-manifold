# W8 publish runbook — `tytolabs/umst-manifold` + cartridge `manifest-bridge`

**Scope:** Unblock remote git consumers of `umst-manifold::manifest` so `umst-concrete-cartridge` (and optionally `umst-supercap-cartridge`) can run **`manifest-bridge`** in GitHub Actions **without** workspace `[patch]`.

**Witness ladder:** R5 — [Manifest bridge + formal witness](GOD_GRADE_WITNESS_LADDER.md#r5--manifest-bridge--formal-witness-deployment-fiber) (paired with `formal-witness` in MaOS drift CI).

**Status SSOT:** [`AGENT_W8_STATUS.txt`](AGENT_W8_STATUS.txt) (local code done); this runbook covers **ops only**.

**Roadmap:** Track **A** in [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) — remaining **~24%** god-grade automation debt is largely W8 + strict default + v2 traces.

**Agent policy:** Do **not** `git push`, `gh`, or `cargo publish` to crates.io unless the operator has confirmed credentials and asked for it. All push/publish steps are **operator-only** checkboxes below.

---

## Prerequisites

- [ ] Local tree matches W8 surface: `umst-manifold/src/lib.rs` has `pub mod manifest` (not feature-gated).
- [ ] `umst-manifold/Cargo.toml` declares `manifest-bridge = []` and `manifold-manifest = []`.
- [ ] `umst-concrete-cartridge` passes locally with patch (baseline):

```bash
cd umst-manifold && cargo check
cd ../umst-concrete-cartridge && cargo test -p umst-concrete-cartridge --features manifest-bridge
```

- [ ] Operator has **write access** to `github.com/tytolabs/umst-manifold` and `github.com/tytolabs/umst-concrete-cartridge` (and supercap if applicable).
- [ ] `umst-formal-double-slit` export digest matches `umst-manifold/artifacts/catalog.lock.json` if you will run strict stack verify before publish.

---

## Phase 0 — Manifold preflight (local, no push)

Run from `MaOS-Workspace/umst-manifold`:

- [ ] `cargo check`
- [ ] `cargo test -p umst-manifold`
- [ ] `cargo doc --no-deps -p umst-manifold 2>&1 | rg 'mod manifest'` → shows `manifest` in public API
- [ ] `cargo publish --dry-run` (optional; confirms crate layout if publishing to crates.io later)
- [ ] `bash scripts/verify_umst_stack.sh` (set `UMST_REQUIRE_FORMAL_EXPORT=1` and `UMST_FORMAL_ROOT` when formal sibling is present)
- [ ] `bash scripts/bidirectional_catalog_check.sh`
- [ ] `rg '^axiom ' ../umst-formal-double-slit/Lean/LandauerLaw.lean` → single `physicalSecondLaw` (TCB unchanged)

Record commit SHA intended for publish: `________________` (from `git rev-parse HEAD` in `umst-manifold`).

---

## Phase 1 — Publish `tytolabs/umst-manifold` `main` (operator + credentials only)

> **Agents:** stop after Phase 0 unless the user explicitly runs push with their credentials.

- [ ] Review diff on `umst-manifold` branch to publish (manifest module, catalog lock, docs only if intended).
- [ ] Commit on local `main` (or release branch) with message noting W8 / `manifest` API.
- [ ] **Operator only — push to GitHub** (requires user credentials; do not run in unattended agent sessions):

```bash
cd umst-manifold
git push origin main   # or: git push origin <tag>  if tagging a release
```

- [ ] **Operator only** — optional crates.io publish (separate from git pin; cartridge uses **git** dep today):

```bash
cargo publish --dry-run   # verify first
# cargo publish           # only if crates.io release is in scope
```

- [ ] Confirm remote `main` contains `manifest`:

```bash
REV=$(git ls-remote https://github.com/tytolabs/umst-manifold.git refs/heads/main | awk '{print $1}')
echo "remote main = $REV"
```

---

## Phase 2 — Clean-clone verify (no MaOS `[patch]`)

- [ ] Clone fresh directory **outside** MaOS workspace (or temp dir without `Cargo.toml` patch):

```bash
git clone https://github.com/tytolabs/umst-manifold.git /tmp/umst-manifold-w8-verify
cd /tmp/umst-manifold-w8-verify
git checkout "$REV"   # or: git pull && use origin/main
cargo check -p umst-manifold
cargo doc --no-deps -p umst-manifold 2>&1 | rg 'mod manifest'
```

- [ ] Expected: build succeeds; `manifest` visible in docs without local path override.

---

## Phase 3 — Bump cartridge git pin (concrete; operator commit)

Repo: `umst-concrete-cartridge` (`crates/umst-concrete-cartridge/Cargo.toml`).

- [ ] Pin `umst-manifold` git dep to published revision (replace `branch = "main"` with rev pin when stabilizing):

```toml
# Example after W8 publish:
umst-manifold = { git = "https://github.com/tytolabs/umst-manifold.git", rev = "<REV_FROM_PHASE_1>" }
```

- [ ] **Remove or narrow** workspace patch in `umst-concrete-cartridge/Cargo.toml`:

```toml
# DELETE or comment out before cartridge GHA merge:
# [patch."https://github.com/tytolabs/umst-manifold.git"]
# umst-manifold = { path = "../umst-manifold" }
```

- [ ] Verify **without** sibling path (critical):

```bash
cd umst-concrete-cartridge
# Ensure no parent workspace patches umst-manifold
cargo test -p umst-concrete-cartridge --features manifest-bridge
cargo test -p umst-concrete-cartridge --features manifest-bridge --test manifest_bridge_catalog_grounding
cargo test -p umst-concrete-cartridge --test formal_anchors
```

- [ ] `rg 'physicalSecondLaw' crates/umst-concrete-cartridge/tests/formal_anchors.rs` — allowlist still TCB-clean.
- [ ] Update `docs/FORMAL_GROUNDING_AUDIT.md` remote CI row when green.
- [ ] **Operator only — push** cartridge branch/PR (user credentials).

### Optional — supercap parity

- [ ] In `umst-supercap-cartridge`: same git `rev` pin; remove local `[patch]` if present.
- [ ] `cargo check -p umst-supercap-cartridge --features manifest-bridge,manifold-gate`
- [ ] `cargo test -p umst-supercap-cartridge --test formal_anchors`
- [ ] Align `docs/FORMAL_SCALING.md` checklist with concrete cartridge.

---

## Phase 4 — Enable GHA `manifest-bridge` (cartridge repo)

File: `umst-concrete-cartridge/.github/workflows/rust.yml` (job `build-test` or dedicated job).

- [ ] Ensure workflow env includes (already present on concrete CI):

  - `CARGO_NET_GIT_FETCH_WITH_CLI: "true"` (anonymous git fetch for public `tytolabs/umst-manifold`)

- [ ] Add step **after** default workspace test (or separate job `manifest-bridge`):

```yaml
- name: manifest-bridge (git-pinned umst-manifold, W8)
  run: cargo test -p umst-concrete-cartridge --features manifest-bridge --verbose
```

- [ ] Optional stricter step:

```yaml
- name: manifest-bridge catalog grounding
  run: cargo test -p umst-concrete-cartridge --features manifest-bridge --test manifest_bridge_catalog_grounding --verbose
```

- [ ] Confirm GHA does **not** rely on `../umst-manifold` path (patch removed in Phase 3).
- [ ] Open PR; wait for green `build-test` + new `manifest-bridge` step.
- [ ] **Operator only — merge** when checks pass.

### MaOS monorepo drift workflow (paired fiber)

- [ ] `umst-manifold` standalone: `.github/workflows/umst-catalog-drift.yml` already runs `verify_umst_stack.sh` with `formal-witness`.
- [ ] After cartridge GHA is green, document release triple in [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md): `formal-witness` + catalog lock + cartridge `manifest-bridge` (see [Track H](PENDING_GOD_GRADE_ROADMAP.md)).

---

## Phase 5 — Close W8 in docs (no push required)

- [ ] Update [`AGENT_STATUS.md`](AGENT_STATUS.md): W8 / S1 / S9 → **DONE** (git + CI).
- [ ] Update [`TODO_COMPLETION.md`](TODO_COMPLETION.md) § concrete-cartridge-wire → remote ✅.
- [ ] Update [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md) diagram: remove “W8 pending on git pin”.
- [ ] `rg 'W8.*PENDING|manifest-bridge.*blocked' umst-manifold/docs` → no stale blockers.
- [ ] Refresh [`AGENT_W8_STATUS.txt`](AGENT_W8_STATUS.txt) “Remaining (ops)” section to **DONE** with `REV` and cartridge CI link.

---

## Done criteria (all must pass)

| Check | Command / signal |
|-------|------------------|
| Remote manifold exposes `manifest` | Clean clone `cargo check` + `rg mod manifest` in `cargo doc` |
| Cartridge tests without patch | `cargo test -p umst-concrete-cartridge --features manifest-bridge` exit 0 |
| Cartridge GHA | `rust.yml` `manifest-bridge` step green on `main` |
| Docs | No W8 “blocked” / “pending publish” in manifold docs |
| TCB | Still only `physicalSecondLaw` in Lean; cartridge `formal_anchors` unchanged |

---

## Rollback (operator)

- [ ] Revert cartridge `rev` pin to last known-good SHA; restore `[patch]` locally only for dev.
- [ ] Revert or disable GHA `manifest-bridge` step if remote manifold regressed.
- [ ] Re-open W8 rows in `AGENT_STATUS.md` / `TODO_COMPLETION.md`.

---

## References

| Doc | Role |
|-----|------|
| [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) § Track A | Canonical task breakdown A.1–A.4 |
| [`AGENT_W8_STATUS.txt`](AGENT_W8_STATUS.txt) | Local W8 code completion record |
| [`../umst-concrete-cartridge/docs/FORMAL_GROUNDING_AUDIT.md`](../umst-concrete-cartridge/docs/FORMAL_GROUNDING_AUDIT.md) | Cartridge gate / manifest-bridge audit |
| [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) § decision 3 | CI pairing `manifest-bridge` + `formal-witness` |
