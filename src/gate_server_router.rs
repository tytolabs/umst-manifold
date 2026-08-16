// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Minimal HTTP/1.1 handler for [`crate::gate::http_manifest`] (stdlib only; one request per connection).
//!
//! **Policy:** stdlib `TcpStream` shim only — routes `GET /health` and `POST /gate` into
//! [`GateHttpRuntime`]; TLS, keep-alive, auth, and multi-request pipelining stay **honest open**.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::gate::http_manifest::{
    gate_json_parse_response, pinned_catalog_bundle_sha256_hex, GateHttpRuntime, MixProposal,
};
use crate::runtime::catalog::traceability::HTTP_SHIM_CATALOG_ID;

/// W29 deepen cell id (stdlib HTTP gate router).
pub const GATE_SERVER_ROUTER_CELL_ID: &str = "W29-041-GATE_SERVER_ROUTER";

/// Honest posture — stdlib shim only; no GREEN / production invent (`MASTER_RETICK=no`).
pub const GATE_SERVER_ROUTER_POSTURE_TAG: &str = "honest-stdlib-http-shim-only";

/// Census schema version for router wire map.
pub const GATE_SERVER_ROUTER_SCHEMA_VERSION: &str = "gate_server_router_wire_census_v1";

/// Pinned health probe path (`gate_server` binary).
pub const ROUTE_HEALTH: &str = "/health";

/// Pinned bulk gate eval path (`gate_server` binary).
pub const ROUTE_GATE: &str = "/gate";

/// One hop in the stdlib HTTP gate router wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateServerRouterWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold stdlib HTTP gate router wire map (cold-edge `TcpStream` → `GateHttpRuntime`).
pub const GATE_SERVER_ROUTER_WIRE_HOPS: &[GateServerRouterWireHop] = &[
    GateServerRouterWireHop {
        ordinal: 1,
        surface: "umst-manifold::gate_server_router::read_request",
        role: "HTTP/1.1 request line + Content-Length body parse",
        wired: true,
    },
    GateServerRouterWireHop {
        ordinal: 2,
        surface: "umst-manifold::gate_server_router::classify_request_line",
        role: "Route discriminant (`/health`, `/gate`, not found)",
        wired: true,
    },
    GateServerRouterWireHop {
        ordinal: 3,
        surface: "umst-manifold::gate::http_manifest::GateHttpRuntime::evaluate_transition",
        role: "Bulk mix proposal → `GateResponse` JSON",
        wired: true,
    },
    GateServerRouterWireHop {
        ordinal: 4,
        surface: "umst-manifold::gate::http_manifest::gate_json_parse_response",
        role: "Malformed JSON fail-closed parse witness",
        wired: true,
    },
    GateServerRouterWireHop {
        ordinal: 5,
        surface: "umst-manifold::bin::gate_server (feature gate-server-bin)",
        role: "Threaded `TcpListener` accept loop",
        wired: true,
    },
    GateServerRouterWireHop {
        ordinal: 6,
        surface: "TLS / auth / keep-alive / HTTP/2",
        role: "Production edge hardening — honest open",
        wired: false,
    },
];

/// Classified HTTP route for a single request line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateHttpRoute {
    /// `GET /health` liveness probe.
    Health,
    /// `POST /gate` bulk mix evaluation.
    Gate,
    /// Any other method/path combination.
    NotFound,
}

/// Classify an HTTP/1.1 request line into a pinned route (case-insensitive method + path).
#[must_use]
pub fn classify_request_line(request_line: &str) -> GateHttpRoute {
    let line = request_line.trim();
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_ascii_uppercase();
    let path = parts.next().unwrap_or("").to_ascii_uppercase();
    if method == "GET" && (path == "/HEALTH" || path.ends_with("/HEALTH")) {
        return GateHttpRoute::Health;
    }
    if method == "POST" && path == "/GATE" {
        return GateHttpRoute::Gate;
    }
    GateHttpRoute::NotFound
}

/// Live production edge (TLS, auth, keep-alive) wired — honest false.
#[must_use]
pub const fn gate_server_router_production_wired() -> bool {
    false
}

/// Whether router metadata is pinned @ HEAD (visibility only; no GREEN invent).
#[must_use]
pub fn gate_server_router_morphism_pinned() -> bool {
    GATE_SERVER_ROUTER_CELL_ID == "W29-041-GATE_SERVER_ROUTER"
        && GATE_SERVER_ROUTER_POSTURE_TAG == "honest-stdlib-http-shim-only"
        && ROUTE_HEALTH == "/health"
        && ROUTE_GATE == "/gate"
        && GATE_SERVER_ROUTER_WIRE_HOPS.len() == 6
        && !gate_server_router_production_wired()
}

/// Blocking request/response exchange (suitable for threaded `TcpListener::accept` loops).
pub fn handle_connection(stream: &mut TcpStream, runtime: &GateHttpRuntime) {
    let peer = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown peer".to_string());
    let reply = match read_request(stream) {
        Ok(req) => build_response(&req, runtime),
        Err(e) => {
            tracing::warn!("gate HTTP read error from {peer}: {e}");
            let hash = pinned_catalog_bundle_sha256_hex();
            let body = serde_json::json!({
                "admissible": false,
                "codes": ["HTTP_BAD_REQUEST"],
                "catalog_id": HTTP_SHIM_CATALOG_ID,
                "catalog_hash_hex": hash,
            })
            .to_string();
            http_payload("400 Bad Request", &body)
        }
    };
    let _ = stream.write_all(&reply);
    let _ = stream.flush();
}

struct ParsedRequest {
    request_line: String,
    body: Vec<u8>,
}

fn read_request(stream: &mut TcpStream) -> std::io::Result<ParsedRequest> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let mut headers = Vec::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        headers.push(line);
    }

    let content_length: usize = headers
        .iter()
        .find_map(|h| {
            let raw = h.as_str().trim_end_matches(['\r', '\n']);
            let lower = raw.to_ascii_lowercase();
            if let Some(rest) = lower.strip_prefix("content-length:") {
                rest.trim().parse().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }

    Ok(ParsedRequest { request_line, body })
}

fn build_response(req: &ParsedRequest, runtime: &GateHttpRuntime) -> Vec<u8> {
    let hash = pinned_catalog_bundle_sha256_hex();

    match classify_request_line(&req.request_line) {
        GateHttpRoute::Health => {
            let b = br#"{"status":"ok"}"#;
            return http_payload_json(b);
        }
        GateHttpRoute::Gate => {
            let body_str = String::from_utf8_lossy(&req.body);
            let json = match serde_json::from_str::<MixProposal>(body_str.as_ref()) {
                Ok(p) => serde_json::to_string(&runtime.evaluate_transition(&p)).unwrap(),
                Err(_) => serde_json::to_string(&gate_json_parse_response()).unwrap(),
            };
            return http_payload("200 OK", &json);
        }
        GateHttpRoute::NotFound => {
            let body =
                serde_json::json!({ "error": "not found", "catalog_hash_hex": hash }).to_string();
            http_payload("404 Not Found", &body)
        }
    }
}

fn http_payload_json(body: &[u8]) -> Vec<u8> {
    let mut out = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        body.len()
    )
    .into_bytes();
    out.extend_from_slice(body);
    out
}

fn http_payload(status: &str, body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::http_manifest::GateHttpRuntime;
    use crate::manifest::UmstManifest;

    fn runtime() -> GateHttpRuntime {
        GateHttpRuntime::from_umst_manifest(&UmstManifest::default())
    }

    fn parsed(line: &str, body: &[u8]) -> ParsedRequest {
        ParsedRequest {
            request_line: line.to_string(),
            body: body.to_vec(),
        }
    }

    fn status_from_payload(payload: &[u8]) -> u16 {
        let head = std::str::from_utf8(&payload[..payload.len().min(128)])
            .expect("HTTP head must be UTF-8");
        head.split_whitespace()
            .nth(1)
            .unwrap_or("0")
            .parse()
            .unwrap_or(0)
    }

    fn body_from_payload(payload: &[u8]) -> String {
        let text = std::str::from_utf8(payload).expect("payload UTF-8");
        let (_, body) = text.split_once("\r\n\r\n").expect("HTTP body separator");
        body.to_string()
    }

    #[test]
    fn gate_server_router_morphism_identity_pinned() {
        assert!(gate_server_router_morphism_pinned());
        assert_eq!(GATE_SERVER_ROUTER_CELL_ID, "W29-041-GATE_SERVER_ROUTER");
        assert_eq!(
            GATE_SERVER_ROUTER_SCHEMA_VERSION,
            "gate_server_router_wire_census_v1"
        );
    }

    #[test]
    fn gate_server_router_posture_tag_honest_not_green() {
        assert!(GATE_SERVER_ROUTER_POSTURE_TAG.contains("honest"));
        assert!(!GATE_SERVER_ROUTER_POSTURE_TAG
            .to_ascii_lowercase()
            .contains("green"));
        assert!(!GATE_SERVER_ROUTER_POSTURE_TAG.contains("production"));
    }

    #[test]
    fn gate_server_router_production_wired_stays_false() {
        assert!(!gate_server_router_production_wired());
        assert!(GATE_SERVER_ROUTER_WIRE_HOPS
            .iter()
            .any(|h| !h.wired && h.surface.contains("TLS")));
    }

    #[test]
    fn classify_request_line_health_case_insensitive() {
        assert_eq!(
            classify_request_line("GET /health HTTP/1.1"),
            GateHttpRoute::Health
        );
        assert_eq!(
            classify_request_line("get /health HTTP/1.1"),
            GateHttpRoute::Health
        );
        assert_eq!(
            classify_request_line("GET /v1/health HTTP/1.1"),
            GateHttpRoute::Health
        );
    }

    #[test]
    fn classify_request_line_gate_post_only() {
        assert_eq!(
            classify_request_line("POST /gate HTTP/1.1"),
            GateHttpRoute::Gate
        );
        assert_eq!(
            classify_request_line("GET /gate HTTP/1.1"),
            GateHttpRoute::NotFound
        );
        assert_eq!(
            classify_request_line("POST /other HTTP/1.1"),
            GateHttpRoute::NotFound
        );
    }

    #[test]
    fn build_response_health_ok_json() {
        let payload = build_response(&parsed("GET /health HTTP/1.1", b""), &runtime());
        assert_eq!(status_from_payload(&payload), 200);
        let body: serde_json::Value = serde_json::from_str(&body_from_payload(&payload)).unwrap();
        assert_eq!(body["status"], "ok");
    }

    #[test]
    fn build_response_gate_admits_valid_mix() {
        let body = r#"{"cement":400,"water":200,"age":28,"predicted_strength":25}"#;
        let payload = build_response(&parsed("POST /gate HTTP/1.1", body.as_bytes()), &runtime());
        assert_eq!(status_from_payload(&payload), 200);
        let json: serde_json::Value = serde_json::from_str(&body_from_payload(&payload)).unwrap();
        assert_eq!(json["admissible"], true);
        assert!(json["catalog_hash_hex"]
            .as_str()
            .map(|s| s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()))
            .unwrap_or(false));
    }

    #[test]
    fn build_response_gate_rejects_malformed_json() {
        let payload = build_response(&parsed("POST /gate HTTP/1.1", b"{not-json"), &runtime());
        assert_eq!(status_from_payload(&payload), 200);
        let json: serde_json::Value = serde_json::from_str(&body_from_payload(&payload)).unwrap();
        assert_eq!(json["admissible"], false);
        assert!(json["codes"]
            .as_array()
            .map(|c| !c.is_empty())
            .unwrap_or(false));
    }

    #[test]
    fn build_response_not_found_includes_catalog_hash() {
        let payload = build_response(&parsed("DELETE /gate HTTP/1.1", b""), &runtime());
        assert_eq!(status_from_payload(&payload), 404);
        let json: serde_json::Value = serde_json::from_str(&body_from_payload(&payload)).unwrap();
        assert_eq!(json["error"], "not found");
        assert!(json["catalog_hash_hex"].as_str().is_some());
    }

    #[test]
    fn wire_hops_gate_runtime_chain_wired() {
        let wired: Vec<_> = GATE_SERVER_ROUTER_WIRE_HOPS
            .iter()
            .filter(|h| h.wired)
            .collect();
        assert_eq!(wired.len(), 5);
        assert!(wired[0].surface.contains("read_request"));
        assert!(wired[2].surface.contains("evaluate_transition"));
    }
}
