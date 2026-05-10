// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Frequency-domain Maxwell **curl–curl** on the DEC primal 1-skeleton (Phase 7): scaffold only.
//!
//! ## Target weak form (phasor notation, \(\partial_t \rightarrow i\omega\))
//! \[
//!   \nabla \times \mu^{-1} \nabla \times \mathbf{E} - \omega^2 \varepsilon \mathbf{E}
//!   = -i\omega \mathbf{J}
//! \]
//! with \(\omega = 2\pi f\) from [`PhotonicsSolver::frequency_hz`].
//!
//! ## Inputs / outputs (contract)
//! | Tensor | Shape | Role |
//! |--------|-------|------|
//! | `e_field` | `[B, N, 3]` | Electric field phasor \(\mathbf{E}\) at nodes |
//! | `relative_permittivity` | `[B, N, 1]` | \(\varepsilon_r\) (real; tensors / loss later) |
//! | `impressed_current` | `[B, N, 3]` | \(\mathbf{J}\) source |
//! | `edges_b1` | `[2, E]` | Undirected primal edges for DEC incidence / curl assembly |
//! | **return** | `[B, N, 3]` | Updated \(\mathbf{E}\) (today: passthrough stub) |
//!
//! Discrete operators will reuse DEC primitives (e.g. [`crate::physics::dec_primal`]) and topology
//! [`crate::physics::topology::EdgeTopology`]-compatible `edges_b1`.
//!
//! ## Deferred work (curl–curl on DEC)
//! 1. **Topology**: validated `edges_b1`, oriented incidence, optional dual data if needed for Hodge.
//! 2. **Operators**: discrete curl / weak curl–curl blocks from edge–node incidence (DEC \(d_1\), etc.).
//! 3. **Materials**: \(\varepsilon_r\), \(\mu_r^{-1}\) on the right discrete spaces (nodal vs edge).
//! 4. **Assembly**: sparse or structured matvec for \(K - \omega^2 M\) (or split operators + iterative solve).
//! 5. **Solve**: linear system or fixed-point relaxation; boundary / radiation BCs as extensions.
//!
//! ## Default builds (`solver-experimental` **off**)
//! [`PhotonicsSolver::solve_maxwell_curl_curl`] **passthrough**: returns `e_field` unchanged (`cargo test` green).
//!
//! ## `--features solver-experimental`
//! Same **passthrough** numerically; additionally compiles a tensor-shaped **zero residual** stub (same layout as
//! `e_field`) as a placeholder until full assembly lands — see [`maxwell_curl_curl_residual_zero_stub`].

use burn::tensor::{backend::Backend, Int, Tensor};

/// Phase 7 photonics driver: holds the **driving frequency** \(f\) (Hz) for phasor solves.
pub struct PhotonicsSolver {
    pub frequency_hz: f32,
}

impl PhotonicsSolver {
    /// Solve (or relax) the discrete curl–curl system for the electric field phasor.
    ///
    /// # Shapes (contract)
    /// - `e_field`: `[B, N, 3]` — electric field phasor components per node.
    /// - `relative_permittivity`: `[B, N, 1]` — relative permittivity \(\varepsilon_r\) (real part; extend later for tensors / loss).
    /// - `impressed_current`: `[B, N, 3]` — impressed current density \(\mathbf{J}\) (source term).
    /// - `edges_b1`: `[2, E]` — undirected edge pairs for the primal 1-skeleton (curl / gradient assembly).
    /// - Returns updated `e_field` `[B, N, 3]`.
    ///
    /// ## Default builds (`solver-experimental` **off**)
    /// Returns `e_field` unchanged (documented no-op / Phase 7 stub).
    ///
    /// ## `--features solver-experimental`
    /// Documented no-op: returns `e_field` unchanged until DEC curl–curl is implemented.
    #[allow(unused_variables)]
    pub fn solve_maxwell_curl_curl<B: Backend<FloatElem = f32>>(
        &self,
        e_field: Tensor<B, 3>,
        relative_permittivity: Tensor<B, 3>,
        impressed_current: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
    ) -> Tensor<B, 3> {
        #[cfg(not(feature = "photonics-scaffold"))]
        {
            e_field
        }

        #[cfg(feature = "photonics-scaffold")]
        {
            solve_maxwell_curl_curl_experimental(
                self,
                e_field,
                relative_permittivity,
                impressed_current,
                edges_b1,
            )
        }
    }
}

#[cfg(feature = "photonics-scaffold")]
fn solve_maxwell_curl_curl_experimental<B: Backend<FloatElem = f32>>(
    _solver: &PhotonicsSolver,
    e_field: Tensor<B, 3>,
    _relative_permittivity: Tensor<B, 3>,
    _impressed_current: Tensor<B, 3>,
    _edges_b1: Tensor<B, 2, Int>,
) -> Tensor<B, 3> {
    // Passthrough numerically: full DEC curl–curl assembly / solve not wired yet.
    let zero_residual = maxwell_curl_curl_residual_zero_stub(&e_field);
    e_field.add(zero_residual)
}

/// Experimental-only placeholder for the discrete curl–curl **residual** \(r(\mathbf{E})\).
///
/// Returns a tensor **shaped like** `e_field` (`[B, N, 3]`) filled with zeros — stand-in for
/// \((K - \omega^2 M)\mathbf{E} + i\omega\mathbf{J}\) once DEC operators are assembled.
#[cfg(feature = "photonics-scaffold")]
#[inline]
pub(crate) fn maxwell_curl_curl_residual_zero_stub<B: Backend<FloatElem = f32>>(
    e_field: &Tensor<B, 3>,
) -> Tensor<B, 3> {
    Tensor::<B, 3>::zeros_like(e_field)
}
