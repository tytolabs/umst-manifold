# Mechanics — geometric nonlinearity vs shipped small-strain models

**Status:** scope / honesty note (not a shipped solver feature).  
**Cross-links:** main solver table row for `mechanics::VectorMechanicsSolver` in [`docs/Solver-Status.md`](../Solver-Status.md); R2.1 plate gate index in [`docs/PHYSICS_CAPABILITY_GAPS.md`](../PHYSICS_CAPABILITY_GAPS.md#r21-plate-gaps-index).

This memo contrasts **large deformation / geometrically nonlinear** solid mechanics with what `umst-manifold` actually implements today, so downstream coupling (THMC, fracture, topology) is not misread as finite-strain or updated-Lagrangian truth.

---

## 1. What is implemented (tensor contracts)

### 1.1 Bar network — `VectorMechanicsSolver` (`src/physics/mechanics.rs`)

| Quantity | Shape | Role |
| --- | --- | --- |
| Nodal displacement `u` | `[B, N, 3]` | Unknown / post-solve field |
| Reference vertex coords | `[N, 3]` | **Fixed** reference geometry for the solve |
| Edge tangent `edge_unit`, ref. length `edge_len` | `[B, E, 3]`, `[B, E, 1]` | Built from **undeformed** segment vectors `c_tgt − c_src` |
| Axial stiffness `k_axial` | `[B, E, 1]` | `(EA/L_ref) · damage\_factor` with `L_ref` from reference coords |
| Voigt strain from edge displacements | `[B, N, 6]` | `voigt_strain_from_edge_displacement` |
| Cauchy stress (Voigt+Hooke path) | `[B, N, 3, 3]` | `isotropic_hooke_sigma` |
| Bar-network “stress” (default path) | `[B, N, 3, 3]` | Rank-one axial post-process from bar forces |

**Kinematic strain used for Voigt recovery:** per edge, axial engineering / small-strain measure

\[
\varepsilon_{\mathrm{ax}} = \frac{(\mathbf u_{\mathrm{tgt}}-\mathbf u_{\mathrm{src}})\cdot \hat{\mathbf t}}{L_{\mathrm{ref}}},
\]

with \(\hat{\mathbf t}\) and \(L_{\mathrm{ref}}\) from **reference** node positions. Strain is then spread as a rank-one tensor \(\varepsilon_{\mathrm{ax}}\,\hat{\mathbf t}\otimes\hat{\mathbf t}\) and **averaged to nodes** by incident-edge count (see module rustdoc on `voigt_strain_from_edge_displacement`).

**Equilibrium operator:** projected CG on `K u = f` where the discrete bar `matvec` uses the same **reference** `edge_unit` and `k_axial ∝ 1/L_ref`. There is **no** rebuild of `L` or `t̂` from `coords + u` inside the inner loop — i.e. no **updated Lagrangian** (UL) or **total Lagrangian** (TL) geometric stiffness from changing length/direction.

### 1.2 Q1 hex — linear elasticity (`src/physics/q1_hex_elasticity.rs`)

| Quantity | Convention |
| --- | --- |
| Element / node layout | Structured brick; corners from fixed spacing `(dx, dy, dz)` |
| Strain–displacement | Infinitesimal (small) strain via shape-function gradients; **B-bar / SRI** mixes centroid volumetric operator vs full \(2^3\) Gauss for deviatoric/shear rows |
| Constitutive | Constant isotropic **`D(E, ν)`** in Voigt form — **linear** Hooke law in small strain |

The module header explicitly describes **linear elasticity** on a Cartesian lattice. Displacement enters **linearly** in the strain operator; there is no Green–Lagrange or Almansi strain, no hyperelastic strain-energy density, and no push-forward of stress with evolving Jacobian of the map from reference to current placement.

---

## 2. “Small strain” vs named finite-deformation models

**Shipped meaning of \(\varepsilon\) here:** small-displacement-gradient **infinitesimal** strain tensor in the lab frame (Voigt `[εxx, εyy, εzz, εxy, εyz, εxz]` with **tensor** shears, not engineering \(\gamma=2\varepsilon\) — see `mechanics.rs` rustdoc).

**St. Venant–Kirchhoff (SVK) — *not* what this code path is:**

- SVK is a **hyperelastic** model: strain measure is **Green–Lagrange** \(\mathbf E = \tfrac12(\mathbf F^\top\mathbf F - \mathbf I)\), stress work is naturally with **2nd Piola–Kirchhoff** \(\mathbf S = \lambda\,\mathrm{tr}(\mathbf E)\mathbf I + 2\mu\mathbf E\), and equilibrium is posed in **reference** configuration (often with \(\mathbf P = \mathbf F\mathbf S\) for weak form).
- That is **geometrically nonlinear** (finite \(\mathbf F\)) even before plasticity or damage.

**What `isotropic_hooke_sigma` actually does:** given a Voigt **small** \(\boldsymbol\varepsilon\), form Cauchy stress \(\boldsymbol\sigma = \lambda\,\mathrm{tr}(\boldsymbol\varepsilon)\mathbf I + 2\mu\boldsymbol\varepsilon\) (then optional nodal rotation sandwich). That matches **classical small-strain isotropic elasticity** in the **current** lab frame when `rotation = I`. It does **not** implement SVK’s \(\mathbf S(\mathbf E)\) nor \(\mathbf F\)–based work conjugates.

**Truth boundary for large deformation:** once \(\|\nabla\mathbf u\|\) is not \(\ll 1\) (or bar extension \(\Delta L/L\) is not small), a small-strain \(\boldsymbol\varepsilon\) and Cauchy \(\boldsymbol\sigma\) from linear Hooke **cease** to be consistent with the same boundary-value problem as a proper finite-strain formulation; stiffness should generally depend on **current** geometry (UL) or use a finite strain measure (TL / hyperelastic). None of that is in the shipped bar or Q1-hex operators above.

---

## 3. Application gaps (documented, not implied by CI)

### 3.1 Fresh concrete slumping

Slumping combines **large viscoplastic / fluid-like deformation**, often **self-contact** and **free-surface** kinematics, and time-dependent rheology. The research lane here is **elastic / bar-network equilibrium** (plus linear Q1 hex checks) on **fixed** reference graphs or bricks — it does **not** certify slump flow, yield surface evolution, or finite-strain remeshing.

### 3.2 Buckling

Linearized buckling needs **geometric stiffness** (stress stiffening) or a fully nonlinear equilibrium path with **singular / indefinite** tangent in the post-buckled branch. The shipped `packed_bar_network_equilibrium` / hex PCG paths solve a **single positive-(semi)definite-type linear elasticity problem** per call (damage modifies stiffness scalars but does not add a **consistent** \(K_G(\boldsymbol\sigma)\) from prestress). **Arc-length continuation**, **imperfection sensitivity**, and **mode tracking** are out of scope for the current operators.

### 3.3 Updated / total Lagrangian

**Missing:** recomputation of `edge_len` / `edge_unit` from **deformed** positions `coords + u` when forming stiffness or strain; hyperelastic tangents \(\partial^2 W/\partial\mathbf F^2\); corotational or TL UL split steppers. Any future “go nonlinear” work should name the configuration (reference vs current) for both **strain** and **stress** and align fracture / THMC strain helpers with that same convention.

---

## 4. Code pointers

| Topic | Location |
| --- | --- |
| Bar equilibrium, Voigt strain, Hooke Cauchy | `src/physics/mechanics.rs` — `packed_bar_network_equilibrium`, `voigt_strain_from_edge_displacement`, `isotropic_hooke_sigma`, `bar_matvec` |
| Q1 hex linear operator | `src/physics/q1_hex_elasticity.rs` — `hex_k_times_u_accumulate`, `bbar_times_u`, `build_d_voigt` |
| Solver status / verification | `docs/Solver-Status.md` mechanics row |

---

## 5. Related honesty items already in-repo

- Kirchhoff plate **ratio band** vs strict thin-plate gate: mechanics row in [`Solver-Status.md`](../Solver-Status.md); v0.4 follow-up [**§R2.1**](../../../composer_prompts/v0.4_phase_3_followup_for_composer.md#r21--q1-hex-sri-for-kirchhoff-plate-5-gate); companion index in [`PHYSICS_CAPABILITY_GAPS.md`](../PHYSICS_CAPABILITY_GAPS.md#r21-plate-gaps-index).

### R2.1 checklist — strict SSSS Kirchhoff vs `plate_centre_deflection_kirchhoff_ratio_q1_hex_locked_band` {#r21-plate-checklist}

**Strict SSSS Kirchhoff acceptance (target gate — not default CI today)**

- [ ] Centre \(w\) matches Kirchhoff SSSS centre formula within ~**5%** at the agreed **\(L/h\)** (e.g. \(\approx 20\)), vs `kirchhoff_centre_w_ssss` in `tests/verification/mechanics_analytic.rs`.
- [ ] **Boundary data** implements classical **SSSS** on all supported edges of the *plate problem* (not merely a full-thickness `u_z=0` mask on one brick face).
- [ ] **Thin-plate regime:** formulation + BCs avoid **shear-locking** / spurious transverse-shear dominance at that \(L/h\) so the comparison to Kirchhoff is meaningful.
- [ ] **Q1 hex / SRI–B-bar** (or successor) is **consistent** with the chosen plate reduction and facet BC parity where the brick differs from a shell DOF set (`src/physics/q1_hex_elasticity.rs` module notes).

**Current ratio-band CI (`plate_centre_deflection_kirchhoff_ratio_q1_hex_locked_band`)**

- [ ] Pins **`w / w_Kirchhoff`** into fixed open band **`(5×10⁻⁵, 0.02)`** on the **32×32×4** extruded slab with documented loads/materials.
- [ ] Documents **locked Q1** behaviour: **not** a within-5% Kirchhoff accuracy claim; guards solve/residual regressions while \(w\) stays in the band.
- [ ] Acknowledges **non-SSSS** brick setup (e.g. bottom `u_z=0` support vs classical edge data) per test rustdoc in `mechanics_analytic.rs`.

- THMC / fracture use **post-mechanics** small-strain tensors on the bar path — see THMC and fracture lanes in [`Solver-Status.md`](../Solver-Status.md); they inherit the **same** kinematic limitations stated here.
