// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//!
//! Minimal thermodynamic gate HTTP server (stdlib `TcpListener`, `POST /gate`).
//!
//! **Honest boundary:** cold-edge HTTP shim only — not production gateway wiring,
//! not physics GREEN, not master retick. Router lives in [`umst_manifold::gate_server_router`];
//! evaluator SSOT in [`umst_manifold::gate::http_manifest`].
//!
//! ```text
//! UMST_GATE_ADDR=0.0.0.0:8787 cargo run -p umst-manifold --bin gate_server --features gate-server
//! ```

use std::net::TcpListener;

use umst_manifold::gate::http_manifest::GateHttpRuntime;
use umst_manifold::gate_server_router::handle_connection;
use umst_manifold::manifest::UmstManifest;
use umst_manifold::runtime::catalog::traceability::HTTP_SHIM_CATALOG_ID;

/// W29 deepen cell id (gate_server binary posture).
pub const CELL_ID: &str = "W29-017-GATE_SERVER";

/// Default bind when `UMST_GATE_ADDR` is unset.
pub const DEFAULT_BIND_ADDR: &str = "0.0.0.0:8787";

/// Catalog surface routed by this binary.
pub const CATALOG_ID: &str = HTTP_SHIM_CATALOG_ID;

/// Honest posture tag — HTTP shim landed; production flip blocked.
pub const POSTURE_TAG: &str = "HTTP_SHIM_COLD_EDGE";

/// Router module SSOT (request parsing + response framing).
pub const ROUTER_MODULE: &str = "umst-manifold/src/gate_server_router.rs";

/// Evaluator module SSOT (mix transition + catalog hash).
pub const EVALUATOR_MODULE: &str = "umst-manifold/src/gate/http_manifest.rs";

/// Integration test SSOT (localhost roundtrip; feature `gate-server-bin`).
pub const INTEGRATION_TEST: &str = "umst-manifold/tests/gate_server_http.rs";

/// Honest deepen fence for meta / fleet probes.
pub const HONEST_FENCE: &str =
    "http_shim_landed=true production_wired=false physics_green=false master_retick=false";

/// Whether the stdlib HTTP listener binary is compiled (feature `gate-server-bin`).
pub const HTTP_SHIM_LANDED: bool = true;

/// Whether TLS, auth, rate limits, or fleet gateway routing are wired.
pub const TRANSPORT_HARDENING_LANDED: bool = false;

/// Whether operator may claim physics GREEN from this binary alone.
pub const PHYSICS_GREEN_CLAIM_AUTHORIZED: bool = false;

/// Whether master retick / production fleet clearance is authorized.
pub const MASTER_RETICK_AUTHORIZED: bool = false;

/// Honest production wiring — **false** until SEC-GW-WRAP + operator measure.
#[must_use]
pub const fn gate_server_production_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at cold-edge tier.
const _: () = assert!(!gate_server_production_wired());

/// Typed probe for `gate_server` binary posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateServerPostureProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub catalog_id: &'static str,
    pub default_bind_addr: &'static str,
    pub http_shim_landed: bool,
    pub transport_hardening_landed: bool,
    pub physics_green_claim_authorized: bool,
    pub master_retick_authorized: bool,
    pub production_wired: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for gate_server done-when checks.
#[must_use]
pub const fn gate_server_posture_probe() -> GateServerPostureProbe {
    GateServerPostureProbe {
        cell_id: CELL_ID,
        posture_tag: POSTURE_TAG,
        catalog_id: CATALOG_ID,
        default_bind_addr: DEFAULT_BIND_ADDR,
        http_shim_landed: HTTP_SHIM_LANDED,
        transport_hardening_landed: TRANSPORT_HARDENING_LANDED,
        physics_green_claim_authorized: PHYSICS_GREEN_CLAIM_AUTHORIZED,
        master_retick_authorized: MASTER_RETICK_AUTHORIZED,
        production_wired: gate_server_production_wired(),
        honest_fence: HONEST_FENCE,
    }
}

/// HTTP shim landed with production / GREEN / master claims honestly blocked.
#[must_use]
pub fn gate_server_posture_honest(probe: &GateServerPostureProbe) -> bool {
    probe.cell_id == CELL_ID
        && probe.posture_tag == POSTURE_TAG
        && probe.catalog_id == HTTP_SHIM_CATALOG_ID
        && probe.default_bind_addr == DEFAULT_BIND_ADDR
        && probe.http_shim_landed
        && !probe.transport_hardening_landed
        && !probe.physics_green_claim_authorized
        && !probe.master_retick_authorized
        && !probe.production_wired
        && probe.honest_fence.contains("http_shim_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master_retick=false")
}

/// Validate gate_server posture honesty — fail closed on fake production claims.
pub fn validate_gate_server_posture_honesty() -> Result<(), &'static str> {
    let probe = gate_server_posture_probe();
    if probe.production_wired {
        return Err("gate_server_production_wired must stay false until SEC-GW-WRAP");
    }
    if probe.physics_green_claim_authorized {
        return Err("gate_server must not authorize physics GREEN from HTTP shim alone");
    }
    if probe.master_retick_authorized {
        return Err("gate_server must not authorize master retick from cold-edge binary");
    }
    if !gate_server_posture_honest(&probe) {
        return Err("gate_server_posture_honest failed");
    }
    Ok(())
}

/// Resolve bind address from `UMST_GATE_ADDR` or [`DEFAULT_BIND_ADDR`].
#[must_use]
pub fn resolve_bind_addr() -> String {
    std::env::var("UMST_GATE_ADDR").unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_string())
}

/// Build default HTTP runtime from [`UmstManifest::default`].
#[must_use]
pub fn default_gate_http_runtime() -> GateHttpRuntime {
    GateHttpRuntime::from_umst_manifest(&UmstManifest::default())
}

/// Startup banner for operator logs (stderr).
#[must_use]
pub fn startup_banner(addr: &str, catalog_id: &str) -> String {
    format!(
        "umst-manifold gate_server listening on http://{addr} (POST /gate, GET /health) catalog_id={catalog_id} posture={POSTURE_TAG}"
    )
}

fn main() {
    let addr = resolve_bind_addr();
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| panic!("bind {addr}: {e}"));
    let runtime = default_gate_http_runtime();
    eprintln!(
        "{}",
        startup_banner(&addr, runtime.evaluator.catalog_id())
    );
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_connection(&mut stream, &runtime),
            Err(e) => tracing::warn!("gate_server accept error: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_server_posture_metadata_locked() {
        assert_eq!(CELL_ID, "W29-017-GATE_SERVER");
        assert_eq!(DEFAULT_BIND_ADDR, "0.0.0.0:8787");
        assert_eq!(CATALOG_ID, "umst.gate.http_shim");
        assert_eq!(POSTURE_TAG, "HTTP_SHIM_COLD_EDGE");
        assert!(HTTP_SHIM_LANDED);
        assert!(!TRANSPORT_HARDENING_LANDED);
        assert!(!PHYSICS_GREEN_CLAIM_AUTHORIZED);
        assert!(!MASTER_RETICK_AUTHORIZED);
        assert!(!gate_server_production_wired());
    }

    #[test]
    fn gate_server_posture_module_paths_honest() {
        assert!(ROUTER_MODULE.contains("gate_server_router"));
        assert!(EVALUATOR_MODULE.contains("http_manifest"));
        assert!(INTEGRATION_TEST.contains("gate_server_http"));
        assert_eq!(
            HONEST_FENCE,
            "http_shim_landed=true production_wired=false physics_green=false master_retick=false"
        );
    }

    #[test]
    fn gate_server_posture_probe_http_shim_not_production() {
        let probe = gate_server_posture_probe();
        assert_eq!(probe.cell_id, CELL_ID);
        assert_eq!(probe.catalog_id, HTTP_SHIM_CATALOG_ID);
        assert!(probe.http_shim_landed);
        assert!(!probe.transport_hardening_landed);
        assert!(!probe.physics_green_claim_authorized);
        assert!(!probe.master_retick_authorized);
        assert!(!probe.production_wired);
        assert!(gate_server_posture_honest(&probe));
    }

    #[test]
    fn gate_server_posture_validate_honesty() {
        assert!(validate_gate_server_posture_honesty().is_ok());
        assert!(!gate_server_production_wired());
    }

    #[test]
    fn gate_server_resolve_bind_addr_defaults_without_env() {
        std::env::remove_var("UMST_GATE_ADDR");
        assert_eq!(resolve_bind_addr(), DEFAULT_BIND_ADDR);
    }

    #[test]
    fn gate_server_default_runtime_catalog_id() {
        let runtime = default_gate_http_runtime();
        assert_eq!(runtime.evaluator.catalog_id(), HTTP_SHIM_CATALOG_ID);
    }

    #[test]
    fn gate_server_startup_banner_includes_posture() {
        let banner = startup_banner("127.0.0.1:8787", HTTP_SHIM_CATALOG_ID);
        assert!(banner.contains("POST /gate"));
        assert!(banner.contains(HTTP_SHIM_CATALOG_ID));
        assert!(banner.contains(POSTURE_TAG));
    }
}
