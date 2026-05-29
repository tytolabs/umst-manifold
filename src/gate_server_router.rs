// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Minimal HTTP/1.1 handler for [`crate::gate::http_manifest`] (stdlib only; one request per connection).

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::gate::http_manifest::{
    gate_json_parse_response, pinned_catalog_bundle_sha256_hex, GateHttpRuntime, MixProposal,
};
use crate::runtime::catalog::traceability::HTTP_SHIM_CATALOG_ID;

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
    let line = req.request_line.trim();
    let upper = line.to_ascii_uppercase();
    let hash = pinned_catalog_bundle_sha256_hex();

    if upper.starts_with("GET ") && upper.contains("/health") {
        let b = br#"{"status":"ok"}"#;
        return http_payload_json(b);
    }

    let is_gate = upper.starts_with("POST ") && line.contains("/gate");
    if !is_gate {
        let body =
            serde_json::json!({ "error": "not found", "catalog_hash_hex": hash }).to_string();
        return http_payload("404 Not Found", &body);
    }

    let body_str = String::from_utf8_lossy(&req.body);
    let json = match serde_json::from_str::<MixProposal>(body_str.as_ref()) {
        Ok(p) => serde_json::to_string(&runtime.evaluate_mix(&p)).unwrap(),
        Err(_) => serde_json::to_string(&gate_json_parse_response()).unwrap(),
    };
    http_payload("200 OK", &json)
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
