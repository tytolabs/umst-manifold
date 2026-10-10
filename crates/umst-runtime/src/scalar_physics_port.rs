// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MPL-2.0
//! Scalar physics port — pure `f32`/`f64` cement scalars from legacy `umst-concrete-ffi/src/scalar_physics.rs`.

use umst_manifold::core::material_transition::ReactionExtentKineticsSpec;
use umst_manifold::core::MaterialTransitionParams;
use umst_manifold::gate::ThermodynamicStateSnapshot;

/// Intrinsic strength scale (MPa) — matches legacy `CEMENT_DEFAULT_S_INTRINSIC_MPA` witness (240).
pub const SCALAR_PORT_S_INTRINSIC_MPA: f64 = 240.0;

/// Legacy `CementMaterialParams` closure witness for mix thermo snapshots (parity with FFI path).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ScalarPortCementClosure;

impl MaterialTransitionParams for ScalarPortCementClosure {
    fn reaction_enthalpy_j_per_kg(&self) -> Option<f64> {
        Some(450.0)
    }

    fn default_intrinsic_strength_mpa(&self) -> Option<f64> {
        Some(SCALAR_PORT_S_INTRINSIC_MPA)
    }

    fn reaction_extent_kinetics_spec(&self) -> ReactionExtentKineticsSpec {
        ReactionExtentKineticsSpec {
            arrhenius_prefactor_s: 1.0e-6,
            activation_energy_j_per_mol: 40_000.0,
            gas_constant_j_per_mol_k: 8.314_463,
            t_min_k: 250.0,
            t_boost_ref_k: 293.15,
            t_boost_per_k: 0.02,
            exothermic_k_per_alpha_rate: 5.0,
            stiffness_e_scale_pa: 30e9,
            stiffness_nu: 0.2,
        }
    }
}

/// Avrami–Parrott hydration degree α ∈ [0, 1].
#[must_use]
pub fn hydration_degree(age_days: f32, temp_c: f32, scm_ratio: f32) -> f32 {
    let alpha_max = 0.95 - scm_ratio * 0.15;
    let k_ref = 0.55f32;
    let t_ref_k = 293.15f32;
    let t_k = temp_c + 273.15;
    let e_over_r = 5000.0f32;
    let temp_factor = (e_over_r * (1.0 / t_ref_k - 1.0 / t_k)).exp();
    let scm_factor = 1.0 - scm_ratio * 0.4;
    let k = k_ref * temp_factor * scm_factor;
    let alpha = alpha_max * (1.0 - (-k * age_days.sqrt()).exp());
    alpha.clamp(0.0, 1.0)
}

/// Powers gel-space compressive strength (MPa).
#[must_use]
pub fn strength_powers(
    wc_ratio: f32,
    degree_hydration: f32,
    air_content: f32,
    intrinsic_strength: f32,
) -> f32 {
    if wc_ratio > 100.0 {
        return 0.0;
    }
    let vg_volume_gel = 0.68 * degree_hydration;
    let vc_volume_capillary = wc_ratio - 0.36 * degree_hydration;
    let space = vg_volume_gel + vc_volume_capillary + air_content;
    if space <= 0.001 {
        return 0.0;
    }
    let x = vg_volume_gel / space;
    intrinsic_strength * x.powi(3)
}

/// Thermodynamic snapshot from mix scalars (Powers closure).
#[must_use]
pub fn thermo_snapshot_from_mix(w_c: f64, alpha: f64, temp: f64) -> ThermodynamicStateSnapshot {
    ThermodynamicStateSnapshot::from_mix_calibrated_with_params(
        w_c,
        alpha,
        temp,
        SCALAR_PORT_S_INTRINSIC_MPA,
        &ScalarPortCementClosure,
    )
}

/// C-ABI struct mirror for Haskell `Storable` consumers.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CThermodynamicState {
    pub density: f64,
    pub free_energy: f64,
    pub hydration_degree: f64,
    pub strength: f64,
    pub max_strength: f64,
}

#[must_use]
pub fn c_state_from_mix(w_c: f64, alpha: f64, temp: f64) -> CThermodynamicState {
    let snap = thermo_snapshot_from_mix(w_c, alpha, temp);
    CThermodynamicState {
        density: snap.density,
        free_energy: snap.free_energy,
        hydration_degree: snap.reaction_extent,
        strength: snap.strength,
        max_strength: SCALAR_PORT_S_INTRINSIC_MPA,
    }
}
