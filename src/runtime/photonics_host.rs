// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Host / CLI IO boundary for photonics DEC patch policy.
//!
//! [`crate::physics::solvers::PhotonicsDecPatchConfig`] is pure config with **no `std::env`**
//! reads in `src/physics/**`. Binaries, cartridge runners, and future `umst-cli` subcommands
//! should call [`photonics_dec_patch_config_from_env`] (or the pure parsers) here and inject
//! the result into [`crate::physics::solvers::PhotonicsSolver::dec_patch_config`].
//!
//! **W29-128 deepen:** curl-constitutive env parse + host honesty fence. Not physics GREEN,
//! not `PRODUCTION_WIRED`, not `MASTER` / OP-5 — host IO only; matrix **#6** stays partial
//! on the physics lane (`photonics_lane_fence_holds`).

#[cfg(feature = "photonics")]
use crate::physics::solvers::photonics::{photonics_lane_fence_holds, DecPatchCurlConstitutive};
#[cfg(feature = "photonics")]
use crate::physics::solvers::{DecPatchCsrInnerMode, PhotonicsDecPatchConfig, PhotonicsSolver};

/// W29 deepen cell — photonics host IO honest fence bundle.
pub const W29_128_PHOTONICS_HOST_DEEPEN_STEP: &str = "W29-128-PHOTONICS_HOST";

/// Honest physics posture — host parsers do not close the Maxwell lane.
#[cfg(feature = "photonics")]
pub const PHOTONICS_HOST_PHYSICS_GREEN: bool = false;

/// Honest production posture — env injection is not a production wire.
#[cfg(feature = "photonics")]
pub const PHOTONICS_HOST_PRODUCTION_WIRED: bool = false;

/// Honest MASTER retick eligibility — always refused at this module.
#[cfg(feature = "photonics")]
pub const PHOTONICS_HOST_MASTER: bool = false;

/// Honest OP-5 claim — always refused at this module.
#[cfg(feature = "photonics")]
pub const PHOTONICS_HOST_OP5: bool = false;

/// Compile-time refuse invent flags.
#[cfg(feature = "photonics")]
const _: () = assert!(!PHOTONICS_HOST_PHYSICS_GREEN);
#[cfg(feature = "photonics")]
const _: () = assert!(!PHOTONICS_HOST_PRODUCTION_WIRED);
#[cfg(feature = "photonics")]
const _: () = assert!(!PHOTONICS_HOST_MASTER);
#[cfg(feature = "photonics")]
const _: () = assert!(!PHOTONICS_HOST_OP5);

/// Env key: force Krylov / skip dense Gauss–Jordan fallback (`"1"` only).
#[cfg(feature = "photonics")]
pub const ENV_FORCE_KRYLOV: &str = "UMST_PHOTONICS_DEC_PATCH_FORCE_KRYLOV";

/// Env key: CSR inner solve policy (`auto` / `on` / `off` aliases).
#[cfg(feature = "photonics")]
pub const ENV_CSR_INNER: &str = "UMST_PHOTONICS_DEC_PATCH_CSR_INNER";

/// Env key: Whitney curl constitutive on `[B,N,9]` (`eps_sym` default / `eps_inv`).
#[cfg(feature = "photonics")]
pub const ENV_CURL_CONSTITUTIVE: &str = "UMST_PHOTONICS_DEC_PATCH_CURL_CONSTITUTIVE";

/// Locked honest-fence string (no invent GREEN / PRODUCTION_WIRED / MASTER / OP-5).
#[cfg(feature = "photonics")]
pub const PHOTONICS_HOST_HONEST_FENCE: &str = concat!(
    "dec_patch_env_parsers_landed=true",
    "|curl_constitutive_env_landed=true",
    "|host_io_boundary=true",
    "|physics_core_env_free=true",
    "|matrix_six_closed=false",
    "|production_wired=false",
    "|physics_green=false",
    "|master=false",
    "|op5=false",
);

/// Measured honesty posture for the photonics host IO surface.
#[cfg(feature = "photonics")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhotonicsHostHonestyFence {
    pub deepen_step: &'static str,
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub honest_fence: &'static str,
    pub env_keys_landed: bool,
    pub curl_constitutive_parser_landed: bool,
}

#[cfg(feature = "photonics")]
impl PhotonicsHostHonestyFence {
    /// Measured honesty posture for this write_set.
    #[must_use]
    pub const fn measured() -> Self {
        Self {
            deepen_step: W29_128_PHOTONICS_HOST_DEEPEN_STEP,
            physics_green: PHOTONICS_HOST_PHYSICS_GREEN,
            production_wired: PHOTONICS_HOST_PRODUCTION_WIRED,
            master: PHOTONICS_HOST_MASTER,
            op5: PHOTONICS_HOST_OP5,
            honest_fence: PHOTONICS_HOST_HONEST_FENCE,
            env_keys_landed: true,
            curl_constitutive_parser_landed: true,
        }
    }

    /// Fence holds when invent flags stay false and deepen witnesses remain true.
    #[must_use]
    pub const fn holds(self) -> bool {
        !self.physics_green
            && !self.production_wired
            && !self.master
            && !self.op5
            && self.env_keys_landed
            && self.curl_constitutive_parser_landed
            && !self.deepen_step.is_empty()
            && fence_string_honest(self.honest_fence)
    }
}

/// Structural check on the locked fence string (no invent theater bits).
#[cfg(feature = "photonics")]
#[must_use]
pub const fn fence_string_honest(fence: &str) -> bool {
    // `str::contains` is not const-stable on all MSRV paths we care about for asserts;
    // keep a byte-scan that refuses invent tokens.
    !const_str_contains(fence, "production_wired=true")
        && !const_str_contains(fence, "physics_green=true")
        && !const_str_contains(fence, "master=true")
        && !const_str_contains(fence, "op5=true")
        && !const_str_contains(fence, "matrix_six_closed=true")
        && const_str_contains(fence, "curl_constitutive_env_landed=true")
        && const_str_contains(fence, "host_io_boundary=true")
}

#[cfg(feature = "photonics")]
const fn const_str_contains(hay: &str, needle: &str) -> bool {
    let h = hay.as_bytes();
    let n = needle.as_bytes();
    if n.is_empty() {
        return true;
    }
    if h.len() < n.len() {
        return false;
    }
    let mut i = 0;
    while i + n.len() <= h.len() {
        let mut j = 0;
        let mut ok = true;
        while j < n.len() {
            if h[i + j] != n[j] {
                ok = false;
                break;
            }
            j += 1;
        }
        if ok {
            return true;
        }
        i += 1;
    }
    false
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER / OP-5 invent on the host surface.
#[cfg(feature = "photonics")]
pub fn validate_photonics_host_honesty() -> Result<(), &'static str> {
    let probe = PhotonicsHostHonestyFence::measured();
    if PHOTONICS_HOST_PRODUCTION_WIRED || probe.production_wired {
        return Err("PHOTONICS_HOST_PRODUCTION_WIRED must stay false — host IO only");
    }
    if PHOTONICS_HOST_PHYSICS_GREEN || probe.physics_green {
        return Err("PHOTONICS_HOST_PHYSICS_GREEN must stay false — matrix #6 partial");
    }
    if PHOTONICS_HOST_MASTER || probe.master {
        return Err("PHOTONICS_HOST_MASTER must stay false — no invent MASTER");
    }
    if PHOTONICS_HOST_OP5 || probe.op5 {
        return Err("PHOTONICS_HOST_OP5 must stay false — no invent OP-5");
    }
    if !probe.holds() {
        return Err("photonics_host honest fence failed structural check");
    }
    if !probe.honest_fence.contains("production_wired=false")
        || !probe.honest_fence.contains("physics_green=false")
        || !probe.honest_fence.contains("master=false")
        || !probe.honest_fence.contains("op5=false")
        || !probe.honest_fence.contains("matrix_six_closed=false")
    {
        return Err("photonics_host honest_fence missing refuse bits");
    }
    Ok(())
}

/// Honesty probe — fence holds; pure parsers round-trip default config.
#[cfg(feature = "photonics")]
#[must_use]
pub fn photonics_host_honesty_probe() -> bool {
    if validate_photonics_host_honesty().is_err() {
        return false;
    }
    let cfg = photonics_dec_patch_config_from_raw(None, None, None);
    !cfg.force_krylov
        && cfg.csr_inner == DecPatchCsrInnerMode::Auto
        && cfg.curl_constitutive == DecPatchCurlConstitutive::EpsSymAvg
}

/// Parse `UMST_PHOTONICS_DEC_PATCH_FORCE_KRYLOV` (absent → `false`).
///
/// Only the literal `"1"` forces Krylov / skips dense Gauss–Jordan fallback — matches the
/// pre-`bb40684` physics-core env contract.
#[cfg(feature = "photonics")]
#[must_use]
pub fn parse_force_krylov_env(value: Option<&str>) -> bool {
    matches!(value, Some("1"))
}

/// Parse `UMST_PHOTONICS_DEC_PATCH_CSR_INNER` (absent → [`DecPatchCsrInnerMode::Auto`]).
#[cfg(feature = "photonics")]
#[must_use]
pub fn parse_csr_inner_env(value: Option<&str>) -> DecPatchCsrInnerMode {
    match value {
        Some(s) if s == "0" || s.eq_ignore_ascii_case("off") || s.eq_ignore_ascii_case("false") => {
            DecPatchCsrInnerMode::Off
        }
        Some(s)
            if s == "1"
                || s.eq_ignore_ascii_case("on")
                || s.eq_ignore_ascii_case("true")
                || s.eq_ignore_ascii_case("force") =>
        {
            DecPatchCsrInnerMode::On
        }
        _ => DecPatchCsrInnerMode::Auto,
    }
}

/// Parse `UMST_PHOTONICS_DEC_PATCH_CURL_CONSTITUTIVE`
/// (absent / unknown → [`DecPatchCurlConstitutive::EpsSymAvg`]).
///
/// Accepts `eps_sym` / `sym` / `forward` → [`EpsSymAvg`];
/// `eps_inv` / `inv` / `eps-inv` / `eps_inv_sym_avg` → [`EpsInvSymAvg`].
#[cfg(feature = "photonics")]
#[must_use]
pub fn parse_curl_constitutive_env(value: Option<&str>) -> DecPatchCurlConstitutive {
    match value {
        Some(s)
            if s.eq_ignore_ascii_case("eps_inv")
                || s.eq_ignore_ascii_case("inv")
                || s.eq_ignore_ascii_case("eps-inv")
                || s.eq_ignore_ascii_case("eps_inv_sym_avg")
                || s.eq_ignore_ascii_case("EpsInvSymAvg") =>
        {
            DecPatchCurlConstitutive::EpsInvSymAvg
        }
        _ => DecPatchCurlConstitutive::EpsSymAvg,
    }
}

/// Pure assembly from raw env string slots (no `std::env` — unit-testable).
#[cfg(feature = "photonics")]
#[must_use]
pub fn photonics_dec_patch_config_from_raw(
    force_krylov: Option<&str>,
    csr_inner: Option<&str>,
    curl_constitutive: Option<&str>,
) -> PhotonicsDecPatchConfig {
    PhotonicsDecPatchConfig {
        force_krylov: parse_force_krylov_env(force_krylov),
        csr_inner: parse_csr_inner_env(csr_inner),
        curl_constitutive: parse_curl_constitutive_env(curl_constitutive),
    }
}

/// Read `UMST_PHOTONICS_DEC_PATCH_*` from the process environment.
#[cfg(feature = "photonics")]
#[must_use]
pub fn photonics_dec_patch_config_from_env() -> PhotonicsDecPatchConfig {
    photonics_dec_patch_config_from_raw(
        std::env::var(ENV_FORCE_KRYLOV).ok().as_deref(),
        std::env::var(ENV_CSR_INNER).ok().as_deref(),
        std::env::var(ENV_CURL_CONSTITUTIVE).ok().as_deref(),
    )
}

/// Construct a [`PhotonicsSolver`] with host-injected DEC patch policy.
#[cfg(feature = "photonics")]
#[must_use]
pub fn photonics_solver_from_env(frequency_hz: f32) -> PhotonicsSolver {
    PhotonicsSolver {
        frequency_hz,
        dec_patch_config: photonics_dec_patch_config_from_env(),
    }
}

/// Inject a pure [`PhotonicsDecPatchConfig`] into a frequency-bearing solver shell.
#[cfg(feature = "photonics")]
#[must_use]
pub fn photonics_solver_with_dec_patch(
    frequency_hz: f32,
    dec_patch_config: PhotonicsDecPatchConfig,
) -> PhotonicsSolver {
    PhotonicsSolver {
        frequency_hz,
        dec_patch_config,
    }
}

#[cfg(all(test, feature = "photonics"))]
mod tests {
    use super::*;

    #[test]
    fn force_krylov_only_literal_one() {
        assert!(!parse_force_krylov_env(None));
        assert!(!parse_force_krylov_env(Some("0")));
        assert!(!parse_force_krylov_env(Some("true")));
        assert!(!parse_force_krylov_env(Some("")));
        assert!(parse_force_krylov_env(Some("1")));
    }

    #[test]
    fn csr_inner_env_aliases_match_legacy_physics() {
        assert_eq!(parse_csr_inner_env(None), DecPatchCsrInnerMode::Auto);
        assert_eq!(
            parse_csr_inner_env(Some("auto")),
            DecPatchCsrInnerMode::Auto
        );
        assert_eq!(parse_csr_inner_env(Some("OFF")), DecPatchCsrInnerMode::Off);
        assert_eq!(parse_csr_inner_env(Some("0")), DecPatchCsrInnerMode::Off);
        assert_eq!(
            parse_csr_inner_env(Some("false")),
            DecPatchCsrInnerMode::Off
        );
        assert_eq!(parse_csr_inner_env(Some("on")), DecPatchCsrInnerMode::On);
        assert_eq!(parse_csr_inner_env(Some("1")), DecPatchCsrInnerMode::On);
        assert_eq!(parse_csr_inner_env(Some("force")), DecPatchCsrInnerMode::On);
        assert_eq!(
            parse_csr_inner_env(Some("weird")),
            DecPatchCsrInnerMode::Auto
        );
    }

    #[test]
    fn curl_constitutive_env_aliases() {
        assert_eq!(
            parse_curl_constitutive_env(None),
            DecPatchCurlConstitutive::EpsSymAvg
        );
        assert_eq!(
            parse_curl_constitutive_env(Some("eps_sym")),
            DecPatchCurlConstitutive::EpsSymAvg
        );
        assert_eq!(
            parse_curl_constitutive_env(Some("forward")),
            DecPatchCurlConstitutive::EpsSymAvg
        );
        assert_eq!(
            parse_curl_constitutive_env(Some("eps_inv")),
            DecPatchCurlConstitutive::EpsInvSymAvg
        );
        assert_eq!(
            parse_curl_constitutive_env(Some("EPS-INV")),
            DecPatchCurlConstitutive::EpsInvSymAvg
        );
        assert_eq!(
            parse_curl_constitutive_env(Some("inv")),
            DecPatchCurlConstitutive::EpsInvSymAvg
        );
        assert_eq!(
            parse_curl_constitutive_env(Some("eps_inv_sym_avg")),
            DecPatchCurlConstitutive::EpsInvSymAvg
        );
        assert_eq!(
            parse_curl_constitutive_env(Some("nope")),
            DecPatchCurlConstitutive::EpsSymAvg
        );
    }

    #[test]
    fn from_raw_wires_all_three_knobs() {
        let cfg = photonics_dec_patch_config_from_raw(Some("1"), Some("off"), Some("eps_inv"));
        assert!(cfg.force_krylov);
        assert_eq!(cfg.csr_inner, DecPatchCsrInnerMode::Off);
        assert_eq!(
            cfg.curl_constitutive,
            DecPatchCurlConstitutive::EpsInvSymAvg
        );
    }

    #[test]
    fn from_raw_defaults_match_physics_default() {
        let cfg = photonics_dec_patch_config_from_raw(None, None, None);
        let def = PhotonicsDecPatchConfig::default();
        assert_eq!(cfg.force_krylov, def.force_krylov);
        assert_eq!(cfg.csr_inner, def.csr_inner);
        assert_eq!(cfg.curl_constitutive, def.curl_constitutive);
    }

    #[test]
    fn from_env_defaults_without_vars() {
        // Pure path — do not mutate process env in unit tests.
        let cfg = photonics_dec_patch_config_from_raw(None, None, None);
        assert!(!cfg.force_krylov);
        assert_eq!(cfg.csr_inner, DecPatchCsrInnerMode::Auto);
        assert_eq!(cfg.curl_constitutive, DecPatchCurlConstitutive::EpsSymAvg);
    }

    #[test]
    fn solver_with_dec_patch_injects_config() {
        let cfg = photonics_dec_patch_config_from_raw(Some("1"), Some("on"), Some("eps_inv"));
        let solver = photonics_solver_with_dec_patch(2.45e9, cfg);
        assert_eq!(solver.frequency_hz, 2.45e9);
        assert!(solver.dec_patch_config.force_krylov);
        assert_eq!(solver.dec_patch_config.csr_inner, DecPatchCsrInnerMode::On);
        assert_eq!(
            solver.dec_patch_config.curl_constitutive,
            DecPatchCurlConstitutive::EpsInvSymAvg
        );
    }

    #[test]
    fn env_key_constants_are_stable() {
        assert_eq!(ENV_FORCE_KRYLOV, "UMST_PHOTONICS_DEC_PATCH_FORCE_KRYLOV");
        assert_eq!(ENV_CSR_INNER, "UMST_PHOTONICS_DEC_PATCH_CSR_INNER");
        assert_eq!(
            ENV_CURL_CONSTITUTIVE,
            "UMST_PHOTONICS_DEC_PATCH_CURL_CONSTITUTIVE"
        );
    }

    #[test]
    fn w29_128_honest_fence_refuses_green_production_master_op5() {
        validate_photonics_host_honesty().expect("photonics_host honest fence");
        let probe = PhotonicsHostHonestyFence::measured();
        assert!(probe.holds());
        assert!(photonics_host_honesty_probe());
        assert!(!PHOTONICS_HOST_PHYSICS_GREEN);
        assert!(!PHOTONICS_HOST_PRODUCTION_WIRED);
        assert!(!PHOTONICS_HOST_MASTER);
        assert!(!PHOTONICS_HOST_OP5);
        assert_eq!(probe.deepen_step, W29_128_PHOTONICS_HOST_DEEPEN_STEP);
        assert!(!probe.honest_fence.contains("production_wired=true"));
        assert!(!probe.honest_fence.contains("physics_green=true"));
        assert!(!probe.honest_fence.contains("master=true"));
        assert!(!probe.honest_fence.contains("op5=true"));
        // Physics lane fence remains partial (host does not invent matrix-six close).
        assert!(photonics_lane_fence_holds());
    }

    #[test]
    fn honest_fence_string_locked() {
        assert!(fence_string_honest(PHOTONICS_HOST_HONEST_FENCE));
        assert!(!fence_string_honest(
            "production_wired=true|physics_green=false"
        ));
        assert!(!fence_string_honest(
            "matrix_six_closed=true|host_io_boundary=true|curl_constitutive_env_landed=true"
        ));
    }
}
