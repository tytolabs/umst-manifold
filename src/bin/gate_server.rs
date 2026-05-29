// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//!
//! Minimal thermodynamic gate HTTP server (stdlib `TcpListener`, `POST /gate`).
//!
//! ```text
//! UMST_GATE_ADDR=0.0.0.0:8787 cargo run -p umst-manifold --bin gate_server --features gate-server
//! ```

use std::net::TcpListener;

use umst_manifold::gate::{http_manifest::GateHttpRuntime, GateEvaluator};
use umst_manifold::gate_server_router::handle_connection;
use umst_manifold::manifest::UmstManifest;

fn main() {
    let addr = std::env::var("UMST_GATE_ADDR").unwrap_or_else(|_| "0.0.0.0:8787".to_string());
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| panic!("bind {addr}: {e}"));
    let runtime = GateHttpRuntime::from_umst_manifest(&UmstManifest::default());
    eprintln!(
        "umst-manifold gate_server listening on http://{addr} (POST /gate, GET /health) catalog_id={}",
        runtime.evaluator.catalog_id()
    );
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_connection(&mut stream, &runtime),
            Err(e) => tracing::warn!("gate_server accept error: {e}"),
        }
    }
}
