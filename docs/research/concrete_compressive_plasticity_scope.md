# Concrete compressive plasticity vs tensile phase-field (scope note)

**Workspace anchors:** `src/physics/solvers/fracture_field.rs` (`PhaseFieldFractureSolver`, feature `fracture-at2`), mechanics in `src/physics/mechanics.rs` (bar / Voigt elasticity, no yield surface). **Public status:** [`Solver-Status.md`](../Solver-Status.md) → row `solvers::fracture_field`. **Composer plan (Phase 2 / 3):** [`../../composer_prompts/v0.4_phase_2_3_master_plan.md`](../../composer_prompts/v0.4_phase_2_3_master_plan.md) (e.g. P2.4 fracture Γ work, P3.1 staggered `u`/`d` + mechanics).

---

## 1. What the repo implements today (tensile-biased damage drive)

The AT2-style damage update uses a **scalar tensile strain-energy surrogate** \(\psi^+\) built from the **positive part of the principal strains** of the symmetric small-strain tensor \(\varepsilon\):

- After approximate diagonalization (cyclic Jacobi on each \(3\times3\) block), diagonal entries are treated as eigenvalues \(\lambda_i\).
- \(\psi^+ = \tfrac{1}{2}\sum_i \langle\lambda_i\rangle_+^2\) with \(\langle x\rangle_+ = \max(0,x)\).

The discrete AT2 drive uses \(2(1-d)\,\psi^+\) on the RHS of the nodal damage equation (see `update_damage_experimental` in `fracture_field.rs`). **Pure hydrostatic compression** (all \(\lambda_i < 0\)) yields \(\psi^+ = 0\): the implemented phase-field **does not accumulate damage from compressive principal strains alone**.

Mechanics in the staggered helpers remains **linear elastic** (Young-type stiffness on bars, possibly degraded by \(g(d)\) at the call site) — there is **no** Drucker–Prager, Mohr–Coulomb, cap hardening, or crushing porosity law in the manifold solver tree reviewed for this note.

---

## 2. What is *not* in the tree (search summary)

Targeted searches over `*.rs` under `umst-manifold` for **Drucker–Prager**, **Menétrey–Willam**, **cap** plasticity, and generic **continuum plasticity / yield** surfaces did **not** hit a constitutive implementation (aside from unrelated identifiers such as pressure increments named `dp` in other solvers). **Bingham-type viscoplasticity** exists in `rheology_flow.rs` (fluid lane), not as a solid crushing model for concrete struts.

---

## 3. Envelope sketch (solid mechanics intuition)

Concrete failure under multiaxial stress is often summarized in **meridian** plots: **deviatoric measure** (e.g. \(\sqrt{J_2}\) or equivalent pressure \(q\)) vs **mean stress** \(p\) (or \(I_1\)). Tensile **fracture** and **compressive crushing / shear** occupy different regions of that plane.

```text
        sqrt(J2) or q
              ^
              |     *  tensile cracking / mode-I
              |    /
              |   /  phase-field ψ+ (this repo) tracks
              |  /   "positive principal strain" energy —
              | /    aligned with tensile side of behavior
              |/
   -----------+--------------> p (mean stress, compression +)
              |\
              | \   Drucker–Prager / MC shear envelope
              |  \  (not implemented)
              |   \
              |____\____  cap / hardening — compaction limit
                         (not implemented)
```

**Drucker–Prager (DP):** smooth cone in \((p,q)\) space — shear-driven failure with pressure sensitivity (higher \(p\) → higher shear capacity). **Cap models:** add a **porosity / compaction** cap so highly compressed states cannot carry unlimited mean stress (relevant to pore collapse, not only shear).

---

## 4. Why highly compressed slender struts are unsafe to trust *without* a crushing model

1. **Missing compressive limit:** Linear elasticity + tensile-only \(\psi^+\) damage does **not** cap axial compressive capacity. In topology-style bar networks, optimizers or load paths can concentrate **large compressive axial forces** in slender members; the model still returns an elastic axial stress until numerical or stability limits intervene.

2. **No confinement / multiaxial concrete physics:** Real concrete crushing is **pressure-sensitive** and often **nonlocal** (strain localization, confinement from surrounding material). A DP / cap / CDP-style law is the standard engineering guardrail for **compressive** and **shear-dominated** regimes.

3. **Fracture variable does not substitute for crushing:** With \(\psi^+\) from **positive** principal strains only, **compression-driven** failure modes (spalling under high triaxial compression, crushing bands, etc.) are **not represented** by the current damage driver. Trusting peak compressive strut loads for **safety-relevant** conclusions would **overpredict** capacity and **miss** the correct failure mechanism.

---

## 5. Suggested direction (out of current v0.4 fracture row scope)

If compressive reliability matters for a use case (printed concrete lattices, optimized struts, impact, confinement-sensitive design):

- Add a **solid plasticity** lane (return-mapping or explicit) with at least **DP or Mohr–Coulomb** + optional **cap** for compaction, **or** document explicit **stress / strain caps** as a stopgap.
- Keep **phase-field** for **tensile** cracking; treat **crushing** as a **separate** internal variable or plastic multiplier so tensile spectral splits are not asked to carry compressive physics.

This file is **documentation / scope** only; it does not change solver behavior.
