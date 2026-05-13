# Contributing to UMST

We welcome contributions to the Unified Material-State Tensor (UMST) Manifold.

## Code of Conduct
Please ensure all discussions and pull requests remain respectful, academic, and rigorous.

## Reporting Issues
When opening an issue, please provide a minimal reproducible example (MRE) if you suspect a physics violation or a tensor dimension mismatch.

## Pull Requests
All pull requests must adhere to the following standards:
1. **Rust Toolchain**: Pass `cargo clippy -- -D warnings` and `cargo fmt -- --check`. Our Minimum Supported Rust Version (MSRV) is **1.75**.
2. **Computational Rigour**: Preserve $O(1)$ activation memory guarantees (no dense BPTT backends or un-adjointed ODE integrators).
3. **Physics Guarantees**: Pass thermodynamic validation tests (`cargo test`). Any PR modifying the `ThermodynamicCBF` or the `Cellular Sheaf` Laplacian must include mathematical proof or citation in the PR description demonstrating conservation of mass and energy.

## Striatus / Track B8 (cartridge sibling)

When touching Striatus, Track L print-ready JSON, or **`closeout-int-striatus`**: read **`docs/CI_GAP_NOTES.md`** (section *Striatus script vs B8 rollup*) and the **[`docs/Solver-Status.md`](docs/Solver-Status.md#int-striatus--todo-close-criteria-honest)** *int-striatus* checklist. **`bash umst-concrete-cartridge/scripts/verify_striatus_coupled_gates.sh`**, when run from a parent directory that contains **`umst-concrete-cartridge`** at the expected relative path, may exit **0** with **`gates_track_b8_all_pass`** still **`false`**; **`UMST_REQUIRE_B8=1`** pytest **must** fail until the sidecar flips **`true`** — that is intentional, not a spurious CI failure.
