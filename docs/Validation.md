SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
<!--
-->

# Validation

The UMST Manifold ships with a validation envelope that is deliberately
narrower than its capability surface — every claim in this document is
backed by an automated test in [`tests/`](../tests).

## V.1 Discrete exterior calculus

| Identity | Test | Reference |
|----------|------|-----------|
| `d ∘ d = 0` on every closed 1-chain | [`tests/dec_identities.rs::d_squared_zero`](../tests/dec_identities.rs) | Crane et al. 2013 §3 |
| Discrete Stokes on a triangulated disk | [`tests/dec_identities.rs::stokes_triangle`](../tests/dec_identities.rs) | Hirani 2003 §2.4 |
| Hodge Laplacian symmetry: `Δ = Δᵀ` | [`tests/dec_identities.rs::laplacian_symmetric`](../tests/dec_identities.rs) | — |
| Mass conservation under random topology mutation | [`tests/conservation.rs::mass_conserved_under_severing`](../tests/conservation.rs) | Crane et al. §4 |

These run on a deterministic 32-vertex test graph and on a 1024-vertex
Delaunay triangulation of a unit square.

## V.2 Adjoint sensitivity

| Property | Test | Tolerance |
|----------|------|-----------|
| Adjoint gradient agrees with finite-difference gradient on a linear ODE | [`tests/adjoint.rs::linear_ode_gradient_matches_fd`](../tests/adjoint.rs) | $\le 10^{-4}$ relative |
| Constant activation memory: peak allocator usage independent of horizon $T$ | [`tests/adjoint.rs::activation_memory_constant`](../tests/adjoint.rs) | within $1.5\times$ of $T=1$ |
| Time-reversal: $\mathbf{a}(0)$ recovers the analytic adjoint | [`tests/adjoint.rs::reverse_terminal_condition`](../tests/adjoint.rs) | $\le 10^{-6}$ |

## V.3 Thermodynamic CBF

| Property | Test | Reference |
|----------|------|-----------|
| Admissible Carnot cycle is accepted | [`tests/cbf.rs::carnot_admissible`](../tests/cbf.rs) | classical |
| Reverse Carnot cycle (entropy decrease) is rejected | [`tests/cbf.rs::reverse_carnot_rejected`](../tests/cbf.rs) | classical |
| Erasure cost saturates the Landauer bound to within 1 % | [`tests/cbf.rs::landauer_saturation`](../tests/cbf.rs) | Landauer 1961 |

## V.4 Type-state pattern

| Property | Test |
|----------|------|
| `VerifiedUMST` is constructible only via the gateway | [`tests/type_state.rs::no_external_construction`](../tests/type_state.rs) (compile-fail) |
| Inadmissible state cannot be lifted to `VerifiedUMST` | [`tests/type_state.rs::inadmissible_rejected`](../tests/type_state.rs) |

## How to reproduce

```bash
git clone https://github.com/tytolabs/umst-manifold
cd umst-manifold
cargo test --all-features --release
```

CI runs the full suite on every push and pull request — see
[`.github/workflows/rust.yml`](../.github/workflows/rust.yml).

## What this document does *not* claim

- The CBF is not a *proof* of the Second Law. It is a runtime gate. Bugs
  in the gate can let inadmissible states through, which is why
  contributions that touch dissipation must add a regression test.
- Adjoint memory is $\mathcal{O}(1)$ in the *integration horizon*. Total
  memory still scales with parameter count and adjoint state width.
- "Conservation by construction" is conservation under exact arithmetic.
  In practice, floating-point round-off introduces a residual; the
  conservation tests assert this residual is below $10^{-10}$ in
  absolute mass for a unit-mass setup.
