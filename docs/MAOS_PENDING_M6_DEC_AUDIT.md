# Matrix **#6** / **m6-dec** — DEC Maxwell audit (`solve_maxwell_curl_curl` + `PhotonicsDecFacesPatch`)

**Purpose:** Snap **shipped vs still open** for verification matrix [**#6**](VERIFICATION_COMPLETION_MATRIX.md) (photonics lane **50%**) against code and tests. **Authority:** [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) row **#6** *Exact acceptance criterion* and [**Solver lanes — Photonics**](Solver-Status.md#solver-lane-photonics) — if other audits disagree, deferrals in **`Solver-Status.md`** / this matrix row win ([`Solver-Status.md`](Solver-Status.md) footnote on **m6-dec** vs [`GAP_AUDIT.md`](GAP_AUDIT.md)).

## API surface (`photonics.rs`)

| Item | Location | Role |
| --- | --- | --- |
| **`PhotonicsDecFacesPatch`** | [`src/physics/solvers/photonics.rs`](../src/physics/solvers/photonics.rs) (`struct`, ~L523) | Optional **`faces_b2`** COO + **`face_column_ranges`** passed into **`solve_maxwell_curl_curl`** to enable the **small-patch** path. |
| **`PhotonicsSolver::solve_maxwell_curl_curl`** | Same file (~L566) | Entry point: uniform **x-chain** TE reduction **or**, when **`dec_patch: Some(_)`** validates, **`solve_maxwell_dec_patch_direct`**. |
| **`solve_maxwell_dec_patch_direct`** | Same file (`#[cfg(feature = "photonics")]`) | Host **dense** Gauss–Jordan on **3N** DoFs (lossless) or **stacked real** \(2\cdot 3N\) when **`max|eps_r_imag|>1e-6`**; **`PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT = 64`**; lossy + **`N>64`** refuses (no CG for that split). |
| **`dec_maxwell_assembly`** | Same module (~L1106) | Re-export of **`primal_d1_*`** from **`dec_primal`** for tests / integration — **not** the full production Maxwell stack. |

Rustdoc on **`solve_maxwell_curl_curl`** documents: patch assembly uses a **diagonal primal-edge \(\star_1\)** curl leg + nodal \(k_0^2 \varepsilon\) with **3×3** tensor support, **gauge pin** at node **0** matching **`e_field`** (real \(\mathbf{E}\) returned on tensor API; stacked solve recovers \(\Im\mathbf{E}\) internally for lossy **`eps_r_imag`**).

## `photonics_fresnel` tests: **`solve_maxwell_dec_patch_*`**

Five named patch solves live in [`tests/verification/photonics_fresnel.rs`](../tests/verification/photonics_fresnel.rs) (module rustdoc):

1. **`solve_maxwell_dec_patch_quad_split_pin_residual_tight`**
2. **`solve_maxwell_dec_patch_quad_split_tensor_eps_residual`**
3. **`solve_maxwell_dec_patch_quad_split_embedded_r3_residual`**
4. **`solve_maxwell_dec_patch_two_quads_strip_residual`**
5. **`solve_maxwell_dec_patch_quad_split_scalar_eps_imag_stacked_residual`** — nodal scalar **`eps_r_imag`**, stacked \(2\cdot 3N\) dense + residual check.

They exercise **`PhotonicsDecFacesPatch`** → **`solve_maxwell_curl_curl`** on **hand-built** quad-split / two-quad strips embedded in **ℝ³**, including nodal **`[1,N,9]`** permittivity where named.

## Comparison to [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) row **#6**

| Matrix column | Alignment |
| --- | --- |
| **Goal** | Production **2D/3D** DEC vector curl–curl; tensor **ε**; tighter Fresnel — **not** fully met (patch is **2-surface** milestones only; Fresnel margin still “loose” per lane). |
| **Shipped hooks** | Matches: **`photonics_fresnel`** (chain + Fresnel + DEC patch tests), **`photonics_curl_curl_stub_default_build`**, **`dec_identities`** annihilation/adjoint — cited correctly in matrix row. |
| **Exact acceptance criterion** | **Open:** (1) Fresnel LS tighten or discrete **`r_disc`** check; (2) **circumdual / assembled-mesh** **Hodge** and/or **cube** from **assembled** topology into **`solve_maxwell_curl_curl`** (not only test COO); (3) **sparse** Krylov at large **N** (including lossy), **tensor imaginary \(\varepsilon\)** + **PML** on patch path, **BCs** beyond gauge **pin**. |

## Comparison to [**Solver-Status — Photonics**](Solver-Status.md#solver-lane-photonics)

The photonics table row and expanded lane repeat the same story: **partial** closure on **1-D TE chain**, **small-N dense** **`PhotonicsDecFacesPatch`** vector DEC (diagonal primal **\(\star_1\)** curl leg, **scalar nodal `eps_r_imag`** in \(k_0^2\) mass for **`N≤64`** via stacked real), plus **`dec_identities`** as **DEC topology** evidence **not** equivalent to closing matrix **#6**.

## Shipped vs still open (requested buckets)

### Shipped (evidence in-tree)

- **Uniform x-chain TE:** **`solve_maxwell_curl_curl`** ≡ scalar Helmholtz path for \(E_y\); piecewise scalar and tensor **yy** chain regressions (`curl_curl_y_mode_matches_scalar_helmholtz*`).
- **Small-patch vector DEC:** five **`solve_maxwell_dec_patch_*`** tests + Burn assembly annihilation/adjoint helpers on the same quad-split topology.
- **Tensor ε on patch:** **`solve_maxwell_dec_patch_quad_split_tensor_eps_residual`** (and chain **`curl_curl_y_mode_matches_scalar_helmholtz_piecewise_eps_tensor_yy`**) — **real**, **`[B,N,9]`** where exercised.
- **DEC identities:** [`tests/dec_identities.rs`](../tests/dec_identities.rs) — \(d_1 d_0 = 0\), adjoint checks on triangles / quad-split / two-quads (**no** photonics solve).

### Still open (matrix **#6** / lane “Next PR acceptance”)

| Topic | Gap |
| --- | --- |
| **Hodge / metric** | Patch curl leg uses **diagonal primal-edge \(\star_1\)** lump — **not** circumcentric/barycentric dual **Hodge** / full edge–face metric blocks. |
| **Sparse** | Inner solve is **dense** Gauss–Jordan on **≤ 64** nodes — **no** production Krylov/sparse at large **N**. |
| **3D volume** | Complexes are **embedded 2D** simplicial sheets ( **`faces_b2`** only) — **no** volumetric **3D** tet/hex curl–curl closure. |
| **Complex ε** | **Scalar** nodal **`eps_r_imag`** (`[B,N,1]`) couples \(\Re\mathbf{E}\leftrightarrow\Im\mathbf{E}\) in stacked \(2\cdot 3N\) dense solve when **`N≤64`**; tensor API returns **\(\Re\mathbf{E}\)** only. **Lossy + `N>64`** still pass-through; **tensor / anisotropic imaginary \(\varepsilon\)** and **PML** on patch path **not** wired like scalar FDFD lane. |
| **BCs** | **Gauge pin** at node **0** only — **no** general radiation / absorbing / physical boundary data on patch boundary. |
| **Assembly** | Incidence/metrics are **test-authored COO**, not lifted from **full mesh / manifold state**. |
| **Fresnel** | **`two_half_spaces_fresnel_te_no_pml_matches_analytic`** uses relaxed LS — matrix asks for **tighter** margin or **discrete** reference check. |

## Test gate (this audit)

```bash
cargo test --features solver-experimental --test photonics_fresnel --test dec_identities
```

**Status:** passes on the audited workspace revision (green harness does **not** imply matrix row **#6** closure).

## Recommendation

**Keep `m6-dec` / matrix #6 pending** until the **Exact acceptance criterion** in [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) row **#6** is satisfied — specifically **circumdual Hodge** or metric-weighted DEC **and/or** assembled **cube**/mesh wiring into **`solve_maxwell_curl_curl`**, plus **sparse** scaling (including **lossy** large-**N**), **tensor imaginary \(\varepsilon\)** + **PML** on the patch path, **BCs** beyond gauge **pin**, and the stated **Fresnel** tightening — alongside documented CI gates. Partial milestones (**`PhotonicsDecFacesPatch`**, dense patch with **real** + **scalar imaginary \(\varepsilon''\)**, nodal tensor **ε** in tests) are **necessary evidence** only; they do **not** close the row.
