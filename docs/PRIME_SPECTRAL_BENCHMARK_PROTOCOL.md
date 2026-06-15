# Prime-Spectral Benchmark Protocol (Increment 3)

**Status:** Pre-registered before benchmark coding.  
**Orchestration:** Any **Composer Agent** (see `docs/COMPOSER_PRIME_SPECTRAL_HANDOFF.md`) may execute Inc 3 — agent-agnostic.

---

## Scope

Compare Helmholtz-only vs prime-spectral guidance on topology density evolution.  
**Hard constraint:** four-conjunct thermodynamic gate on scalars must not regress.

Prime-spectral is **not** a gate conjunct (`umst.guidance.prime_spectral` ∉ `GATE_REGISTRY`).

---

## Modes

| Mode | Filter stack | Purpose |
|------|--------------|---------|
| A | Helmholtz only | Baseline |
| B | PrimeSpectral only | Mangoldt/sparse hypothesis |
| C | Helmholtz ∘ PrimeSpectral | Recommended default if any mode wins |
| D | Coprime stride sparse schedule | Resonance-avoidance |

Env: `UMST_PRIME_SPECTRAL_MODE=A|B|C|D`

---

## Tier 1 — Fast grid (32×32)

Pattern: `umst-manifold/tests/topology_filter.rs`

| Metric | Pass | Fail / kill |
|--------|------|-------------|
| Iterations to ‖ρ−ρ\*‖ < tol | ≥10% vs Mode A | No improvement → stop zeta R&D |
| Wall time per 100 steps | ≥10% at equal tol | |
| Gate admissibility on thermo scalars | 100% | **Hard stop** |
| DEC identities (`dec_identities.rs`) | green | **Hard stop** |

---

## Tier 2 — Striatus B6

**Context:** B6 compliance (`c1 < 0.6·c0_uniform`) is **FAIL** and **out of scope** for PrimeSpectral Inc 3.

| Metric | Pass (regression) | Not a PrimeSpectral success metric |
|--------|-------------------|-------------------------------------|
| `greyness < 0.15` | Must not regress | |
| `xy_var > 0.1` | Must not regress | |
| `|vf_export − target| ≤ 0.01` | Must not regress | |
| `c1 < 0.6·c0_uniform` | — | Out of scope |
| Outer iteration wall time | ≥5% at equal vf/greyness | Secondary |

Env: `UMST_SHELL_PRIME_SPECTRAL=1`, compose with `UMST_SHELL_HELM` where applicable.

---

## Kill switch

If **no mode** beats Helmholtz-only on Tier 1 **and** Tier 2 regression holds:

- Stop zeta-zero formal R&D branch
- Retain Lean gate-preservation layer
- Reframe as optional sparse/coprime guidance

If Mode C wins → proceed Inc 4 + optional `UMST_SHELL_PRIME_SPECTRAL=1` default.

---

## SSOT output

Merged JSON: `outputs/prime_spectral_inc3_latest.json`  
Verdict: `outputs/prime-spectral-inc3-verdict.md`
