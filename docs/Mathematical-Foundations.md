<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Mathematical Foundations of the UMST Manifold

The UMST Manifold replaces traditional continuous field theory (Navier-Stokes, standard PDE solvers on dense grids) with a purely topological framework based on **Discrete Exterior Calculus (DEC)**.

## 1. The Cellular Sheaf

A material is fundamentally a network of interactions, not a block of continuous space. We represent the material as a 1-skeleton graph $G = (V, E)$, where:
- $V$ (Voxels) contain scalar material properties (temperature, mass, chemical potential).
- $E$ (Edges) contain flow properties (heat flux, stress vectors, damage).

The relationship between vertices and edges is mapped by the sparse Boundary Matrix $B_1$.

## 2. The Topological Laplacian

To compute how heat or stress moves through the material, we do not use 3D spatial convolutions. Instead, we compute the graph Laplacian:

$$L = B_1^T \cdot W \cdot B_1$$

Where $W$ is the diagonal matrix of edge weights. If a fracture occurs between two voxels, the corresponding edge weight $w_i \to 0$, mathematically severing the connection and perfectly redirecting the physical flow. This fundamentally solves the **Fracture Paradox** that plagues standard differentiable physics simulators.

## 3. The Adjoint Neural ODE

Because material evolution occurs over time, we model the system as a Neural Ordinary Differential Equation (ODE):

$$\frac{d \mathbf{S}}{dt} = f_\theta(\mathbf{S}, t)$$

Instead of using Backpropagation Through Time (BPTT)—which stores every intermediate state in GPU memory and quickly crashes—the `umst-manifold` uses the **Adjoint State Method**. We solve a secondary ODE backwards in time to recover exact gradients with $O(1)$ memory footprint.

## 4. The Thermodynamic Control Barrier (CBF)

Every transition in the manifold must satisfy the Second Law of Thermodynamics. The `ThermodynamicCBF` acts as a hard filter on the Neural ODE, enforcing **Landauer's Principle**:

$$Q \ge k_B T \ln 2 \cdot H_{extracted}$$

If a proposed topological mutation requires more free energy to "compute" than the material physically possesses in its local ledger, the transition is mathematically rejected.
