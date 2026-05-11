// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Acoustic / elastodynamic wave propagation (Phase 8) — **scaffold**.
//!
//! Newmark-β time integration on the semi-discrete wave equation is the intended path; this module
//! pins the solver surface (time step, Newmark parameters, and tensor contracts) for DEC assembly
//! and coupling with [`crate::physics::laplacian`] / mechanics operators in later work.

use burn::tensor::{backend::Backend, Tensor};

/// Semi-discrete acoustic / elastic wave integrator (Newmark family).
///
/// [`Self::newmark_beta`] and [`Self::newmark_gamma`] select the implicitness / damping of the
/// average-acceleration scheme (e.g. average acceleration: β = ¼, γ = ½; linear acceleration:
/// β = ⅙, γ = ½).
pub struct AcousticWaveSolver {
    /// Physical time step Δt (seconds).
    pub dt: f32,
    /// Newmark β parameter (scheme-dependent; typical ¼ for unconditional stability with γ = ½).
    pub newmark_beta: f32,
    /// Newmark γ parameter (typically ½ for second-order accuracy).
    pub newmark_gamma: f32,
}

impl AcousticWaveSolver {
    /// One explicit documentation pass for the semi-discrete wave step (Phase 8 stub).
    ///
    /// # Tensor contracts (documentation)
    ///
    /// | Argument | Intended shape | Role |
    /// |----------|------------------|------|
    /// | `displacement` | `[B, N, 3]` | Nodal displacement **u** (m). |
    /// | `velocity` | `[B, N, 3]` | Nodal velocity **u̇** (m/s). |
    /// | `acceleration` | `[B, N, 3]` | Nodal acceleration **ü** (m/s²). |
    /// | `nodal_density` | `[B, N, 1]` | Mass density ρ (kg/m³) per node. |
    /// | `elasticity` | `[B, N, 3, 3]` | Nodal stiffness matrix **C** (Pa) mapping strain ↔ stress in the linearized elastic/acoustic law; anisotropy is encoded per node. |
    ///
    /// Returns updated `(displacement, velocity, acceleration)` with identical shapes.
    ///
    /// ## Default builds (`solver-experimental` **off**)
    /// Returns the inputs unchanged — documented no-op so default `cargo test` stays green.
    ///
    /// ## `--features solver-experimental`
    /// Implicit Newmark-β tensor step for **M ü + K u = 0** (homogeneous, no damping). Assumptions
    /// (lumped `a·ρ` mass diagonal, local `C·u` stiffness contraction, per-node 3×3 solve) are spelled
    /// out on the internal `step_wave_experimental` helper.
    #[allow(unused_variables)]
    pub fn step_wave<B: Backend<FloatElem = f32>>(
        &self,
        displacement: Tensor<B, 3>,
        velocity: Tensor<B, 3>,
        acceleration: Tensor<B, 3>,
        nodal_density: Tensor<B, 3>,
        elasticity: Tensor<B, 4>,
    ) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>) {
        #[cfg(not(feature = "acoustics-newmark"))]
        {
            let _ = (
                self.dt,
                self.newmark_beta,
                self.newmark_gamma,
                nodal_density.clone(),
                elasticity.clone(),
            );
            (displacement, velocity, acceleration)
        }

        #[cfg(feature = "acoustics-newmark")]
        {
            step_wave_experimental(
                self,
                displacement,
                velocity,
                acceleration,
                nodal_density,
                elasticity,
            )
        }
    }
}

#[cfg(feature = "acoustics-newmark")]
const LUMPED_MASS_SCALE: f32 = 1.0;

#[cfg(feature = "acoustics-newmark")]
const DET_EPS: f32 = 1e-30_f32;

#[cfg(feature = "acoustics-newmark")]
fn elem_bn33<B: Backend<FloatElem = f32>>(m: &Tensor<B, 4>, i: usize, j: usize) -> Tensor<B, 3> {
    m.clone().narrow(2, i, 1).narrow(3, j, 1).squeeze::<3>(3)
}

/// `(K u)[b,n,:] = Σ_j C[b,n,i,j] u[b,n,j]` with shapes `[B,N,3]`.
#[cfg(feature = "acoustics-newmark")]
fn contract_elasticity_displacement<B: Backend<FloatElem = f32>>(
    elasticity: Tensor<B, 4>,
    displacement: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let u_col = displacement.unsqueeze_dim::<4>(3);
    elasticity.matmul(u_col).squeeze::<3>(3)
}

#[cfg(feature = "acoustics-newmark")]
fn diagonal_mass_matrix_bn33<B: Backend<FloatElem = f32>>(
    m_scalar_bn1: Tensor<B, 3>,
    device: &B::Device,
) -> Tensor<B, 4> {
    let [b, n, one] = m_scalar_bn1.dims();
    debug_assert_eq!(one, 1);
    let zero = Tensor::<B, 3>::zeros([b, n, 1], device);
    let row0 = Tensor::cat(vec![m_scalar_bn1.clone(), zero.clone(), zero.clone()], 2);
    let row1 = Tensor::cat(vec![zero.clone(), m_scalar_bn1.clone(), zero.clone()], 2);
    let row2 = Tensor::cat(vec![zero.clone(), zero, m_scalar_bn1], 2);
    Tensor::cat(
        vec![
            row0.unsqueeze_dim::<4>(2),
            row1.unsqueeze_dim::<4>(2),
            row2.unsqueeze_dim::<4>(2),
        ],
        2,
    )
}

/// Returns `S^{-1} rhs` for batched 3×3 systems `S · x = rhs`, shapes `[B,N,3,3]` and `[B,N,3]`.
#[cfg(feature = "acoustics-newmark")]
fn solve_batched_3x3<B: Backend<FloatElem = f32>>(
    s: Tensor<B, 4>,
    rhs_bn3: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let s00 = elem_bn33(&s, 0, 0);
    let s01 = elem_bn33(&s, 0, 1);
    let s02 = elem_bn33(&s, 0, 2);
    let s10 = elem_bn33(&s, 1, 0);
    let s11 = elem_bn33(&s, 1, 1);
    let s12 = elem_bn33(&s, 1, 2);
    let s20 = elem_bn33(&s, 2, 0);
    let s21 = elem_bn33(&s, 2, 1);
    let s22 = elem_bn33(&s, 2, 2);

    let det = s00
        .clone()
        .mul(
            s11.clone()
                .mul(s22.clone())
                .sub(s12.clone().mul(s21.clone())),
        )
        .sub(
            s01.clone().mul(
                s10.clone()
                    .mul(s22.clone())
                    .sub(s12.clone().mul(s20.clone())),
            ),
        )
        .add(
            s02.clone().mul(
                s10.clone()
                    .mul(s21.clone())
                    .sub(s11.clone().mul(s20.clone())),
            ),
        );
    // Positive definite `S` ⇒ det > 0; clamp avoids division by zero if inputs degenerate.
    let det_safe = det.clone().clamp_min(DET_EPS);

    let cof00 = s11
        .clone()
        .mul(s22.clone())
        .sub(s12.clone().mul(s21.clone()));
    let cof01 = s02
        .clone()
        .mul(s21.clone())
        .sub(s01.clone().mul(s22.clone()));
    let cof02 = s01
        .clone()
        .mul(s12.clone())
        .sub(s02.clone().mul(s11.clone()));
    let cof10 = s12
        .clone()
        .mul(s20.clone())
        .sub(s10.clone().mul(s22.clone()));
    let cof11 = s00
        .clone()
        .mul(s22.clone())
        .sub(s02.clone().mul(s20.clone()));
    let cof12 = s10
        .clone()
        .mul(s02.clone())
        .sub(s00.clone().mul(s12.clone()));
    let cof20 = s10
        .clone()
        .mul(s21.clone())
        .sub(s20.clone().mul(s11.clone()));
    let cof21 = s01
        .clone()
        .mul(s20.clone())
        .sub(s00.clone().mul(s21.clone()));
    let cof22 = s00.mul(s11).sub(s01.mul(s10));

    let inv00 = cof00.div(det_safe.clone());
    let inv01 = cof01.div(det_safe.clone());
    let inv02 = cof02.div(det_safe.clone());
    let inv10 = cof10.div(det_safe.clone());
    let inv11 = cof11.div(det_safe.clone());
    let inv12 = cof12.div(det_safe.clone());
    let inv20 = cof20.div(det_safe.clone());
    let inv21 = cof21.div(det_safe.clone());
    let inv22 = cof22.div(det_safe);

    let row0 = Tensor::cat(vec![inv00, inv01, inv02], 2).unsqueeze_dim::<4>(2);
    let row1 = Tensor::cat(vec![inv10, inv11, inv12], 2).unsqueeze_dim::<4>(2);
    let row2 = Tensor::cat(vec![inv20, inv21, inv22], 2).unsqueeze_dim::<4>(2);
    let inv_s = Tensor::cat(vec![row0, row1, row2], 2);

    let rhs_col = rhs_bn3.unsqueeze_dim::<4>(3);
    inv_s.matmul(rhs_col).squeeze::<3>(3)
}

#[cfg(feature = "acoustics-newmark")]
/// Experimental-only integrator: implicit Newmark-β step for **M ü + K u = 0**.
///
/// **Modelling simplifications**
///
/// 1. **Lumped mass:** `M` is diagonal and isotropic per node,
///    `m_{[b,n]} = a · ρ_{[b,n,0]}` with [`LUMPED_MASS_SCALE`] as `a`. Nodal volume / quadrature
///    lumping is **not** assembled from DEC mesh data; callers fold that into `a` or into effective ρ.
/// 2. **Stiffness–displacement product:** `(K u)[b,n,:] = C[b,n,:,:] · u[b,n,:]` (batched matvec).
///    This stands in for a mesh-assembled sparse **K** from gradients / DEC; only the local 3×3
///    nodal operator is used.
/// 3. **Implicit acceleration solve:** effective Jacobian `S = m I + β Δt² C` is inverted **per node**
///    via closed-form 3×3 inverse; `det(S)` is clamped below by [`DET_EPS`] for numerical stability.
/// 4. **Homogeneous:** no Rayleigh damping, no external force term **f**.
fn step_wave_experimental<B: Backend<FloatElem = f32>>(
    solver: &AcousticWaveSolver,
    displacement: Tensor<B, 3>,
    velocity: Tensor<B, 3>,
    acceleration: Tensor<B, 3>,
    nodal_density: Tensor<B, 3>,
    elasticity: Tensor<B, 4>,
) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>) {
    let dt = solver.dt;
    let beta = solver.newmark_beta;
    let gamma = solver.newmark_gamma;
    let dt2 = dt * dt;
    let half_minus_beta = 0.5_f32 - beta;
    let acc_n = acceleration.clone();

    let device = displacement.device();
    let m_scalar = nodal_density.mul_scalar(LUMPED_MASS_SCALE);
    let m_mat = diagonal_mass_matrix_bn33(m_scalar, &device);

    // û = u_n + Δt v_n + Δt² (½−β) a_n  — appears inside K·û on the RHS of the acceleration equation.
    let u_tilde = displacement
        .clone()
        .add(velocity.clone().mul_scalar(dt))
        .add(acc_n.clone().mul_scalar(dt2 * half_minus_beta));
    let ku_tilde = contract_elasticity_displacement(elasticity.clone(), u_tilde);

    // (M + β Δt² K) a_{n+1} = −K û  (homogeneous; implicit Newmark on M ü + K u = 0).
    let s_mat = m_mat.add(elasticity.mul_scalar(beta * dt2));
    let rhs_acc = ku_tilde.mul_scalar(-1.0_f32);
    let acc_next = solve_batched_3x3(s_mat, rhs_acc);

    // u_{n+1} = u_n + Δt v_n + Δt² [(½−β) a_n + β a_{n+1}]
    let u_next = displacement.add(velocity.clone().mul_scalar(dt)).add(
        acc_n
            .clone()
            .mul_scalar(dt2 * half_minus_beta)
            .add(acc_next.clone().mul_scalar(dt2 * beta)),
    );

    // v_{n+1} = v_n + Δt [(1−γ) a_n + γ a_{n+1}]
    let v_next = velocity.add(
        acc_n
            .mul_scalar(dt * (1.0_f32 - gamma))
            .add(acc_next.clone().mul_scalar(dt * gamma)),
    );

    (u_next, v_next, acc_next)
}

// --- 1-D periodic bar (dispersion / energy benchmarks) -------------------------------------------

#[cfg(feature = "acoustics-newmark")]
#[derive(Debug, Clone)]
/// Implicit Newmark-β integrator for the **1-D periodic** continuum model `ρ ∂²u/∂t² = E ∂²u/∂x²`
/// on a uniform grid (central finite-difference stiffness, lumped nodal mass `m = ρ Δx`).
///
/// This path captures **spatial coupling** along the bar; it complements [`AcousticWaveSolver::step_wave`],
/// which applies a purely nodal 3×3 contraction intended for DEC / coupling hooks and does **not**
/// assemble a graph Laplacian.
///
/// **Discretisation:** `(K u)_i = (E/Δx²) (2 u_i − u_{i−1} − u_{i+1})` with periodic indices.
///
/// **Stability (documentation):** the average-acceleration Newmark pair `(β, γ) = (¼, ½)` is
/// unconditionally stable for the undamped second-order system. **Explicit** central-difference
/// companions require `Δt ≤ CFL · Δx / c` with `c = √(E/ρ)` and `CFL` typically in `(0, 1]`.
///
/// **Phase 3 / R2.4 note:** return-map checks advance by \(T = 2\pi/\Omega\) where \(\Omega^2=\lambda_K/m\)
/// for lumped mass \(m=\rho\Delta x\) on the periodic stencil. Using only the FD dispersion
/// \(\omega_{\mathrm{disp}}=(2c/\Delta x)|\sin(k\Delta x/2)|\) (without the \(1/\Delta x\) factor from the
/// mass lump) mis-times \(T\) on a fixed bar length and produces order-one relative \(L^2\) slip vs `u₀`
/// at large \(n\) — see `tests/verification/acoustics_plane_wave.rs` (`semi_discrete_omega` helper).
///
/// formal_anchor: Literature  
/// formal_citation: Newmark 1959; Hughes 2000, §9.1 (implicit Newmark for structural dynamics)
pub struct AcousticNewmarkBar1dPeriodic {
    /// Number of grid points (cells = `n` on `[0, L)` with periodic wrap).
    pub n: usize,
    /// Domain length `L` (m).
    pub length: f32,
    /// Young's modulus `E` (Pa).
    pub youngs_modulus: f32,
    /// Mass density `ρ` (kg/m³).
    pub density: f32,
    pub newmark_beta: f32,
    pub newmark_gamma: f32,
}

#[cfg(feature = "acoustics-newmark")]
#[derive(Debug)]
/// Workspace reused across [`AcousticNewmarkBar1dPeriodic::step`] calls (Cholesky factor for fixed `Δt`).
pub struct AcousticNewmarkBar1dWork {
    /// Lower-triangular Cholesky factor of `S = M + β Δt² K` in **f64** (row-major, lower triangle).
    chol: Vec<f64>,
    ku: Vec<f32>,
    u_tilde: Vec<f32>,
    rhs: Vec<f32>,
    sol_y: Vec<f64>,
    sol_x: Vec<f64>,
    last_dt: f32,
}

#[cfg(feature = "acoustics-newmark")]
impl AcousticNewmarkBar1dPeriodic {
    pub fn dx(&self) -> f32 {
        self.length / self.n as f32
    }

    pub fn wave_speed(&self) -> f32 {
        (self.youngs_modulus / self.density).sqrt()
    }

    pub fn workspace(&self) -> AcousticNewmarkBar1dWork {
        let n = self.n;
        AcousticNewmarkBar1dWork {
            chol: vec![0.0_f64; n * n],
            ku: vec![0.0_f32; n],
            u_tilde: vec![0.0_f32; n],
            rhs: vec![0.0_f32; n],
            sol_y: vec![0.0_f64; n],
            sol_x: vec![0.0_f64; n],
            last_dt: -1.0_f32,
        }
    }

    /// Total mechanical energy `½ Σ m u̇² + ½ uᵀ K u` for the discrete periodic bar.
    pub fn mechanical_energy(&self, u: &[f32], v: &[f32]) -> f32 {
        debug_assert_eq!(u.len(), self.n);
        debug_assert_eq!(v.len(), self.n);
        let dx = self.dx();
        let m = self.density * dx;
        let mut ke = 0.0_f32;
        for &vi in v {
            ke += 0.5_f32 * m * vi * vi;
        }
        self.elastic_energy(u) + ke
    }

    /// Elastic energy `½ uᵀ K u` with the periodic stiffness above.
    pub fn elastic_energy(&self, u: &[f32]) -> f32 {
        debug_assert_eq!(u.len(), self.n);
        let mut ku = vec![0.0_f32; self.n];
        apply_k_periodic_1d(u, self.youngs_modulus, self.dx(), self.n, &mut ku);
        0.5_f32 * dot(u, &ku)
    }

    /// Advance one implicit Newmark step (`M ü + K u = 0`, undamped).
    pub fn step(
        &self,
        ws: &mut AcousticNewmarkBar1dWork,
        dt: f32,
        u: &mut [f32],
        v: &mut [f32],
        a: &mut [f32],
    ) {
        debug_assert_eq!(u.len(), self.n);
        debug_assert_eq!(v.len(), self.n);
        debug_assert_eq!(a.len(), self.n);

        self.prepare(ws, dt);

        let beta = self.newmark_beta;
        let gamma = self.newmark_gamma;
        let dt2 = dt * dt;
        let half_minus_beta = 0.5_f32 - beta;

        let acc_n: Vec<f32> = a.to_vec();
        for i in 0..self.n {
            ws.u_tilde[i] = u[i] + dt * v[i] + dt2 * half_minus_beta * acc_n[i];
        }
        apply_k_periodic_1d(
            &ws.u_tilde,
            self.youngs_modulus,
            self.dx(),
            self.n,
            &mut ws.ku,
        );
        for i in 0..self.n {
            ws.rhs[i] = -ws.ku[i];
        }
        cholesky_solve_lower64(&ws.chol, self.n, &ws.rhs, &mut ws.sol_y, &mut ws.sol_x, a);

        for i in 0..self.n {
            u[i] += dt * v[i] + dt2 * (half_minus_beta * acc_n[i] + beta * a[i]);
        }
        for i in 0..self.n {
            v[i] += dt * ((1.0_f32 - gamma) * acc_n[i] + gamma * a[i]);
        }
    }

    fn prepare(&self, ws: &mut AcousticNewmarkBar1dWork, dt: f32) {
        if (ws.last_dt - dt).abs() <= 1e-12_f32.max(dt * 1e-7_f32) && ws.last_dt >= 0.0_f32 {
            return;
        }
        let dx = self.dx() as f64;
        let m_node = self.density as f64 * dx;
        let alpha =
            self.newmark_beta as f64 * (dt as f64).powi(2) * self.youngs_modulus as f64 / (dx * dx);
        fill_system_matrix_s64(&mut ws.chol, self.n, m_node, alpha);
        cholesky_decompose_lower64(&mut ws.chol, self.n).expect("SPD system matrix");
        ws.last_dt = dt;
    }
}

#[cfg(feature = "acoustics-newmark")]
fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

#[cfg(feature = "acoustics-newmark")]
fn apply_k_periodic_1d(u: &[f32], e: f32, dx: f32, n: usize, out: &mut [f32]) {
    let c = e / (dx * dx);
    for i in 0..n {
        let im = if i == 0 { n - 1 } else { i - 1 };
        let ip = if i + 1 == n { 0 } else { i + 1 };
        out[i] = c * (2.0_f32 * u[i] - u[im] - u[ip]);
    }
}

#[cfg(feature = "acoustics-newmark")]
fn fill_system_matrix_s64(mat: &mut [f64], n: usize, m: f64, alpha: f64) {
    debug_assert_eq!(mat.len(), n * n);
    mat.fill(0.0_f64);
    for i in 0..n {
        mat[i * n + i] = m + 2.0_f64 * alpha;
        let ip = (i + 1) % n;
        let im = (i + n - 1) % n;
        mat[i * n + ip] = -alpha;
        mat[i * n + im] = -alpha;
    }
}

#[cfg(feature = "acoustics-newmark")]
fn cholesky_decompose_lower64(a: &mut [f64], n: usize) -> Result<(), ()> {
    for i in 0..n {
        for j in 0..=i {
            let mut sum = a[i * n + j];
            for k in 0..j {
                sum -= a[i * n + k] * a[j * n + k];
            }
            if i == j {
                if sum <= 1e-30_f64 {
                    return Err(());
                }
                a[i * n + j] = sum.sqrt();
            } else {
                let d = a[j * n + j];
                if d.abs() < 1e-30_f64 {
                    return Err(());
                }
                a[i * n + j] = sum / d;
            }
        }
        for j in (i + 1)..n {
            a[i * n + j] = 0.0_f64;
        }
    }
    Ok(())
}

#[cfg(feature = "acoustics-newmark")]
fn cholesky_solve_lower64(
    l: &[f64],
    n: usize,
    b: &[f32],
    y: &mut [f64],
    x: &mut [f64],
    a_out: &mut [f32],
) {
    for i in 0..n {
        let mut s = b[i] as f64;
        for k in 0..i {
            s -= l[i * n + k] * y[k];
        }
        let d = l[i * n + i];
        y[i] = s / d;
    }
    for i in (0..n).rev() {
        let mut s = y[i];
        for k in (i + 1)..n {
            s -= l[k * n + i] * x[k];
        }
        let d = l[i * n + i];
        x[i] = s / d;
    }
    for i in 0..n {
        a_out[i] = x[i] as f32;
    }
}

#[cfg(all(test, feature = "acoustics-newmark"))]
mod cholesky_residual_tests {
    use super::*;

    fn matvec64(s: &[f64], n: usize, x: &[f64], y: &mut [f64]) {
        for i in 0..n {
            let mut sum = 0.0_f64;
            for j in 0..n {
                sum += s[i * n + j] * x[j];
            }
            y[i] = sum;
        }
    }

    fn residual_norm64(s: &[f64], n: usize, x: &[f64], b: &[f64]) -> f64 {
        let mut ax = vec![0.0_f64; n];
        matvec64(s, n, x, &mut ax);
        let mut r = 0.0_f64;
        for i in 0..n {
            let d = ax[i] - b[i];
            r += d * d;
        }
        r.sqrt()
    }

    #[test]
    fn cholesky_solve_matches_system_matrix() {
        let n = 32_usize;
        let dx = 1.0_f64 / n as f64;
        let m_node = 1.0_f64 * dx;
        let dt = 0.001_f64;
        let beta = 0.25_f64;
        let e = 1.0_f64;
        let alpha = beta * dt * dt * e / (dx * dx);
        let mut s = vec![0.0_f64; n * n];
        fill_system_matrix_s64(&mut s, n, m_node, alpha);
        let s_orig = s.clone();
        cholesky_decompose_lower64(&mut s, n).expect("cholesky");
        let mut b = vec![0.0_f32; n];
        b[0] = 1.0_f32;
        let mut y = vec![0.0_f64; n];
        let mut x = vec![0.0_f64; n];
        let mut x_out = vec![0.0_f32; n];
        cholesky_solve_lower64(&s, n, &b, &mut y, &mut x, &mut x_out);
        let mut b64 = vec![0.0_f64; n];
        b64[0] = 1.0_f64;
        let mut x64 = vec![0.0_f64; n];
        for i in 0..n {
            x64[i] = x_out[i] as f64;
        }
        let res = residual_norm64(&s_orig, n, &x64, &b64);
        assert!(
            res < 1e-8_f64,
            "Cholesky residual ||S x - b|| too large: {res}"
        );
    }

    #[test]
    fn cholesky_solve_matches_system_matrix_n128() {
        let n = 128_usize;
        let dx = 1.0_f64 / n as f64;
        let m_node = 1.0_f64 * dx;
        let dt = (1.0_f64 / n as f64) / 1000.0_f64;
        let beta = 0.25_f64;
        let e = 1.0_f64;
        let alpha = beta * dt * dt * e / (dx * dx);
        let mut s = vec![0.0_f64; n * n];
        fill_system_matrix_s64(&mut s, n, m_node, alpha);
        let s_orig = s.clone();
        cholesky_decompose_lower64(&mut s, n).expect("cholesky");
        let mut b = vec![0.0_f32; n];
        b[0] = 1.0_f32;
        let mut y = vec![0.0_f64; n];
        let mut x = vec![0.0_f64; n];
        let mut x_out = vec![0.0_f32; n];
        cholesky_solve_lower64(&s, n, &b, &mut y, &mut x, &mut x_out);
        let mut b64 = vec![0.0_f64; n];
        b64[0] = 1.0_f64;
        let mut x64 = vec![0.0_f64; n];
        for i in 0..n {
            x64[i] = x_out[i] as f64;
        }
        let res = residual_norm64(&s_orig, n, &x64, &b64);
        assert!(
            res < 1e-6_f64,
            "Cholesky residual n=128 ||S x - b|| too large: {res}"
        );
    }
}
