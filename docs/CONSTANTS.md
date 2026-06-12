# Constants registry (integration-contracts D3)

**Module:** `src/constants_registry.rs`  
**CI:** `scripts/check_constants.py` (non-blocking lane in `rust.yml`)

## `GroundedConst` pattern

Each migrated row is an immutable `GroundedConst { name, value, evidence }` pointing at a single Rust `const`
or `umst-math` re-export. No runtime registry mutation.

## Migrated rows

| `name` | Value SSOT | Feature gate |
| --- | --- | --- |
| `mechanics_default_pcg_rel_tol` | `MechanicsInnerLoopConfig::default().pcg_tolerance` | always |
| `mechanics_default_max_cg_iterations` | `MechanicsInnerLoopConfig::default().max_cg_iterations` | always |
| `landauer_bit_energy_300k_j` | `constants::landauer_bit_energy_joules(300.0)` | always |
| `hex_pcg_max_iter_default_striatus` | `q1_hex_elasticity::HEX_PCG_MAX_ITER_DEFAULT_STRIATUS` | topology / voigt |
| `hex_pcg_rel_tol_f32` | `q1_hex_elasticity::HEX_PCG_REL_TOL_F32` | topology / voigt |
| `hex_pcg_rel_tol_f64` | `q1_hex_elasticity::HEX_PCG_REL_TOL_F64` | topology / voigt |
| `thmc_dense_newton_max_stacked_dofs` | `THMC_DENSE_NEWTON_MAX_STACKED_DOFS` | `thmc-coupled` |
| `k_boltzmann_j_per_k` | `umst_math::landauer::K_B` | `math-constants` |

## TODO rows (not migrated)

Listed in `THMC_FLOATS_TODO` — hydration / kinetics literals remain in `solvers/thmc.rs` until Wave 1 cartridge calibration:

- `HYDRATION_ARRHENIUS_PREFACTOR_S`
- `HYDRATION_ACTIVATION_ENERGY_J_PER_MOL`
- `HYDRATION_T_MIN_K`
- `HYDRATION_T_BOOST_REF_K`
- `HYDRATION_T_BOOST_PER_K`
- `HYDRATION_EXOTHERMIC_K_PER_ALPHA_RATE`
- `UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K`

## Deliberately not done

- No literal duplication of THMC floats in the registry.  
- No edits to `q1_hex_elasticity.rs` constants (re-export only).
