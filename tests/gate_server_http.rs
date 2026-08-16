// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Integration: `TcpListener` on port **`0`** + [`umst_manifold::gate_server_router::handle_connection`].
//!
//! Run (with crate feature):
//! ```text
//! cargo test -p umst-manifold --features gate-server-bin gate_server_http
//! ```

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

use serde_json::Value;
use umst_manifold::gate::http_manifest::GateHttpRuntime;
use umst_manifold::gate_server_router::handle_connection;
use umst_manifold::manifest::UmstManifest;

fn read_http_response(stream: TcpStream) -> (u16, String) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone"));
    let mut status_line = String::new();
    reader.read_line(&mut status_line).expect("status");
    let status: u16 = status_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);

    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).expect("header");
        if line.trim().is_empty() {
            break;
        }
        let lower = line.to_ascii_lowercase();
        if lower.starts_with("content-length:") {
            content_length = line
                .split(':')
                .nth(1)
                .expect("Content-Length header must include colon-separated value")
                .trim()
                .parse()
                .expect("Content-Length must parse as usize");
        }
    }

    let mut body = vec![0u8; content_length];
    reader
        .read_exact(&mut body)
        .expect("HTTP response body must match Content-Length");
    (
        status,
        String::from_utf8(body).expect("HTTP response body must be valid UTF-8"),
    )
}

#[test]
fn post_gate_json_roundtrip_localhost() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let runtime = GateHttpRuntime::from_umst_manifest(&UmstManifest::default());

    let (tx, rx) = mpsc::channel();
    let jh = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        tx.send(()).expect("notify");
        handle_connection(&mut stream, &runtime);
    });

    let mut client = TcpStream::connect(addr).expect("connect");
    rx.recv().expect("server accepted");

    let body = r#"{"cement":400,"water":200,"age":28,"predicted_strength":25}"#;
    let req = format!(
        "POST /gate HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    client.write_all(req.as_bytes()).expect("write");
    let (status, json_body) = read_http_response(client);
    jh.join().expect("join server");

    assert_eq!(status, 200, "unexpected status; body:\n{}", json_body);
    let v: Value = serde_json::from_str(&json_body).expect("response json");
    assert_eq!(
        v["admissible"],
        true,
        "expected admit; codes={:?} catalog={:?} raw={}",
        v["codes"],
        v["catalog_hash_hex"].as_str(),
        json_body
    );
    assert_eq!(v["codes"], serde_json::json!([]));
    assert!(v["catalog_hash_hex"]
        .as_str()
        .map(|s| s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()))
        .unwrap_or(false));
}
