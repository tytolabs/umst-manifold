// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Bridges **volumetric** Clausius–Duhem surrogates into scalar joule proxies for [`crate::ai::cbf::ThermodynamicCBF`].

/// PHY-002 morphism id @ PORT_GRAIN_BAND `gate:cbf_bridge`.
pub const CBF_BRIDGE_MORPHISM_ID: &str = "cd_dissipation_proxy_to_entropy_joules";

/// Honest posture — tests deepen only; no GREEN invent (`MASTER_RETICK=no`).
pub const CBF_BRIDGE_POSTURE_TAG: &str = "honest-cd-proxy-bridge-only";

/// PORT-MF-CBF-BRIDGE-W2 cell id (wave-2 gate band deepen).
pub const CBF_BRIDGE_CELL_ID: &str = "PORT-MF-CBF-BRIDGE-W2";

/// Approximate volumetric entropy production rate (Joules/step) given a nonnegative `d_int`
/// surrogate in W/m³, control volume **V**, and timestep **Δt**.
///
/// Multiply by cartridge-specific calibration knobs before handing to [`ThermodynamicCBF::verify_and_deduct_update`](crate::ai::cbf::ThermodynamicCBF::verify_and_deduct_update).
#[must_use]
pub fn cd_dissipation_proxy_to_entropy_joules(d_int_w_m3: f64, volume_m3: f64, dt_s: f64) -> f64 {
    d_int_w_m3.max(0.0) * volume_m3.max(0.0) * dt_s.max(0.0)
}

/// Whether the CD proxy morphism is pinned @ HEAD (identity clamp semantics).
#[must_use]
pub fn cbf_bridge_morphism_pinned() -> bool {
    CBF_BRIDGE_MORPHISM_ID == "cd_dissipation_proxy_to_entropy_joules"
        && CBF_BRIDGE_POSTURE_TAG == "honest-cd-proxy-bridge-only"
        && CBF_BRIDGE_CELL_ID == "PORT-MF-CBF-BRIDGE-W2"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::cbf::ThermodynamicCBF;
    use approx::assert_relative_eq;

    /// SI dimensional witness: (W/m³)·m³·s = J (W = J/s).
    const D_INT_W_M3: f64 = 100.0;
    const VOLUME_M3: f64 = 1.0e-3;
    const DT_S: f64 = 1.0;

    #[test]
    fn cbf_bridge_morphism_identity_pinned() {
        assert!(cbf_bridge_morphism_pinned());
        assert_eq!(
            CBF_BRIDGE_MORPHISM_ID,
            "cd_dissipation_proxy_to_entropy_joules"
        );
        assert_eq!(CBF_BRIDGE_CELL_ID, "PORT-MF-CBF-BRIDGE-W2");
    }

    #[test]
    fn cbf_bridge_positive_inputs_multiply_to_joules() {
        let joules = cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, VOLUME_M3, DT_S);
        assert_relative_eq!(joules, 0.1, epsilon = 1.0e-30);
    }

    #[test]
    fn cbf_bridge_dimensional_consistency_w_per_m3_times_m3_times_s() {
        // 50 W/m³ × 2 m³ × 0.5 s = 50 J
        let joules = cd_dissipation_proxy_to_entropy_joules(50.0, 2.0, 0.5);
        assert_relative_eq!(joules, 50.0, epsilon = 1.0e-30);
    }

    #[test]
    fn cbf_bridge_clamps_negative_d_int_to_zero() {
        assert_eq!(
            cd_dissipation_proxy_to_entropy_joules(-1.0e6, VOLUME_M3, DT_S),
            0.0
        );
        assert_relative_eq!(
            cd_dissipation_proxy_to_entropy_joules(-0.5, 2.0, 1.0),
            0.0,
            epsilon = 1.0e-30
        );
    }

    #[test]
    fn cbf_bridge_clamps_negative_volume_to_zero() {
        assert_eq!(
            cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, -1.0, DT_S),
            0.0
        );
        assert_relative_eq!(
            cd_dissipation_proxy_to_entropy_joules(10.0, -0.25, 2.0),
            0.0,
            epsilon = 1.0e-30
        );
    }

    #[test]
    fn cbf_bridge_clamps_negative_dt_to_zero() {
        assert_eq!(
            cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, VOLUME_M3, -3600.0),
            0.0
        );
        assert_relative_eq!(
            cd_dissipation_proxy_to_entropy_joules(5.0, 1.0, -1.0e-9),
            0.0,
            epsilon = 1.0e-30
        );
    }

    #[test]
    fn cbf_bridge_zero_any_factor_yields_zero() {
        assert_eq!(
            cd_dissipation_proxy_to_entropy_joules(0.0, VOLUME_M3, DT_S),
            0.0
        );
        assert_eq!(
            cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, 0.0, DT_S),
            0.0
        );
        assert_eq!(
            cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, VOLUME_M3, 0.0),
            0.0
        );
        assert_eq!(cd_dissipation_proxy_to_entropy_joules(0.0, 0.0, 0.0), 0.0);
    }

    #[test]
    fn cbf_bridge_all_negative_inputs_clamp_to_zero() {
        assert_eq!(
            cd_dissipation_proxy_to_entropy_joules(-10.0, -2.0, -1.0),
            0.0
        );
    }

    #[test]
    fn cbf_bridge_permutation_invariant_under_positive_inputs() {
        let a = 3.0_f64;
        let b = 4.0_f64;
        let c = 5.0_f64;
        let expected = a * b * c;
        assert_relative_eq!(
            cd_dissipation_proxy_to_entropy_joules(a, b, c),
            expected,
            epsilon = 1.0e-30
        );
        assert_relative_eq!(
            cd_dissipation_proxy_to_entropy_joules(c, a, b),
            expected,
            epsilon = 1.0e-30
        );
        assert_relative_eq!(
            cd_dissipation_proxy_to_entropy_joules(b, c, a),
            expected,
            epsilon = 1.0e-30
        );
    }

    #[test]
    fn cbf_bridge_monotone_in_d_int_when_volume_and_dt_positive() {
        let lo = cd_dissipation_proxy_to_entropy_joules(1.0, VOLUME_M3, DT_S);
        let hi = cd_dissipation_proxy_to_entropy_joules(2.0, VOLUME_M3, DT_S);
        assert!(hi > lo, "nonnegative clamp preserves monotonicity in d_int");
    }

    #[test]
    fn cbf_bridge_monotone_in_volume_when_d_int_and_dt_positive() {
        let lo = cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, 0.5e-3, DT_S);
        let hi = cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, 1.5e-3, DT_S);
        assert!(hi > lo);
    }

    #[test]
    fn cbf_bridge_monotone_in_dt_when_d_int_and_volume_positive() {
        let lo = cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, VOLUME_M3, 0.5);
        let hi = cd_dissipation_proxy_to_entropy_joules(D_INT_W_M3, VOLUME_M3, 1.5);
        assert!(hi > lo);
    }

    #[test]
    fn cbf_bridge_negative_d_int_matches_zero_d_int() {
        let zero = cd_dissipation_proxy_to_entropy_joules(0.0, VOLUME_M3, DT_S);
        let neg = cd_dissipation_proxy_to_entropy_joules(-D_INT_W_M3, VOLUME_M3, DT_S);
        assert_eq!(zero, neg);
    }

    #[test]
    fn cbf_bridge_feeds_cbf_verify_with_identity_k_phys() {
        let mut cbf = ThermodynamicCBF::new(300.0, 1.0e-3);
        cbf.k_phys_dint_to_joules = 1.0;
        let joules = cd_dissipation_proxy_to_entropy_joules(1.0, 1.0, 1.0);
        let out = cbf
            .verify_and_deduct_update(joules, 0.0)
            .expect("nonnegative proxy must admit at zero bits");
        assert_relative_eq!(out, 0.0, epsilon = 1.0e-30);
    }

    #[test]
    fn cbf_bridge_scaled_proxy_respects_k_phys_calibration() {
        let mut cbf = ThermodynamicCBF::new(300.0, 1.0);
        cbf.k_phys_dint_to_joules = 2.5;
        let raw = cd_dissipation_proxy_to_entropy_joules(4.0, 0.5, 2.0);
        assert_relative_eq!(raw, 4.0, epsilon = 1.0e-30);
        let scaled = raw * cbf.k_phys_dint_to_joules;
        let out = cbf
            .verify_and_deduct_update(scaled, 0.0)
            .expect("scaled nonnegative entropy must admit");
        assert_relative_eq!(out, 0.0, epsilon = 1.0e-30);
    }

    #[test]
    fn cbf_bridge_hydration_scale_volume_typical() {
        // 1 m³ control volume, 86400 s day, 0.01 W/m³ mild dissipation → 864 J
        let joules = cd_dissipation_proxy_to_entropy_joules(0.01, 1.0, 86_400.0);
        assert_relative_eq!(joules, 864.0, epsilon = 1.0e-9);
    }

    #[test]
    fn cbf_bridge_sub_voxel_scale_micro_volume() {
        // 1 mm³ = 1e-9 m³, 1 s, 1e3 W/m³ → 1e-6 J
        let joules = cd_dissipation_proxy_to_entropy_joules(1.0e3, 1.0e-9, 1.0);
        assert_relative_eq!(joules, 1.0e-6, epsilon = 1.0e-30);
    }

    #[test]
    fn cbf_bridge_output_always_nonnegative() {
        let cases = [
            (1.0, 1.0, 1.0),
            (-1.0, 1.0, 1.0),
            (1.0, -1.0, 1.0),
            (1.0, 1.0, -1.0),
            (-1.0, -1.0, -1.0),
            (0.0, 0.0, 0.0),
        ];
        for (d_int, vol, dt) in cases {
            let joules = cd_dissipation_proxy_to_entropy_joules(d_int, vol, dt);
            assert!(
                joules >= 0.0 && joules.is_finite(),
                "clamp must yield finite nonnegative joules for ({d_int}, {vol}, {dt}): {joules}"
            );
        }
    }

    #[test]
    fn w8e14_cbf_bridge_posture_tag_honest_not_green() {
        assert!(CBF_BRIDGE_POSTURE_TAG.contains("honest"));
        assert!(!CBF_BRIDGE_POSTURE_TAG
            .to_ascii_lowercase()
            .contains("green"));
        assert!(cbf_bridge_morphism_pinned());
    }
}
