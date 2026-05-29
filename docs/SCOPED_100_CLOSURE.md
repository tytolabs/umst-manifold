# Scoped true 100% — honest closure (god-grade)

**As of:** 2026-05-22  
**SSOT:** [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) · [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) · [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md)

---

## Headline (do not over-claim)

| Ceiling | Status | Honest label |
|---------|--------|--------------|
| **In-repo automation (17 rows)** | **17 / 17 = 100%** | All checklist automation criteria green when run per quick-verify |
| **Hot-path Lean enforcement** | **18 / 69 ≈ 26%** · **18 / 119 ≈ 15%** | **By design** — not a failure mode |
| **Lean on inference / robot loop** | **No** | Export + lock + parity only; never `lake` per step |
| **Scoped true 100%** | **1 / 3 blockers → Done** | **~33%** at Done morphism (B3); B1 W8 + B2 FFI open |

**Policy:** **17/17 automation** does **not** mean hot-path proof coverage, git-published cartridges (W8), or FFI extraction. Release `StrictCatalogMatch` default is **Done** (B3). See [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md).

---

## Functional programming closure (Blocker → Evidence → Done)

Each scoped item is one morphism. **Done** requires operator/product sign-off or a horizon program milestone — not merely local `cargo test` green.

```
Blocker ──evidence──▶ Done
         (partial Evidence ≠ Done)
```

| ID | Blocker (domain) | Owner | Cannot automate | Evidence today | Done criterion |
|----|------------------|-------|-----------------|----------------|----------------|
| **B1** | **W8** — remote git + GHA without workspace `[patch]` | **human** (operator credentials) | `git push`, `gh`, cartridge PR merge, org trust | **Prep (machine):** `bash scripts/w8_publish_readiness.sh` exit **0** @ workspace — lock **119** + `0697014f…`, **16/16** markers, secrets hygiene, `manifest-bridge` + `[patch]` ([`AGENT_W8_STATUS.txt`](AGENT_W8_STATUS.txt)) | Phases 1–4 + done table in [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) |
| **B2** | **FFI** — extracted witnesses / attestation beyond digest pin | **human + code** (long horizon) | Full Lean→runtime certificate per lemma; no v1 CI row | R0 digest + `formal-witness` attestation only ([`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md)) | Separate FFI program + reviewed attestation API |
| **B3** | ~~**Strict prod default**~~ | **Done** (in-repo) | — | `not(debug_assertions)` → `StrictCatalogMatch`; `for_staging()`; `for_release_profile()`; `manifest_strict_witness` **4/4**; `ci_god_grade_profile` **3/3**; gateway auto lock digest ([`umst_manifest.rs`](../src/manifest/umst_manifest.rs), [`ppo.rs`](../src/ai/ppo.rs)) | H.1–H.2 closed: release default strict + `ManifoldGateway::new` pins lock digest |

**Automation 17/17:** rows 1–17 in [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) § Automation criteria — **all at Done** (in-repo). **B1–B3** are **outside** that denominator.

---

## Remaining blocker count and % to scoped true 100%

| Metric | Value |
|--------|-------|
| **Remaining blockers (Done morphisms)** | **2** — B1 W8 · B2 FFI |
| **Blockers at Done** | **1** — **B3** strict prod default + auto digest |
| **% to scoped true 100%** | **~33%** (1/3 Done) |

**Partial Evidence (does not advance Done):**

| Blocker | Partial % of that blocker | Why not Done |
|---------|---------------------------|--------------|
| B1 W8 | ~70% prep automated | `w8_publish_readiness.sh` + `w8_publish_readiness` test green; remote `git ls-remote` / GHA without `[patch]` still ❌ |
| B3 Strict | **Done** | `default_grounding_contract` + `for_staging()` + gateway/UMST lock digest helpers |
| B2 FFI | 0% in v1 | Horizon — not scheduled in automation rows |

**Optional scoped view (2 blockers):** Remaining **W8 + FFI** only — **~33%** at Done (B3 closed).

**Do not report:** hot-path **100%**, Lean-on-robot **100%**, or unscoped god-grade **100%**.

---

## B1 — W8 org steps (numbered, human-owned)

**Runbook SSOT:** [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) · **Local code:** ✅ per [`AGENT_W8_STATUS.txt`](AGENT_W8_STATUS.txt)

| Step | Phase | Work (human) | Verify (cannot run unattended push) |
|------|-------|--------------|-------------------------------------|
| 1 | **0** | Manifold preflight: `cargo check`, `cargo test`, `verify_umst_stack.sh`, `bidirectional_catalog_check.sh` | Agent/CI may run; record `REV` |
| 2 | **1** | Operator `git push origin main` on `tytolabs/umst-manifold` | `git ls-remote … refs/heads/main` |
| 3 | **2** | Clean-clone verify **without** MaOS `[patch]` | `/tmp/umst-manifold-w8-verify` `cargo check` + `rg 'mod manifest'` in `cargo doc` |
| 4 | **3** | Cartridge git `rev` pin; **remove** `[patch]`; push cartridge | `cargo test -p umst-concrete-cartridge --features manifest-bridge` (no sibling path) |
| 5 | **4** | Enable GHA `manifest-bridge` job in cartridge `rust.yml` | Green workflow on `main` |
| 6 | **5** | Close W8 in docs (`AGENT_STATUS`, `TODO_COMPLETION` remote row) | `rg 'W8.*PENDING'` → no stale blockers |

**Cannot automate:** credentials for `git push`, PR merge, org repo permissions.

### B1 prep Evidence (machine-verified — does not advance Done)

| Check | Command / signal | Status |
|-------|------------------|--------|
| Prep gate script | `bash scripts/w8_publish_readiness.sh` → `w8_publish_readiness: OK` | ✅ workspace |
| Regression test | `cargo test --test w8_publish_readiness` | ✅ |
| Lock R0 pin | `module_count=119`, digest prefix `0697014f` | ✅ |
| God-grade wiring | **16/16** markers present in `verify_umst_stack.sh` | ✅ |
| Secrets hygiene | no tracked/staged `.env` / credentials paths | ✅ |
| Manifold API | `pub mod manifest`; `manifest-bridge` feature | ✅ |
| Cartridge local | `cargo test -p umst-concrete-cartridge --features manifest-bridge` with workspace `[patch]` | ✅ |
| Full stack (optional) | `UMST_W8_RUN_FULL_STACK=1 UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/w8_publish_readiness.sh` | operator |

**Step counts (W8 publish morphism):** **1 machine-verified** prep step (script + test) · **5 human-only** steps (Phases 1–5 in table above). Phase **0** preflight is covered by the prep script; Phases **1–5** unchanged and still require operator credentials.

---

## B2 — FFI horizon (strict, outside v1)

| Field | Value |
|-------|--------|
| **Meaning** | Extracted proof witnesses or FFI attestation linking Lean terms to runtime certificates |
| **Owner** | **human + code** (formal / long program) |
| **Test today** | None in scoped v1 — digest attestation only: `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` |
| **Cannot automate** | Per-lemma extraction pipeline, TCB review for any new runtime axiom |
| **Blocks** | Full formal–runtime equivalence — **not** gate-law or `verify_umst_stack` PASS |

---

## B3 — Strict production default — **Done** (2026-05-22)

| Field | Value |
|-------|--------|
| **Gap** | ~~`UmstManifestBuilder::default()` advisory~~ → release `StrictCatalogMatch` via `default_grounding_contract()`; debug uses `for_staging()` |
| **Owner** | **Done** (in-repo) |
| **Test** | `manifest_strict_witness` **4/4**; `ci_god_grade_profile` **3/3**; `formal_witness` gateway pin test; `ManifoldGateway::new` + `with_lock_catalog_schema_digest()` |
| **Evidence** | G-04: `default_grounding_contract()` + `UMST_RELEASE_MANIFEST_PROFILE=1`; G-05: `UMST_LOCK_UPSTREAM_CATALOG_DIGEST_HEX` + `lock_upstream_catalog_digest_bytes()`; `verify_umst_stack.sh` exit **0** @ **2026-05-22**; [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) |

---

## Quick verify (automation 17/17 — in-repo only)

```bash
cd umst-manifold
cargo test --test gate_kleisli --test gate_reject_catalog_id --test gate_adversarial
cargo test --features formal-witness --test manifest_strict_witness
cargo test --features ros2-contract,serde --test epistemic_trace_schema
cargo test --features trace-calibration --test trace_calibration
cargo test --test regime_soundness_claims_allowlist
UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh
```

**Scoped blockers:** no single command closes B1–B2; run `bash scripts/w8_publish_readiness.sh` for W8 prep, then human Phases 1–5 after operator push.

---

## Cross-links

| Doc | Role |
|-----|------|
| [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) | 17 automation rows + org/horizon |
| [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) | Three ceilings |
| [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) | Verify ledger timestamps |
| [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md) | Executive rollup (sync scoped % here when B* close) |
