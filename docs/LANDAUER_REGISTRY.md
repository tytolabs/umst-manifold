# Landauer / CODATA registry (P3 axiom)

**Module:** `umst-math::landauer_registry`  
**Feature:** `umst-math` crate feature `math-constants`  
**Lean lineage:** `UMST.FormalDoubleSlit.LandauerBound`, `UMST.Formal.LandauerLaw`  
**Zenodo:** [10.5281/zenodo.19159660](https://doi.org/10.5281/zenodo.19159660)

## Purpose

Compile-time registry of Boltzmann / Landauer constants used by manifold gate accounting,
η_cog denominators, and UCRS phase-entropy witnesses. Pure FP: no `std::fs`, no env reads.

## `LandauerRegistry` pattern

Each row is an immutable [`LandauerConst`](../../umst-math/src/landauer_registry.rs)
`{ name, value, si_unit, provenance }`. Values delegate to [`umst-math::landauer`](../../umst-math/src/landauer.rs)
where applicable (`K_B`, `landauer_bit_energy_joules`).

| `name` | SSOT | Unit |
| --- | --- | --- |
| `k_boltzmann_j_per_k` | `landauer::K_B` (CODATA 2018) | J/K |
| `ln_two` | `std::f64::consts::LN_2` | 1 |
| `host_temperature_reference_k` | `300.0` (ambient anchor) | K |
| `landauer_bit_energy_300k_j` | `k_B · 300 · ln 2` | J/bit |

## Feature gates

| Crate | Feature | Effect |
| --- | --- | --- |
| `umst-math` | `math-constants` | compiles `landauer_registry` module |
| `umst-manifold` | `math-constants` | enables optional `umst-math` dep + `umst-math/math-constants` |

Manifold runtime Landauer bit energy remains in `src/constants.rs` (delegates to
`umst_math::landauer::landauer_bit_energy_joules` when `math-constants` is on).
Solver-facing grounded rows live in `src/constants_registry.rs` (`k_boltzmann_j_per_k`).

## Deliberately not done

- No duplicate of the full cockpit §24a [`constants::registry`](../../umst-math/src/constants/registry.rs) table.
- No THMC reaction-extent floats (see `docs/CONSTANTS.md` TODO rows).
- No IO or operator env overrides in this module (those stay in egoff cockpit layers).
