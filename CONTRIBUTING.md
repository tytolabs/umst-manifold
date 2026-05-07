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
