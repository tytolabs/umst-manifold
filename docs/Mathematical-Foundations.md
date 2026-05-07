<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Mathematical Foundations

This document specifies the mathematical objects that the UMST Manifold
implements, in the order in which they appear in the source. Each section
maps directly to a module so that readers can move between equation and
code without translation.

## 1. The cellular sheaf

A material configuration is represented as a cellular sheaf
$\mathcal{F}$ over an oriented graph
$G = (V, E)$ — the *1-skeleton* of the underlying spatial complex. To
each vertex $v \in V$ we attach a stalk
$\mathcal{F}(v) = \mathbb{R}^{f}$ of scalar features (temperature,
porosity, age, damage), and to each edge $e \in E$ we attach a stalk
$\mathcal{F}(e) = \mathbb{R}^{g}$ of flow quantities (heat flux, mass
flux, normal stress). For each incidence $v \prec e$ the sheaf carries
a restriction map
$\rho_{v \prec e} : \mathcal{F}(v) \to \mathcal{F}(e)$.

In code, the sheaf is stored sparsely as
[`UnifiedMaterialStateTensor`](../src/core/tensors.rs):

| Field | Shape | Role |
|-------|-------|------|
| `coords` | `[N, 5]` | sparse spacetime indices `(b, t, x, y, z)` |
| `edges_b1` | `[\|E\|, 2]` | $B_1$ vertex–edge incidence |
| `faces_b2` | `[\|F\|, 2]` | $B_2$ edge–face incidence |
| `scalar_features` | `[N, f]` | vertex stalks |
| `vector_features` | `[N, g, 3]` | edge / 1-form stalks |
| `matrix_features` | `[N, h, 3, 3]` | 2-form / stress stalks |

Working on the 1-skeleton — rather than a dense Eulerian grid — means
that topology change (fracture, contact, severing) is a sparse
combinatorial update on `edges_b1` rather than a tensor reshape.

## 2. Discrete exterior calculus

The boundary matrix $B_1$ encodes the discrete exterior derivative
$d : C^0(G) \to C^1(G)$, taking 0-cochains (vertex-valued functions) to
1-cochains (edge-valued forms):

$$
(d \, \omega)(e) = \omega(\mathrm{head}(e)) - \omega(\mathrm{tail}(e)).
$$

Its formal adjoint $d^{*} = B_1^{\top}$ takes 1-cochains back to
0-cochains. The discrete Hodge Laplacian on vertices is

$$
\Delta_0 \;=\; d^{*} d \;=\; B_1^{\top} B_1.
$$

Two identities hold by construction in
[`physics::laplacian`](../src/physics/laplacian.rs):

1. **Discrete Stokes** — for a 0-cochain $\omega$ and a 1-chain $C$,
   $\sum_{e \in C} (d \omega)(e)
   \;=\; \omega(\partial C^{+}) - \omega(\partial C^{-})$.
2. **`d \circ d = 0`** — at the level of $B_2 B_1$ the discrete cohomology
   complex is exact, so curl-of-gradient and divergence-of-curl vanish
   numerically, not asymptotically.

These are the discrete analogues of the smooth identities
$\int_{\partial \Omega} \omega = \int_{\Omega} d\omega$ and
$d^2 = 0$, and they are the reason mass and energy fluxes are conserved
across topology changes.

## 3. Type-state admissibility

Every physical update returns a `UnifiedMaterialStateTensor`. Before
that state is consumed by an agent, it passes through an admissibility
check that yields a witness type:

```rust
pub trait Proof: Send + Sync + 'static {}
pub struct ClausiusDuhemProof;
impl Proof for ClausiusDuhemProof {}

pub struct VerifiedUMST<B: Backend, P: Proof> { /* ... */ }
```

A `VerifiedUMST<_, ClausiusDuhemProof>` value can only be obtained
through the gateway constructor, which calls
[`ai::cbf::ThermodynamicCBF`](../src/ai/cbf.rs). Downstream APIs accept
`VerifiedUMST` only, so an unchecked state cannot reach them. This is
the standard *type-state pattern*; the law itself is enforced inside
the constructor — the type system enforces that the constructor was
*called*.

## 4. The thermodynamic control barrier

Let $\Psi$ be the Helmholtz free energy density and $D \ge 0$ the
dissipation. The Clausius–Duhem inequality requires

$$
D \;=\; \boldsymbol{\sigma} : \dot{\boldsymbol{\varepsilon}}
        \;-\; \dot{\Psi}
        \;-\; \frac{\mathbf{q} \cdot \nabla T}{T}
        \;\ge\; 0.
$$

The Landauer bound on logical erasure adds a lower bound on the energy
expended per bit of state collapse:

$$
\Delta Q_{\text{erase}} \;\ge\; k_{B} T \ln 2 \cdot \Delta H_{\text{state}},
$$

where $k_{B}$ is Boltzmann's constant and $\Delta H_{\text{state}}$ is
the change in Shannon entropy of the agent's state distribution.

The CBF in [`ai/cbf.rs`](../src/ai/cbf.rs) tracks an explicit energy
budget `available_credit_joules`, deducts the Landauer cost of every
information-gain step, and rejects any candidate transition whose
dissipation is negative or whose erasure cost exceeds the remaining
budget. The CBF is a runtime gate — bugs in it can let inadmissible
states through, which is why every dissipation-touching change requires
a regression test against a closed-form admissible/inadmissible pair.

## 5. The adjoint sensitivity method

The agent–manifold interaction is modelled as a continuous-time ODE

$$
\dot{\mathbf{z}}(t) \;=\; f_{\theta}\!\left(\mathbf{z}(t),\, t\right),
\qquad \mathbf{z}(0) = \mathbf{z}_0,
$$

with loss $L\!\left(\mathbf{z}(T)\right)$ at the terminal state. Reverse-mode
backpropagation through an explicit ODE solver would store every
intermediate activation, costing memory linear in the integration
horizon. The adjoint method
([Pontryagin 1962](https://en.wikipedia.org/wiki/Pontryagin%27s_minimum_principle);
[Chen et al. 2018](https://arxiv.org/abs/1806.07366))
recovers gradients by integrating a *backwards* ODE:

$$
\frac{d \mathbf{a}(t)}{d t}
  \;=\; -\,\mathbf{a}(t)^{\!\top}
        \frac{\partial f_{\theta}}{\partial \mathbf{z}},
\qquad
\frac{d L}{d \theta}
  \;=\; -\!\int_{T}^{0}
        \mathbf{a}(t)^{\!\top}
        \frac{\partial f_{\theta}}{\partial \theta}\, dt,
$$

with terminal condition
$\mathbf{a}(T) = \partial L / \partial \mathbf{z}(T)$.

The forward trajectory is *not* stored. Activation memory is therefore
$\mathcal{O}(1)$ in the number of integration steps; only parameters
$\theta$ and the rolling adjoint state $\mathbf{a}(t)$ live in memory.
The implementation is in [`ai/adjoint.rs`](../src/ai/adjoint.rs).

## 6. Cartridge interface

A *domain cartridge* is a struct that implements

```rust
pub trait IScienceCartridge<B: Backend> {
    fn compute_all(&self, mix: &MixTensor<B>) -> PhysicalResult<B>;
    fn compute_topology(
        &self,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> PhysicalResult<B>;
}
```

`PhysicalResult<B>` carries four 2-tensors of shape
`[Batch, N_active_voxels]`: `free_energy`, `dissipation`, `safety_margin`,
`cost`. The first two feed the CBF; the third feeds risk-aware control;
the fourth feeds multi-objective optimisation.

Authoring a new cartridge — concrete, polymers, alloys, mycelial
biomaterials — is exactly: implement these two methods. No other
changes to the manifold are required.

## 7. Symbols

| Symbol | Meaning |
|--------|---------|
| $G = (V, E)$ | oriented 1-skeleton |
| $B_1, B_2$ | vertex–edge and edge–face incidence matrices |
| $d, d^{*}$ | discrete exterior derivative and its adjoint |
| $\Delta_0 = d^{*} d$ | Hodge Laplacian on 0-cochains |
| $\Psi, D$ | Helmholtz free energy density, dissipation |
| $k_{B}, T$ | Boltzmann constant, absolute temperature |
| $\mathbf{z}, \mathbf{a}$ | ODE state and adjoint state |
| $\theta$ | learned parameters |

## References

- Crane, K., de Goes, F., Desbrun, M., Schröder, P. (2013).
  *Digital geometry processing with discrete exterior calculus.*
  ACM SIGGRAPH course notes.
- Hirani, A. N. (2003).
  *Discrete exterior calculus.* PhD thesis, Caltech.
- Chen, R. T. Q., Rubanova, Y., Bettencourt, J., Duvenaud, D. (2018).
  *Neural Ordinary Differential Equations.* NeurIPS.
- Pontryagin, L. S. (1962). *The Mathematical Theory of Optimal
  Processes.* Wiley-Interscience.
- Coleman, B. D., Noll, W. (1963). *The thermodynamics of elastic
  materials with heat conduction and viscosity.* Arch. Ration. Mech.
  Anal. 13, 167–178.
- Landauer, R. (1961). *Irreversibility and heat generation in the
  computing process.* IBM J. Res. Dev. 5, 183–191.
