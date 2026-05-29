# W8 publish runbook — `tytolabs/umst-manifold` + cartridge `manifest-bridge`

**Scope:** Unblock remote git consumers of `umst-manifold::manifest` so `umst-concrete-cartridge` (and optionally `umst-supercap-cartridge`) can run **`manifest-bridge`** in GitHub Actions **without** workspace `[patch]`.

**Witness ladder:** R5 — [Manifest bridge + formal witness](GOD_GRADE_WITNESS_LADDER.md#r5--manifest-bridge--formal-witness-deployment-fiber) (paired with `formal-witness` in MaOS drift CI).

**Status (2026-05-29):** **Phase 1 DONE** — `tytolabs/umst-manifold` `main` @ **`fe22437`** (`pub mod manifest`, CI green). **G-02 DONE** — concrete cartridge git `rev = fe22437`, GHA `manifest-bridge` without `[patch]`. **G-03** supercap remote bridge optional. SSOT detail: [`AGENT_W8_STATUS.txt`](AGENT_W8_STATUS.txt), [`AGENT_STATUS.md`](AGENT_STATUS.md).

**Roadmap:** Track **A** in [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) — remaining org polish is **G-03** + strict-default / v2 traces (not blocking concrete).

**Agent policy:** Do **not** `git push`, `gh`, or `cargo publish` to crates.io unless the operator has confirmed credentials and asked for it. Push/publish steps below are **historical / rollback** reference unless reopening W8.

---

## Prerequisites (baseline — satisfied @ fe22437)

- [x] Local tree matches W8 surface: `umst-manifold/src/lib.rs` has `pub mod manifest` (not feature-gated).
- [x] `umst-manifold/Cargo.toml` declares `manifest-bridge = []` and `manifold-manifest = []`.
- [x] `umst-concrete-cartridge` passes with **git pin** (no patch required):

```bash
cd umst-manifold && cargo check
cd ../umst-concrete-cartridge && cargo test -p umst-concrete-cartridge --features manifest-bridge
```

- [x] Operator had write access for Phase 1 publish (completed).
- [x] `umst-formal-double-slit` export digest matches `umst-manifold/artifacts/catalog.lock.json` for stack verify (`0697014f…`, **119** modules).

---

## Phase 0 — Manifold preflight (local, no push)

Run from `MaOS-Workspace/umst-manifold`:

- [x] `cargo check`
- [x] `cargo test -p umst-manifold`
- [x] `cargo doc --no-deps -p umst-manifold 2>&1 | rg 'mod manifest'` → shows `manifest` in public API
- [ ] `cargo publish --dry-run` (optional; crates.io not required for git consumers)
- [x] `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` (when formal sibling present)
- [x] `bash scripts/bidirectional_catalog_check.sh` (when formal sibling present)
- [x] `rg '^axiom ' ../umst-formal-double-slit/Lean/LandauerLaw.lean` → single `physicalSecondLaw`
- [x] `bash scripts/w8_publish_readiness.sh` — prep gate; accepts git-pinned cartridge **or** workspace `[patch]`

Published commit SHA: **`fe2243716112f2504b063e55eb1e15e97ced4bdc`** (`fe22437`).

---

## Phase 1 — Publish `tytolabs/umst-manifold` `main` — **DONE**

> Completed **2026-05-29**. Remote `main` exposes `manifest`; manifold CI [run 26649667467](https://github.com/tytolabs/umst-manifold/actions/runs/26649667467) green.

```bash
REV=fe2243716112f2504b063e55eb1e15e97ced4bdc
git ls-remote https://github.com/tytolabs/umst-manifold.git refs/heads/main
# expect prefix fe22437…
```

---

## Phase 2 — Clean-clone verify (no MaOS `[patch]`) — **DONE**

```bash
git clone https://github.com/tytolabs/umst-manifold.git /tmp/umst-manifold-w8-verify
cd /tmp/umst-manifold-w8-verify
git checkout fe2243716112f2504b063e55eb1e15e97ced4bdc
cargo check -p umst-manifold
cargo doc --no-deps -p umst-manifold 2>&1 | rg 'mod manifest'
```

---

## Phase 3 — Bump cartridge git pin (concrete) — **DONE (G-02)**

Repo: `umst-concrete-cartridge` (`crates/umst-concrete-cartridge/Cargo.toml`).

- [x] Pin `umst-manifold` git dep to **`rev = fe22437`**
- [x] Workspace `[patch]` **removed** (no `../umst-manifold` override in workspace `Cargo.toml`)
- [x] Verify without sibling path:

```bash
cd umst-concrete-cartridge
cargo test -p umst-concrete-cartridge --features manifest-bridge
cargo test -p umst-concrete-cartridge --features manifest-bridge --test manifest_bridge_catalog_grounding
cargo test -p umst-concrete-cartridge --test formal_anchors
```

- [x] `docs/FORMAL_GROUNDING_AUDIT.md` remote CI row updated

### Optional — supercap parity (**G-03**, open)

- [ ] In `umst-supercap-cartridge`: same git `rev` pin; remove local `[patch]` if present.
- [ ] `cargo check -p umst-supercap-cartridge --features manifest-bridge,manifold-gate`
- [ ] `cargo test -p umst-supercap-cartridge --test formal_anchors`
- [ ] Wire GHA `manifest-bridge` on supercap (Track **I.3**)

---

## Phase 4 — Enable GHA `manifest-bridge` (concrete) — **DONE**

File: `umst-concrete-cartridge/.github/workflows/rust.yml`.

- [x] `CARGO_NET_GIT_FETCH_WITH_CLI: "true"`
- [x] Step `manifest-bridge tests (pinned umst-manifold)` — green on `main` without `../umst-manifold`

### MaOS monorepo drift workflow (paired fiber)

- [x] `umst-manifold` standalone: `.github/workflows/umst-catalog-drift.yml` runs `verify_umst_stack.sh` with `formal-witness`.
- [x] Release triple documented in [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md): `formal-witness` + catalog lock + cartridge `manifest-bridge` on git **`fe22437`**.

---

## Phase 5 — Close W8 in docs — **DONE** (manifold tree)

- [x] [`AGENT_STATUS.md`](AGENT_STATUS.md): W8 / S1 / S9 → **DONE**
- [x] [`TODO_COMPLETION.md`](TODO_COMPLETION.md): remote **G-02** ✅
- [ ] [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md) — remove stale “W8 pending” diagram labels if any remain
- [x] `bash scripts/w8_publish_readiness.sh` accepts git pin (not patch-only)
- [x] [`AGENT_W8_STATUS.txt`](AGENT_W8_STATUS.txt) — remote publish + G-02 closed

---

## Done criteria (all must pass)

| Check | Command / signal | Status |
|-------|------------------|--------|
| Remote manifold exposes `manifest` | Clean clone `cargo check` + `manifest` in `cargo doc` | ✅ **fe22437** |
| Cartridge tests without patch | `cargo test -p umst-concrete-cartridge --features manifest-bridge` exit 0 | ✅ **G-02** |
| Cartridge GHA | `rust.yml` `manifest-bridge` step green on `main` | ✅ |
| Docs | No W8 “blocked” / “pending publish” in manifold VERIFY + runbook | ✅ (this file) |
| TCB | Only `physicalSecondLaw` in Lean; cartridge `formal_anchors` unchanged | ✅ |

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
| [`AGENT_W8_STATUS.txt`](AGENT_W8_STATUS.txt) | W8 code + remote closure record |
| [`../umst-concrete-cartridge/docs/FORMAL_GROUNDING_AUDIT.md`](../umst-concrete-cartridge/docs/FORMAL_GROUNDING_AUDIT.md) | Cartridge gate / manifest-bridge audit |
| [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) § decision 3 | CI pairing `manifest-bridge` + `formal-witness` |
| [`VERIFY.md`](VERIFY.md) §3.2 | Cartridge verify commands (git pin vs optional patch) |
