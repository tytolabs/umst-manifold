<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# FP categorical view: DEC as primal↔dual structure

**Epic:** `maos-fp-categorical-v04` · **Task:** `fp-v04-3-dec-morphism`

This note gives a **category-flavoured** reading of the discrete exterior calculus (DEC) operators implemented in `umst-manifold`: how **primal** cochains on vertices and edges (and optional 2-cells) relate to **dual** weak operators, and why the Burn kernels behave like **natural transformations** between functorial assignments of finite-dimensional vector spaces to a fixed oriented graph.

It complements the equation-first walkthrough in [Mathematical-Foundations.md](Mathematical-Foundations.md) §2, the integration-oriented sketch in [Category-of-Material-Updates.md](Category-of-Material-Updates.md), and the cross-cutting audit in [FP_CATEGORICAL_HARDENING_AUDIT.md](FP_CATEGORICAL_HARDENING_AUDIT.md).

---

## 1. Objects: what the graph fixes

Fix an oriented graph \(G = (V, E)\) and, when present, a partition of signed `faces_b2` columns into 2-cells (see [`UnifiedMaterialStateTensor::faces_b2`](../src/core/tensors.rs)).

| Discrete space | Typical tensor shape (batch \(B\), channels \(C\)) | Role |
|----------------|-----------------------------------------------------|------|
| Primal \(C^0\) | `[B, \|V\|, C]` | Nodal 0-cochains (scalars per vertex). |
| Primal \(C^1\) | `[B, \|E\|, C]` | Edge 1-cochains (fluxes, increments, edge-wise moduli after reduction). |
| Primal \(C^2\) (patch) | `[B, F, C]` | Face 2-cochains after `faces_b2` assembly (`F` = number of column ranges). |

The **object** of study is not “a single vector space” but the **indexed family** of spaces \(\mathcal{C}^k(G)\) (here, concrete `Tensor` carriers) for a **fixed** incidence pattern `edges_b1` (and optional `faces_b2`). Changing topology means changing the object \(G\); naturality below is **internal** to a fixed \(G\) (vertex/edge relabelings that preserve incidence).

---

## 2. Functors: linear DEC as structure maps

Two contravariant-style pictures coexist in the code comments:

- **Primal coboundary** \(d_0 : C^0 \to C^1\) — oriented edge increment “head minus tail”. Implementation: [`primal_scalar_edge_increment`](../src/physics/dec_primal.rs) with [`EdgeTopology`](../src/physics/topology.rs).
- **Weak divergence / codifferential** \(B_1^\top : C^1 \to C^0\) — scatter signed edge contributions to endpoints. Implementation: [`primal_divergence_from_edge_flux`](../src/physics/dec_primal.rs) and the convenience wrapper [`primal_divergence_from_edge_flux_topo`](../src/physics/dec_primal.rs).

On a **closed** chain, applying divergence to a pure coboundary has **zero row-sum** per channel (mass conservation pattern); see module docs in [`dec_primal.rs`](../src/physics/dec_primal.rs) and the Hodge–Dirac Laplacian in [`TopologicalLaplacian`](../src/physics/laplacian.rs).

When `faces_b2` is populated, the same file exposes **primal** \(d_1\) and \(d_1^\top\) without metric weights:

| Map | Rust |
|-----|------|
| \(d_1 : C^1 \to C^2\) | [`primal_d1_edge_flux_to_faces`](../src/physics/dec_primal.rs) |
| \(d_1^\top : C^2 \to C^1\) | [`primal_d1_transpose_face_flux_to_edges`](../src/physics/dec_primal.rs) |

These are the operators locked by **`dec_*`** tests in [`tests/dec_identities.rs`](../tests/dec_identities.rs) (annihilation \(d_1 \circ d_0 = 0\), unweighted Frobenius adjoint identity for \(d_1/d_1^\top\)).

---

## 3. Natural transformations: same incidence, different state

Let \(\mathbf{V}^0, \mathbf{V}^1\) be the concrete Burn carriers for nodal and edge data (fixed `edges_b1`, batch/channel ranks as in the tensors above).

**Linear primal block.** For fixed topology, the maps \(d_0 : \mathbf{V}^0 \to \mathbf{V}^1\) and \(B_1^\top : \mathbf{V}^1 \to \mathbf{V}^0\) implemented by `dec_primal` are **linear** and depend only on gather/scatter indices derived from `edges_b1`. If you think of a **functor** \(F^k\) from “discrete complexes shaped like \(G\)” to finite-dimensional spaces \(\mathbf{V}^k\) (fixed \(G\) ⇒ fixed \(F^k(G) = \mathbf{V}^k\)), then a **morphism of cochains** \((f^0, f^1)\) that commutes with incidence is exactly a **chain map**; post-composing the universal \(d_0\) with \(f^1\) and pre-composing with \(f^0\) is the **naturality square** for the coboundary as a transformation between functors of coefficients.

In implementation terms: **reordering vertex/edge ids** (consistent permutation of rows/columns of the incidence data) conjugates the operators; the Burn kernels are written so that **only** `edges_b1` / `faces_b2` and the channel-expanded index tensors change—not the algebraic pattern (`gather` for \(d_0\), `scatter` for \(B_1^\top\)).

**Nonlinear edge reductions.** [`DecEdgeOperators`](../src/physics/dec_operators.rs) (`arithmetic_mean_on_edges`, `harmonic_mean_on_edges`) maps nodal **intensive** data to edge slots by **nonlinear** reduction of endpoint values. Functorially this is still “vertex data → edge data” along the same skeleton, but it is **not** a linear natural transformation of \(d_0\); it is a separate morphism family used where material laws need edge-wise moduli or transport reductions (see rustdoc in [`dec_operators.rs`](../src/physics/dec_operators.rs) and call sites such as [`mechanics.rs`](../src/physics/mechanics.rs)).

---

## 4. Primal↔dual wording (engineering sense)

Classical DEC often pairs a **primal** simplicial complex with a **dual** circumcentric or barycentric complex. In this crate’s **graph-first** slice:

- **Primal** language names **nodal** and **edge** unknowns on the oriented 1-skeleton.
- **Dual** language names the **transpose / scatter** operators that realize weak divergence and adjoints of \(d_1\) **without** building an explicit dual mesh: \(B_1^\top\) and [`primal_d1_transpose_face_flux_to_edges`](../src/physics/dec_primal.rs) are the discrete duals in the **algebraic** sense (transpose of the primal incidence operator under unweighted inner products, as stated in `dec_primal` docs).

Metric/Hodge weights, damage masks, and solver-specific compositions **post-compose** on these spaces (e.g. [`TopologicalLaplacian::scalar_laplacian`](../src/physics/laplacian.rs)); they do not change the bare incidence naturality story.

---

## 5. Module map (implementation)

| Concern | Module / type |
|--------|----------------|
| `edges_b1` gather layout, batch/channel index expansion | [`physics::topology`](../src/physics/topology.rs) — `EdgeTopology` |
| Primal \(d_0\), \(B_1^\top\), \(d_1\), \(d_1^\top\) (topology-only) | [`physics::dec_primal`](../src/physics/dec_primal.rs) |
| Nodal → edge **mean** reductions (material/transport) | [`physics::dec_operators`](../src/physics/dec_operators.rs) — `DecEdgeOperators` |
| Barrel re-exports + functorial reading in rustdoc | [`physics::operators`](../src/physics/operators.rs) |
| Masked Laplacian / fracture-aware flow on the 1-skeleton | [`physics::laplacian`](../src/physics/laplacian.rs) — `TopologicalLaplacian` |
| Photonics / electrochemistry integration of the same primitives | [`physics::solvers::photonics`](../src/physics/solvers/photonics.rs), [`physics::solvers::electrochemistry`](../src/physics/solvers/electrochemistry.rs) |

---

## 6. Verification pointers

- DEC identities and incremental patch tests: [`tests/dec_identities.rs`](../tests/dec_identities.rs).
- Audit checklist row **fp-v04-2** (DEC morphisms): [FP_CATEGORICAL_HARDENING_AUDIT.md](FP_CATEGORICAL_HARDENING_AUDIT.md).

Suggested commands when touching Rust in these modules (from `umst-manifold/` or via `--manifest-path umst-manifold/Cargo.toml`):

```text
cargo test --test dec_identities
cargo clippy -p umst-manifold -- -D warnings
```

The substring filter `cargo test dec` also matches unrelated names (for example **`decreases`**); prefer `--test dec_identities` for the seven DEC identity tests. A full `--features solver-experimental` build may fail on snapshots where opt-in lanes do not compile; fix or exclude those features before relying on `cargo test --features solver-experimental dec`.
