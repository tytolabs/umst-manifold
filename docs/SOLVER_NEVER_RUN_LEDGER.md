# Solver never-run ledger (`#[ignore]` inventory)

**Date:** 2026-06-24 (Track C prep)  
**Status:** Read-only inventory — **do not execute** `#[ignore]` tests from this doc without Wave 2 / USER gate.  
**Manifest SSOT:** [`tests/verification/MANIFEST.toml`](../tests/verification/MANIFEST.toml)  
**Wave plan:** [`outputs/.plans/archive/waves/solver-quality-wave-plan.md`](../../outputs/.plans/archive/waves/solver-quality-wave-plan.md) Wave 2.

---

## Inventory (grep `#[ignore` under `umst-manifold` + `umst-concrete-cartridge`)

### umst-manifold — 9 `#[ignore]` test surfaces

| # | Crate / binary | Test function | Ignore reason (abridged) | MANIFEST id | Wave 2 status |
| --- | --- | --- | --- | --- | --- |
| 1 | `rheology_poiseuille` | `chorin_channel_65x17_longrun_wall_normal_l2_vs_regularized_reference` | slow ~2000 Chorin steps; `UMST_RUN_CHORIN_LONGRUN_L2=1` | `manifold.rheology.chorin_longrun` | **NEVER-RUN** |
| 2 | `mechanics_analytic` | `uniform_rho_q1_hex_compliance_vs_kirchhoff_striatus_40x40x4` | slow Striatus 40×40×4 compliance audit | `manifold.mechanics.striatus_compliance` | **pass@2026-06-15** (envelope opened once) |
| 3 | `mechanics_analytic` | `plate_r21_kirchhoff_ssss_centre_w_within_5pct_brick_path_gate` | §R2.1; `UMST_MECHANICS_R21_GATE=1`; assertion open | `manifold.mechanics.r21_kirchhoff_gate` | **NEVER-RUN** |
| 4 | `bar_network_operator_step_a` | `quick_plate_harness_load_pcg_converges` | H4 roof bar PCG stall ~0.94 rel_res | `manifold.mechanics.h4_roof_bar_pcg` | **fail@2026-06-15** |
| 5 | `q1_hex_pcg_bisect_probe` | `q1_hex_unit_sanity_striatus_n` | Striatus 40×40×4 unit sanity | `manifold.mechanics.q1_hex_bisect_striatus` | **pass@2026-06-15** |
| 6 | `lib` (unit in `electrochemistry.rs`) | `full_sg_chain_n256_band_lu_vs_dense_expand_wall_clock_and_residual_parity` | slow N=256 band Jacobian timing | `manifold.electrochemistry.sg_band_jacobian` | **pass@2026-06-15** |
| 7 | `adjoint_q1_hex_matches_bar_in_limit` | `adjoint_q1_hex_compliance_near_bar_z_skeleton_slender_limit` | Phase 1A skeleton bar vs 1×1 Q1 hex rel_err≈0.44 | — | **NEVER-RUN** |
| 8 | `thmc_post_newton_oracle_fixture` | `thmc_post_newton_tol_gated_exit_matches_oracle` | Wave S1 scaffold; oracle until diagnostic wired | — | **NEVER-RUN** (skeleton) |
| 9 | `solve_contract_entry_points` | `wave2_execute_never_run_ignored_envelopes` | Wave 2 meta-runner (does not auto-run children) | `manifold.solve_contract.wave2_skeleton` | **NEVER-RUN** |

### umst-concrete-cartridge — 8 `#[ignore]` test surfaces

| # | Binary | Test function | Ignore reason (abridged) | Notes |
| --- | --- | --- | --- | --- |
| 1 | `proof_status_doc` | `proof_status_refresh_markdown_on_disk` | writes `docs/PROOF-STATUS.md` | docgen only — not physics |
| 2 | `b6_c1_diagnosis` | `b6_c1_reference_triplet` | release, ~minutes per layout | offline diagnosis |
| 3 | `b6_c1_diagnosis` | `b6_c1_spatial_breakdown` | needs `UMST_SHELL_RHO_BIN` from 200-outer | H-c1-A audit |
| 4 | `shell_topology_rib_pattern` | `h5_striatus_density_net_compliance_grad_40x40x4` | slow H5 grad audit | discretization |
| 5 | `shell_topology_rib_pattern` | `h5_density_net_compliance_grad_40x40x4_striatus` | slow H5 grad | discretization |
| 6 | `shell_topology_rib_pattern` | `shell_topology_rib_pattern_full_v04` | slow B6; `UMST_SHELL_RIB_PATTERN=1`; 200-outer acceptance | **acceptance** when `RIB_FULL_ITERS=200` |
| 7 | `shell_topology_rib_pattern` | `shell_topology_rib_pattern_vf_guard_synthetic_pathology` | synthetic `should_panic` tripwire | guard test |
| 8 | `shell_topology_rib_pattern` | `shell_topology_rib_pattern_thesis_reconfig` | thesis re-config; `UMST_SHELL_THESIS_RECONFIG=1` | B6 Goal D — USER gate |

**MANIFEST cartridge rows** (`tests/verification/MANIFEST.toml`) list five historical cartridge envelopes (`cartridge.striatus.*`, `cartridge.shell.*`, …) — reconcile names against live `tests/` binaries when executing Wave 2.

---

## One-shot Wave 2 procedure (USER-gated — Track C does **not** run this)

Execute **once per envelope** in **`--release`**, record **pass/fail**, **wall time**, and **evidence line** in this file + [`Solver-Status.md`](Solver-Status.md) § open themes.

```bash
# 0. From umst-manifold/
cd umst-manifold
export PATH="$HOME/.cargo/bin:$PATH"

# 1. List ignores (read-only sanity)
rg '#\[ignore' --glob '*.rs' -n tests/ src/

# 2. Manifold envelopes (one command per row — examples)
UMST_RUN_CHORIN_LONGRUN_L2=1 cargo test --release -p umst-manifold \
  --features rheology-bingham,solver-experimental \
  --test rheology_poiseuille chorin_channel_65x17_longrun_wall_normal_l2_vs_regularized_reference \
  -- --ignored --exact --nocapture

UMST_MECHANICS_R21_GATE=1 cargo test --release -p umst-manifold \
  --features mechanics-voigt-cauchy --test mechanics_analytic \
  plate_r21_kirchhoff_ssss_centre_w_within_5pct_brick_path_gate -- --ignored --exact

cargo test --release -p umst-manifold --features mechanics-adjoint \
  --test bar_network_operator_step_a quick_plate_harness_load_pcg_converges -- --ignored --nocapture

# 3. Cartridge B6 acceptance (USER B6 sign-off — NOT Track C)
cd ../umst-concrete-cartridge
export UMST_SHELL_RIB_PATTERN=1 UMST_SHELL_RIB_FULL_ITERS=200
cargo test -p umst-concrete-cartridge --test shell_topology_rib_pattern \
  --features solver-experimental shell_topology_rib_pattern_full_v04 --release -- --ignored --nocapture

# 4. Append rows to this ledger:
#    | id | finished@UTC | pass/fail | wall_s | evidence |
```

**Forbidden in Track C prep:** `UMST_SHELL_RIB_FULL_ITERS=200` acceptance runs, `shell_topology_rib_pattern_thesis_reconfig`, and bulk `cargo test -- --ignored` without per-envelope authorization.

---

## Refresh inventory

```bash
cd umst-manifold
rg '#\[ignore' --glob '*.rs' -l tests/ src/ | sort
rg '#\[ignore' ../umst-concrete-cartridge --glob '*.rs' -n
python3 scripts/check_solver_status.py --check-paths  # lane table still valid
```
