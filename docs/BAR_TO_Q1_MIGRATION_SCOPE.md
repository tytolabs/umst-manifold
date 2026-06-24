# Bar → Q1 hex migration scope (Track C S4)

**Status:** Partial — adjoint Q1-hex shipped; bar-limit parity test remains `#[ignore]`.

## Objective

Replace Voigt-bar mechanics surrogates with Q1-hex adjoint compliance on production topology paths without regressing Striatus shell acceptance.

## In scope (shipped)

| Item | Evidence |
|------|----------|
| `AdjointComplianceQ1Hex` kernel | `src/physics/adjoint_q1_hex.rs` |
| Analytic Q1 tests (non-ignored) | `tests/verification/adjoint_q1_hex_compliance_analytic.rs` |
| Bar limit skeleton | `tests/verification/adjoint_q1_hex_matches_bar_in_limit.rs` (**ignored**) |

## Out of scope (this cycle)

- Wide-plate Kirchhoff **R2.1-A** gate (`UMST_MECHANICS_R21_GATE=1`)
- Striatus 40×40×4 production mesh acceptance
- Cartridge B6 200-outer harness

## Migration checklist

1. Un-ignore `adjoint_q1_hex_matches_bar_in_limit` when `rel_err < 0.05` on reference fixture.
2. Wire `MechanicsSolvePort` consumer on THMC stagger path (Wave 9 partial).
3. Update [`Solver-Status.md`](Solver-Status.md) completion row for mechanics research lane.
4. Cross-link [`SOLVER_NEVER_RUN_LEDGER.md`](SOLVER_NEVER_RUN_LEDGER.md) when ignore is cleared.

## Related

- [`BAR_TO_Q1_MIGRATION_SCOPE.md`](BAR_TO_Q1_MIGRATION_SCOPE.md) (this file)
- MaOS B6 harness placeholders: `scripts/b6_harness_setup.sh` (`policy_editable_mask`)
