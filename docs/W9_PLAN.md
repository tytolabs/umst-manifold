# W9 Phase 0 — Agnostic-on-Fork + Cartridge-Port Survey

**Status:** **LANDED** on `main` @ `7431c1b` — tags `v2.0.0-rc1` / `v2.0.0` (2026-06-15). Tier-2c grep-zero, lexicon/agnostic verifiers, `verify_umst_stack` green.  
**Branch:** merged `w9-agnostic-port` → `main` (was `bc87929`).  
**Canonical prompt:** [`docs/COMPOSER_W9_AGNOSTIC_AND_PORT.md`](../../docs/COMPOSER_W9_AGNOSTIC_AND_PORT.md) (workspace root).  
**Date:** 2026-06-15.

---

## Executive summary

Phase 0 surveyed live code in `umst-manifold` and `umst-concrete-cartridge` (read-only). Findings:

1. **Two distinct `ConcreteCartridge` types** share a name but implement **different traits** (`GateEvaluator` vs `IScienceCartridge`). The cartridge `IScienceCartridge` impl does **not** supersede the kernel gate stub.
2. **Gate paths do not call the cartridge for admissibility evidence today.** Transition checks run through host `f64` logic in `mix_proposal.rs` / `ThermodynamicMixFilter`; `ThmcSolver::step` accepts `C: IScienceCartridge` but **`_cartridge` is unused** in `step_experimental`.
3. **Phase B `GateCartridge`** will **invent** a new evidence contract (`transition_evidence`); nothing on `IScienceCartridge` today maps to gate xi/prop/dissipation without extraction work.
4. **Tier 2a locked decision:** **do not hard-delete** kernel `gate::ConcreteCartridge` because `IScienceCartridge` does not replace it — **move** the `GateEvaluator` marker to cartridge (A2), verify, then retire kernel source with `#[deprecated]` re-export one release.

Open PRs (informational, not blockers): manifold **#23**, **#25**; cartridge **#26**, **#27** (docs/ledger hygiene).

---

## Per-item dossier

| Item | Definition (file:line) | Callers / refs | S1–S4 | Frozen wire? | Test coverage | Tier | Strategy |
|------|------------------------|----------------|-------|--------------|---------------|------|----------|
| `MixTensor` | `src/core/tensors.rs:7` — `[Batch, Features]` fractions | `traits::IScienceCartridge::compute_all`, orchestration, cartridge `implementation.rs`, PPO/gateway tests | S1 | serde via UMST paths only | `golden_path_physics_cbf`, cartridge pipeline tests | **T1** | Rename → `StatePoint`; `pub use` deprecated alias |
| `mix_proposal` mod | `src/gate/mix_proposal.rs` (whole file) | `http_manifest`, `mix_eval_registry`, `gate/mod.rs`, `tests/gate_*`, `ros_contract_serde_roundtrip` | S1+S2 | C-ABI `thermodynamic_transition_admissible*` in egoff/ffi (frozen names) | `mix_proposal` unit tests; `gate_parity_fixture`, `gate_dual_run_parity` | **T1** | Rename mod → `transition_proposal`; serde `alias` on JSON fields |
| `MixProposalScalars` | `mix_proposal.rs:18` | HTTP gate IO, registry context | S1+S2 | JSON field names in HTTP contract | `gate_parity_fixture` | **T1** | → `TransitionScalars` |
| `DEFAULT_S_INTRINSIC_MPA`, `Q_HYDRATION_J_PER_KG` | `mix_proposal.rs:11–14` | `ThermodynamicStateSnapshot::from_mix_calibrated`, HTTP Powers closure | S2+S3 | HTTP manifest literals (`strength_intrinsic_mpa`) | `mix_proposal` tests | **T2c** | Cartridge-supplied via trait params; kernel keeps generic form |
| `ThermodynamicMixFilter` | `mix_proposal.rs:101` | `ThermodynamicMixEvaluator`, `HttpMixGateEvaluator` | S1 | — | `mix_proposal` tests, `gate_parity_fixture` | **T1** | → `TransitionFilter` |
| `ThermodynamicMixEvaluator` | `mix_eval_registry.rs:21` | `GateEvaluatorRegistry`, `HttpMixGateEvaluator` | S1+S3 | `catalog_id` = `thermodynamic_mix` | `gate_kleisli`, `embodied_orchestrator` | **T1** | → `TransitionEvaluator` |
| `HttpMixGateEvaluator` | `http_manifest.rs:58` | `gate_server_router`, `from_concrete_cartridge_defaults` | S1+S3 | `catalog_id` = `umst.gate.http_shim`; `gate_family` = `mix_prediction_vs_physics` | `gate_parity_fixture`, `gate_dual_run_parity` | **T1** | → `HttpTransitionEvaluator`; collapse `evaluate_mix*` |
| `from_concrete_cartridge_defaults()` | `http_manifest.rs:78` | `GateHttpRuntime` default ctor | S1 | — | indirect via HTTP tests | **T2b** | Retire; injection-only gate construction |
| `gate::ConcreteCartridge` (kernel) | `gate/concrete_cartridge.rs:12` — ZST, **`GateEvaluator` only** | `gate/mod.rs` re-export; `GATE_REGISTRY_CATALOG_IDS` | S1+S3 | `catalog_id` = `umst.cartridge.concrete.policy`; `gate_family` = `concrete_powers_manifest_defaults` | `gate_parity_fixture` (policy row) | **T2a** | **MOVE** to cartridge as `GateEvaluator` impl; **not delete** (see § Tier 2a) |
| `ConcreteCartridge<B>` (cartridge) | `crates/.../implementation.rs:187` — **`IScienceCartridge`** | CLI, MCP, topology harness, THMC in cartridge | S1 (cartridge repo) | — | `virtual_proxies`, `topology_mix_spec`, B6 harness | **—** | Stays in cartridge; Phase B → `SpatialCartridge` |
| `IScienceCartridge` | `core/traits.rs:54` — `compute_all`, `compute_topology` | orchestration, thmc, ppo, manifest, 15+ test stubs | S1 | trait name stable until Phase B | broad integration tests | **B1** | Phase A: keep single port; Phase B: split hierarchy |
| `SCALAR_HYDRATION_ALPHA` | `core/umst_schema.rs:30` | THMC, cartridge topology, schema docs | S2 | column index frozen for UMST layout | `thmc_drying_shrinkage`, cartridge tests | **T1** | → `SCALAR_INTERNAL_VARIABLE_0` + migration note |
| `HYDRATION_*` kinetics consts | `physics/solvers/thmc.rs:160–175` | `ThmcHydrationKinetics::default`, drying-shrinkage tests | S2 | — | `thmc_drying_shrinkage.rs` (byte-equivalent guard) | **T3** | Rename structure → `ReactionExtentKinetics`; values → cartridge (T2c bridge) |
| `ThmcSolver::step` cartridge param | `thmc.rs:288–296` — `C: IScienceCartridge` | cartridge `implementation.rs:357` (`thmc.step(self,…)`) | S1 | — | `thmc_drying_shrinkage`, `thmc_step_node_positions` | **T3** | Bound relaxes to spatial-only in B3; today param **unused** (`_cartridge`) |
| `THERMODYNAMIC_MIX_CATALOG_ID` | `traceability.rs:159` | `mix_eval_registry`, manifest, embodied tests | S3 | slug `thermodynamic_mix` → digest | `catalog_all_ids_registered` | **T2d** | UNIVERSAL rename + lockstep `catalog.json` |
| `MIX_PREDICTION_VS_PHYSICS_GATE_FAMILY` | `traceability.rs:172` | `HttpMixGateEvaluator::gate_family` | S3 | `gate_family` string | `gate_parity_fixture` | **T2d** | CARTRIDGE — move to cartridge registry |
| `concrete_powers_manifest_defaults` | `concrete_cartridge.rs:28` | kernel `GateEvaluator` only | S3 | `gate_family` | allowlist in `catalog_all_ids_registered` | **T2d** | CARTRIDGE — move out of `GATE_REGISTRY_CATALOG_IDS` |
| `umst.cartridge.concrete.policy` | `concrete_cartridge.rs:24` | `ALLOW_UNUSED_GATE_CATALOG_IDS` | S3 | `catalog_id` | registry tests | **T2d** | CARTRIDGE-owned catalog row |

---

## Traceability matrix (S3)

| catalog_id / gate_family | Rust const + location | catalog.json module (double-slit) | Tier | Digest impact |
|--------------------------|----------------------|---------------------------------|------|---------------|
| `umst.gate.cd_transition` | `CD_TRANSITION_CATALOG_ID` — `traceability.rs:156` | `Gate`, `GateCompat`, `DoubleSlit`, `DEC`, … | **UNIVERSAL** | Rename agnostic only with lockstep edit |
| `clausius_duhem_transition` | `ThermodynamicTransitionEvaluator::gate_family` — `evaluator.rs:65` | (family, not catalog_id) | **UNIVERSAL** | None if family string stable |
| `thermodynamic_mix` | `THERMODYNAMIC_MIX_CATALOG_ID` — `traceability.rs:159` | `Powers` | **UNIVERSAL** | **YES** — `CATALOG_UPDATE_PROTOCOL`, regen digest, pin bump |
| `thermodynamic_mix_transition` | `ThermodynamicMixEvaluator::gate_family` — `mix_eval_registry.rs:49` | `Powers` (family) | **UNIVERSAL** | Family rename with universal badge pass |
| `umst.gate.http_shim` | `HTTP_SHIM_CATALOG_ID` — `traceability.rs:162` | `Powers` | **UNIVERSAL** (shim) | Digest if slug changes |
| `mix_prediction_vs_physics` | `MIX_PREDICTION_VS_PHYSICS_GATE_FAMILY` — `traceability.rs:172` | — (gate_family only) | **CARTRIDGE** | Cartridge fiber digest block |
| `concrete_powers_manifest_defaults` | `concrete_cartridge.rs:28` | — | **CARTRIDGE** | Move to cartridge-declared registry |
| `umst.cartridge.concrete.policy` | `concrete_cartridge.rs:24` | — (allow-unused) | **CARTRIDGE** | New cartridge catalog row when wired |
| `umst.gate.kleisli_unit` | `kleisli.rs:147` | `Constitutional`, `ProbeOptimization`, … | **UNIVERSAL** | Standard |
| `umst.gate.landauer_cbf` | `LANDAUER_CBF_CATALOG_ID` — `traceability.rs:175` | `LandauerLaw`, `MeasurementCost`, … | **UNIVERSAL** | Standard |

---

## Port-split partition (Goal B design input)

### Survey: does gate call cartridge for evidence?

**No.** Evidence today is entirely **in-kernel**:

| Gate path | Evidence source | Cartridge involved? |
|-----------|-----------------|---------------------|
| `ThermodynamicMixFilter::check_transition` | `mix_proposal.rs` Powers `f64` snapshots | **No** |
| `thermodynamic_transition_admissible*` | Pure functions, same file | **No** |
| `ThermodynamicTransitionEvaluator` | `thermo_transition.rs` host CD gate | **No** |
| `HttpMixGateEvaluator` | Wraps `ThermodynamicMixEvaluator` + manifest literals | **No** (defaults named `from_concrete_cartridge_defaults` but no trait call) |
| `gate::ConcreteCartridge` | Returns `catalog_id` / `gate_family` only | **No** physics |
| `ManifoldGateway` / CBF | `PhysicalResult` from `IScienceCartridge::compute_topology` | **Spatial** summary, not transition gate |
| `ThmcSolver::step` | Operator-split internals; `_cartridge` **unused** | Param present; **no evidence pull** |

### `IScienceCartridge` method classification

| Method | Class | Minimal bound for current call sites |
|--------|-------|--------------------------------------|
| `compute_all(&MixTensor)` | **SPATIAL** — bulk constitutive closure | orchestration bulk step, cartridge pipeline |
| `compute_topology(&UMST)` | **SPATIAL** — DEC + solvers + `PhysicalResult` | PPO gateway, adjoint, THMC (unused), manifest |

### Proposed Phase B partition

```text
trait GateCartridge<B> {
    // INVENT (not extract): scalar transition evidence for admissibility
    fn transition_evidence(...) -> TransitionEvidence<B>;
}

trait SpatialCartridge<B>: GateCartridge<B> {
    fn compute_topology(&self, m: &UnifiedMaterialStateTensor<B>) -> PhysicalResult<B>;
    fn compute_all(&self, mix: &StatePoint<B>) -> PhysicalResult<B>;  // renamed
}
```

**GateCartridge extract vs invent:** Phase B must **invent** `transition_evidence` and wire cartridge Powers/Parrott closures into it. Optional bridge: delegate from `transition_evidence` to existing `mix_proposal` logic during migration, then evict cement literals from kernel.

**SpatialCartridge** subsumes all current `IScienceCartridge` impls (`ConcreteCartridge`, test stubs, golden-path bar cartridge).

---

## Tier 2a locked decision — kernel `gate::ConcreteCartridge`

### Evidence

| | Kernel `gate::ConcreteCartridge` | Cartridge `ConcreteCartridge<B>` |
|---|----------------------------------|----------------------------------|
| File | `src/gate/concrete_cartridge.rs` | `crates/umst-concrete-cartridge/src/core/implementation.rs` |
| Trait | `GateEvaluator` | `IScienceCartridge<B>` |
| Type | Zero-sized marker | Generic over `Backend`, holds `Profile` |
| Role | HTTP `gate_server` policy defaults without linking Burn | Tensor physics / topology / THMC |
| `catalog_id` | `umst.cartridge.concrete.policy` | (none on trait; formal blocks say NONE) |

**Conclusion:** Cartridge `IScienceCartridge` impl does **not** supersede kernel stub. They are **homonymous, orthogonal** types.

**Locked decision:** **Do not hard-delete** kernel stub in A2 on “superseded” grounds. **Move-verify-retire:**

1. Add `impl GateEvaluator for ConcreteCartridgePolicy` (or equivalent) in **umst-concrete-cartridge**.
2. Verify `gate_parity_fixture` + `catalog_all_ids_registered` at cartridge pin.
3. Kernel: `#[deprecated]` re-export one release → remove source after green release + zero refs.

---

## Phase A commit graph (A0–A8)

Each commit: **both repos compile + test green** (manifold first, cartridge pin bump second when needed).

| Commit | Scope | Precondition | Postcondition |
|--------|-------|--------------|---------------|
| **A0** | This document only (`docs/W9_PLAN.md`) | `main` @ `bc87929` green | Plan approved gate |
| **A1** | Tier 1 renames (MixTensor→StatePoint, mix_proposal→transition_proposal, …) + deprecated aliases | Lexicon baseline captured | `cargo test` green; serde aliases on frozen fields |
| **A2** | Move kernel `gate::ConcreteCartridge` (`GateEvaluator`) to cartridge; verify destination | A1 green | Cartridge owns policy evaluator; kernel deprecated re-export |
| **A3** | Retire `from_concrete_cartridge_defaults`; injection-only `GateHttpRuntime` | A2 green | No default material in kernel gate ctor |
| **A4** | Evict cement **values** to cartridge trait params (`Q_HYDRATION`, `DEFAULT_S_INTRINSIC`, …) | A3 green | Kernel `mix_proposal` uses injected params or generic placeholders |
| **A5** | Badge tiering: move CARTRIDGE anchors to cartridge registry; universal rename `thermodynamic_mix*` + `catalog.json` + digest | A4 green | `verify_umst_stack.sh` + `catalog_all_ids_registered` green |
| **A6** | Tier 3 THMC kinetics rename (`ThmcHydrationKinetics`→`ReactionExtentKinetics`, …) | A5 green | `thmc_drying_shrinkage.rs` byte-equivalent |
| **A7** | S4 formal mirror + PROOF-STATUS split; S2 lexicon-lint → 0; S1 agnostic-on-fork grep CI | A6 green | All four verifiers green |
| **A8** | Tag `v2.0.0-rc1`; CHANGELOG | A7 green | Phase A acceptance report |

Phase B (B1–B5) starts only after A8 user go-ahead.

---

## Move-verify-retire triples

| Item | Move to | Verify at destination | Retire kernel source |
|------|---------|----------------------|----------------------|
| `gate::ConcreteCartridge` (GateEvaluator) | `umst-concrete-cartridge` policy module | `gate_parity_fixture` policy row; `catalog_all_ids_registered` | `#[deprecated] pub use` → delete after release |
| Cement constants in `mix_proposal.rs` | Cartridge trait defaults / `Profile` | `mix_proposal` tests with injected params | Deprecated trait defaults one release |
| `from_concrete_cartridge_defaults` | Cartridge factory `HttpTransitionEvaluator::from_profile` | HTTP integration tests | Delete kernel shim A3 |
| CARTRIDGE badge strings | Cartridge `GATE_REGISTRY` block | Bidirectional catalog check | Remove from kernel `GATE_REGISTRY_CATALOG_IDS` A5 |
| `ThmcHydrationKinetics` calibration | Cartridge via trait | `thmc_drying_shrinkage` parity | Kernel keeps generic reaction-extent form A6 |

---

## Risk register

| ID | Risk | Mitigation |
|----|------|------------|
| R1 | **Two `ConcreteCartridge` types** — conflation in docs/PRs | Rename kernel moved type to `ConcretePolicyEvaluator` or `ConcreteGateCartridge`; grep CI |
| R2 | **GateCartridge invent** — no existing evidence method on trait | Phase B delivers `transition_evidence`; bridge from `mix_proposal` f64 during migration |
| R3 | **THMC Wave 1 before A6/T3 locked** — rename moving physics | Guard: `thmc_drying_shrinkage` byte-equivalent; stop+report if parity breaks |
| R4 | **Frozen HTTP/C-ABI strings** — `thermodynamic_transition_admissible`, JSON fields | `serde(alias)`, C-ABI alias layer in egoff; schema bump |
| R5 | **Digest pin cascade** on universal badge rename | Single logical step: Rust + `catalog.json` + digest regen + cartridge pin |
| R6 | **ThmcSolver bound** — today requires full `IScienceCartridge` but ignores cartridge | A6 documents; B3 relaxes to `SpatialCartridge` only on solver paths |
| R7 | **Open PRs #23–27** — ledger/docs drift | Rebase after Phase A doc-only merges; no kernel conflict expected |

---

## Deprecate-not-delete list

| Symbol | Removal ticket | Earliest removal |
|--------|----------------|------------------|
| `MixTensor` (alias) | Post A1 + one release | After cartridge pin on agnostic rename tag |
| `mix_proposal` mod (alias) | Post A1 | Same |
| `from_concrete_cartridge_defaults` | Post A3 | One release after injection-only gate |
| `gate::ConcreteCartridge` re-export | Post A2 | After cartridge owns `GateEvaluator` |
| `ThermodynamicMixFilter` (alias) | Post A1 | One release |
| `HYDRATION_*` module consts | Post A6 + cartridge override | When `ReactionExtentKinetics` fully cartridge-fed |
| `PREDICTION_VS_PHYSICS_CATALOG_ID_DEPRECATED` | Telemetry sunset | When HTTP clients migrate to `http_shim` |

---

## Planning-complete checklist

- [x] Per-item dossier table (definition, callers, S1–S4, frozen-wire, tests, tier)
- [x] Traceability matrix (catalog_id / gate_family → tier + digest impact)
- [x] Port-split partition with **gate→cartridge evidence survey** (answer: **no calls today**)
- [x] Tier 2a locked decision documented with evidence (**move, not delete**)
- [x] Phase A commit graph A0–A8
- [x] Move-verify-retire triples
- [x] Risk register (ConcreteCartridge homonym, GateCartridge invent, THMC timing)
- [x] Deprecate-not-delete list
- [x] Open PRs #23–27 noted
- [x] **User approval** — waived 2026-06-15; stack merged without pause

---

## Files surveyed (read-only)

- `src/core/traits.rs` — `IScienceCartridge` (2 methods)
- `src/gate/concrete_cartridge.rs` — kernel `GateEvaluator` stub
- `src/gate/mix_proposal.rs` — Powers transition + cement constants
- `src/runtime/catalog/traceability.rs` — badge registry SSOT
- `src/physics/solvers/thmc.rs` — `IScienceCartridge` bound sites (unused cartridge)
- `umst-concrete-cartridge/.../implementation.rs` — `IScienceCartridge for ConcreteCartridge<B>`

**Explicitly not touched (per batch scope):** `q1_hex_elasticity.rs`, `ai/topology.rs`, `shell_topology_rib_pattern.rs`, c1 threshold, 200-outer run, manifold→cartridge `[patch]`.

---

## Parallel development — prime-spectral fence (2026-06-15)

**W9 owns:** Tier-2c injected closures, `injection_mechanism_fixture.rs` (111/222 sentinels), gate parity, lexicon/agnostic verifiers, catalog lock @ **122** / digest `c61b1bef…`.

**Prime-spectral-research** (branch `prime-spectral-research`) owns benchmarks, witness tests, protocol MD — **CLOSED / AMBER** per [`outputs/prime-spectral-research/FINAL_FINDING.md`](../../outputs/prime-spectral-research/FINAL_FINDING.md). **Do not merge research WIP onto `main`.** Durable pointer for the surviving NTT lead: [umst-manifold#26](https://github.com/tytolabs/umst-manifold/issues/26) (parked; not load-bearing).

| Surface | W9 rule |
|---------|---------|
| **Established (keep compiling, do not delete or grow)** | `src/physics/prime_spectral_filter.rs` + single `physics/mod.rs` decl (feature-gated; **no** `prime_spectral_research` mod). Lean prime-spectral bundle + `umst.guidance.prime_spectral` catalog id live in formal repos — landed on `main` via catalog allowlist. |
| **Research lane only** | `tests/prime_spectral_*`, benchmark targets, topology_solver hooks — stay on `prime-spectral-research`, not `main`. |
| **`traceability.rs`** | W9 badge rows landed; `umst.guidance.prime_spectral` rows from formal mirror. |
| **`shell_topology_rib_pattern.rs`** | **No-touch** (B6 + prime-spectral Tier-2 testbed). |

**Sign-off:** [`outputs/w9-sign-off-package.md`](../../outputs/w9-sign-off-package.md) — `verify_umst_stack` exit 0 @ 2026-06-15.
