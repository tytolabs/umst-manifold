# Solver Quality Audit — Cross-Group Synthesis

**Audit date:** 2026-06-12  
**Pin:** `fb24eda` (`umst-manifold` @ main)  
**Mode:** Read-only synthesis from five completed subagent audits (mechanics, THMC+gate, fracture/rheology/acoustics/stat-mech, electro/photonics/Helmholtz/DEC, claims ledger).  
**No code changes** were made in this synthesis.

**Subagent sources:** `64d614a8` (mechanics), `4c4f9da3` (THMC), `40224eaf` (fracture/rheology/acoustics/stat-mech), `ad7233f8` (electro/photonics/Helmholtz/DEC), `04c30962` (ledger).

**B6 checklist legend (A–G):**

| ID | Criterion |
|----|-----------|
| **A** | Operator verification (manufactured solution, symmetry/PSD, analytic benchmark) |
| **B** | Convergence honesty (independent residual vs self-report, tol derivation, iter cap basis) |
| **C** | Precision (f32 ill-conditioning, κ exposure) |
| **D** | Claims vs code (`Solver-Status`, `PROOF-STATUS`, feature docs) |
| **E** | Test gate strength (physical tolerances vs finiteness; `#[ignore]` never run?) |
| **F** | Constants without grounding |
| **G** | Schedule/boundary invariants (absorbing-step class) |

**Grade key:** ✓ solid · ◐ partial / scoped · ⚠ stopgap or weak gate · ○ out of scope · GAP documented weakness

---

## 1. Executive summary — cross-cutting B6 lessons

The B6 Striatus shell migration moved the **cartridge plate path** to Q1-hex with strong PCG true-residual probes, but **bar-network quasi-static mechanics remains load-bearing** on coupled production paths: THMC (`thmc.rs`), fracture stagger (`fracture_field.rs`), monolithic Newton \(R_u\) (`thmc_residual.rs`), adjoint TO (`adjoint.rs`), protocols, and AI topology. This is the dominant **operator-split fidelity gap** (B6 H4/H5 analogy): shell uses one discretization; coupled physics uses another.

**Convergence honesty (class B)** splits sharply by lane. Q1-hex PCG exposes binding true residual \(\|P(f-Ku)\|/\|Pf\|\) with periodic refresh — aligned with hypre/Shewchuk practice and B6 acceptance culture. Bar-network f32 PCG stops on **recursive** \(\|r\|/\|b\|\) only, with no true \(f-Ku\) refresh; the 9×8×2 roof harness stalls near 0.94 rel (ignored test). THMC Newton exits re-call `assemble` on the trial state — fresh evaluation, **not** an independent oracle; brute-force oracles exist only in unit tests. Post-monolithic diagnostics in `step` use **explicit-Euler** Laplacian mismatch, not the implicit BE stacked residual being solved.

**Claims ledger hygiene** is uneven. `Solver-Status.md` is mostly honest (partial Γ, 50% photonics, 1-D acoustics scope). `PROOF-STATUS.md` is **stale** (missing monolith/photonics/q1_hex tests; false `#[ignore]` on electrochemistry λ_D gates). **Compiled ≠ validated:** `solver-research-check-pr` runs `cargo check` only. Cartridge `shell_topology_rib_pattern_quick` is documented as CI but requires `solver-experimental` — **not** default cartridge `cargo test`. **20-outer / 60-outer PASS ≠ 200-outer B6 acceptance**; 200-outer logit-offset run is MISTRIAL; volume bisect mechanism is validated, full B6 gate is not.

**Lane-specific highlights:**

- **THMC (75% label):** `ThmcSolver.tol` is **dead** on the live coupled path; JFNK is experimental-only; production monolith uses dense inner ≤64 DOFs.
- **Acoustics (100% label):** Applies to **1-D periodic bar** Newmark only; `AcousticWaveSolver` + bar GMRES is research-grade f32.
- **Fracture / rheology (50% each):** Honest scaffolds; no Herschel–Bulkley; Γ and developed Poiseuille L² are partial or `#[ignore]`.
- **Stat-mech Johnson bridge:** `upscale_potentials [B,4]` is a **virial surrogate**, not Johnson in Burn; f64 reference is strong.
- **Electro / photonics:** PNP λ_D gates use **linearized SG** (Debye–Hückel limit); FDFD chain Thomas is all-f32 with no shipped residual API.
- **Helmholtz:** `apply_straight_through` is **identity-gradient STE**, not a PDE adjoint — documented B6 H2 stopgap; forward Richardson only.

**Local test execution:** Subagents on audit host hit `rustc 1.86` vs `time@0.3.47` requiring `1.88`; CI uses pinned toolchain. THMC agent ran three targeted `--release` tests; others inferred from code + doc inventory.

---

## 2. Per-solver scorecard (B6 A–G)

### 2.1 Mechanics (`VectorMechanicsSolver`, bar network, Q1 hex, adjoint)

| Module / path | A | B | C | D | E | F | G | `Solver-Status` |
|---------------|---|---|---|---|---|---|---|-------------------|
| **Bar network** (`mechanics.rs`) | ✓ | ⚠ | ⚠ | ✓ | ◐ | ✓ | ○ | 25% (shared row) |
| **ExtrudedPlateMechanics** | GAP | GAP | GAP | ✓ | GAP | ○ | GAP | — |
| **Q1 hex** (`q1_hex_elasticity.rs`) | ✓ | ✓ | ◐ | ✓ | ✓ (research) | ✓ | GAP | — |
| **AdjointCompliance (bar)** | ✓ | ✓ | ◐ | ✓ | ✓ | ✓ | ○ | — |
| **AdjointComplianceQ1Hex** | ✓ | GAP | ◐ | ✓ | GAP | ✓ | ○ | — |
| **Gate paths (THMC/fracture/protocols)** | GAP | ○ | ⚠ | ✓ | GAP | ○ | GAP | inherits bar |

**Notes:** Bar operator probes (symmetry, PSD, MMS) pass; roof equilibrium ignored. Q1 probes (`q1_hex_pcg_residual_probe`) not in `solver-stable-pr`. Kirchhoff story split: 5.5% lateral-\(u_z\) test green under stable-CI vs `#[ignore]` brick-path O(1) error.

### 2.2 THMC (`ThmcSolver`, residuals, JFNK)

| Path | A | B | C | D | E | F | G |
|------|---|---|---|---|---|---|---|
| **P1 Operator-split (default)** | ◐ | ✗ | ◐ | ◐ | ◐ | ✗ | ◐ |
| **P2 Implicit \((T,\alpha)\) Newton** | ✓ | ◐ | ◐ | ◐ | ✓ | ✗ | ◐ |
| **P3 Monolithic \((T,h,\alpha,u)\)** | ◐ | ◐ | ◐ | ✗ | ✓ | ✗ | ◐ |
| **P4/P5 Dense residual helpers** | ✓ | ◐ | ◐ | ✓ | ✓ | ✗ | n/a |
| **P6 JFNK single-step (experimental)** | ◐ | ◐ | ◐ | ◐ | ✗ | n/a | n/a |
| **P7 Chained monolithic** | ◐ | ◐ | ◐ | ✓ | ✓ | n/a | n/a |
| **P9 Mechanics PCG inner (bar)** | ◐ | ◐ | ◐ | ◐ | ◐ | n/a | ◐ |

**Aggregate:** A C+ · B D+ · C C · D C− · E B− · F D · G B · **~50–60% on verification axis** vs **75%** completion label.

### 2.3 Fracture AT2 (`fracture_field.rs`)

| A | B | C | D | E | F | G | Grade |
|---|---|---|---|---|---|---|-------|
| ✓ | ◐ | ◐ | ✓ | ⚠ | ✓ | ○ | **C+** |

Partial Γ harnesses (\(<2\%\) \(D_h\) vs \(G_c\) at \(h/l_0=¼\)); `g(d)` documented but not applied in `update_damage`; spectral ψ⁺ via cyclic Jacobi.

### 2.4 Rheology Bingham (`rheology_flow.rs`, `rheology_analytic.rs`)

| A | B | C | D | E | F | G | Grade |
|---|---|---|---|---|---|---|-------|
| ✓ | ⚠ | ◐ | ✓ | ⚠ | ✓ | ○ | **C** |

**No Herschel–Bulkley.** Analytic Buckingham layer solid; Chorin graph solver lacks MAC + open x BCs; steady L² only in `#[ignore]` long-run.

### 2.5 Acoustics Newmark (`acoustics.rs`)

| A | B | C | D | E | F | G | Grade |
|---|---|---|---|---|---|---|-------|
| ✓ | ✓ | ✓ | ✓ | ◐ | ✓ | ○ | **A−** (1-D bar only) |

Return map \(<2\%\) L², undamped energy drift \(<0.5\%\) @ n=128. **100% row = 1-D bar**, not `AcousticWaveSolver` 3-D f32 GMRES.

### 2.6 Statistical mechanics (Vinet, Johnson, LJ bridge)

| Lane | A | B | C | D | E | F | G | Grade |
|------|---|---|---|---|---|---|---|-------|
| **Vinet EOS** | ✓ | n/a | n/a | ✓ | ✓ | ◐ | n/a | **B−** |
| **Johnson f64 reference** | ✓ | n/a | ◐ | ✓ | ◐ | ✓ | ◐ | **C+** |
| **LJ → Burn bridge** | ✓ | n/a | ◐ | ✓ | ◐ | ✓ | ◐ | virial surrogate |

`upscale_potentials [B,4]` third-order virial with Padé \(B_3^*\); `[B,2]` is \(K \propto \varepsilon/\sigma^3\) placeholder. γ_gc is KB-style scalar proxy, not Widom integral.

### 2.7 Electrochemistry PNP (`electrochemistry.rs`)

| A | B | C | D | E | F | G | Grade |
|---|---|---|---|---|---|---|-------|
| ✓ | ◐ | ✓ | ✓ | ◐ | ✓ | ○ | **B−** |

SG + Thomas chain solid; λ_D gates at N=256 use **`linearize_sg_fickian: true`**; f32 state export after f64 Newton; Picard default all-f32.

### 2.8 Photonics FDFD (`photonics.rs`)

| A | B | C | D | E | F | G | Grade |
|---|---|---|---|---|---|---|-------|
| ✓ | ◐ | ◐ | ✓ | ◐ | ✓ | ○ | **C+** |

1-D TE + small DEC patches (N≤512); hand-rolled f32 complex Thomas; no production residual on chain path; gradients through solve explicitly not a goal.

### 2.9 Topology filter / Helmholtz (`topology_filter.rs`)

| A | B | C | D | E | F | G | Grade |
|---|---|---|---|---|---|---|-------|
| ✓ | ◐ | ◐ | ✓ | ⚠ | ◐ | ⚠ | **C** |

Forward: Richardson on \((I-sL)\tilde\rho=\rho\). **`apply_straight_through` = STE** (`ρ_st + (filtered−ρ).detach()`), not implicit adjoint. `max_cg_iterations` is Richardson budget, not CG.

### 2.10 DEC (`dec_primal`, `dec_operators`)

| A | B | C | D | E | F | G | Grade |
|---|---|---|---|---|---|---|-------|
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | n/a | **B** |

`dec_identities.rs` (13 tests): symmetry, adjoint, \(d_1\circ d_0\approx 0\). Maxwell assembly lives in photonics patch only.

### 2.11 Gate (`src/gate/**`)

| A | B | C | D | E | F | G | Grade |
|---|---|---|---|---|---|---|-------|
| n/a | n/a | n/a | ✓ | ✓ | ◐ | n/a | **orthogonal** |

Clausius–Duhem / Kleisli / CBF admissibility — **no PDE solve**, no THMC residual validation. B6 “residual trust” analogy is verdict replay, not JFNK.

---

## 3. Ranked findings (merged, deduplicated)

| Rank | Sev | Finding | Lanes | B6-class analogy |
|------|-----|---------|-------|------------------|
| **1** | P0 | **Bar-network remains load-bearing** on THMC, fracture, monolithic \(R_u\), adjoint TO, protocols, AI topology — while Striatus shell uses Q1 hex | Mechanics, THMC, fracture | **H4/H5** — wrong physics operator on production coupling paths |
| **2** | P0 | **9×8×2 roof bar f64 PCG stalls** (~0.94 rel); operator A-gates pass, equilibrium gate `#[ignore]` | Mechanics | **H4 Step A pass, Step B fail** |
| **3** | P0 | **`ThmcSolver.tol` unused** on live coupled path; operator-split has **no residual exit** | THMC | Optimizer stopping without independent KKT/residual gate |
| **4** | P0 | **Post-Newton diagnostics** in `step` use explicit-Euler Laplacian, **not** implicit BE stacked \(R\) | THMC | Reporting audit metric on wrong functional (Voigt p=3 vs p=1 class) |
| **5** | P1 | **Q1 hex true residual strong; bar f32 recursive-only weak** — no periodic \(f-Ku\) refresh | Mechanics | **B** — convergence honesty / Signal 1 risk |
| **6** | P1 | **Kirchhoff claims internally split** — 5.5% lateral-\(u_z\) green vs `#[ignore]` brick O(1); docs under-specify | Mechanics | **D** — claims vs code |
| **7** | P1 | **Compiled ≠ validated** — `solver-research-check-pr` is `cargo check` only; cartridge quick path not default CI | Ledger | B6 “green” without physics test |
| **8** | P1 | **20-outer PASS ≠ 200-outer B6** — greyness/c1 skipped when `UMST_SHELL_RIB_FULL_ITERS < 200`; 200-outer MISTRIAL | Ledger, Helmholtz | Smoke mistaken for acceptance |
| **9** | P1 | **PROOF-STATUS stale** vs `Solver-Status` (THMC monolith, photonics, q1_hex; false electrochemistry `#[ignore]`) | THMC, ledger | Ledger / harness disagreement |
| **10** | P1 | **Newton/JFNK exit = same `assemble` path**; brute-force oracle **tests only** | THMC | Self-reported compliance without second evaluator |
| **11** | P1 | **Helmholtz STE not true adjoint** — identity on backward pass; masks filter sensitivities in B6 TO | Helmholtz | B6 H2 stopgap vs PDE adjoint claim |
| **12** | P1 | **Photonics f32 Thomas** — silent near-singular `C::div`; no f64 chain path; no shipped residual API | Photonics | **C** — f32 ill-conditioning |
| **13** | P1 | **PNP λ_D gates use linearized SG** — validates Debye–Hückel limit, not full Gouy–Chapman SG | Electrochemistry | Claim overreach on screening verification |
| **14** | P1 | **Acoustics 100% = 1-D bar only** — `AcousticWaveSolver` + bar GMRES uncertified | Acoustics | Completion % ≠ PR-green / scope |
| **15** | P1 | **No Herschel–Bulkley** — Bingham + Roussel λ only | Rheology | Scope name mismatch |
| **16** | P1 | **Chorin graph ≠ developed Poiseuille** — no MAC, no open x BCs; L² in `#[ignore]` | Rheology | Partial operator on stated benchmark |
| **17** | P1 | **Johnson bridge = virial surrogate** — bridge test 0.2–5× ratio at dilute ρ, not Johnson \(K\) in Burn | Stat-mech | `[B,4]` misread as Johnson validation |
| **18** | P1 | **Γ-convergence partial** — toy 1-D chains; not sharp-interface limit with coupled ψ⁺ mechanics | Fracture | 50% honest if “partial” stressed |
| **19** | P2 | **`ExtrudedPlateMechanics` drops PCG telemetry** — callers cannot assert tol | Mechanics | **B** |
| **20** | P2 | **Default CI skips most mechanics** — q1_hex/adjoint probes need feature gates | Mechanics | **E** — test gate strength |
| **21** | P2 | **Six manifold + five cartridge `#[ignore]` tests** plausibly never executed (roof PCG, Chorin L², H5 grad, full B6) | All | **E** |
| **22** | P2 | **`g(d)` stiffness degradation not applied** in `update_damage` | Fracture | Doc vs implementation |
| **23** | P2 | **JFNK only on `one_damped_newton_step_with_quasi_static_r_u`**; `step` chain uses dense inner | THMC | Experimental gate not on production path |
| **24** | P2 | **Monolith default `stacked_residual_l2_tolerance: 0`** → fixed iterations regardless of \(\|R\|\) | THMC | Fixed outer count |
| **25** | P2 | **64-DOF dense cap** — no sparse/JFNK at production \(N\) | THMC | Scale cliff (bar vs shell) |
| **26** | P3 | **`BarMatvecOperator` unused** — acoustics calls `bar_matvec` directly | Mechanics | Architecture drift |
| **27** | P3 | **`VERIFICATION_SCOPE_INDEX.md` missing**; cartridge `check_solver_status.py` not wired in CI | Ledger | Broken doc links |
| **+** | — | **Positives:** Q1 PCG probes; bar adjoint 1% FD; Newmark bar energy/return-map; Johnson f64 reference; DEC identities; SG mass drift test; Solver-Status honesty on partial lanes | — | — |

---

## 4. Recommended verification fixtures (S / M / L)

Grouped by gap; existing tests cited where subagents named them.

### 4.1 Mechanics / bar vs Q1 split

| Size | Fixture | Purpose |
|------|---------|---------|
| **S** | 2-node + 4-node bar chains (`adjoint_compliance_analytic`) | Analytic tip / compliance / adjoint FD |
| **S** | `9×8×2` + `q1_hex_pcg_residual_probe` | True vs recursive residual, κ·ε floor |
| **S** | `bar_network_operator_step_a` symmetry/PSD/MMS (skip ignored PCG) | Operator A-gate without roof stall |
| **M** | `8×8×4` ratio-band + masked residual `<1e-3` | Stable-CI regression |
| **M** | `38×38×4` lateral-\(u_z\) Kirchhoff 5.5% vs `plate_r21_*` `#[ignore]` | Clarify R2.1-A vs R2.1-B status |
| **M** | `adjoint_q1_hex_compliance_analytic` 8×8×2 | TO sensitivity 1% FD |
| **L** | `40×40×4` Striatus + PCG descent/residual probes | Production PCG honesty |
| **L** | `bar_network_roof_mechanism_probe` + ignored `quick_plate_harness_load_pcg_converges` | Bar PCG stall root-cause |

### 4.2 THMC / Newton residual trust

| Size | Fixture | Purpose |
|------|---------|---------|
| **S** | Post-Newton: \(\|R_{\mathrm{assemble}}-R_{\mathrm{brute}}\|_\infty<\epsilon\) on 2-node chain | Independent oracle at exit (test feature flag) |
| **S** | JFNK vs dense `delta_red` on 5-node chain | GMRES path parity |
| **M** | Manufactured 1D coupled \((T,h,\alpha)\) with known BE solution | Operator + monolith A-class |
| **M** | `step` + monolith + `tol_exit>0`: exit uses stacked \(R\), not explicit diagnostic | B-class regression |
| **L** | Grid refinement \(N=8,16,32\) monotone \(\|R\|\) | Production-scale credibility |
| **L** | THMC step → gate receipt JSONL with post-step \(\|R\|\) | Auditable verdict trail |

### 4.3 Fracture / rheology / acoustics / stat-mech

| Size | Fixture | Purpose |
|------|---------|---------|
| **S** | `update_damage_smoke_tiny_chain` | AT2 finite \(d\in[0,1]\) |
| **S** | `analytic_newtonian_centreline_is_gh2_over_8mu` | Buckingham limit |
| **S** | `plane_wave_return_map_n64_l2_within_two_percent` | 1-D Newmark |
| **S** | `vinet_pressure_vanishes_at_reference_volume` | Vinet stable lane |
| **M** | `at2_gamma_convergence_three_length_scales` | \(D_h\) vs \(G_c\) |
| **M** | `chorin_channel_65x17_thirty_substeps_remain_finite` | Graph split stability |
| **M** | `statmech_virial_pressure_autodiff_matches_fd_wrt_rho_star` | ∂P/∂ρ* |
| **M** | `johnson_lj1993_eos_compressibility_matches_pressure_over_rho_t_supercritical_grid` | f64 EOS algebra |
| **L** | `staggered_fracture_mechanics_chain.rs` | u↔d wiring |
| **L** | `chorin_channel_65x17_longrun_wall_normal_l2_vs_regularized_reference` `#[ignore]` | Steady Poiseuille profile |
| **L** | `at2_gamma_convergence_psi_plus_outer_strain_ramp_smoke` | Multi-ρ tensile Γ schedule |

### 4.4 Electrochemistry / photonics / Helmholtz / DEC

| Size | Fixture | Purpose |
|------|---------|---------|
| **S** | `sg_zero_field_matches_explicit_fickian_graph_laplacian` | SG zero-field limit |
| **S** | `poisson_chain_uniform_rho_matches_h_squared_rhs_scaling` | Poisson h² RHS |
| **S** | `dec_identities` (13 tests) | DEC annihilation / adjoint |
| **S** | `helmholtz_delta_blob_fwhm_matches_green_scale` | Forward filter coupling |
| **M** | `pnp_debye_layer` λ_D gates **with `linearize_sg_fickian: false`** (when implemented) | Full SG screening claim |
| **M** | `solve_maxwell_dec_patch_quad_split_pin_residual_tight` | FDFD patch residual |
| **M** | `helmholtz_striatus_autodiff` 40×40×4 | STE smoke |
| **L** | FDFD 1-D Thomas + shipped \(\|b-Ax\|/\|b\|\) probe | f32 Thomas risk gate |
| **L** | Helmholtz **implicit adjoint** vs FD on ρ (replace STE) | True TO filter gradient |
| **L** | Cartridge `shell_topology_rib_pattern_full_v04` 200-outer post-MISTRIAL fix | B6 acceptance re-run |

---

## 5. Claims reclassification (`Solver-Status` / `PROOF-STATUS` audit)

**Classification:** `PROVEN (Lean)` | `VALIDATED` (physics/analytic test on automated CI) | `SMOKE` (finite/no-panic/bracket) | `STATED` (docs/manual only)

**CI path legend (manifold):** `default` = `cargo test` (no solver features); `stable-pr` = `solver-stable`; `check-pr` = `cargo check solver-research` only; `phase4-pr` = release subset; `research-main` = `cargo test --release --features solver-experimental` on `main`.

### 5.1 Manifold main table (`docs/Solver-Status.md`)

| Solver row | Doc completion | Doc claim (verbatim) | Honest class | CI path that earns it | Downgrade / overclaim |
|---|---|---|---|---|---|
| `solvers::topology_solver` | 25% | stable lane; `topology_continuation.rs`, `topology_filter.rs` | **VALIDATED** (heat/SIMP evolution smoke) | `default`, `stable-pr` | Shell B6/B8 explicitly **deferred to cartridge** — not 25% of Striatus |
| `mechanics::VectorMechanicsSolver`, `AdjointCompliance`, `AdjointComplianceQ1Hex` | 25% | "Shipped quasi-static bar / plate / Q1-hex paths + discrete adjoint checks" | **VALIDATED** (bar/adjoint); **SMOKE** (Q1 plate); **STATED** (Kirchhoff ≤5%) | Bar/adjoint: `default`; Q1 hex: `research-main`; Kirchhoff: `#[ignore]` only | Kirchhoff gate **open** — doc already says O(1) error; do not cite thin-plate accuracy |
| `solvers::fracture_field` | 50% | "AT2 relaxation, length-scale and **partial** Γ-type harnesses" | **VALIDATED** (partial Γ, staggered smokes); **STATED** (sharp-interface Γ-limit) | Partial: `default` + `research-main` (feature-gated chains) | "50%" is honest if "partial" is stressed; not production THMC stagger |
| `solvers::acoustics` | **100%** | "Newmark vs dense reference; return-map checks" | **VALIDATED** (1-D periodic bar) | **`research-main` only** — not `default` or PR test | **100% ≠ PR-green**; optional 3-D graph extension still **open** |
| `solvers::electrochemistry` | 75% | "λ_D screening-length LS gates on **256**-cell chains" | **VALIDATED** (Picard + Debye LS on N=256 when feature on) | `research-main` (`pnp_debye_layer`) | PROOF-STATUS still says "λ_D gates `#[ignore]`" — **false** (stale index) |
| `solvers::photonics` | 50% | "small embedded **DEC** patch tests" | **VALIDATED** (patches); **SMOKE** (stub default build) | 2D/3D: `phase4-pr`; fresnel: `research-main` | "Production DEC + Krylov" remains **STATED** |
| `solvers::rheology_flow` | 50% | "short-channel smokes … **not** long-run steady L²" | **SMOKE** (Chorin bracket); **STATED** (developed L²) | Smokes: `research-main`; longrun: `#[ignore]` | Doc is honest on L²; do not claim Poiseuille steady-state fidelity |
| `solvers::thmc` | 75% | "implicit (T, α) Newton … stacked (T, h, α, u) dense damped Newton on **≤ 64** DOFs" | **VALIDATED** (drying/shrinkage + tiny monolith) | Drying: `research-main`; monolith: `phase4-pr` | Large-N monolith / stagger **STATED** |
| `solvers::statistical_mechanics` | 25% | "Johnson upscale bridge tests; **γ_gc** and virial-backed bridges remain **open**" | **VALIDATED** (Vinet EOS); **SMOKE** (LJ placeholder bridge); **STATED** (physical γ_gc) | Vinet/LJ: `default`; Johnson upscale: `phase4-pr` | Placeholder `upscale_potentials` must not be read as Johnson **K** validation |

### 5.2 Open themes (manifold `Solver-Status.md` §Open themes)

| # | Theme | Doc % | Honest class | Notes |
|---|---|---|---|---|
| 1 | Topology / shell / Striatus (B6/B8) | 25% | **SMOKE** (manifold topology tests); **STATED** (B6 acceptance) | Cartridge owns B6; 200-outer **not closed** |
| 2 | Mechanics Kirchhoff | 25% | **STATED** | `plate_r21_…` `#[ignore]`, fails by design |
| 3 | Fracture Γ / THMC stagger | 50% | **VALIDATED** partial / **STATED** full | Matches fracture row |
| 4 | Acoustics extension | 100% | **STATED** (beyond 1-D bar) | Theme 4 "optional" = not validated |
| 5–10 | Electrochem scale, photonics prod, rheology channel, THMC large-N, statmech virial, vector dynamics | 25–75% | Mix per parent row | No upgrade beyond §5.1 |

### 5.3 Cartridge Striatus / B6 (mirror `docs/Solver-Status.md` — not in manifold main table)

| Entry | Doc claim (verbatim) | Honest class | Downgrade |
|---|---|---|---|
| `shell_topology_rib_pattern_quick` | "CI — compact … Gates: VF ±15% …" | **STATED** as default CI | Requires `solver-experimental`; **not** in cartridge default `cargo test` |
| `shell_topology_rib_pattern_full_v04` | B6: 40×40×4, 200 outers, seed 42 | **STATED** (acceptance open) | `#[ignore]`; 200-outer **MISTRIAL**; Step E **re-run pending** |
| B6 attempt log 2026-06-12 20-outer | "**PASS — first all-green B6 run**" … "**Volume arc closed.**" | **SMOKE** (20-outer schedule-regime) | **Not** 200-outer B6; greyness/c1 on shortened horizon only |
| Milestone 2026-06-12 | "B6 volume arc **closed**" … "First all-green 20-outer" | **VALIDATED** (volume bisect mechanism); **STATED** (B6 gate) | Volume path earned; **B6 acceptance not earned** |
| 200-outer logit-offset | "**MISTRIAL†**" — sym boundary measured wrong state | **SMOKE** (run healthy, gates wrong state) | Harness fix landed; honest re-run **not yet recorded** |
| `b6-c0-uniform-at-target-vf` | "**closed** (2026-06-12, p=1 fix)" | **STATED** (gate definition) | `c1 < 0.6·c0` on 200-outer export **unknown** |
| `gates_track_b8_all_pass` | rollup in `print_ready.json` | **STATED** unless exporter run on passing `final.npy` | Skips in `test_print_ready` unless `UMST_REQUIRE_B8=1` |

### 5.4 PROOF-STATUS index drift (manifold `docs/PROOF-STATUS.md` vs `Solver-Status.md`)

| PROOF-STATUS row | Issue | Honest fix |
|---|---|---|
| `electrochemistry` | "λ_D … gates `#[ignore]`" | **Stale** — gates run in `pnp_debye_layer.rs` on `research-main` |
| `mechanics` | Omits all `adjoint_q1_hex_*` paths | Index incomplete — Solver-Status is authoritative |
| `thmc` | Omits `thmc_monolithic_newton_chain.rs` | Index incomplete |
| `photonics` | Omits `photonics_curl_curl_{2d,3d}_*.rs` | Index incomplete |
| `statmech` | Omits `statmech_lj_johnson_upscale_bridge.rs`, `statmech_mechanics_fracture_bridge.rs` | Index incomplete |

### 5.5 Verbatim overclaims → honest downgrade (quote ledger)

| Verbatim quote | Source | Downgrade |
|---|---|---|
| "Completion (%) … **100** means the public acceptance story for that lane is met end-to-end on the stated CI path" | manifold Solver-Status §Completion | Acoustics **100%** is **research-main-only**; PR + default builds do not run it |
| "**PASS — first all-green B6 run**" | cartridge Solver-Status B6 log 2026-06-12 | **20-outer smoke PASS**, not B6 (200 outer + full gates) |
| "B6 volume arc **closed**" | cartridge Solver-Status Milestone | Volume **mechanism closed**; B6 **acceptance not closed** (200-outer MISTRIAL; c1 gate open) |
| "λ_D exponential-fit gates `#[ignore]`" | manifold PROOF-STATUS | **False** — remove `#[ignore]` from index |
| "`- [`shell_topology_rib_pattern_quick`]: CI`" | cartridge test module doc | **Misleading** — not in default-feature CI; needs `solver-experimental` job |
| "Prose rule: say **verified on CI** only for behaviours exercised by the **Verification** paths" | manifold Solver-Status | **`solver-research-check-pr` is compile-only** — "verified on CI" must exclude PR check-only path |

### 5.6 `#[ignore]` inventory (plausibly never executed)

**Manifold (6):** `quick_plate_harness_load_pcg_converges`; `uniform_rho_q1_hex_compliance_vs_kirchhoff_striatus_40x40x4`; `plate_r21_kirchhoff_ssss_centre_w_within_5pct_brick_path_gate`; `q1_hex_unit_sanity_striatus_n`; `chorin_channel_65x17_longrun_wall_normal_l2_vs_regularized_reference`; `full_sg_chain_n256_band_lu_vs_dense_expand_wall_clock_and_residual_parity`.

**Cartridge (5):** `h5_striatus_density_net_compliance_grad_40x40x4`; `h5_density_net_compliance_grad_40x40x4_striatus`; `shell_topology_rib_pattern_full_v04`; `shell_topology_rib_pattern_vf_guard_synthetic_pathology`; `proof_status_refresh_markdown_on_disk` (regenerator, N/A physics).

---

## 6. NO FIXES

This document is a **read-only audit synthesis**. No source changes, commits, test runs (beyond subagent spot-checks), or documentation edits outside this file were performed.

**Explicit non-actions:**

- No bar-network → Q1-hex migration on THMC/fracture/adjoint paths.
- No `ThmcSolver.tol` wiring, post-Newton residual class correction, or PROOF-STATUS sync.
- No Helmholtz implicit adjoint or STE removal.
- No FDFD f64 path or Thomas residual harness.
- No PNP full-SG λ_D gate rerun configuration.
- No CI workflow changes (`solver-experimental` default, `check_solver_status.py` on cartridge).
- No B6 200-outer re-run or MISTRIAL closure.

Switch to Agent mode with an explicit fix scope if any of the above is desired.

---

## Appendix — subagent pin integrity

| Agent | Scope | Local tests |
|-------|-------|-------------|
| `64d614a8` | Mechanics, adjoint, Q1 hex, bar callers | Blocked (rustc pin) |
| `4c4f9da3` | THMC, JFNK, gate inventory | 3× `--release` passed |
| `40224eaf` | Fracture, rheology, acoustics, stat-mech | Not executed (read-only) |
| `ad7233f8` | PNP, FDFD, Helmholtz, DEC | Not executed (read-only) |
| `04c30962` | Ledger, `check_solver_status.py`, `#[ignore]` | Script pass (manifold); cartridge mirror fail (expected) |

`check_solver_status.py`: manifold passes (9 rows); cartridge mirror fails (no Solver table). `VERIFICATION_SCOPE_INDEX.md` referenced from cartridge but absent in manifold `docs/`.
