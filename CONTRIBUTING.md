# Contributing to UMST

We welcome contributions to the Unified Material-State Tensor (UMST) Manifold.

## Code of Conduct
Please ensure all discussions and pull requests remain respectful, academic, and rigorous.

## Reporting Issues
When opening an issue, please provide a minimal reproducible example (MRE) if you suspect a physics violation or a tensor dimension mismatch.

## Pull Requests
All pull requests must:
1. Pass `cargo clippy -- -D warnings`
2. Preserve O(1) activation memory guarantees (no dense BPTT backends)
3. Pass thermodynamic validation tests (`cargo test`)
