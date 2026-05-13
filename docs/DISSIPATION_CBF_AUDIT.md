# Dissipation × `PhysicalResult` × ThermodynamicCBF — solver audit

**Workspace:** `umst-manifold`  
**Scope:** `src/physics/solvers/**/*.rs` searched for `dissipation` and `PhysicalResult` (case-sensitive, whole-word style substrings as in ripgrep).  
**Date:** 2026-05-12  

## Methodology

- **Grep:** `dissipation` **or** `PhysicalResult` under `src/physics/solvers/` → **no matches** (solvers do not name these symbols).
- **Architecture (as coded):** `PhysicalResult` is defined in `src/core/traits.rs` and produced only by implementations of `IScienceCartridge` (`compute_all` / `compute_topology`). Stateful solvers return updated tensors (e.g. `ThmcState` in `src/physics/solvers/thmc.rs`), not `PhysicalResult`.
- **CBF entry:** `ManifoldGateway::evaluate_topology_step` in `src/ai/ppo.rs` is the in-crate caller that reduces `PhysicalResult::dissipation` and passes a batch vector into `ThermodynamicCBF::verify_tensor_update` in `src/ai/cbf.rs`.

## Solver → `PhysicalResult.dissipation` populated? → ThermodynamicCBF touchpoints

| Solver (module / primary type) | `dissipation` / `PhysicalResult` in solver sources? | Field populated by this solver? | ThermodynamicCBF touchpoints |
|--------------------------------|------------------------------------------------------|----------------------------------|------------------------------|
| `solvers::thmc` (`ThmcSolver`, `ThmcState`, …) | **No** (docs mention cartridge; code uses `_cartridge` in `step_experimental`) | **No** — advances `ThmcState` only; does **not** call `compute_topology` or fill `PhysicalResult` | **Indirect:** orchestration calls `ThmcSolver::step`; any CBF path needs a **separate** cartridge functor (or test harness) to build `PhysicalResult`. **Gap:** cartridge parameter is currently unused in the experimental step body. |
| `solvers::thmc_residual` (THMC residual layouts / Newton helpers) | **No** | **No** — residual assembly for implicit THMC blocks | **None** in solver; CBF only if a caller maps residuals or state deltas into a cartridge’s `dissipation` (not implemented here). |
| `solvers::thmc_jfnk` (`thmc-coupled` + `solver-experimental`) | **No** | **No** | **None** |
| `solvers::fracture_field` (`PhaseFieldFractureSolver`, …) | **No** | **No** — updates damage / phase-field tensors | **None**; fracture energy release could inform a cartridge’s `dissipation` / `free_energy`, but that mapping is **not** in this module. |
| `solvers::rheology_flow` (`BinghamFlowSolver`, …) | **No** (“viscous” appears in momentum docs/code, not `dissipation`) | **No** | **None**; viscous dissipation could be projected into `PhysicalResult` by a cartridge — **not** emitted here. |
| `solvers::topology_solver` (`TopologySolver`, density diffusion) | **No** | **No** | **None** |
| `solvers::acoustics` (`AcousticWaveSolver`, optional Newmark types) | **No** | **No** | **None** |
| `solvers::electrochemistry` (`ElectroChemicalSolver`, PNP helpers) | **No** | **No** | **None** |
| `solvers::photonics` (`PhotonicsHelmholtzSolver`, …) | **No** | **No** | **None** |
| `solvers::statistical_mechanics` (bulk modulus / LJ-style helpers) | **No** | **No** | **None** |
| `solvers::fixed_point` | **No** | **No** | **None** |
| `solvers::lj_johnson_1993_reference` (`f64` reference EOS) | **No** | **No** | **None** |

### Policy / merge layer (outside `solvers/` but on the CBF path)

| Component | Populates `dissipation`? | CBF |
|-----------|--------------------------|-----|
| `IScienceCartridge` impls (cartridges, tests, `ManifoldGateway`’s `C`) | **Yes, if** the impl sets `PhysicalResult::dissipation` | `ManifoldGateway::evaluate_topology_step` → `ThermodynamicCBF::verify_tensor_update` |
| `TopologyPhysicsOrchestrator::run_plan_step` | **No** — returns `ThmcState` only | **No** direct CBF call |

## ThermodynamicCBF — actual touchpoints (honest)

1. **`ManifoldGateway::evaluate_topology_step`** (`src/ai/ppo.rs`)  
   - Reads `physical_result.dissipation`, forms `d_int = dissipation.sum_dim(1).squeeze(1)`, calls `cbf.verify_tensor_update(d_int, info_gain)`.  
   - **Reward** uses `free_energy`, `dissipation`, `cost`, optional `safety_margin` / `information_density`.

2. **`ThermodynamicCBF::verify_tensor_update`** (`src/ai/cbf.rs`)  
   - **Implementation (2026-05-12):** batch-sums `d_int`, maps it through **`k_phys_dint_to_joules`** (default `1.0`) into joule-equivalent entropy production, and adds it to the Landauer × `1.05` floor before `verify_and_deduct_update`. Negative batch sums are clamped to zero for the material branch. Cartridges still own **unit calibration** — adjust `k_phys_dint_to_joules` when `PhysicalResult::dissipation` is not already in joule-compatible units (see struct rustdoc).

3. **`ThermodynamicCBF::verify_and_deduct_update`** (`src/ai/cbf.rs`)  
   - Host-side Clausius–Duhem style check: `generalized_entropy = entropy_production_joules - landauer_erasure` must be ≥ 0, plus credit check. Callable independently of solvers.

4. **Tests:** `tests/golden_path_physics_cbf.rs` constructs `PhysicalResult` (including non-trivial `dissipation` proxies) **after** solver / functor steps, then builds `d_int = dissipation.sum_dim(1).squeeze(1)` (same reduction as `ManifoldGateway` in `src/ai/ppo.rs`) before `ThermodynamicCBF::verify_tensor_update` and merge — closest shipped **end-to-end** story; **not** inside `src/physics/solvers/`.

5. **`src/core/emergence.rs`** — `compute_dissipation_hotspots` / nodal dissipation shaping for monitoring; **not** wired to `ThermodynamicCBF`.

## Gaps (concise)

- **Solver directory:** zero references to `PhysicalResult` or `dissipation`; all “second law at the interface” lives in **traits + cartridges + gateway**, per `src/core/traits.rs` and `src/physics/orchestration.rs` docs.
- **THMC step vs cartridge:** `ThmcSolver::step_experimental` takes a cartridge but uses **`_cartridge`** — no constitutive functor run inside the coupled tick, so **no automatic bridge** from THMC internal physics to `PhysicalResult.dissipation`.
- **CBF vs `d_int`:** **closed at scalar gate** — `verify_tensor_update` now feeds batch-summed `d_int` into entropy production (with `k_phys_dint_to_joules`). **Credit** is still deducted from Landauer erasure only; tightening that coupling is out of scope for this audit slice.
- **Units / calibration:** `PhysicalResult` tensors remain generic `f32` channels; use **`ThermodynamicCBF::k_phys_dint_to_joules`** (default `1.0`) at the gateway / cartridge boundary when mapping to joule-equivalent entropy production for `verify_and_deduct_update`.

## Striatus coupled gates & B8 rollup (exec cross-check)

**Date:** 2026-05-12 — `umst-concrete-cartridge/scripts/verify_striatus_coupled_gates.sh`

| Phase | Result |
|-------|--------|
| `cargo test -p umst-concrete-cartridge` (default + `solver-experimental`) | **PASS** |
| `pytest notebooks/tests/test_print_ready.py` | **1 passed, 1 skipped** (`test_print_ready_track_b8_topology_gates` skipped when env unset) |
| Committed `notebooks/_artifacts/striatus_shell_v0.4.print_ready.json` | **`gates_track_b8_all_pass`: false** — `gate_topo_complexity_b7` **false**, `gate_density_xy_variance_b8` **false** (B8 topology rollup **not** green) |
| `scripts/check_solver_status.py` (step 4) | **PASS** after adding `statmech_mechanics_fracture_bridge.rs` to the cartridge `docs/Solver-Status.md` statistical-mechanics **Verification** cell (`--check-statmech-verification-set`) |

**`closeout-int-striatus`:** keep **open** until **`gates_track_b8_all_pass`** is **true** in the checked-in print-ready artefact (do not mark the integration done on a false rollup). Use **`UMST_REQUIRE_B8=1`** on the pytest step when Ring-1 enforcement is desired.

## References (in-repo)

- `src/core/traits.rs` — `PhysicalResult`, `IScienceCartridge`  
- `src/ai/ppo.rs` — `ManifoldGateway`, `d_int`, reward reductions  
- `src/ai/cbf.rs` — `ThermodynamicCBF`  
- `src/physics/orchestration.rs` — plan step → `ThmcSolver::step`; doc on `PhysicalResult` feeding via cartridge + merge  
- `tests/golden_path_physics_cbf.rs` — functor / THMC → `PhysicalResult` → CBF  
