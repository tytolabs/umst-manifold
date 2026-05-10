// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Monolithic THMC coupling (Phase 5) — **orchestration skeleton**.
//!
//! Full design: implicit Euler + Newton on coupled residuals; sub-calls to transport
//! ([`crate::physics::protocols::ScalarTransport`]), mechanics ([`crate::physics::protocols::MechanicsEquilibrium`]),
//! fracture ([`super::fracture_field::PhaseFieldFractureSolver`]); cartridge supplies constitutive closures via
//! [`crate::core::traits::IScienceCartridge`].
//!
//! ## Newton residual checklist (implicit coupled step — target layout)
//! Coupled residuals \(R(U)\) for a monolithic implicit step would partition as:
//! - **\(R_T\) — thermal:** heat equation / transport residual for nodal temperature (diffusion + sources).
//! - **\(R_h\) — hydrologic:** moisture / pore-fluid proxy residual (transport + coupling).
//! - **\(R_u\) — mechanical:** equilibrium / momentum residual for displacement (stress divergence + body forces).
//! - **\(R_\alpha\) — chemical / hydration:** hydration degree evolution residual (kinetics vs stored \(\alpha\)).
//!
//! **Thermal / hydrologic Newton check:** after each iterate, \(R_T=\sum|T_{\mathrm{new}}-T_{\mathrm{old}}-\Delta t\,\mathrm{lap}_T|\),
//! \(R_h=\sum|h_{\mathrm{new}}-h_{\mathrm{old}}-\Delta t\,\mathrm{lap}_h|\) with \(T_{\mathrm{old}},h_{\mathrm{old}}\) at the
//! **start of that Newton iteration** (mechanics quasi-static, not in residual). With explicit Euler transport this sum is
//! \(\approx 0\) up to float noise, so convergence typically triggers after the first iterate. Mechanics remains a
//! **standalone** equilibrium solve per iterate; hydration uses **explicit** Euler on \(\alpha\) inside the loop; fracture
//! runs **after** the Newton loop.
//! Coupled Jacobians and cartridge closures remain future work.
//!
//! ## Experimental stepping (`solver-experimental`)
//! - **Transport:** [`crate::physics::laplacian::TopologicalLaplacian`] on temperature and humidity with the
//!   current nodal damage mask (non-zero coupling). Explicit Euler: \(U \leftarrow U + \Delta t\,\mathcal{L}(U)\).
//! - **Hydration \(\alpha\):** explicit Euler \(\alpha \leftarrow \mathrm{clip}_{[0,1]}\bigl(\alpha + \Delta t\,f(\alpha,T)\bigr)\)
//!   with Arrhenius-style placeholder \(f\) (`HYDRATION_ARRHENIUS_PREFACTOR_S`, `HYDRATION_ACTIVATION_ENERGY_J_PER_MOL`,
//!   `UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K`). Temperature is taken from **`state.thermal.temperature`** (first channel,
//!   broadcast to \(\alpha\) channels), interpreted as **absolute temperature in kelvin** for the exponential.
//! - **Mechanics:** [`crate::physics::mechanics::VectorMechanicsSolver`] uses [`crate::core::tensors::UnifiedMaterialStateTensor::node_positions`]
//!   (`[N,3]` **SI metres**) when `Some` and shape-valid; otherwise the equilibrium sub-solve is **skipped**.
//!   Integer [`crate::core::tensors::UnifiedMaterialStateTensor::coords`] remain sparse spacetime indices `[N,5]` only.
//! - **Fracture:** [`PhaseFieldFractureSolver::update_damage`] runs after the Newton loop. Strain is taken from
//!   `matrix_features[.., 0, ..]` when shapes align (`[N,F,3,3]` → `[B,N,3,3]`); otherwise strain is zero (documented),
//!   yielding zero tensile driving term until real strain is wired.

use burn::tensor::{backend::Backend, Tensor};

use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;

#[cfg(feature = "thmc-coupled")]
use crate::physics::laplacian::TopologicalLaplacian;
#[cfg(feature = "thmc-coupled")]
use crate::physics::mechanics::VectorMechanicsSolver;
#[cfg(feature = "thmc-coupled")]
use crate::physics::solvers::fracture_field::PhaseFieldFractureSolver;
#[cfg(feature = "thmc-coupled")]
use crate::physics::time_orchestration::MechanicsInnerLoopConfig;

/// Universal gas constant \(R\) for Arrhenius denominator (J·mol⁻¹·K⁻¹). CODATA-compatible float literal.
pub const UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K: f32 = 8.314_463_f32;

/// Placeholder activation energy \(E_a\) for hydration kinetics (J·mol⁻¹); not calibrated to a mix design.
pub const HYDRATION_ACTIVATION_ENERGY_J_PER_MOL: f32 = 40_000.0_f32;

/// Placeholder pre-exponential \(A\) in \(f(\alpha,T) = A\,\exp(-E_a/(RT))\,(1-\alpha)_+\) (s⁻¹).
pub const HYDRATION_ARRHENIUS_PREFACTOR_S: f32 = 1.0e-6_f32;

/// Minimum absolute temperature used in the Arrhenius denominator (K) to avoid blow-up at \(T\to 0\).
pub const HYDRATION_T_MIN_K: f32 = 250.0_f32;

/// Thermal plan: nodal temperature (and optional channels). Shape `[B, N, F_T]`.
pub struct ThermalPlan<B: Backend> {
    pub temperature: Tensor<B, 3>,
}

/// Hydrologic plan: humidity / pore-fluid proxy. Shape `[B, N, F_h]`.
pub struct HydrologicPlan<B: Backend> {
    pub humidity: Tensor<B, 3>,
}

/// Mechanical plan: displacement field. Shape `[B, N, 3]`.
pub struct MechanicalPlan<B: Backend> {
    pub displacement: Tensor<B, 3>,
}

/// Chemical / hydration kinetics plan. Shape `[B, N, F_α]`.
pub struct ChemicalPlan<B: Backend> {
    pub hydration_alpha: Tensor<B, 3>,
}

/// Coupled thermo-hydro-mechanical-chemical state: one tensor bundle per physics plan plus fracture and clock.
pub struct ThmcState<B: Backend> {
    pub thermal: ThermalPlan<B>,
    pub hydro: HydrologicPlan<B>,
    pub mechanical: MechanicalPlan<B>,
    pub chemical: ChemicalPlan<B>,
    /// Continuous damage on nodes, typically `[B, N, 1]` (fracture coupling).
    pub damage: Tensor<B, 3>,
    pub time: f32,
}

/// Newton / block solver controls for coupled stepping.
pub struct ThmcSolver {
    pub dt: f32,
    pub max_newton: usize,
    pub tol: f32,
}

impl ThmcSolver {
    /// One coupled THMC step using cartridge constitutive data.
    ///
    /// # Contract
    /// - Inner tensors `[B, N, …]` align with the active voxel / node count carried by `manifold`.
    /// - Intended to converge residuals \(\|R\| < tol\); adaptive `dt` halving is a follow-up.
    ///
    /// # Errors
    /// - Default builds (without `solver-experimental`): returns `Err` — do not call on production
    ///   hot paths unless the feature is enabled.
    /// - Experimental builds: returns `Err` on node-count mismatch between `state` and `manifold`.
    #[must_use = "THMC state advance must be consumed or propagated; ignoring the result drops the updated physics bundle"]
    pub fn step<B, C>(
        &self,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        #[cfg(feature = "thmc-coupled")]
        {
            self.step_experimental(cartridge, state, manifold)
        }
        #[cfg(not(feature = "thmc-coupled"))]
        {
            let _ = (self.dt, self.max_newton, self.tol);
            let _ = (cartridge, manifold);
            drop(state);
            Err(
                "ThmcSolver::step: thmc-coupled feature is disabled; enable `--features thmc-coupled` (or `solver-experimental` / `solver-tests` for all opt-in solvers), or do not call this entrypoint"
                    .to_string(),
            )
        }
    }

    /// Implicit-step Jacobian hook reserved for autodiff-backed Newton (experimental only).
    ///
    /// Not invoked by [`Self::step`]; exists to pin the `AutodiffBackend` bound for a future
    /// reverse-mode residual assembly without pulling autodiff into the default `Backend` path.
    #[cfg(feature = "thmc-coupled")]
    #[allow(dead_code)]
    fn _implicit_step<B, C>(
        &self,
        _cartridge: &C,
        _state: &ThmcState<B>,
        _manifold: &UnifiedMaterialStateTensor<B>,
    ) where
        B: Backend<FloatElem = f32> + burn::tensor::backend::AutodiffBackend,
        C: IScienceCartridge<B>,
    {
        // Placeholder: implicit Euler residual \(R(U) = U - U^n - \Delta t \, F(U)\) and `backward` on \(\|R\|^2\).
    }

    #[cfg(feature = "thmc-coupled")]
    fn step_experimental<B, C>(
        &self,
        _cartridge: &C,
        mut state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        let device = state.thermal.temperature.device();
        let batch = state.thermal.temperature.dims()[0];
        let n = state.thermal.temperature.dims()[1];
        let n_manifold = manifold.scalar_features.dims()[0];
        let edges_b1 = manifold.edges_b1.clone();

        if n != n_manifold {
            return Err(format!(
                "ThmcSolver::step: ThmcState thermal axis N={n} != manifold.scalar_features rows N={n_manifold}"
            ));
        }

        // Damage mask `[B,N,1]` for transport coefficients (last dim 1; otherwise first channel).
        let damage_m = match state.damage.dims()[2] {
            1 => state.damage.clone(),
            _ => state.damage.clone().slice([0..batch, 0..n, 0..1]),
        };

        let mut converged = false;
        let mut last_total_residual = 0.0_f32;

        // Newton outer loop: explicit transport + hydration + mechanics each iterate; exit when \(R_T+R_h < tol\).
        for _newton in 0..self.max_newton {
            let t_old = state.thermal.temperature.clone();
            let h_old = state.hydro.humidity.clone();

            // Topological diffusion: \(\Delta U\) with flux degraded by nodal damage on edges.
            let lap_t = TopologicalLaplacian::scalar_laplacian(
                t_old.clone(),
                edges_b1.clone(),
                damage_m.clone(),
            );
            let lap_h = TopologicalLaplacian::scalar_laplacian(
                h_old.clone(),
                edges_b1.clone(),
                damage_m.clone(),
            );

            let dt_lap_t = lap_t.mul_scalar(self.dt);
            let dt_lap_h = lap_h.mul_scalar(self.dt);

            // Explicit Euler thermal / hydrologic sub-step (coefficients absorbed into `dt` for this scaffold).
            state.thermal.temperature = t_old.clone().add(dt_lap_t.clone());
            state.hydro.humidity = h_old.clone().add(dt_lap_h.clone());

            // Hydration α: explicit Euler with Arrhenius-style placeholder f(α, T); clip to [0, 1].
            let f_alpha_ch = state.chemical.hydration_alpha.dims()[2];
            let t_bn1 = state
                .thermal
                .temperature
                .clone()
                .slice([0..batch, 0..n, 0..1]);
            let temperature_for_alpha = if f_alpha_ch == 1 {
                t_bn1
            } else {
                t_bn1.expand::<3, _>([batch, n, f_alpha_ch])
            };
            let d_alpha = hydration_arrhenius_rate_placeholder(
                state.chemical.hydration_alpha.clone(),
                temperature_for_alpha,
            );
            state.chemical.hydration_alpha = state
                .chemical
                .hydration_alpha
                .add(d_alpha.mul_scalar(self.dt))
                .clamp(0.0_f32, 1.0_f32);

            // Mechanics: bar-network equilibrium when an SI-metre embedding is supplied (`[N,3]`).
            if let Some(coords_n3) = manifold.node_positions.as_ref() {
                if coords_n3.dims() == [n, 3] {
                    let mask = manifold.displacement_bc_mask.clone();
                    let bm_core = match mask.dims()[..] {
                        [nn, 3, 1] if nn == n => mask.reshape([nn, 3]),
                        [1, nn, 3] if nn == n => {
                            mask.clone().slice([0..1, 0..n, 0..3]).reshape([nn, 3])
                        }
                        _ => {
                            return Err(format!(
                                "ThmcSolver::step: displacement_bc_mask dims {:?} incompatible with N={n} (expected [N,3,1] or [1,N,3])",
                                mask.dims()
                            ));
                        }
                    };
                    let bm = bm_core.unsqueeze_dim::<3>(0).expand::<3, _>([batch, n, 3]);
                    // Constitutive scaffold: uniform Young's modulus / Poisson pair until the cartridge threads
                    // heterogeneous stiffness into `ThmcState` or UMST feature banks.
                    let stiffness_e =
                        Tensor::<B, 3>::zeros([batch, n, 1], &device).add_scalar(30e9_f32);
                    let stiffness_nu =
                        Tensor::<B, 3>::zeros([batch, n, 1], &device).add_scalar(0.2_f32);
                    let stiffness = Tensor::cat(vec![stiffness_e, stiffness_nu], 2);
                    let bf = Tensor::<B, 3>::zeros([batch, n, 3], &device);
                    let inner_cfg = MechanicsInnerLoopConfig::default();
                    let cross_section_area = 0.01_f32;
                    let (u_new, _stress) = VectorMechanicsSolver::solve_equilibrium(
                        state.mechanical.displacement.clone(),
                        coords_n3.clone(),
                        stiffness,
                        bf,
                        edges_b1.clone(),
                        damage_m.clone(),
                        bm,
                        cross_section_area,
                        &inner_cfg,
                    );
                    state.mechanical.displacement = u_new;
                }
            }

            // Residuals \(R_T = \sum|T_{\mathrm{new}}-T_{\mathrm{old}}-\Delta t\,\mathrm{lap}_T|\), same for \(h\) (mechanics quasi-static).
            let r_t = state
                .thermal
                .temperature
                .clone()
                .sub(t_old)
                .sub(dt_lap_t)
                .abs()
                .sum();
            let r_h = state
                .hydro
                .humidity
                .clone()
                .sub(h_old)
                .sub(dt_lap_h)
                .abs()
                .sum();
            last_total_residual = r_t.into_scalar() + r_h.into_scalar();

            if last_total_residual < self.tol {
                converged = true;
                break;
            }
        }

        if !converged {
            eprintln!(
                "warning: ThmcSolver: Newton exhausted max_newton={} without meeting tol={}; last total_residual (R_T+R_h)={}",
                self.max_newton, self.tol, last_total_residual
            );
        }

        // Phase-field fracture: strain from first matrix feature slice, or zeros (see module docs).
        let strain = strain_tensor_from_manifold::<B>(manifold, batch, n, &device);
        let gc = Tensor::<B, 3>::ones([batch, n, 1], &device);
        let fracture = PhaseFieldFractureSolver { length_scale: 1.0 };

        let d_last = state.damage.dims()[2];
        let damage_core = match d_last {
            1 => state.damage.clone(),
            _ => state.damage.clone().slice([0..batch, 0..n, 0..1]),
        };
        let damage_new = fracture.update_damage(strain, damage_core, gc, edges_b1.clone());

        state.damage = if d_last == 1 {
            damage_new
        } else {
            let tail = state.damage.slice([0..batch, 0..n, 1..d_last]);
            Tensor::cat(vec![damage_new, tail], 2)
        };

        state.time += self.dt;
        Ok(state)
    }
}

/// Arrhenius-style hydration rate **placeholder** \(f(\alpha,T) = A\,\exp\!\bigl(-E_a/(R\,T)\bigr)\,(1-\alpha)_+\)
/// with [`HYDRATION_ARRHENIUS_PREFACTOR_S`], [`HYDRATION_ACTIVATION_ENERGY_J_PER_MOL`],
/// [`UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K`], [`HYDRATION_T_MIN_K`].
///
/// `temperature_k` must match `alpha` in `[B, N, *]`; \(T\) is treated as **absolute temperature (K)** on the
/// same mesh as `alpha`.
#[cfg(feature = "thmc-coupled")]
fn hydration_arrhenius_rate_placeholder<B: Backend<FloatElem = f32>>(
    alpha: Tensor<B, 3>,
    temperature_k: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let device = alpha.device();
    let shape = alpha.dims();
    let ones = Tensor::<B, 3>::ones(shape, &device);
    let one_minus_a = ones.sub(alpha).clamp_min(0.0_f32);
    let t_safe = temperature_k.clamp_min(HYDRATION_T_MIN_K);
    let ea_over_rt = Tensor::<B, 3>::zeros(shape, &device)
        .add_scalar(HYDRATION_ACTIVATION_ENERGY_J_PER_MOL)
        .div(t_safe.mul_scalar(UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K));
    ea_over_rt
        .mul_scalar(-1.0_f32)
        .exp()
        .mul(one_minus_a)
        .mul_scalar(HYDRATION_ARRHENIUS_PREFACTOR_S)
}

/// Symmetric strain \(\varepsilon\) per node for [`PhaseFieldFractureSolver::update_damage`].
///
/// Prefers `manifold.matrix_features[.., 0, ..]` reshaped to `[B, N, 3, 3]`. If `N` or channel count disagrees,
/// returns zeros (still runs AT2 relaxation with zero tensile drive).
#[cfg(feature = "thmc-coupled")]
fn strain_tensor_from_manifold<B: Backend<FloatElem = f32>>(
    manifold: &UnifiedMaterialStateTensor<B>,
    batch: usize,
    n: usize,
    device: &B::Device,
) -> Tensor<B, 4> {
    let d = manifold.matrix_features.dims();
    if d[0] == n && d[1] >= 1 {
        manifold
            .matrix_features
            .clone()
            .slice([0..n, 0..1, 0..3, 0..3])
            .reshape([1, n, 3, 3])
            .expand([batch, n, 3, 3])
    } else {
        Tensor::<B, 4>::zeros([batch, n, 3, 3], device)
    }
}
