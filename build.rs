// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Two build-time payloads:
//!
//! 1. **Lean / lock bundle** — `artifacts/catalog.lock.json` (or `UMST_CATALOG`). Emits the
//!    `UMST_CATALOG_LOCK_SHA256_HEX` rustc env consumed by [`catalog_lock_bundle_sha256_hex`].
//! 2. **Witness catalog JSON envelope** (`WitnessCatalog`) — emits `OUT_DIR/catalog_constants.rs`
//!    containing a SHA-256 fingerprint and a `[u8; N]` array. When no witness JSON exists on disk,
//!    a minimal built-in envelope is embedded so **`cargo check` never requires extra files**.
//!
//! Scalar nodal channel layout lives in `artifacts/scalar_layout.lock.json` (Phase 1 section 1B), not in
//! `catalog.lock.json`, which pins formal module metadata only.

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
            emit_catalog_digest_guard(&path, &raw);
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

/// T1 digest guard (cold/build): `composed_catalog_digest_hex` must cover all non-preview fibers.
fn emit_catalog_digest_guard(lock_path: &Path, lock_json: &str) {
    let lock: serde_json::Value = serde_json::from_str(lock_json).unwrap_or_else(|e| {
        panic!(
            "umst-manifold: catalog lock at {} is not valid JSON ({e})",
            lock_path.display()
        );
    });

    let version = lock.get("version").and_then(|v| v.as_u64()).unwrap_or(1);
    if version < 2 {
        return;
    }

    let pins = lock
        .get("fiber_pins")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let non_preview: Vec<&serde_json::Value> = pins
        .iter()
        .filter(|pin| !is_preview_fiber_pin(pin))
        .collect();

    if non_preview.is_empty() {
        return;
    }

    let fp = non_preview_fiber_fingerprint(&lock);
    let stored = lock
        .get("composed_primary_fiber_fingerprint_hex")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if stored.is_empty() {
        panic!(
            "umst-manifold: catalog lock at {} missing composed_primary_fiber_fingerprint_hex \
             (update protocol after non-preview fiber pin change)",
            lock_path.display()
        );
    }
    if stored != fp {
        panic!(
            "umst-manifold: catalog lock at {} non-preview fiber drift: \
             composed_primary_fiber_fingerprint_hex want {fp} got {stored}",
            lock_path.display()
        );
    }

    let composed = lock
        .get("composed_catalog_digest_hex")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if composed.len() != 64 {
        panic!(
            "umst-manifold: catalog lock at {} missing composed_catalog_digest_hex \
             (must update when non-preview fibers change)",
            lock_path.display()
        );
    }
    if let Some(upstream) = lock
        .get("upstream_catalog_digest_hex")
        .and_then(|v| v.as_str())
        .filter(|u| !u.is_empty())
    {
        if composed != upstream {
            panic!(
                "umst-manifold: catalog lock at {} composed_catalog_digest_hex != upstream_catalog_digest_hex \
                 ({composed} vs {upstream})",
                lock_path.display()
            );
        }
    }
}

fn is_preview_fiber_pin(pin: &serde_json::Value) -> bool {
    let role = pin
        .get("lock_role")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    role.contains("preview") || role.contains("track_f")
}

fn non_preview_fiber_fingerprint(lock: &serde_json::Value) -> String {
    let pins = lock
        .get("fiber_pins")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut digests: Vec<String> = pins
        .iter()
        .filter(|pin| !is_preview_fiber_pin(pin))
        .filter_map(|pin| {
            let repo = pin.get("repo")?.as_str()?;
            let digest = pin.get("catalog_digest_hex")?.as_str()?;
            Some(format!("{repo}:{digest}"))
        })
        .collect();

    if digests.is_empty() {
        return String::new();
    }

    digests.sort();
    let payload = digests.join("|");
    let hash = Sha256::digest(payload.as_bytes());
    format!("{hash:x}")
}

/// Phase 1 §1B: nodal scalar channel map lives in `artifacts/scalar_layout.lock.json`, not
/// `catalog.lock.json`. Compares lock `scalar_channel_count` to
/// [`UMST_SCALAR_CHANNEL_COUNT`](src/core/umst_schema.rs) and emits `UMST_SCALAR_CHANNEL_COUNT` for
/// runtime introspection.
fn emit_scalar_layout_guard(manifest_dir: &Path, out_dir: &Path) {
    let rel = "artifacts/scalar_layout.lock.json";
    let path = manifest_dir.join(rel);
    println!("cargo:rerun-if-changed={rel}");
    println!("cargo:rerun-if-changed=src/core/umst_schema.rs");

    let raw = fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "umst-manifold: scalar layout lock missing at {} — required for Phase 1 layout contract ({e})",
            path.display()
        );
    });

    let lock: serde_json::Value = serde_json::from_str(&raw).unwrap_or_else(|e| {
        panic!(
            "umst-manifold: scalar layout lock at {} is not valid JSON ({e})",
            path.display()
        );
    });

    let lock_count = lock
        .get("scalar_channel_count")
        .and_then(|v| v.as_u64())
        .filter(|&n| n >= 1)
        .unwrap_or_else(|| {
            panic!(
                "umst-manifold: scalar layout lock at {} missing or invalid scalar_channel_count",
                path.display()
            );
        }) as usize;

    let channel_ids_len = lock
        .get("channel_ids")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or_else(|| {
            panic!(
                "umst-manifold: scalar layout lock at {} missing channel_ids array",
                path.display()
            );
        });

    if lock_count != channel_ids_len {
        panic!(
            "umst-manifold: scalar layout drift: {} scalar_channel_count={lock_count} != channel_ids.len()={channel_ids_len}",
            path.display()
        );
    }

    let schema_src = fs::read_to_string(manifest_dir.join("src/core/umst_schema.rs")).unwrap_or_else(
        |e| panic!("umst-manifold: cannot read src/core/umst_schema.rs for scalar layout guard ({e})"),
    );
    let schema_count = parse_umst_schema_scalar_channel_count(&schema_src).unwrap_or_else(|| {
        panic!(
            "umst-manifold: src/core/umst_schema.rs missing `pub const UMST_SCALAR_CHANNEL_COUNT: usize = N;`"
        );
    });

    if lock_count != schema_count {
        panic!(
            "umst-manifold: scalar layout drift: {} scalar_channel_count={lock_count} != src/core/umst_schema.rs UMST_SCALAR_CHANNEL_COUNT={schema_count}",
            path.display()
        );
    }

    println!("cargo:rustc-env=UMST_SCALAR_CHANNEL_COUNT={lock_count}");

    let guard_path = out_dir.join("scalar_layout_guard.rs");
    let mut f = fs::File::create(&guard_path).expect("scalar_layout_guard.rs");
    writeln!(
        f,
        "// @generated by build.rs — do not edit\n\
/// Lock-file scalar channel count (must match [`super::UMST_SCALAR_CHANNEL_COUNT`]).\n\
pub const UMST_SCALAR_CHANNEL_COUNT_LOCK: usize = {lock_count};"
    )
    .expect("write scalar_layout_guard.rs");
}

/// Parse `pub const UMST_SCALAR_CHANNEL_COUNT: usize = N;` from hand-written schema source.
fn parse_umst_schema_scalar_channel_count(src: &str) -> Option<usize> {
    const NEEDLE: &str = "pub const UMST_SCALAR_CHANNEL_COUNT: usize = ";
    let start = src.find(NEEDLE)? + NEEDLE.len();
    let rest = &src[start..];
    let end = rest.find(';')?;
    rest[..end].trim().parse().ok()
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
