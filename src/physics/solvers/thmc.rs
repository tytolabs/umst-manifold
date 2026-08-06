// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Monolithic THMC coupling — operator-split **`ThmcSolver::step`** with optional implicit Newton blocks.
//!
//! **Solver status:** [`docs/Solver-Status.md`](../../../docs/Solver-Status.md) — open-index **THMC** bullet and
//! table row `solvers::thmc` (`ThmcSolver`, feature `thmc-coupled`, `tests/verification/thmc_drying_shrinkage.rs`,
//! `tests/verification/thmc_monolithic_newton_chain.rs` — Track **G** / Phase **4A** stacked Newton on a 1D chain).
//! What is verified there vs. placeholder/open here stays in that file; this module is the implementation anchor.
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
//! - **\(R_\alpha\) — chemical / reaction extent:** reaction extent degree evolution residual (kinetics vs stored \(\alpha\)).
//!
//! **Monolithic Newton (Phase 5 — partial, small graph only):** when [`ThmcSolver::monolithic_thmc_newton`] is `Some`,
//! each outer pass replaces the split \((T,\alpha)\to h\to u\) sequence with **dense** damped Newton on the
//! backward-Euler stacked unknowns including quasi-static \(R_u\)
//! (`ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u`) **only
//! while** field-major stacked DOFs \(\le\) [`crate::physics::solvers::THMC_DENSE_NEWTON_MAX_STACKED_DOFS`] (**64**,
//! unified post-**`3394b96`** — see [`docs/Solver-Status.md`](../../../docs/Solver-Status.md) §THMC and matrix **#8**).
//! **No** shipped path performs dense Newton (or a dense Jacobian solve) **above** that cap. At production \(N\), the
//! roadmap is **sparse or matrix-free Jacobians**, **Krylov–JFNK**, **AD-safe** residual-norm **‖R‖** exit criteria,
//! and adaptive `dt` — not a larger dense stack. Without monolithic config, [`ThmcSolver::step`] is an
//! **operator split** per outer `max_newton` pass: **(1)** advance **\(T\)** and **reaction extent \(\alpha\)** (explicit
//! thermal Laplacian + exothermic coupling, or opt-in backward-Euler **\((T,\alpha)\)** damped Newton), **(2)**
//! advance **humidity \(h\)** (topological Laplacian + optional tail drying), **(3)** quasi-static **bar \(u\)** when
//! `[N,3]` `node_positions` and the BC mask allow — then repeat for diagnostics; **after** the outer loop,
//! **once**, [`super::fracture_field::PhaseFieldFractureSolver::update_damage`]. Transport Laplacians use **`damage` frozen at step
//! entry** for every pass (no within-step \(u\!\leftrightarrow\!d\) stagger). This is **not** a Jacobian–Newton
//! solve on the fully coupled residual. **Opt-in (feature `thmc-coupled`):** set
//! [`ThmcSolver::implicit_t_alpha_newton`] to replace the explicit \(T\) + \(\alpha\) updates **per outer pass**
//! with multi-step damped Newton on the backward-Euler \((T,\alpha)\) block (`ThmcImplicitEulerThermalReactionExtentResidual`).
//! Default **`None`** preserves the legacy split. **Milestone:** the
//! `ThmcImplicitEulerThermalReactionExtentResidual` type in `thmc_residual.rs` assembles the backward-Euler
//! residual for the **\(T\)–\(\alpha\)** block only; see `tests/verification/thmc_drying_shrinkage.rs`.
//!
//! **Thermal / hydrologic residual tensors (diagnostic):** after each outer pass,
//! \(R_T=|T_{\mathrm{new}}-T_{\mathrm{old}}-\Delta t\,\mathrm{lap}_T|\),
//! \(R_h=|h_{\mathrm{new}}-h_{\mathrm{old}}-\Delta t\,\mathrm{lap}_h|\) (nodal tensors; **no** `.into_scalar()` reduction on the
//! hot path). Early exit on \(\|R\|<\) `tol` would require a device sync — omitted for autodiff-safe control flow
//! (see [`docs/FP_CATEGORICAL_BURN.md`](../../../docs/FP_CATEGORICAL_BURN.md) § *Hotspots* — operator-split THMC vs CG helper).
//! Mechanics remains a **standalone** equilibrium solve per pass (after \(h\)); **\(\alpha\)** is updated in the
//! same sub-step as \(T\) (explicit or implicit BE block). Fracture runs **once** after all outer passes.
//! Coupled Jacobians and cartridge closures remain future work. **No** global finite-difference or AD
//! Jacobian is assembled on [`ThmcSolver::step`]; `max_newton` only repeats the same operator-split pattern
//! (diagnostic residuals only—no full THMC Newton correction) **unless** [`ThmcSolver::implicit_t_alpha_newton`] or
//! [`ThmcSolver::monolithic_thmc_newton`] is `Some` — see below. The optional dense Jacobian for \((T,\alpha)\)
//! applies only when [`ThmcSolver::implicit_t_alpha_newton`] is `Some` (feature `thmc-coupled`). **Phase 5:** when
//! [`ThmcSolver::monolithic_thmc_newton`] is `Some`, each outer pass runs
//! `ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u` instead of the
//! split \((T,\alpha)\to h\to u\) sequence (small graphs only — see struct rustdoc).
//!
//! **Calibration surface (cross-ref Solver-Status THMC):** [`ReactionExtentKinetics`] bundles Arrhenius /
//! exothermic / T-boost / mechanics **E** scale defaults (same shipped numbers as the legacy
//! `REACTION_EXTENT_*` module constants). Cartridge-backed calibration and the open monolithic Jacobian
//! remain future work.
//!
//! ## Coupled stepping (`thmc-coupled`; also enabled via `solver-research` / `solver-experimental` meta-features)
//! Per outer pass (see ordering above): **`damage` for Laplacian weights is fixed at step entry** (not updated between passes).
//! - **\(T\) and reaction extent \(\alpha\) (first):** thermal [`crate::physics::laplacian::TopologicalLaplacian`] on \(T\) with that mask;
//!   exothermic heat \(\propto \dot\alpha\); then either explicit Euler on \(T\) and explicit \(\alpha \leftarrow \mathrm{clip}_{\[0,1\]}\bigl(\alpha + \Delta t\,f(\alpha,T)\bigr)\)
//!   with Arrhenius-style \(f\) (`REACTION_EXTENT_ARRHENIUS_PREFACTOR_S`, …), or opt-in backward-Euler Newton on the coupled \((T,\alpha)\) block only.
//!   Temperature driving \(f\) uses **`state.thermal.temperature`** (first channel, broadcast), **kelvin**.
//! - **Humidity \(h\) (second):** same Laplacian on \(h\) with the **same** step-entry damage mask; explicit Euler plus optional tail drying.
//! - **Mechanics \(u\) (third):** [`crate::physics::mechanics::VectorMechanicsSolver`] when [`crate::core::tensors::UnifiedMaterialStateTensor::node_positions`]
//!   is `[N,3]` **SI metres** and shape-valid; otherwise the equilibrium sub-solve is **skipped**.
//!   Integer [`crate::core::tensors::UnifiedMaterialStateTensor::coords`] remain sparse spacetime indices `[N,5]` only.
//! - **Fracture:** [`super::fracture_field::PhaseFieldFractureSolver::update_damage`] runs after the outer Newton loop. When SI
//!   [`UnifiedMaterialStateTensor::node_positions`] are present as `[N,3]` (same `N` as state) and the
//!   displacement BC mask is compatible so the bar equilibrium sub-step runs, strain fed to fracture is
//!   `strain_tensor_from_bar_network_displacement` built from
//!   **`state.mechanical.displacement`** and those coordinates (post-mechanics \(\varepsilon(\mathbf u)\)).
//!   If positions are missing or not `[N,3]`, strain falls back to `matrix_features[.., 0, ..]` when shapes
//!   align (`[N,F,3,3]` → `[B,N,3,3]`); otherwise zeros — same rule as
//!   `strain_tensor_for_fracture_from_manifold` (public stub for cartridges / tests).
//!
//! # Honest boundary (W29-081)
//!
//! Operator-split THMC + opt-in dense Newton (stacked DOFs ≤
//! [`crate::physics::solvers::THMC_DENSE_NEWTON_MAX_STACKED_DOFS`]) are research/verification surfaces
//! under feature `thmc-coupled`. Contracts are exercised by `cargo test -p umst-manifold thmc`.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`, not OP-5.

/// W29 deepen cell — THMC solver honest fence bundle.
pub const W29_THMC_DEEPEN_CELL: &str = "W29-081-THMC";

/// Honest posture tag — operator-split / capped dense Newton research lane.
pub const THMC_POSTURE_TAG: &str = "honest-thmc-operator-split-research-lane";

/// Honest physics posture — unit/verification contracts pass; does not certify fleet physics GREEN.
pub const THMC_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by this module alone.
pub const THMC_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const THMC_MASTER: bool = false;

/// OP-5 fleet claim — refused at this deepen fence.
pub const THMC_OP5_CLAIMED: bool = false;

/// Whether the operator-split + optional dense Newton surfaces are landed in this module.
pub const THMC_SOLVER_SURFACE_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const THMC_HONEST_FENCE: &str =
    "thmc_solver_landed=true operator_split_wired=true dense_newton_capped=true production_wired=false physics_green=false master=false op5=false";

/// Typed probe for THMC solver posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThmcPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5_claimed: bool,
    pub solver_surface_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the THMC solver module.
#[must_use]
pub fn thmc_honest_posture_bundle() -> ThmcPostureProbe {
    ThmcPostureProbe {
        physics_green: THMC_PHYSICS_GREEN,
        production_wired: THMC_PRODUCTION_WIRED,
        master: THMC_MASTER,
        op5_claimed: THMC_OP5_CLAIMED,
        solver_surface_landed: THMC_SOLVER_SURFACE_LANDED,
        honest_fence: THMC_HONEST_FENCE,
        posture_tag: THMC_POSTURE_TAG,
        deepen_cell: W29_THMC_DEEPEN_CELL,
    }
}

/// THMC surface landed with production/master/GREEN/OP-5 honestly open.
#[must_use]
pub fn thmc_posture_honest(probe: &ThmcPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5_claimed
        && probe.solver_surface_landed
        && probe.honest_fence.contains("thmc_solver_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
}

#[cfg(feature = "thmc-coupled")]
use burn::tensor::Int;
use burn::tensor::{backend::Backend, Tensor};

use crate::core::field::{
    DamageField, DisplacementField, Field, HumidityField, ReactionExtentField, TemperatureField,
};
#[cfg(feature = "thmc-coupled")]
use crate::core::field::StiffnessField;
use crate::core::material_phase::ThmcEnvelope;
#[cfg(feature = "thmc-coupled")]
use crate::core::material_phase::MaterialPhaseKind;
#[cfg(feature = "thmc-coupled")]
use crate::core::field::StepEntryDamageMask;
use crate::core::material_transition::ReactionExtentKineticsSpec;
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;

use crate::physics::error::PhysicsError;
#[cfg(feature = "thmc-coupled")]
use crate::physics::laplacian::TopologicalLaplacian;
#[cfg(feature = "thmc-coupled")]
use crate::physics::solvers::thmc_residual::{
    ThmcMonolithicImplicitUnknownLayout, THMC_DENSE_NEWTON_MAX_STACKED_DOFS,
};
#[cfg(feature = "thmc-coupled")]
use crate::physics::solvers::thmc_split_passes::{
    newton_split_chain, transport_residual_l2, ThmcNewtonScratch, ThmcPhaseRoute, ThmcStepCtx,
};

/// Bundles reaction extent kinetics and the **uncalibrated** mechanics stiffness scale used in [`ThmcSolver::step`].
///
/// Defaults match the legacy module constants; override per solver instance for mix-specific calibration.
#[derive(Clone, Debug)]
pub struct ReactionExtentKinetics {
    pub arrhenius_prefactor_s: f32,
    pub activation_energy_j_per_mol: f32,
    pub gas_constant_j_per_mol_k: f32,
    pub t_min_k: f32,
    pub t_boost_ref_k: f32,
    pub t_boost_per_k: f32,
    pub exothermic_k_per_alpha_rate: f32,
    /// Young’s-modulus scale \(E \propto \alpha\) multiplier (Pa) at full reaction extent.
    pub stiffness_e_scale_pa: f32,
    pub stiffness_nu: f32,
}

impl Default for ReactionExtentKinetics {
    fn default() -> Self {
        Self::from_spec(ReactionExtentKineticsSpec::substrate_neutral())
    }
}

impl From<ReactionExtentKineticsSpec> for ReactionExtentKinetics {
    fn from(spec: ReactionExtentKineticsSpec) -> Self {
        Self::from_spec(spec)
    }
}

impl ReactionExtentKinetics {
    #[must_use]
    pub fn from_spec(spec: ReactionExtentKineticsSpec) -> Self {
        Self {
            arrhenius_prefactor_s: spec.arrhenius_prefactor_s,
            activation_energy_j_per_mol: spec.activation_energy_j_per_mol,
            gas_constant_j_per_mol_k: spec.gas_constant_j_per_mol_k,
            t_min_k: spec.t_min_k,
            t_boost_ref_k: spec.t_boost_ref_k,
            t_boost_per_k: spec.t_boost_per_k,
            exothermic_k_per_alpha_rate: spec.exothermic_k_per_alpha_rate,
            stiffness_e_scale_pa: spec.stiffness_e_scale_pa,
            stiffness_nu: spec.stiffness_nu,
        }
    }

    /// Scalar Arrhenius rate \(f(\alpha,T)\) (1/s) matching the tensor path in `reaction_extent_arrhenius_rate`.
    #[must_use]
    pub fn alpha_rate_scalar(&self, alpha: f32, temperature_k: f32) -> f32 {
        let one_m = (1.0_f32 - alpha).max(0.0_f32);
        let t = temperature_k.max(self.t_min_k);
        let ea_rt = self.activation_energy_j_per_mol / (self.gas_constant_j_per_mol_k * t);
        let arr = self.arrhenius_prefactor_s * (-ea_rt).exp() * one_m;
        let boost =
            1.0_f32 + self.t_boost_per_k * (temperature_k - self.t_boost_ref_k).max(0.0_f32);
        arr * boost
    }

    /// Project runtime kinetics into the cartridge spec witness (stiffness centralizer input).
    #[must_use]
    pub fn as_kinetics_spec(&self) -> ReactionExtentKineticsSpec {
        ReactionExtentKineticsSpec {
            arrhenius_prefactor_s: self.arrhenius_prefactor_s,
            activation_energy_j_per_mol: self.activation_energy_j_per_mol,
            gas_constant_j_per_mol_k: self.gas_constant_j_per_mol_k,
            t_min_k: self.t_min_k,
            t_boost_ref_k: self.t_boost_ref_k,
            t_boost_per_k: self.t_boost_per_k,
            exothermic_k_per_alpha_rate: self.exothermic_k_per_alpha_rate,
            stiffness_e_scale_pa: self.stiffness_e_scale_pa,
            stiffness_nu: self.stiffness_nu,
        }
    }

    /// XS-3 step 5: typed [`StiffnessField`] centralizer for THMC mechanics fan-in.
    ///
    /// `alpha_bn1` must be the clipped `[B, N, 1]` reaction-extent channel (caller-owned slice/clamp).
    #[cfg(feature = "thmc-coupled")]
    #[must_use]
    pub fn stiffness_field_from_alpha_bn1<B: Backend<FloatElem = f32>>(
        &self,
        alpha_bn1: Tensor<B, 3>,
        device: &B::Device,
    ) -> StiffnessField<B> {
        StiffnessField::from_alpha_kinetics(alpha_bn1, &self.as_kinetics_spec(), device)
    }
}

/// Universal gas constant \(R\) for Arrhenius denominator (J·mol⁻¹·K⁻¹). CODATA-compatible float literal.
pub const UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K: f32 = 8.314_463_f32;

/// Thermal plan: nodal temperature (and optional channels). Shape `[B, N, F_T]`.
#[derive(Clone, Debug)]
pub struct ThermalPlan<B: Backend> {
    pub temperature: TemperatureField<B>,
}

/// Hydrologic plan: humidity / pore-fluid proxy. Shape `[B, N, F_h]`.
#[derive(Clone, Debug)]
pub struct HydrologicPlan<B: Backend> {
    pub humidity: HumidityField<B>,
}

/// Mechanical plan: displacement field. Shape `[B, N, 3]`.
#[derive(Clone, Debug)]
pub struct MechanicalPlan<B: Backend> {
    pub displacement: DisplacementField<B>,
}

/// Chemical / reaction extent kinetics plan. Shape `[B, N, F_α]`.
#[derive(Clone, Debug)]
pub struct ChemicalPlan<B: Backend> {
    pub reaction_extent: ReactionExtentField<B>,
}

/// Coupled thermo-hydro-mechanical-chemical state: one tensor bundle per physics plan plus fracture and clock.
#[derive(Clone, Debug)]
pub struct ThmcState<B: Backend> {
    pub thermal: ThermalPlan<B>,
    pub hydro: HydrologicPlan<B>,
    pub mechanical: MechanicalPlan<B>,
    pub chemical: ChemicalPlan<B>,
    /// Continuous damage on nodes, typically `[B, N, 1]` (fracture coupling).
    pub damage: DamageField<B>,
    pub time: f32,
}

/// Destructured THMC field tensors plus simulation time.
type ThmcTensorBundle<B> = (
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    f32,
);

impl<B: Backend> ThermalPlan<B> {
    #[inline]
    #[must_use]
    pub fn from_temperature(tensor: Tensor<B, 3>) -> Self {
        Self { temperature: Field::new(tensor) }
    }
    #[deprecated(since = "0.2.0", note = "use .temperature.as_tensor() — FP P3.1 migration")]
    #[inline]
    pub fn temperature_tensor(&self) -> &Tensor<B, 3> {
        self.temperature.as_tensor()
    }
}

impl<B: Backend> HydrologicPlan<B> {
    #[inline]
    #[must_use]
    pub fn from_humidity(tensor: Tensor<B, 3>) -> Self {
        Self { humidity: Field::new(tensor) }
    }
    #[deprecated(since = "0.2.0", note = "use .humidity.as_tensor() — FP P3.1 migration")]
    #[inline]
    pub fn humidity_tensor(&self) -> &Tensor<B, 3> {
        self.humidity.as_tensor()
    }
}

impl<B: Backend> MechanicalPlan<B> {
    #[inline]
    #[must_use]
    pub fn from_displacement(tensor: Tensor<B, 3>) -> Self {
        Self { displacement: Field::new(tensor) }
    }
    #[deprecated(since = "0.2.0", note = "use .displacement.as_tensor() — FP P3.1 migration")]
    #[inline]
    pub fn displacement_tensor(&self) -> &Tensor<B, 3> {
        self.displacement.as_tensor()
    }
}

impl<B: Backend> ChemicalPlan<B> {
    #[inline]
    #[must_use]
    pub fn from_reaction_extent(tensor: Tensor<B, 3>) -> Self {
        Self { reaction_extent: Field::new(tensor) }
    }
    #[deprecated(since = "0.2.0", note = "use .reaction_extent.as_tensor() — FP P3.1 migration")]
    #[inline]
    pub fn reaction_extent_tensor(&self) -> &Tensor<B, 3> {
        self.reaction_extent.as_tensor()
    }
}

impl<B: Backend> ThmcState<B> {
    #[must_use]
    pub fn from_tensors(
        temperature: Tensor<B, 3>,
        humidity: Tensor<B, 3>,
        displacement: Tensor<B, 3>,
        reaction_extent: Tensor<B, 3>,
        damage: Tensor<B, 3>,
        time: f32,
    ) -> Self {
        Self {
            thermal: ThermalPlan::from_temperature(temperature),
            hydro: HydrologicPlan::from_humidity(humidity),
            mechanical: MechanicalPlan::from_displacement(displacement),
            chemical: ChemicalPlan::from_reaction_extent(reaction_extent),
            damage: Field::new(damage),
            time,
        }
    }

    #[must_use]
    pub fn into_thmc_tensors(
        self,
    ) -> ThmcTensorBundle<B> {
        (
            self.thermal.temperature.into_tensor(),
            self.hydro.humidity.into_tensor(),
            self.mechanical.displacement.into_tensor(),
            self.chemical.reaction_extent.into_tensor(),
            self.damage.into_tensor(),
            self.time,
        )
    }

    #[deprecated(since = "0.2.0", note = "use .damage.as_tensor() — FP P3.1 migration")]
    #[inline]
    pub fn damage_tensor(&self) -> &Tensor<B, 3> {
        self.damage.as_tensor()
    }

    /// Hydrate scalar THMC plan fields from UMST [`UnifiedMaterialStateTensor::typed_views`].
    ///
    /// Inverse of [`crate::physics::thmc_umst_sync::sync_thmc_to_umst`] on temperature, humidity,
    /// and damage. Displacement, reaction extent, and time are taken from `template` when provided;
    /// otherwise displacement and reaction extent default to zeros and time to `0.0` (UMST scalar
    /// columns do not carry those plans in P3.6).
    pub fn hydrate_from_umst_typed_views(
        umst: &UnifiedMaterialStateTensor<B>,
        template: Option<&Self>,
    ) -> Result<Self, PhysicsError> {
        let views = umst.typed_views().map_err(|e| PhysicsError::Domain {
            detail: format!("hydrate_from_umst_typed_views: typed_views: {e:?}"),
        })?;
        let n = umst.scalar_features.dims()[0];
        let device = umst.scalar_features.device();
        let (displacement, reaction_extent, time) = match template {
            Some(t) => (
                t.mechanical.displacement.clone(),
                t.chemical.reaction_extent.clone(),
                t.time,
            ),
            None => (
                Field::new(Tensor::<B, 3>::zeros([1, n, 3], &device)),
                Field::new(Tensor::<B, 3>::zeros([1, n, 1], &device)),
                0.0_f32,
            ),
        };
        Ok(Self {
            thermal: ThermalPlan {
                temperature: views.temperature,
            },
            hydro: HydrologicPlan {
                humidity: views.humidity,
            },
            mechanical: MechanicalPlan { displacement },
            chemical: ChemicalPlan { reaction_extent },
            damage: views.damage,
            time,
        })
    }
}

/// Coupled **thermo–hydro–mechanical–chemical** stepper for one material graph (`thmc-coupled`).
///
/// [`Self::step`] advances [`ThmcState`] on a [`UnifiedMaterialStateTensor`] topology: by default an
/// **operator split** (thermal + reaction extent, then humidity, then optional bar equilibrium, then
/// fracture damage once). Optional Newton paths replace parts of that split — see
/// [`Self::implicit_t_alpha_newton`] and [`Self::monolithic_thmc_newton`].
///
/// **Fail-fast guards (monolithic branch):** when [`Self::monolithic_thmc_newton`] is `Some`,
/// [`Self::step`] returns `Err` before any inner solve if `batch != 1`, `node_positions` is not
/// `[N,3]`, [`Self::drying_last_node_evaporation_k`] is positive (monolithic \(R_h\) is pure BE
/// diffusion), [`Self::implicit_t_alpha_newton`] is also set, `iterations < 2`, or
/// [`crate::physics::solvers::ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count`] for the live
/// `(N, F_T, F_h, F_α)` exceeds [`crate::physics::solvers::THMC_DENSE_NEWTON_MAX_STACKED_DOFS`] (dense Jacobian workspace cap — same as the standalone
/// `(T,\alpha)` implicit helper).
#[derive(Clone)]
pub struct ThmcSolver {
    pub dt: f32,
    pub max_newton: usize,
    pub tol: f32,
    /// reaction extent kinetics + mechanics stiffness scales (see module **Calibration surface**).
    pub reaction_extent_kinetics: ReactionExtentKinetics,
    /// Capillary evaporation sink on the **last** node index (`N-1`) when `> 0`:
    /// after the Laplacian humidity update, \(h_{N-1} \leftarrow h_{N-1} - \Delta t\,k\,(h_{N-1}-h_\infty)\).
    /// Intended for 1D drying-facet benchmarks (`tests/verification/thmc_drying_shrinkage.rs`).
    pub drying_last_node_evaporation_k: f32,
    /// Ambient humidity in \(\[0,1\]\) paired with [`Self::drying_last_node_evaporation_k`].
    pub drying_ambient_h: f32,
    /// When `Some`, replace the explicit \(T\) + \(\alpha\) split **once per outer pass** with damped
    /// Newton on the backward-Euler \((T,\alpha)\) residual ([`ThmcImplicitTAlphaNewtonConfig`]).
    /// Default **`None`**: legacy explicit split (unchanged behaviour).
    pub implicit_t_alpha_newton: Option<ThmcImplicitTAlphaNewtonConfig>,
    /// Opt-in **Phase 5** dense damped Newton on backward-Euler \((T,h,\alpha,\mathbf u)\) with
    /// quasi-static bar \(R_u\) (`ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u`).
    ///
    /// **Requires:** `batch == 1`, `[N,3]` SI `node_positions`, compatible `displacement_bc_mask`,
    /// stacked DOFs \(\le\) [`crate::physics::solvers::THMC_DENSE_NEWTON_MAX_STACKED_DOFS`], and **`drying_last_node_evaporation_k == 0`** (pure implicit diffusion \(R_h\)).
    /// Mutually exclusive with [`Self::implicit_t_alpha_newton`]. Default **`None`**.
    ///
    /// [`ThmcMonolithicNewtonConfig::stacked_residual_l2_tolerance`] /
    /// [`ThmcMonolithicNewtonConfig::stacked_residual_relative_to_initial`]: optional stacked \(\|R\|_2\)
    /// early-exit predicates wired into
    /// `ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u`
    /// (host scalar reads after each residual evaluation).
    ///
    /// Call [`Self::step`] as usual, or `Self::step_monolithic_implicit` to assert this branch is configured.
    pub monolithic_thmc_newton: Option<ThmcMonolithicNewtonConfig>,
    /// Kleisli penalize accumulator — immutable [`super::thmc_step::ThmcStepGateEvidence`] per step (warm drain).
    #[cfg(feature = "thmc-coupled")]
    pub step_gate_evidence: Vec<super::thmc_step::ThmcStepGateEvidence>,
    /// Mechanics port witnesses from [`crate::physics::mechanics_solve_port`] (warm drain).
    #[cfg(all(feature = "thmc-coupled", feature = "mechanics-adjoint"))]
    pub mechanics_solve_reports: Vec<crate::solve_report::SolveReport>,
    /// Intrinsic strength (MPa) for mix-calibrated gate snapshot lift at post-step evidence hook.
    /// Default aligns with [`super::thmc_step::THMC_GATE_LIFT_S_INTRINSIC_MPA_DEFAULT`] cartridge lift scale.
    #[cfg(feature = "thmc-coupled")]
    pub gate_intrinsic_strength_mpa: f64,
    /// Injectable gate witness — default [`super::thmc_step::DEFAULT_GATE_CARTRIDGE`].
    #[cfg(feature = "thmc-coupled")]
    pub gate_cartridge: &'static dyn crate::runtime::gate::GateCartridge,
}

#[cfg(feature = "thmc-coupled")]
impl std::fmt::Debug for ThmcSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThmcSolver")
            .field("dt", &self.dt)
            .field("max_newton", &self.max_newton)
            .field("tol", &self.tol)
            .field(
                "gate_intrinsic_strength_mpa",
                &self.gate_intrinsic_strength_mpa,
            )
            .field("gate_cartridge", &"<dyn GateCartridge>")
            .finish_non_exhaustive()
    }
}

#[cfg(not(feature = "thmc-coupled"))]
impl std::fmt::Debug for ThmcSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThmcSolver")
            .field("dt", &self.dt)
            .field("max_newton", &self.max_newton)
            .field("tol", &self.tol)
            .finish_non_exhaustive()
    }
}

impl Default for ThmcSolver {
    fn default() -> Self {
        Self {
            dt: 0.01_f32,
            max_newton: 2_usize,
            tol: 1e-3_f32,
            reaction_extent_kinetics: ReactionExtentKinetics::default(),
            drying_last_node_evaporation_k: 0.0_f32,
            drying_ambient_h: 0.5_f32,
            implicit_t_alpha_newton: None,
            monolithic_thmc_newton: None,
            #[cfg(feature = "thmc-coupled")]
            step_gate_evidence: Vec::new(),
            #[cfg(all(feature = "thmc-coupled", feature = "mechanics-adjoint"))]
            mechanics_solve_reports: Vec::new(),
            #[cfg(feature = "thmc-coupled")]
            gate_intrinsic_strength_mpa: super::thmc_step::THMC_GATE_LIFT_S_INTRINSIC_MPA_DEFAULT,
            #[cfg(feature = "thmc-coupled")]
            gate_cartridge: super::thmc_step::DEFAULT_GATE_CARTRIDGE,
        }
    }
}

#[cfg(feature = "thmc-coupled")]
impl ThmcSolver {
    /// Drain accumulated post-step gate evidence for PPO / gateway penalize morphisms.
    pub fn drain_gate_evidence(&mut self) -> Vec<super::thmc_step::ThmcStepGateEvidence> {
        std::mem::take(&mut self.step_gate_evidence)
    }

    /// Override mix-calibrated intrinsic strength (MPa) for gate snapshot lift.
    #[must_use]
    pub fn with_gate_intrinsic_strength_mpa(mut self, mpa: f64) -> Self {
        self.gate_intrinsic_strength_mpa = mpa;
        self
    }

    /// Route post-step evidence through an injectable [`crate::runtime::gate::GateCartridge`].
    #[must_use]
    pub fn with_gate_cartridge(
        mut self,
        cartridge: &'static dyn crate::runtime::gate::GateCartridge,
    ) -> Self {
        self.gate_cartridge = cartridge;
        self
    }

    /// Deprecated alias for [`Self::with_gate_cartridge`].
    #[must_use]
    #[deprecated(since = "0.1.0", note = "use with_gate_cartridge instead")]
    #[allow(deprecated)]
    pub fn with_transition_gate(mut self, gate: super::thmc_step::TransitionGateWitness) -> Self {
        self.gate_cartridge = gate.cartridge();
        self
    }

    /// Drain mechanics [`crate::solve_report::SolveReport`] witnesses from the operator-split loop.
    #[cfg(feature = "mechanics-adjoint")]
    pub fn drain_mechanics_solve_reports(&mut self) -> Vec<crate::solve_report::SolveReport> {
        std::mem::take(&mut self.mechanics_solve_reports)
    }
}


impl ThmcSolver {
    /// One coupled THMC step using cartridge constitutive data.
    ///
    /// # Contract
    /// - Inner tensors `[B, N, …]` align with the active voxel / node count carried by `manifold`.
    /// - Explicit split path: fixed `max_newton` passes; exits early when stacked transport
    ///   residual \(\|R\|_2 \le\) [`Self::tol`].
    /// - Monolithic path: uses [`ThmcMonolithicNewtonConfig::stacked_residual_l2_tolerance`] when set.
    ///
    /// # Errors
    /// - Builds **without** `thmc-coupled`: returns `Err` — do not call on production hot paths unless
    ///   the feature is enabled (directly or through `solver-research` / `solver-experimental`).
    /// - Experimental builds: returns `Err` on node-count mismatch between `state` and `manifold`.
    #[must_use = "THMC state advance must be consumed or propagated; ignoring the result drops the updated physics bundle"]
    pub fn step<B, C>(
        &mut self,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &mut UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, PhysicsError>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        #[cfg(feature = "thmc-coupled")]
        {
            self.step_experimental(cartridge, state, manifold, ThmcPhaseRoute::Solid)
        }
        #[cfg(not(feature = "thmc-coupled"))]
        {
            let _ = (
                self.dt,
                self.max_newton,
                self.tol,
                self.drying_last_node_evaporation_k,
                self.drying_ambient_h,
                self.implicit_t_alpha_newton.clone(),
                self.monolithic_thmc_newton.clone(),
            );
            let _ = (cartridge, manifold);
            drop(state);
            Err(PhysicsError::Domain {
                detail: "ThmcSolver::step: thmc-coupled feature is disabled; enable `--features thmc-coupled` (or `solver-experimental` / `solver-tests` for all opt-in solvers), or do not call this entrypoint".to_string(),
            })
        }
    }

    /// One coupled THMC step using the **Phase 5** monolithic backward-Euler Newton path only.
    ///
    /// Equivalent to [`Self::step`] after configuring [`Self::monolithic_thmc_newton`] with
    /// [`ThmcMonolithicNewtonConfig`]; this entrypoint returns `Err` if that field is `None` so call sites
    /// can assert the dense \((T,h,\alpha,\mathbf u)\) + quasi-static \(R_u\) branch is intended.
    ///
    /// **Requires** `thmc-coupled` (same as [`Self::step`]). See [`Self::monolithic_thmc_newton`] for geometry
    /// and `drying_last_node_evaporation_k == 0` constraints.
    #[cfg(feature = "thmc-coupled")]
    #[must_use = "THMC state advance must be consumed or propagated; ignoring the result drops the updated physics bundle"]
    pub fn step_monolithic_implicit<B, C>(
        &mut self,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &mut UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, PhysicsError>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        if self.monolithic_thmc_newton.is_none() {
            return Err(
                "ThmcSolver::step_monolithic_implicit: monolithic_thmc_newton must be Some(ThmcMonolithicNewtonConfig { .. })"
                    .into(),
            );
        }
        self.step(cartridge, state, manifold)
    }

    /// Phase-routed THMC advance on algebraic [`ThmcEnvelope`] (MP2b U3).
    ///
    /// Dispatches operator-split substeps by [`MaterialPhaseKind`]:
    /// - **Fluid:** transport \((T,h,\alpha)\) only — no quasi-static \(u\), no fracture epilogue.
    /// - **Setting:** transport + optional bar equilibrium when `node_positions` is `[N,3]`.
    /// - **Solid:** full split + fracture damage epilogue (parity with legacy [`Self::step`]).
    ///
    /// **Requires** `thmc-coupled`. Legacy flat golden tests continue to call [`Self::step`].
    #[cfg(feature = "thmc-coupled")]
    pub fn step_envelope<B, C>(
        &mut self,
        cartridge: &C,
        envelope: &mut ThmcEnvelope<B>,
        manifold: &mut UnifiedMaterialStateTensor<B>,
    ) -> Result<(), PhysicsError>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        let route = match envelope.kind() {
            MaterialPhaseKind::Fluid => ThmcPhaseRoute::Fluid,
            MaterialPhaseKind::Setting => ThmcPhaseRoute::Setting,
            MaterialPhaseKind::Solid => ThmcPhaseRoute::Solid,
        };
        #[allow(deprecated)]
        let flat_pre = envelope.to_flat_state();
        let flat_post = self.step_experimental(cartridge, flat_pre, manifold, route)?;
        envelope.sync_from_flat_state(&flat_post);
        Ok(())
    }

    #[cfg(not(feature = "thmc-coupled"))]
    pub fn step_envelope<B, C>(
        &mut self,
        _cartridge: &C,
        _envelope: &mut ThmcEnvelope<B>,
        _manifold: &mut UnifiedMaterialStateTensor<B>,
    ) -> Result<(), PhysicsError>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        Err(PhysicsError::Domain {
            detail: "ThmcSolver::step_envelope: thmc-coupled feature is disabled; enable `--features thmc-coupled` (or `solver-experimental` / `solver-tests` for all opt-in solvers), or do not call this entrypoint".to_string(),
        })
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
        &mut self,
        _cartridge: &C,
        state: ThmcState<B>,
        manifold: &mut UnifiedMaterialStateTensor<B>,
        route: ThmcPhaseRoute,
    ) -> Result<ThmcState<B>, PhysicsError>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        let device = state.thermal.temperature.as_tensor().device();
        let batch = state.thermal.temperature.as_tensor().dims()[0];
        let n = state.thermal.temperature.as_tensor().dims()[1];
        let n_manifold = manifold.scalar_features.dims()[0];
        let edges_b1 = manifold.edges_b1.clone();

        if n != n_manifold {
            return Err(format!(
                "ThmcSolver::step: ThmcState thermal axis N={n} != manifold.scalar_features rows N={n_manifold}"
            ).into());
        }

        // Pre-step snapshot for post-step gate evidence hook (p5-thmc-wire; see `thmc_step.rs`).
        let pre_step = state.clone();

        // Damage mask frozen at step entry (FP P3.2).
        let damage_m = StepEntryDamageMask::from_step_entry_damage(&state.damage, batch, n);

        if self.monolithic_thmc_newton.is_some() && self.implicit_t_alpha_newton.is_some() {
            return Err(
                "ThmcSolver::step: monolithic_thmc_newton and implicit_t_alpha_newton are mutually exclusive; set one to None"
                    .into(),
            );
        }

        if let Some(mc) = self.monolithic_thmc_newton.as_ref() {
            if mc.iterations < 2 {
                return Err(
                    "ThmcSolver::step: monolithic_thmc_newton.iterations must be >= 2".into(),
                );
            }
            if batch != 1 {
                return Err(format!(
                    "ThmcSolver::step: monolithic_thmc_newton requires batch size 1, got {batch}"
                ).into());
            }
            let coords_ok = manifold
                .node_positions
                .as_ref()
                .map(|p| p.dims() == [n, 3])
                .unwrap_or(false);
            if !coords_ok {
                return Err(
                    "ThmcSolver::step: monolithic_thmc_newton requires manifold.node_positions with shape [N,3]"
                        .into(),
                );
            }
            if self.drying_last_node_evaporation_k > 0.0_f32 {
                return Err(
                    "ThmcSolver::step: monolithic_thmc_newton requires drying_last_node_evaporation_k == 0 (pure implicit diffusion R_h)"
                        .into(),
                );
            }
            let f_t = state.thermal.temperature.as_tensor().dims()[2];
            let f_h = state.hydro.humidity.as_tensor().dims()[2];
            let f_a = state.chemical.reaction_extent.as_tensor().dims()[2];
            let m_dof = ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(
                n, f_t, f_h, f_a,
            );
            if m_dof > THMC_DENSE_NEWTON_MAX_STACKED_DOFS {
                let cap = THMC_DENSE_NEWTON_MAX_STACKED_DOFS;
                return Err(format!(
                    "ThmcSolver::step: monolithic_thmc_newton stacked DOFs > {cap} (dense Jacobian cap is {cap}), got {m_dof}",
                ).into());
            }
        }

        let step_ctx = ThmcStepCtx {
            dt: self.dt,
            edges_b1: edges_b1.clone(),
            damage_m: damage_m.clone(),
            device: device.clone(),
            batch,
            n,
            drying_last_node_evaporation_k: self.drying_last_node_evaporation_k,
            drying_ambient_h: self.drying_ambient_h,
            reaction_extent_kinetics: self.reaction_extent_kinetics.clone(),
            implicit_t_alpha_newton: self.implicit_t_alpha_newton.clone(),
            monolithic_thmc_newton: self.monolithic_thmc_newton.clone(),
            manifold,
        };

        // RW-FP-P53: Kleisli Newton iteration — short-circuits on first `PhysicsError` (P54 orchestrator
        // `try_fold` @ `2a9ad0a` is orthogonal; this is the interior THMC outer-Newton tail only).
        let (state, _) = (0..self.max_newton).try_fold((state, false), |(state, converged), _| {
            if converged {
                return Ok::<_, PhysicsError>((state, true));
            }
            let scratch = ThmcNewtonScratch::from_state(&state, &step_ctx);
            let state = newton_split_chain(state, &scratch, &step_ctx, self, route)?;
            let converged = transport_residual_l2(&state, &scratch) <= self.tol;
            Ok((state, converged))
        })?;

        let epilogue_ctx = super::thmc_epilogue::ThmcPostStepCtx {
            batch,
            n,
            edges_b1,
            dt: self.dt,
        };
        let (state, gate_evidence) = super::thmc_epilogue::thmc_post_step_epilogue(
            self,
            _cartridge,
            &pre_step,
            state,
            manifold,
            &epilogue_ctx,
            route.runs_fracture_epilogue(),
        )?;
        self.step_gate_evidence.push(gate_evidence);
        Ok(state)
    }
}

/// Inner Newton / Krylov controls for the implicit thermal block (Phase 3.2 monolithic Newton seed).
///
/// The implicit-Euler thermal residual
/// \[ R_T(T_{\mathrm{new}}) = (T_{\mathrm{new}}-T_{\mathrm{old}})/\Delta t - \kappa\,\mathcal{L}(T_{\mathrm{new}}) \]
/// is **linear** in \(T_{\mathrm{new}}\); a single outer Newton step converges modulo the inner CG
/// tolerance. The struct exposes Newton-flavoured fields (`max_iterations`, `damping`,
/// `finite_diff_eps`) so the same surface generalises to the coupled \((T,h,u,c)\) residual when
/// the off-diagonal blocks land.
#[derive(Clone, Copy, Debug)]
pub struct ThmcNewtonConfig {
    pub max_iterations: usize,
    pub residual_tolerance: f32,
    pub finite_diff_eps: f32,
    /// Damping factor in \((0, 1]\) applied to the Krylov step (`< 1` ⇒ under-relaxed Richardson).
    pub damping: f32,
}

impl Default for ThmcNewtonConfig {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            residual_tolerance: 1.0e-6_f32,
            finite_diff_eps: 1.0e-6_f32,
            damping: 1.0_f32,
        }
    }
}

/// Opt-in **multi-step damped Newton** on the backward-Euler \((T,\alpha)\) block (implementation:
/// `ThmcImplicitEulerThermalReactionExtentResidual::damped_newton_iterations` in `thmc_residual.rs`).
///
/// When [`ThmcSolver::implicit_t_alpha_newton`] is `Some`, each outer `max_newton` pass replaces the
/// usual explicit thermal increment + explicit reaction extent \(\alpha\) update with a Newton solve on
/// the same analytic residual used in verification tests. Humidity, mechanics, and fracture
/// substeps are unchanged. **Requires** `thmc-coupled` (otherwise [`ThmcSolver::step`] does not run
/// this path); `batch` must be **1** and stacked \((T,\alpha)\) DOFs \(\le\) [`crate::physics::solvers::THMC_DENSE_NEWTON_MAX_STACKED_DOFS`].
#[derive(Clone, Debug, PartialEq)]
pub struct ThmcImplicitTAlphaNewtonConfig {
    /// Chains `ThmcImplicitEulerThermalReactionExtentResidual::damped_newton_iterations` in `thmc_residual.rs` — must be
    /// **≥ 2** (the helper rejects smaller values).
    pub iterations: usize,
    pub damping: f32,
    pub fd_eps: f32,
}

impl Default for ThmcImplicitTAlphaNewtonConfig {
    fn default() -> Self {
        Self {
            iterations: 3_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
        }
    }
}

/// Opt-in **Phase 5** damped Newton on backward-Euler \((T,h,\alpha,\mathbf u)\) with quasi-static bar
/// \(R_u\) — `ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u`.
///
/// Wired from [`ThmcSolver::step`] when [`ThmcSolver::monolithic_thmc_newton`] is `Some` (requires `thmc-coupled`).
///
/// **Scope:** each Newton iteration builds a **dense** finite-difference Jacobian in host workspace
/// sized for at most [`crate::physics::solvers::THMC_DENSE_NEWTON_MAX_STACKED_DOFS`] stacked unknowns per batch (see
/// [`crate::physics::solvers::ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count`]). Larger problems must
/// use the split path until a sparse or matrix-free stack lands. **Mutually exclusive** with
/// [`ThmcSolver::implicit_t_alpha_newton`]. Requires facet drying sink
/// [`ThmcSolver::drying_last_node_evaporation_k`] **== 0** so \(R_h\) matches the implicit diffusion
/// residual assembled in `thmc_residual`. Integration tests live in
/// `tests/verification/thmc_drying_shrinkage.rs` (`thmc_step_monolithic_*`, …).
#[derive(Clone, Debug, PartialEq)]
pub struct ThmcMonolithicNewtonConfig {
    /// Maximum damped Newton iterations on the stacked residual (each step rebuilds the dense FD Jacobian).
    ///
    /// Must be **≥ 2** (matches the residual helper contract).
    pub iterations: usize,
    pub damping: f32,
    pub fd_eps: f32,
    /// Stacked \(\|R\|_2\) **absolute** early exit: when **`> 0`**, require \(\|R\|_2\) **strictly below**
    /// this value whenever this predicate is active (together with any active [`stacked_residual_relative_to_initial`](Self::stacked_residual_relative_to_initial)).
    pub stacked_residual_l2_tolerance: f32,
    /// Optional stacked \(\|R\|_2\) **relative** gate on the initial \(\|R_0\|_2\): when **`Some(k)`** with
    /// **`k > 0`**, also require \(\|R\|_2 < k\cdot\max(\|R_0\|_2,\varepsilon)\). Every **enabled** tolerance
    /// predicate must hold before early exit.
    pub stacked_residual_relative_to_initial: Option<f32>,
}

impl Default for ThmcMonolithicNewtonConfig {
    fn default() -> Self {
        Self {
            iterations: 4_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
            stacked_residual_l2_tolerance: 0.0_f32,
            stacked_residual_relative_to_initial: None,
        }
    }
}

#[cfg(feature = "thmc-coupled")]
impl ThmcSolver {
    /// One implicit-Euler thermal step on the graph Laplacian:
    /// \[ (I/\Delta t - \kappa\,\mathcal{L})\,T_{\mathrm{new}} = T_{\mathrm{old}}/\Delta t. \]
    /// Solved with conjugate gradients on the SPD LHS operator. Returns `Ok((T_new, residual_norms))`
    /// when the final L2 residual is below [`ThmcNewtonConfig::residual_tolerance`], or
    /// [`PhysicsError::Diverged`] after `max_iterations` without meeting tolerance.
    /// `residual_norms[k]` is the L2 norm of the *physical* residual
    /// \(R = (T_k - T_{\mathrm{old}})/\Delta t - \kappa\,\mathcal{L}(T_k)\) after iteration `k`.
    /// `residual_norms[0]` is the norm at the initial guess (`T_old`).
    ///
    /// **Device sync (E4 / [`docs/FP_CATEGORICAL_BURN.md`](../../../docs/FP_CATEGORICAL_BURN.md)):** CG coefficients
    /// \(\alpha_k\), \(\beta_k\) stay **tensor-native** (broadcast `reshape`, same pattern as packed mechanics PCG).
    /// **ConvergenceRequired — telemetry only:** `.into_scalar()` hits are limited to **L2 residual telemetry**
    /// (each iteration’s \(\|r\|_2\) pushed to `residual_norms`) and the **tol** branch reads that `Vec<f32>` —
    /// not per-iteration scalars for \(\alpha\) / \(\beta\) (avoid host reductions in **hot inner micro-loops**
    /// beyond this contract). Stacked THMC Newton / JFNK ‖R‖₂ reporting uses the same tier in
    /// [`thmc_residual`](crate::physics::solvers::thmc_residual) (rank‑1 `into_data` via `tensor1_f32_thmc`;
    /// equivalent `.into_scalar()` there remains acceptable **outer** telemetry debt — see that module’s rustdoc).
    ///
    /// **Boundary mask** (`[B,N,1]`): nodes with value `0` are treated as Dirichlet; their
    /// increment is zeroed (locking them to `T_old`) and they are excluded from the convergence
    /// norm so clamped DOFs do not pollute it. The CG residual `r = b - A x` is mathematically
    /// equal to `-R` (since `b = T_old/dt` and `A x = x/dt - κ L x`), so we report `||r||_2`
    /// directly.
    pub fn step_thermal_implicit<B: Backend<FloatElem = f32>>(
        &self,
        dt: f32,
        t_old: Tensor<B, 3>,
        kappa: f32,
        edges_b1: Tensor<B, 2, Int>,
        boundary_mask: Tensor<B, 3>,
        cfg: ThmcNewtonConfig,
    ) -> Result<(Tensor<B, 3>, Vec<f32>), PhysicsError> {
        let device = t_old.device();
        let dims = t_old.dims();
        // No damage attenuation for this verification path; full conductivity on every edge.
        let damage_zero = Tensor::<B, 3>::zeros(dims, &device);

        // A x = x/dt - kappa * L(x)
        let a_op = |x: Tensor<B, 3>| -> Tensor<B, 3> {
            let lx = TopologicalLaplacian::scalar_laplacian(
                x.clone(),
                edges_b1.clone(),
                damage_zero.clone(),
            );
            x.div_scalar(dt).sub(lx.mul_scalar(kappa))
        };

        let b = t_old.clone().div_scalar(dt);
        let mut x = t_old.clone();

        // Residual r = (b - A x) * mask  (mask = 1 on free DOFs, 0 on Dirichlet)
        let mut r = b.clone().sub(a_op(x.clone())).mul(boundary_mask.clone());
        let mut p = r.clone();
        let mut rs_old_t = r.clone().mul(r.clone()).sum();

        let mut residual_norms: Vec<f32> = Vec::with_capacity(cfg.max_iterations + 1);
        residual_norms.push(
            rs_old_t
                .clone()
                .clamp_min(0.0_f32)
                .sqrt()
                .into_scalar()
                .max(0.0_f32),
        );

        let bc_shape = [dims[0], 1, 1];
        for _ in 0..cfg.max_iterations {
            if residual_norms.last().copied().unwrap_or(f32::INFINITY) < cfg.residual_tolerance {
                break;
            }
            let ap = a_op(p.clone()).mul(boundary_mask.clone());
            let pap = p.clone().mul(ap.clone()).sum().clamp_min(1.0e-30_f32);
            let alpha_t = rs_old_t.clone().div(pap).mul_scalar(cfg.damping);
            let alpha_bc = alpha_t.reshape(bc_shape);
            x = x.add(p.clone().mul(alpha_bc.clone()));
            r = r.sub(ap.mul(alpha_bc)).mul(boundary_mask.clone());
            let rs_new_t = r.clone().mul(r.clone()).sum();
            residual_norms.push(
                rs_new_t
                    .clone()
                    .clamp_min(0.0_f32)
                    .sqrt()
                    .into_scalar()
                    .max(0.0_f32),
            );
            let beta_t = rs_new_t
                .clone()
                .div(rs_old_t.clone().clamp_min(1.0e-30_f32));
            let beta_bc = beta_t.reshape(bc_shape);
            p = r.clone().add(p.mul(beta_bc));
            rs_old_t = rs_new_t;
        }

        let eq_rel = residual_norms.last().copied().unwrap_or(f32::INFINITY);
        if eq_rel >= cfg.residual_tolerance {
            return Err(PhysicsError::Diverged {
                eq_rel,
                pcg_iterations: residual_norms.len().saturating_sub(1),
            });
        }

        Ok((x, residual_norms))
    }
}

/// Typed reaction extent rate (FP P3.2).
#[cfg(feature = "thmc-coupled")]
pub fn reaction_extent_rate_field<B: Backend<FloatElem = f32>>(
    k: &ReactionExtentKinetics,
    alpha: &ReactionExtentField<B>,
    temperature_for_alpha: &TemperatureField<B>,
    device: &B::Device,
) -> ReactionExtentField<B> {
    Field::new(reaction_extent_rate_tensor(
        k, alpha.as_tensor().clone(), temperature_for_alpha.as_tensor().clone(), device,
    ))
}

/// Full tensor reaction extent rate \(\dot\alpha(\alpha,T)\) used in [`ThmcSolver::step`] and implicit residuals:
/// Arrhenius core `reaction_extent_arrhenius_rate` times the high-temperature boost factor.
#[cfg(feature = "thmc-coupled")]
pub fn reaction_extent_rate_tensor<B: Backend<FloatElem = f32>>(
    k: &ReactionExtentKinetics,
    alpha: Tensor<B, 3>,
    temperature_for_alpha: Tensor<B, 3>,
    device: &B::Device,
) -> Tensor<B, 3> {
    let d0 = reaction_extent_arrhenius_rate(k, alpha, temperature_for_alpha.clone());
    let t_boost_ref = Tensor::<B, 3>::full(temperature_for_alpha.dims(), k.t_boost_ref_k, device);
    let arrhenius_temp_boost = temperature_for_alpha
        .sub(t_boost_ref)
        .clamp_min(0.0_f32)
        .mul_scalar(k.t_boost_per_k)
        .add_scalar(1.0_f32);
    d0.mul(arrhenius_temp_boost)
}

/// Arrhenius-style reaction extent rate \(f(\alpha,T) = A\,\exp\!\bigl(-E_a/(R\,T)\bigr)\,(1-\alpha)_+\)
/// using fields from `k` (defaults align with legacy `REACTION_EXTENT_*` module constants).
#[cfg(feature = "thmc-coupled")]
fn reaction_extent_arrhenius_rate<B: Backend<FloatElem = f32>>(
    k: &ReactionExtentKinetics,
    alpha: Tensor<B, 3>,
    temperature_k: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let device = alpha.device();
    let shape = alpha.dims();
    let ones = Tensor::<B, 3>::ones(shape, &device);
    let one_minus_a = ones.sub(alpha).clamp_min(0.0_f32);
    let t_safe = temperature_k.clamp_min(k.t_min_k);
    let ea_over_rt = Tensor::<B, 3>::zeros(shape, &device)
        .add_scalar(k.activation_energy_j_per_mol)
        .div(t_safe.mul_scalar(k.gas_constant_j_per_mol_k));
    ea_over_rt
        .mul_scalar(-1.0_f32)
        .exp()
        .mul(one_minus_a)
        .mul_scalar(k.arrhenius_prefactor_s)
}

/// Order-of-magnitude **notional** total shrinkage strain (dimensionless) aligned to CEB-FIP MC2010-style
/// reporting for bulk binder (drying + autogenous trend; not a certified structural design value).
///
/// `ambient_rh_percent` is exterior RH in percent (e.g. `50.0`); `age_days` is equivalent exposure duration.
#[cfg(feature = "thmc-coupled")]
pub fn mc2010_style_notional_shrink_strain(
    binder_liquid_ratio: f32,
    reaction_extent: f32,
    ambient_rh_percent: f32,
    age_days: f32,
) -> f32 {
    let rh = (ambient_rh_percent / 100.0_f32).clamp(0.0_f32, 1.0_f32);
    let t = (age_days / 28.0_f32).sqrt().clamp(0.0_f32, 1.5_f32);
    let w = (binder_liquid_ratio / 0.4_f32).clamp(0.5_f32, 1.2_f32);
    let a = reaction_extent.clamp(0.1_f32, 1.0_f32);
    1.05e-3_f32 * w * a.sqrt() * (1.0 - rh) * t
}

/// Maps **saturation deficit** \((h_{\mathrm{init}}-h)\in\[0,1\]\) on an exposed facet to a shrink strain
/// increment scale used with the THMC humidity proxy (verification hook — coefficients track
/// [`mc2010_style_notional_shrink_strain`] order of magnitude).
#[cfg(feature = "thmc-coupled")]
pub fn shrink_strain_from_saturation_loss(
    humidity_loss_01: f32,
    binder_liquid_ratio: f32,
    reaction_extent: f32,
) -> f32 {
    let coeff = 1.1e-3_f32
        * (binder_liquid_ratio / 0.4_f32).clamp(0.5_f32, 1.2_f32)
        * reaction_extent.sqrt().clamp(0.2_f32, 1.0_f32);
    coeff * humidity_loss_01.clamp(0.0_f32, 1.0_f32)
}

/// Nodal tensor analogue of [`shrink_strain_from_saturation_loss`] (same clamps, elementwise).
#[cfg(feature = "thmc-coupled")]
pub fn shrink_strain_from_saturation_loss_tensor<B: Backend<FloatElem = f32>>(
    humidity_loss_01: Tensor<B, 3>,
    binder_liquid_ratio: f32,
    reaction_extent: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let w = (binder_liquid_ratio / 0.4_f32).clamp(0.5_f32, 1.2_f32);
    let alpha_term = reaction_extent.sqrt().clamp(0.2_f32, 1.0_f32);
    let coeff = alpha_term.mul_scalar(1.1e-3_f32 * w);
    coeff.mul(humidity_loss_01.clamp(0.0_f32, 1.0_f32))
}

#[cfg(test)]
mod w29_081_thmc_deepen_tests {
    use super::*;

    #[test]
    fn thmc_honest_posture_refuses_green_production_master_op5() {
        let probe = thmc_honest_posture_bundle();
        assert!(thmc_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5_claimed);
        assert!(probe.solver_surface_landed);
        assert_eq!(probe.deepen_cell, W29_THMC_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, THMC_POSTURE_TAG);
        assert!(probe.honest_fence.contains("dense_newton_capped=true"));
    }

    #[test]
    fn thmc_default_solver_keeps_opt_in_newton_none() {
        let s = ThmcSolver::default();
        assert!(s.implicit_t_alpha_newton.is_none());
        assert!(s.monolithic_thmc_newton.is_none());
        assert!(s.drying_last_node_evaporation_k == 0.0_f32);
        assert!(s.max_newton >= 1);
        assert!(s.dt > 0.0_f32);
    }

    #[test]
    fn thmc_reaction_extent_kinetics_alpha_rate_vanishes_at_full_extent() {
        let k = ReactionExtentKinetics::default();
        let rate = k.alpha_rate_scalar(1.0_f32, 300.0_f32);
        assert!(rate.abs() < 1e-12_f32, "full extent ⇒ Arrhenius (1-α)_+ = 0, got {rate}");
        let mid = k.alpha_rate_scalar(0.5_f32, 300.0_f32);
        assert!(mid > 0.0_f32, "mid extent at room T should be positive");
    }

    #[test]
    fn thmc_monolithic_newton_config_default_iterations_ge_two() {
        let cfg = ThmcMonolithicNewtonConfig::default();
        assert!(cfg.iterations >= 2);
        assert!(cfg.damping > 0.0_f32 && cfg.damping <= 1.0_f32);
        assert!(cfg.fd_eps > 0.0_f32);
        assert_eq!(cfg.stacked_residual_relative_to_initial, None);
    }

    #[test]
    fn thmc_implicit_t_alpha_newton_config_default_iterations_ge_two() {
        let cfg = ThmcImplicitTAlphaNewtonConfig::default();
        assert!(cfg.iterations >= 2);
        assert!(cfg.damping > 0.0_f32 && cfg.damping <= 1.0_f32);
        assert!(cfg.fd_eps > 0.0_f32);
    }
}

#[cfg(all(test, feature = "thmc-coupled"))]
mod xs3_step5_tests {
    use super::*;
    use burn_ndarray::NdArray;

    type B = NdArray;

    #[test]
    fn reaction_extent_kinetics_stiffness_centralizer_matches_from_alpha_kinetics() {
        let device = Default::default();
        let kinetics = ReactionExtentKinetics::default();
        let alpha = Tensor::<B, 3>::from_floats([[[0.5], [0.75]]], &device);
        let via_kinetics = kinetics.stiffness_field_from_alpha_bn1(alpha.clone(), &device);
        let via_field = StiffnessField::from_alpha_kinetics(
            alpha,
            &kinetics.as_kinetics_spec(),
            &device,
        );
        assert_eq!(
            via_kinetics.as_tensor().dims(),
            via_field.as_tensor().dims(),
        );
    }
}
