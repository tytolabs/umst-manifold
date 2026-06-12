# Mechanics SSOT (integration-contracts D2)

**Module:** `src/physics/mechanics_operator.rs`  
**Audit:** P0 #1 (bar load-bearing on coupled paths), finding **#2** (9×8×2 roof PCG stall)

## Two discretizations

| Discretization | SSOT module | Consumers (today) |
| --- | --- | --- |
| **Bar network** (DEC 1-skeleton) | `mechanics::VectorMechanicsSolver` | `thmc`, `fracture_field`, `adjoint`, `topology` |
| **Q1 hex** (extruded brick) | `extruded_plate`, `q1_hex_elasticity` | `AdjointComplianceQ1Hex`, verification harnesses |

The [`MechanicsOperator`](../../src/physics/mechanics_operator.rs) trait is the typed morphism for quasi-static equilibrium
\(K(\rho)u = f\). **This wave** ships the trait + deprecated [`BarNetworkMechanicsAdapter`] only — **zero consumer ports**.

## Call-site inventory (read-only survey)

| Module | Symbol | Notes |
| --- | --- | --- |
| `solvers::thmc` | `VectorMechanicsSolver::solve_equilibrium` | operator-split outer pass |
| `solvers::fracture_field` | `solve_equilibrium`, `voigt_strain_from_edge_displacement` | staggered u↔d open |
| `solvers::thmc_residual` | `projected_bar_equilibrium_residual` | monolithic \(R_u\) |
| `adjoint` | `packed_bar_network_equilibrium` | f64 inner PCG |
| `protocols::MechanicsEquilibrium` | delegates to bar `solve_equilibrium` | namespace alias |

## Migration order (Wave 3)

1. **THMC \(R_u\)** — highest P0 load-bearing path.  
2. **Fracture stagger** — `update_damage_staggered` inner elasticity.  
3. **Adjoint TO** — discrete compliance sensitivity chain.

## Parity guarantees (this wave)

- `bar_adapter_two_node_bit_identical_to_direct`  
- `bar_adapter_nine_node_bit_identical_to_direct`  

Both assert bit-identical `(u, σ)` vs direct `VectorMechanicsSolver::solve_equilibrium`.

## Deliberately not done

- No `MechanicsOperator` impl for Q1 hex (Wave 3).  
- No edits to `mechanics.rs` bar kernel.  
- No cartridge / Striatus harness changes.
