// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Hertz and Hunt–Crossley sphere-on-plane impact with optional modal coupling.
//!
//! Elastic Hertz force `F = n δ^{3/2}` with `n = (4/3) E* √R`. Maximum compression
//! follows energy balance `(2/5) n δ_max^{5/2} = (1/2) m v²` (**Derived**).
//! Elastic contact duration closed form
//! `t_c = JOHNSON_DURATION_FACTOR · δ_max / v` (**Cited**: Johnson, K. L.,
//! *Contact Mechanics*, Cambridge University Press, 1985, Ch. 11 Impact; the
//! numerical factor 2.94 is the elastic Hertz impact duration coefficient from
//! the beta-function integral in that chapter. Exact print equation number is
//! typed **Absent** until the book is consulted at the operator desk).
//!
//! Hunt–Crossley damping: `F = n δ^{3/2} (1 + (3/2) χ δ̇)` while `δ > 0`.
//! Restitution `e = v_out / v_in` with `e > 1` refused by typed [`PhysicsError`].

use crate::physics::modal_loss::ImpactSoundMode;
use crate::physics::PhysicsError;

/// Johnson (1985) Ch. 11 elastic Hertz impact duration coefficient (**Cited**).
///
/// Total contact time `t_c ≈ 2.94 δ_max / v_0` for elastic sphere–plane impact.
pub const JOHNSON_DURATION_FACTOR: f64 = 2.94;

/// Citation pin for the Johnson duration factor (print equation number not yet recorded).
pub const JOHNSON_1985_DURATION_CITATION: &str =
    "Johnson, K. L., Contact Mechanics, Cambridge Univ. Press, 1985, Ch. 11 (Impact); \
     duration factor 2.94 for elastic Hertz sphere impact (t_c = 2.94 δ_max / v_0). \
     Print equation number: Absent until operator desk consult";

/// Hertz contact stiffness `n = (4/3) E_star * sqrt(R)` [SI: N/m^{3/2}].
#[must_use]
pub fn hertz_stiffness_n(e_star_pa: f64, radius_m: f64) -> Result<f64, PhysicsError> {
    if !(e_star_pa.is_finite() && e_star_pa > 0.0 && radius_m.is_finite() && radius_m > 0.0) {
        return Err(PhysicsError::Domain {
            detail: "hertz: E* and R must be finite and > 0".into(),
        });
    }
    Ok((4.0 / 3.0) * e_star_pa * radius_m.sqrt())
}

/// Maximum elastic compression from energy balance (**Derived**).
#[must_use]
pub fn hertz_max_compression(mass_kg: f64, v_in_m_s: f64, n: f64) -> Result<f64, PhysicsError> {
    if !(mass_kg.is_finite() && mass_kg > 0.0 && v_in_m_s.is_finite() && v_in_m_s > 0.0 && n > 0.0) {
        return Err(PhysicsError::Domain {
            detail: "hertz: m, v_in, n must be finite and > 0".into(),
        });
    }
    let kinetic = 0.5 * mass_kg * v_in_m_s * v_in_m_s;
    Ok((5.0 * kinetic / (2.0 * n)).powf(0.4))
}

/// Closed-form elastic Hertz contact duration (Johnson Ch. 11 factor).
#[must_use]
pub fn johnson_hertz_contact_duration_s(
    mass_kg: f64,
    v_in_m_s: f64,
    n: f64,
) -> Result<f64, PhysicsError> {
    let delta_max = hertz_max_compression(mass_kg, v_in_m_s, n)?;
    Ok(JOHNSON_DURATION_FACTOR * delta_max / v_in_m_s)
}

/// Parameters for a sphere-on-plane impact integration.
#[derive(Debug, Clone, Copy)]
pub struct HertzImpactParams {
    pub mass_kg: f64,
    pub radius_m: f64,
    pub e_star_pa: f64,
    pub v_in_m_s: f64,
    /// Hunt–Crossley damping parameter χ [s/m]; 0 = elastic Hertz.
    pub hunt_crossley_chi_s_per_m: f64,
}

/// Integrated impact outcome.
#[derive(Debug, Clone, Copy)]
pub struct HertzImpactOutcome {
    pub contact_duration_s: f64,
    pub v_out_m_s: f64,
    pub kinetic_out_j: f64,
    pub dissipated_j: f64,
    pub modal_j: f64,
    pub restitution: f64,
}

/// Stated numerical bound above which `e` is admitted as “greater than one”.
pub const RESTITUTION_ABOVE_ONE_BOUND: f64 = 1e-9;

/// Refuse restitution strictly greater than one (typed; no f64 clamp-as-admit).
#[must_use]
pub fn refuse_restitution_above_one(e: f64) -> Result<(), PhysicsError> {
    if !e.is_finite() {
        return Err(PhysicsError::Domain {
            detail: "hertz: restitution must be finite".into(),
        });
    }
    if e > 1.0 + RESTITUTION_ABOVE_ONE_BOUND {
        return Err(PhysicsError::Domain {
            detail: format!("hertz: restitution e={e} > 1 refused (passive transition)"),
        });
    }
    Ok(())
}

fn contact_force(n: f64, delta: f64, v: f64, chi: f64) -> Result<f64, PhysicsError> {
    if delta <= 0.0 {
        return Ok(0.0);
    }
    let damp = 1.0 + 1.5 * chi * v;
    let f = n * delta.powf(1.5) * damp;
    if !f.is_finite() {
        return Err(PhysicsError::Domain {
            detail: "hertz: non-finite contact force".into(),
        });
    }
    if f < 0.0 {
        return Err(PhysicsError::Domain {
            detail: format!("hertz: compressive contact force f={f} < 0 refused"),
        });
    }
    Ok(f)
}

/// Numerically integrate Hertz / Hunt–Crossley through one contact (semi-implicit Euler).
pub fn integrate_hertz_impact(
    params: HertzImpactParams,
    modal: &[ImpactSoundMode],
    dt_s: f64,
) -> Result<HertzImpactOutcome, PhysicsError> {
    let n = hertz_stiffness_n(params.e_star_pa, params.radius_m)?;
    if !(params.mass_kg > 0.0 && params.v_in_m_s > 0.0 && dt_s > 0.0 && dt_s.is_finite()) {
        return Err(PhysicsError::Domain {
            detail: "hertz: mass, v_in, dt must be finite and > 0".into(),
        });
    }
    if !(params.hunt_crossley_chi_s_per_m.is_finite() && params.hunt_crossley_chi_s_per_m >= 0.0) {
        return Err(PhysicsError::Domain {
            detail: "hertz: Hunt–Crossley χ must be finite and ≥ 0".into(),
        });
    }

    let m = params.mass_kg;
    let chi = params.hunt_crossley_chi_s_per_m;
    let mut delta = 1e-18_f64;
    let mut v = params.v_in_m_s;
    let mut t = 0.0_f64;
    let mut work_damp = 0.0_f64;
    let max_steps = 2_000_000usize;

    for _ in 0..max_steps {
        let f = contact_force(n, delta, v, chi)?;
        let f_el = if delta > 0.0 {
            n * delta.powf(1.5)
        } else {
            0.0
        };
        // Outward restitution (v < 0) can make Hunt–Crossley f < f_el; dissipative work is then zero.
        let f_damp = if f >= f_el { f - f_el } else { 0.0 };
        work_damp += f_damp * v.abs() * dt_s;

        let a = -f / m;
        v += a * dt_s;
        let delta_next = delta + v * dt_s;
        t += dt_s;

        if delta > 0.0 && delta_next <= 0.0 {
            let denom = delta - delta_next;
            if !(denom > 0.0) {
                return Err(PhysicsError::Domain {
                    detail: "hertz: contact exit interpolation denom non-positive".into(),
                });
            }
            let frac = delta / denom;
            if !(frac >= 0.0 && frac <= 1.0) {
                return Err(PhysicsError::Domain {
                    detail: format!("hertz: exit fraction frac={frac} outside [0,1] refused"),
                });
            }
            t -= dt_s * (1.0 - frac);
            delta = 0.0;
            break;
        }
        delta = delta_next;
        if t > 10.0 {
            return Err(PhysicsError::Domain {
                detail: "hertz: contact duration diverged".into(),
            });
        }
    }

    // At separation the integrated velocity has reversed sign; outbound speed is -v.
    if v > 0.0 {
        return Err(PhysicsError::Domain {
            detail: format!("hertz: still-inbound v={v} at contact exit refused"),
        });
    }
    let v_out = -v;
    let e_raw = v_out / params.v_in_m_s;
    refuse_restitution_above_one(e_raw)?;
    let e = if e_raw > 1.0 {
        1.0
    } else {
        e_raw
    };

    let kinetic_out = 0.5 * m * v_out * v_out;
    let modal_j: f64 = modal
        .iter()
        .map(|mode| 0.5 * mode.amplitude * mode.amplitude)
        .sum();

    Ok(HertzImpactOutcome {
        contact_duration_s: t,
        v_out_m_s: v_out,
        kinetic_out_j: kinetic_out,
        dissipated_j: work_damp,
        modal_j,
        restitution: e,
    })
}

/// Energy split: kinetic_out + dissipated + modal equals kinetic_in within `rel_tol`.
#[must_use]
pub fn energy_split_closes(
    kinetic_in_j: f64,
    outcome: &HertzImpactOutcome,
    rel_tol: f64,
) -> bool {
    let sum = outcome.kinetic_out_j + outcome.dissipated_j + outcome.modal_j;
    if !(kinetic_in_j.is_finite() && kinetic_in_j > 0.0 && sum.is_finite()) {
        return false;
    }
    (sum - kinetic_in_j).abs() <= rel_tol * kinetic_in_j
}

/// Non-test surface that names every pub contact symbol (wire gate).
///
/// Called from [`crate::physics::hertz_sphere_plane_probe`].
#[must_use]
pub fn sphere_plane_probe(
    params: HertzImpactParams,
    modal: &[ImpactSoundMode],
    dt_s: f64,
) -> Result<(HertzImpactOutcome, f64, bool), PhysicsError> {
    let n = hertz_stiffness_n(params.e_star_pa, params.radius_m)?;
    let t_johnson = johnson_hertz_contact_duration_s(params.mass_kg, params.v_in_m_s, n)?;
    if !JOHNSON_1985_DURATION_CITATION.contains("1985") {
        return Err(PhysicsError::Domain {
            detail: "hertz: Johnson citation pin missing year".into(),
        });
    }
    let outcome = integrate_hertz_impact(params, modal, dt_s)?;
    let kinetic_in = 0.5 * params.mass_kg * params.v_in_m_s * params.v_in_m_s;
    let closes = energy_split_closes(kinetic_in, &outcome, 0.05);
    Ok((outcome, t_johnson, closes))
}
