// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Witness catalog embedding + pinned **catalog lock bundle** semantics.
//!
//! - **Lock bundle digest** (`UMST_CATALOG_LOCK_SHA256_HEX`) covers `artifacts/catalog.lock.json`
//!   (or `UMST_CATALOG`).
//! - **Witness envelope** — JSON [`WitnessCatalog`] parsed from embedded bytes emitted as
//!   `OUT_DIR/catalog_constants.rs` (see [`WITNESS_CATALOG_EMBEDDED_BYTES`]).

pub mod catalog_pin;
pub mod sec_catalog_pin;
pub mod traceability;
pub mod witness_priority;
pub mod semantic_witness;

pub use catalog_pin::{
    catalog_sha3_pin_witness_ok, composed_fiber_fingerprint_guard_holds,
    lock_bundle_content_address_hex, non_preview_fiber_fingerprint_hex, pin_witness_ok,
    CatalogPinMismatch, EXPECTED_MODULE_COUNT, EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX,
};
pub use sec_catalog_pin::{
    catalog_digest, catalog_digest_hash_policy, catalog_pin_manifold_probe,
    catalog_pin_manifold_wired, catalog_pin_production_wired, manifold_catalog_pin_ceremony_closed,
    resolve_catalog_digest, sec_catalog_pin_p1542_a2_honest, sec_catalog_pin_p1542_a2_probe,
    verify_ssot_catalog_digest_hex, CatalogDigestAttachMode, CatalogPinManifoldError,
    CatalogPinManifoldProbe, FLEET_P1542_A2_JOB_ID, FLEET_P1542_A2_RECEIPT_PATH, JOB_ID,
    MANIFOLD_CATALOG_PIN_WIRE_HOPS, BOARD_SLICE_ID as CATALOG_PIN_BOARD_SLICE_ID,
};

pub use semantic_witness::{
    bundled_semantic_witness_section_present, lookup_bundled_semantic_cold_witness,
    lookup_semantic_cold_witness, semantic_witness_section_quickcheck, SemanticColdProof,
    SemanticWitnessSection, DEFAULT_SEMANTIC_COLD_WITNESS_ID, SEMANTIC_CBF_CATALOG_ID,
    SEMANTIC_COLD_HOT_POLICY_VERSION,
};
pub use witness_priority::{
    tcb_axiom_token_allowed, WitnessLearningSignal, WitnessPriorityQueue, WitnessTcbAxiom,
    LANDAUER_LAW_LEAN_MODULE, PHYSICAL_SECOND_LAW_AXIOM,
};

mod witness_embed {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/catalog_constants.rs"));
}

pub use witness_embed::{WITNESS_CATALOG_EMBEDDED_LEN, WITNESS_CATALOG_EMBEDDED_SHA256_HEX};

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

pub const CATALOG_SCHEMA_STUB_REVISION: &str = "catalog.schema.stub.v1";

/// Environment variable overriding the filesystem path for [`WitnessCatalog::load_default`].
pub const ENV_WITNESS_CATALOG_PATH: &str = "UMST_CATALOG_PATH";

/// Blake3-SHA-equivalent semantics: **`env!`** binding for the verbatim lock-bundle bytes fingerprint.
///
/// Produced from `artifacts/catalog.lock.json` (or `UMST_CATALOG`).
#[inline]
pub fn catalog_lock_bundle_sha256_hex() -> &'static str {
    env!("UMST_CATALOG_LOCK_SHA256_HEX")
}

/// Raw 32-byte digest matching [`catalog_lock_bundle_sha256_hex`].
#[inline]
pub fn catalog_lock_bundle_sha256_bytes() -> [u8; 32] {
    decode_sha256_hex(catalog_lock_bundle_sha256_hex())
        .expect("UMST_CATALOG_LOCK_SHA256_HEX must be 64 hex chars")
}

/// Composed R0 digest from lock JSON (`upstream_catalog_digest_hex` / `composed_catalog_digest_hex`).
///
/// Emitted at build time as `UMST_LOCK_UPSTREAM_CATALOG_DIGEST_HEX` from `artifacts/catalog.lock.json`.
#[inline]
pub fn lock_upstream_catalog_digest_hex() -> &'static str {
    env!("UMST_LOCK_UPSTREAM_CATALOG_DIGEST_HEX")
}

/// Raw 32-byte R0 catalog digest for formal-witness / UMST `catalog_schema_digest` auto-fill.
#[inline]
pub fn lock_upstream_catalog_digest_bytes() -> [u8; 32] {
    decode_sha256_hex(lock_upstream_catalog_digest_hex())
        .expect("UMST_LOCK_UPSTREAM_CATALOG_DIGEST_HEX must be 64 hex chars")
}

fn decode_sha256_hex(s: &str) -> Result<[u8; 32], ()> {
    if s.len() != 64 {
        return Err(());
    }
    let b = s.as_bytes();
    let mut out = [0_u8; 32];
    for i in 0..32 {
        let hi = hex_nibble(b[2 * i])?;
        let lo = hex_nibble(b[2 * i + 1])?;
        out[i] = (hi << 4) | lo;
    }
    Ok(out)
}

fn hex_nibble(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}

/// Raw lock JSON bundled at crate root (`include_str!(...)` embed).
#[inline]
pub fn bundled_catalog_lock_json() -> &'static str {
    include_str!("../../../artifacts/catalog.lock.json")
}

/// Per-fiber Lean catalog pin (schema v2). See [`docs/DUAL_PIN_ARCHITECTURE.md`](../../../docs/DUAL_PIN_ARCHITECTURE.md).
///
/// Preview fibers (`lock_role` contains `preview` or `track_f`, e.g. `umst-ucrs`) are
/// digest-locked for audit but excluded from [`CatalogLock::composed_digest_hex`].
/// Optional [`Self::commit_stamp`] is populated on witnessed commit-only egress
/// (see [`docs/RUNTIME_TOPOLOGY.md`](../../../docs/RUNTIME_TOPOLOGY.md)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogFiberPin {
    pub repo: String,
    pub catalog_digest_hex: String,
    pub module_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_path: Option<String>,
    /// UCRS witness stamp at last witnessed commit; absent at cold pin time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_stamp: Option<String>,
}

/// Whether a fiber pin is preview / Track F (excluded from composed R0 digest).
#[must_use]
pub fn is_preview_fiber_pin(pin: &CatalogFiberPin) -> bool {
    let role = pin.lock_role.as_deref().unwrap_or("").to_ascii_lowercase();
    role.contains("preview") || role.contains("track_f")
}

/// Manifold runtime lock (`artifacts/catalog.lock.json`). v1 monolith or v2 dual-pin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogLock {
    pub version: u32,
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_repo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_catalog_digest_hex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composition_rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composed_catalog_digest_hex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composed_primary_fiber_fingerprint_hex: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fiber_pins: Vec<CatalogFiberPin>,
    /// HCOM-004 cold semantic proof witnesses (build-time Lean exports).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_witnesses: Option<SemanticWitnessSection>,
}

impl CatalogLock {
    /// Parse the bundled lock artifact.
    #[inline]
    pub fn from_bundled() -> Result<Self, CatalogLoadError> {
        Self::from_json_str(bundled_catalog_lock_json())
    }

    fn from_json_str(raw: &str) -> Result<Self, CatalogLoadError> {
        Ok(serde_json::from_str(raw)?)
    }

    /// Composed R0 digest: v2 `composed_catalog_digest_hex`, else v1 `upstream_catalog_digest_hex`.
    #[must_use]
    pub fn composed_digest_hex(&self) -> Option<&str> {
        self.composed_catalog_digest_hex
            .as_deref()
            .or(self.upstream_catalog_digest_hex.as_deref())
    }
}

fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// Structural invariants for v1 monolith or v2 dual-pin locks (no Lean regen).
#[must_use]
pub fn catalog_lock_quickcheck(lock: &CatalogLock) -> bool {
    if lock.role != "manifold_runtime_lock" {
        return false;
    }
    let composed = match lock.composed_digest_hex() {
        Some(d) if is_sha256_hex(d) => d,
        _ => return false,
    };
    if lock.version >= 2 {
        let upstream = lock.upstream_catalog_digest_hex.as_deref();
        let explicit = lock.composed_catalog_digest_hex.as_deref();
        if let (Some(u), Some(c)) = (upstream, explicit) {
            if u != c {
                return false;
            }
        }
        for pin in &lock.fiber_pins {
            if pin.repo.is_empty() || !is_sha256_hex(&pin.catalog_digest_hex) {
                return false;
            }
        }
    } else if lock.version == 1 {
        if lock.fiber_pins.is_empty() {
            // v1 monolith: upstream pin only
        } else {
            return false;
        }
    } else {
        return false;
    }
    lock.module_count.is_some() && composed.len() == 64
}

/// Minimal invariant on the pinned lock artifact + nonempty bundle digest constant.
///
/// Intended for callers that want **static** reassurance without runtime file I/O.
/// Accepts **v1 monolith** (`upstream_catalog_digest_hex` only) or **v2 dual-pin**
/// (`fiber_pins` + `composed_catalog_digest_hex` == `upstream_catalog_digest_hex`).
#[must_use]
pub fn witness_catalog_quickcheck_ok() -> bool {
    !catalog_lock_bundle_sha256_hex().is_empty()
        && catalog_lock_bundle_sha256_hex() != "0".repeat(64).as_str()
        && CatalogLock::from_bundled()
            .map(|lock| catalog_lock_quickcheck(&lock))
            .unwrap_or(false)
}

/// Parsed witness envelope embedded at compile time (`build.rs`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessCatalog {
    pub version: u32,
    #[serde(default)]
    pub witnesses: Vec<WitnessRecord>,
}

/// One bounded witness checkpoint identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessRecord {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// [`WitnessCatalog`] load / JSON parse failures.
#[derive(Debug)]
pub enum CatalogLoadError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for CatalogLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogLoadError::Io(e) => write!(f, "io error: {e}"),
            CatalogLoadError::Json(e) => write!(f, "json error: {e}"),
        }
    }
}

impl std::error::Error for CatalogLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CatalogLoadError::Io(e) => Some(e),
            CatalogLoadError::Json(e) => Some(e),
        }
    }
}

impl From<io::Error> for CatalogLoadError {
    fn from(value: io::Error) -> Self {
        CatalogLoadError::Io(value)
    }
}

impl From<serde_json::Error> for CatalogLoadError {
    fn from(value: serde_json::Error) -> Self {
        CatalogLoadError::Json(value)
    }
}

impl WitnessCatalog {
    /// Parse embedded JSON bytes emitted by **`build.rs`**.
    #[inline]
    pub fn from_embedded() -> Result<Self, CatalogLoadError> {
        Self::from_json_slice(&witness_embed::WITNESS_CATALOG_EMBEDDED_BYTES)
    }

    #[inline]
    pub fn from_path(path: &Path) -> Result<Self, CatalogLoadError> {
        let raw = fs::read(path)?;
        Self::from_json_slice(&raw)
    }

    /// When [`ENV_WITNESS_CATALOG_PATH`] is set — load that path; otherwise [`Self::from_embedded`].
    pub fn load_default() -> Result<Self, CatalogLoadError> {
        match std::env::var_os(ENV_WITNESS_CATALOG_PATH) {
            Some(raw) => Self::from_path(Path::new(&raw)),
            None => Self::from_embedded(),
        }
    }

    fn from_json_slice(raw: &[u8]) -> Result<Self, CatalogLoadError> {
        let v: WitnessCatalog = serde_json::from_slice(raw)?;
        Ok(v)
    }

    /// Hex-encoded SHA-256 fingerprint of embedded witness-catalog bytes (build-time derived).
    #[inline]
    pub fn embedded_bundle_sha256_hex() -> &'static str {
        WITNESS_CATALOG_EMBEDDED_SHA256_HEX
    }

    #[inline]
    pub fn embedded_len() -> usize {
        WITNESS_CATALOG_EMBEDDED_LEN
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
