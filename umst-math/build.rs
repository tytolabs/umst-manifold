// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Formal catalog pin for [`formal_catalog`](src/formal_catalog.rs): the one place the ecosystem derives it.
//!
//! Reads the pinned formal export `../artifacts/upstream_catalog.json` (umst-formal-double-slit's merged
//! catalog at the umst-manifold `.umst-pins.toml` commit), recomputes its digest with `export_catalog.py`'s rule
//! (SHA-256 of the sort-keys, compact, ASCII-escaped JSON body without the `digest` key) and refuses the build
//! when that differs from the export's stated `digest`. Emits `OUT_DIR/formal_catalog_pin.rs` with the digest,
//! the module and edge row counts, the SHA-256 of `../artifacts/catalog.lock.json`, and both file paths.

use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), String> {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or("CARGO_MANIFEST_DIR unset")?);
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or("OUT_DIR unset")?);
    let artifacts = manifest_dir.join("../artifacts");
    let export_path = artifacts.join("upstream_catalog.json");
    let lock_path = artifacts.join("catalog.lock.json");
    println!("cargo:rerun-if-changed={}", export_path.display());
    println!("cargo:rerun-if-changed={}", lock_path.display());

    let raw = fs::read_to_string(&export_path).map_err(|e| {
        format!(
            "umst-math: pinned formal export missing at {} ({e})",
            export_path.display()
        )
    })?;
    let mut export: serde_json::Value = serde_json::from_str(&raw).map_err(|e| {
        format!(
            "umst-math: pinned formal export at {} is not valid JSON ({e})",
            export_path.display()
        )
    })?;
    let stated = export
        .as_object_mut()
        .and_then(|body| body.remove("digest"))
        .and_then(|d| d.as_str().map(str::to_owned))
        .ok_or_else(|| {
            format!(
                "umst-math: formal export at {} has no digest",
                export_path.display()
            )
        })?;
    let mut canon = String::new();
    write_export_canonical_json(&export, &mut canon)?;
    let digest = format!("{:x}", Sha256::digest(canon.as_bytes()));
    if digest != stated {
        return Err(format!(
            "umst-math: formal export at {} digest {stated} does not match its body ({digest}); re-copy it from \
             umst-formal-double-slit artifacts/catalog.json (scripts/bump_catalog_lock.py)",
            export_path.display()
        ));
    }
    let rows = |key: &str| {
        export
            .get(key)
            .and_then(|v| v.as_array())
            .map(Vec::len)
            .ok_or_else(|| {
                format!(
                    "umst-math: formal export at {} has no {key}",
                    export_path.display()
                )
            })
    };
    let lock_bytes = fs::read(&lock_path).map_err(|e| {
        format!(
            "umst-math: catalog lock missing at {} ({e})",
            lock_path.display()
        )
    })?;
    let lock_sha256 = format!("{:x}", Sha256::digest(&lock_bytes));
    let canonical = |p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let generated = format!(
        "/// Digest of the pinned formal export, recomputed from its body by umst-math `build.rs`.\n\
         pub const FORMAL_CATALOG_DIGEST_HEX: &str = \"{digest}\";\n\
         /// Module rows of the pinned formal export.\n\
         pub const FORMAL_CATALOG_MODULE_COUNT: u32 = {};\n\
         /// Module graph edge rows of the pinned formal export.\n\
         pub const FORMAL_CATALOG_MODULE_GRAPH_EDGE_COUNT: u32 = {};\n\
         /// SHA-256 of the umst-manifold catalog lock bytes this crate was built against.\n\
         pub const MANIFOLD_CATALOG_LOCK_SHA256_HEX: &str = \"{lock_sha256}\";\n\
         /// Path of the pinned formal export this crate was built from.\n\
         pub const FORMAL_CATALOG_EXPORT_PATH: &str = {:?};\n\
         /// Path of the umst-manifold catalog lock this crate was built against.\n\
         pub const MANIFOLD_CATALOG_LOCK_PATH: &str = {:?};\n",
        rows("modules")?,
        rows("module_graph_edges")?,
        canonical(&export_path).display().to_string(),
        canonical(&lock_path).display().to_string(),
    );
    fs::write(out_dir.join("formal_catalog_pin.rs"), generated)
        .map_err(|e| format!("umst-math: write OUT_DIR/formal_catalog_pin.rs ({e})"))
}

/// Python `json.dumps(v, sort_keys=True, separators=(",", ":"), ensure_ascii=True)`; a float has no defined
/// canonical form here and is refused.
fn write_export_canonical_json(v: &serde_json::Value, out: &mut String) -> Result<(), String> {
    use serde_json::Value;
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) if n.is_f64() => {
            return Err(format!("umst-math: formal export carries a float ({n})"));
        }
        Value::Number(n) => out.push_str(&n.to_string()),
        Value::String(s) => write_export_canonical_str(s, out),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_export_canonical_json(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            let mut entries: Vec<(&String, &Value)> = map.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            out.push('{');
            for (i, (k, item)) in entries.into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_export_canonical_str(k, out);
                out.push(':');
                write_export_canonical_json(item, out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

fn write_export_canonical_str(s: &str, out: &mut String) {
    use std::fmt::Write as _;
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                let mut units = [0u16; 2];
                for u in c.encode_utf16(&mut units) {
                    let _ = write!(out, "\\u{u:04x}");
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}
