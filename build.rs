// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Two build-time payloads:
//!
//! 1. **Lean / lock bundle** — `artifacts/catalog.lock.json` (or `UMST_CATALOG`). Emits the
//!    `UMST_CATALOG_LOCK_SHA256_HEX` rustc env consumed by [`catalog_lock_bundle_sha256_hex`].
//! 2. **Witness catalog JSON envelope** (`WitnessCatalog`) — emits `OUT_DIR/catalog_constants.rs`
//!    containing a SHA-256 fingerprint and a `[u8; N]` array. When no witness JSON exists on disk,
//!    a minimal built-in envelope is embedded so **`cargo check` never requires extra files**.
//! 3. **Scalar layout drift guard** — compares `scalar_channel_count` in the catalog lock JSON
//!    against `UMST_SCALAR_CHANNEL_COUNT` in `src/core/umst_schema.rs`; emits
//!    `OUT_DIR/scalar_layout_guard.rs` with `compile_error!` on mismatch.

use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

/// Minimal envelope when neither `witness_catalog.json` nor `UMST_CATALOG_BUILD_JSON` exist.
const WITNESS_CATALOG_FALLBACK_JSON: &str = r#"{"version":1,"witnesses":[{"id":"umst.catalog.builtin.min","description":"build.rs fallback (no witness_catalog.json)"}]}"#;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));

    emit_catalog_lock_digest(&manifest_dir);
    emit_scalar_layout_guard(&manifest_dir, &out_dir);
    let witness_bytes = resolve_witness_catalog_bytes(&manifest_dir);
    write_catalog_constants(&out_dir, &witness_bytes);
}

fn emit_catalog_lock_digest(manifest_dir: &Path) {
    let rel = env::var("UMST_CATALOG").unwrap_or_else(|_| "artifacts/catalog.lock.json".into());
    let path = if PathBuf::from(&rel).is_absolute() {
        PathBuf::from(rel)
    } else {
        manifest_dir.join(rel)
    };

    println!("cargo:rerun-if-env-changed=UMST_CATALOG");
    println!("cargo:rerun-if-changed={}", path.display());

    let (hex, upstream_hex) = match fs::read_to_string(&path) {
        Ok(raw) => {
            let bundle = Sha256::digest(raw.as_bytes());
            let bundle_hex = format!("{bundle:x}");
            let upstream_hex = parse_lock_upstream_digest_hex(&raw).unwrap_or_else(|| {
                println!(
                    "cargo:warning=umst-manifold: catalog lock at {} missing upstream_catalog_digest_hex — using zero upstream digest",
                    path.display()
                );
                "0".repeat(64)
            });
            (bundle_hex, upstream_hex)
        }
        Err(e) => {
            println!(
                "cargo:warning=umst-manifold: catalog lock missing at {} — using zero digest ({})",
                path.display(),
                e
            );
            let zeros = "0".repeat(64);
            (zeros.clone(), zeros)
        }
    };

    println!("cargo:rustc-env=UMST_CATALOG_LOCK_SHA256_HEX={hex}");
    println!("cargo:rustc-env=UMST_LOCK_UPSTREAM_CATALOG_DIGEST_HEX={upstream_hex}");
}

/// v2 `composed_catalog_digest_hex` or v1 `upstream_catalog_digest_hex` from lock JSON.
fn parse_lock_upstream_digest_hex(lock_json: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(lock_json).ok()?;
    let hex = v
        .get("composed_catalog_digest_hex")
        .or_else(|| v.get("upstream_catalog_digest_hex"))?
        .as_str()?;
    if hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        Some(hex.to_ascii_lowercase())
    } else {
        None
    }
}

fn catalog_lock_path(manifest_dir: &Path) -> PathBuf {
    let rel = env::var("UMST_CATALOG").unwrap_or_else(|_| "artifacts/catalog.lock.json".into());
    if PathBuf::from(&rel).is_absolute() {
        PathBuf::from(rel)
    } else {
        manifest_dir.join(rel)
    }
}

/// `scalar_channel_count` from lock JSON (Phase 1 layout functor spike).
fn parse_lock_scalar_channel_count(lock_json: &str) -> Option<usize> {
    let v: serde_json::Value = serde_json::from_str(lock_json).ok()?;
    let n = v.get("scalar_channel_count")?.as_u64()?;
    usize::try_from(n).ok()
}

/// Hand-written `UMST_SCALAR_CHANNEL_COUNT` from `src/core/umst_schema.rs`.
fn parse_schema_scalar_channel_count(manifest_dir: &Path) -> Option<usize> {
    let path = manifest_dir.join("src/core/umst_schema.rs");
    println!("cargo:rerun-if-changed={}", path.display());
    let raw = fs::read_to_string(&path).ok()?;
    const NEEDLE: &str = "pub const UMST_SCALAR_CHANNEL_COUNT: usize = ";
    let start = raw.find(NEEDLE)? + NEEDLE.len();
    let tail = &raw[start..];
    let end = tail.find(';')?;
    tail[..end].trim().parse().ok()
}

/// Emit `compile_error!` when catalog lock and `umst_schema.rs` scalar widths diverge.
fn emit_scalar_layout_guard(manifest_dir: &Path, out_dir: &Path) {
    let lock_path = catalog_lock_path(manifest_dir);
    let lock_count = fs::read_to_string(&lock_path)
        .ok()
        .and_then(|raw| parse_lock_scalar_channel_count(&raw));
    let schema_count = parse_schema_scalar_channel_count(manifest_dir);

    let path = out_dir.join("scalar_layout_guard.rs");
    let mut f = fs::File::create(&path).expect("scalar_layout_guard.rs");

    match (lock_count, schema_count) {
        (Some(lock), Some(schema)) if lock == schema => {
            writeln!(
                f,
                "// @generated by build.rs — scalar layout guard OK (count = {lock})\n"
            )
            .expect("write");
        }
        (Some(lock), Some(schema)) => {
            writeln!(
                f,
                "// @generated by build.rs\n\
compile_error!(\"umst-manifold scalar layout drift: catalog.lock.json scalar_channel_count={lock} != umst_schema.rs UMST_SCALAR_CHANNEL_COUNT={schema}\");"
            )
            .expect("write");
        }
        (lock, schema) => {
            let lock_msg = lock
                .map(|n| n.to_string())
                .unwrap_or_else(|| "missing scalar_channel_count in catalog lock".into());
            let schema_msg = schema
                .map(|n| n.to_string())
                .unwrap_or_else(|| "missing UMST_SCALAR_CHANNEL_COUNT in umst_schema.rs".into());
            writeln!(
                f,
                "// @generated by build.rs\n\
compile_error!(\"umst-manifold scalar layout guard incomplete: lock={lock_msg}, schema={schema_msg}\");"
            )
            .expect("write");
        }
    }
}

fn resolve_witness_catalog_bytes(manifest_dir: &Path) -> Vec<u8> {
    println!("cargo:rerun-if-env-changed=UMST_CATALOG_BUILD_JSON");
    println!("cargo:rerun-if-changed=witness_catalog.json");

    if let Ok(p) = env::var("UMST_CATALOG_BUILD_JSON") {
        let path = PathBuf::from(&p);
        return fs::read(&path).unwrap_or_else(|e| {
            panic!(
                "UMST_CATALOG_BUILD_JSON points at {} but cannot be read: {e}",
                path.display()
            );
        });
    }

    let default_path = manifest_dir.join("witness_catalog.json");
    if default_path.is_file() {
        return fs::read(&default_path).unwrap_or_else(|e| {
            panic!(
                "failed reading witness catalog at {}: {e}",
                default_path.display()
            );
        });
    }

    WITNESS_CATALOG_FALLBACK_JSON.as_bytes().to_vec()
}

fn write_catalog_constants(out_dir: &Path, bytes: &[u8]) {
    let digest = Sha256::digest(bytes);
    let hex = format!("{digest:x}");
    let len = bytes.len();

    let path = out_dir.join("catalog_constants.rs");
    let mut f = fs::File::create(&path).expect("catalog_constants.rs");
    writeln!(
        f,
        "// @generated by build.rs\n\
/// SHA-256 (hex, lowercase) of embedded witness-catalog JSON.\n\
pub const WITNESS_CATALOG_EMBEDDED_SHA256_HEX: &str = \"{hex}\";\n\
/// Byte length of [`WITNESS_CATALOG_EMBEDDED_BYTES`].\n\
pub const WITNESS_CATALOG_EMBEDDED_LEN: usize = {len};\n\
pub static WITNESS_CATALOG_EMBEDDED_BYTES: [u8; {len}] = ["
    )
    .expect("write");

    for (i, b) in bytes.iter().enumerate() {
        if i % 16 == 0 {
            write!(f, "\n    ").expect("write");
        }
        write!(f, "{b}, ").expect("write");
    }
    writeln!(f, "\n];").expect("write");
}
