# FP categorical IO — Kleisli / IO barrier (`fp-v04-4-kleisli-io`)

**Epic:** `maos-fp-categorical-v04` · **Task:** `fp-v04-4-kleisli-io`

This note pins **where host-visible effects live** in the UMST stack: differentiable tensors stay inside solver/cartridge **kernels**; **scalar sync**, **filesystem**, and **JSON-RPC** belong **outside** the tight Newton/CG inner loops or behind explicit boundaries (`ManifoldGateway`, façade runners).

---

## 1. ManifoldGateway / cartridge boundary as an IO monad barrier

**Reading (informal):** Treat effectful host reads/writes as living in an **IO-like** layer. Spatial physics stays on the Burn tensor graph (`Tensor<B, …>`). Crossing to `f32` / `f64` host control flow for rewards, Landauer bits, or persistence is a deliberate **barrier** — analogous to extracting a value from a monad at the **edge** of a Kleisli pipeline (compose morphisms inside `Tensor`; **bind** to policy / persistence only at the gateway).

| Layer | Responsibility | Typical effects |
|-------|------------------|-----------------|
| **`IScienceCartridge::compute_*`** | Functorial physics → `PhysicalResult` | Pure-ish tensor ops; avoid gratuitous `.into_scalar()` in inner iterations |
| **`ManifoldGateway::evaluate_topology_step`** | Policy-facing gate | `sum_dim` / `squeeze` reductions; `Result<_, String>` for CBF rejection; **no** `.into_scalar()` on the scalar reward path here (uses tensor reductions; see `src/ai/ppo.rs`) |
| **`ThermodynamicCBF::verify_tensor_update`** | Bits + dissipation → host | Canonical **`.into_scalar()`** on batch-summed `info_gain` and batch-summed `d_int` for Landauer, material entropy branch, and credit (**documented** in `ppo.rs`, `cbf.rs`) |
| **Cartridge façade / CLI / MCP** | Serialization, RPC | `serde_json`, `std::fs`, stdin/stdout JSON-RPC |

Primary sources:

- `src/ai/ppo.rs` — module docs under **“IO barrier (lazy solver cores, maos-fp-categorical-v04)”**.
- `docs/Mathematical-Foundations.md` § Cartridge interface.

---

## 2. Grep verification recipes (reproduce inventories)

Run from workspace root (`MaOS-Workspace`) unless noted. **`rg`** = ripgrep (`grep -R` equivalents noted).

### 2.1 `into_scalar` — solver-adjacent (`umst-manifold`)

```bash
rg -n 'into_scalar' umst-manifold/src/physics/solvers umst-manifold/src/physics/mechanics.rs umst-manifold/src/physics/adjoint.rs
```

### 2.2 `into_scalar` — gateway / RL barrier (`umst-manifold`)

```bash
rg -n 'into_scalar' umst-manifold/src/ai umst-manifold/src/core/emergence.rs
```

### 2.3 Host tensor escapes — CI policy (`umst-manifold`)

```bash
bash umst-manifold/scripts/check_physics_no_gradient_break.sh
# Pattern inside script: into_scalar|into_data under src/physics (allowlist-aware)
```

### 2.4 File I/O — manifold physics tree

```bash
rg -n 'std::fs|\bFile::|read_to_string|write\(' umst-manifold/src/physics
```

### 2.5 File I/O + JSON — concrete cartridge (solver-adjacent façade)

```bash
rg -n 'std::fs|\bFile::|read_to_string|serde_json|jsonrpc' umst-concrete-cartridge/crates/umst-concrete-cartridge/src
rg -n 'jsonrpc|serde_json' umst-concrete-cartridge/crates/umst-mcp/src
```

---

## 3. Inventory — `into_scalar` (solver-adjacent paths)

Snapshot aligned with grep recipes above (comments / docs lines included).

| File | Role |
|------|------|
| `src/physics/solvers/electrochemistry.rs` | CG inner diagnostics, convergence checks, comparison helpers |
| `src/physics/solvers/fracture_field.rs` | Weighted reductions / norms |
| `src/physics/solvers/rheology_flow.rs` | Poisson/CG residuals, diagnostics; **many** hits under `#[cfg(test)]` per allowlist notes |
| `src/physics/solvers/thmc.rs` | Newton CG scalar reductions (`rs_old`, `p_ap`, …) — **allowlisted** |
| `src/physics/solvers/thmc_residual.rs` | Combined residual energy `s`, functor summaries |
| `src/physics/mechanics.rs` | Equilibrium CG norms / guards |
| `src/physics/adjoint.rs` | Compliance scalar `c_raw` from `comp.sum()` |

**Gateway / policy (not inner solvers):** `src/ai/topology.rs` (optimization stepping), `src/ai/cbf.rs` (bits resolved), `src/core/emergence.rs` (diagnostics).

**Concrete cartridge (domain façade):** `umst-concrete-cartridge/.../implementation.rs`, `pipeline/orchestrator.rs`, `facade/mod.rs`, `mix_layout.rs`, examples — **scalar UX / reporting**, not manifold Newton kernels.

---

## 4. Inventory — file I/O

| Location | Finding |
|----------|---------|
| `umst-manifold/src/physics/**` | **No** `std::fs` / `File::` / `read_to_string` in solver tree (grep-empty as of this audit). |
| `umst-concrete-cartridge/.../calibration.rs` | **`fs::read_to_string`** for calibration inputs — appropriate **cartridge** concern. |
| `umst-manifold/src/ai/ppo.rs` (docs) | States crate does not load UMST from disk; I/O belongs in cartridges or runners. |

---

## 5. Inventory — JSON-RPC (solver-adjacent transport)

| Location | Role |
|----------|------|
| `umst-concrete-cartridge/crates/umst-mcp/src/main.rs` | Stdio **JSON-RPC 2.0** MCP server (`umst_predict`, `umst_audit`, …) — **transport boundary**, not in manifold solver kernels. |
| `umst-concrete-cartridge/crates/umst-mcp/tests/integration.rs` | Integration tests for JSON-RPC frames. |
| `umst-concrete-cartridge/.../facade/mod.rs` | Wire DTOs; **`serde_json`** called from **outside** this crate per module docs. |

---

## 6. Result / Option chaining — recommendations

**Current pattern (keep):** `ManifoldGateway::evaluate_topology_step` returns `Result<(VerifiedUMST, Tensor<B,1>), String>` and uses `match` on `cbf.verify_tensor_update` — clear **short-circuit** on CBF failure.

**Guidelines (minimal churn):**

1. **Prefer `?` only** where error type matches (`Result<T, E>` with consistent `E`). For `String` errors, explicit `match` or `map_err` is fine; avoid wide refactors.
2. **Optional physics:** keep `Option<Tensor<…>>` (e.g. `temperature_delta`) **unpacked once** at merge sites (`apply_physics_to_umst`) rather than nested `if let` chains in hot loops.
3. **Do not** replace inner-solver `.into_scalar()` with `Option` without a numerics review — those reads are often **convergence predicates**, not recoverable errors.

**Code edits:** None applied in this pass; gateway path is already `Result`-based and covered by `tests/gateway_info_gain.rs` / `tests/golden_path_physics_cbf.rs`.

---

## 7. Findings — severity

| ID | Finding | Severity | Notes |
|----|---------|----------|-------|
| F-1 | Newton/CG and functor layers use **many** `.into_scalar()` / `.into_data()` reads for norms and host-side logic | **M** | Expected for iterative solvers; keep out of **training** inner loops where possible; `ppo.rs` documents reward path without extra scalars. |
| F-2 | `scripts/check_physics_no_gradient_break.sh` vs tree may **deviate** (non-allowlisted files still containing patterns) | **M** | Policy drift: either extend allowlist with rationale or reduce host escapes — run script in CI to enforce. |
| F-3 | JSON-RPC / `serde_json` concentrated in **umst-mcp** and façade runners — **not** in `umst-manifold` solvers | **S** | Correct separation for IO monad barrier. |
| F-4 | File read in **calibration** path (`calibration.rs`) | **S** | Appropriate cartridge-level IO. |

---

## 8. Verification log (this deliverable)

| Check | Command | Result |
|-------|---------|--------|
| Targeted test (gateway IO path) | `cd umst-manifold && cargo test --test gateway_info_gain` | **PASS** (2026-05-11) |
| Clippy (package) | `cd umst-manifold && cargo clippy -p umst-manifold --all-targets -- -D warnings` | **PASS** (2026-05-11) |
| Physics gradient script | `bash umst-manifold/scripts/check_physics_no_gradient_break.sh` | **FAIL** locally — see F-2 |

---

## 9. Related docs

- `docs/Category-of-Material-Updates.md` — categorical material updates epic.
- `docs/Mathematical-Foundations.md` — cartridge + gateway semantics.

If `FP_CATEGORICAL_HARDENING_AUDIT.md` is added alongside this workstream, add a **single** cross-reference line there pointing to this file (avoid bulk edits).
