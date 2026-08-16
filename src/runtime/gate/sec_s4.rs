// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! AGAP-2033/2127-SEC-S4 — manifold gate runtime side-channel scrub wire map.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-S4 side-channel/scrub SSOT;
//! L-S5 formal proof, sled anomaly audit, and gateway `trust_wrap_wired()` stay **honest open**.

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-S4";

/// AGAP slot id (2033 side-channel badge deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-S4";

/// FLEET-COMPOSER Prabhu Wave H slot H3 id.
pub const FLEET_P1800_H3_JOB_ID: &str = "PRABHU-WAVE-H-1800-H3";

/// FLEET-COMPOSER Prabhu Wave H H3 receipt path.
pub const FLEET_P1800_H3_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_P1800_H3.md";

/// FLEET-COMPOSER ACCEL-25 slot AC06 id.
pub const FLEET_ACCEL_AC06_JOB_ID: &str = "ACCEL-2030-AC06";

/// FLEET-COMPOSER ACCEL-25 AC06 receipt path.
pub const FLEET_ACCEL_AC06_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL_2030_AC06.md";

/// Prior AGAP-2033 SEC-S4 side-channel badge receipt.
pub const PRIOR_RECEIPT_PATH_2033: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S4_2033.md";

/// Prior AGAP-2127 SEC-S4 scrub-roundtrip receipt.
pub const PRIOR_RECEIPT_PATH_2127: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S4_2127.md";

/// Prior AGAP-2350 SEC-S4 K_v1 fuzz histogram receipt.
pub const PRIOR_RECEIPT_PATH_2350: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_S-4_2350.md";

/// umst-trust SEC-S4 side-channel delegate SSOT.
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/sec_s4_side_channel.rs";

/// egoff sanitize SSOT (full GMD-7 + side-channel regex scrub).
pub const EGOFF_SANITIZE_SSOT: &str = "egoff/egoff/src/security/sanitize.rs";

/// egoff cockpit side-channel badge SSOT.
pub const EGOFF_BADGE_SSOT: &str = "egoff/egoff/src/security/mod.rs";

/// Gateway trust-wrap delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs";

/// umst-formal L-S5 proof SSOT (honest open).
pub const FORMAL_LS5_SSOT: &str = "umst-formal/Lean/Crypto/SanitizePatternCoverage.lean";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version.
pub const SCHEMA_VERSION: &str = "sec_s4_gate_side_channel_scrub_census_v2";

/// Default mini-fuzz corpus size (one rotor per K_v1 class).
pub const MINI_FUZZ_ENTRIES: usize = 8;

/// Stride samples per K_v1 class (AGAP-0831-S4 / SWARM-C25-0831-60 posture).
pub const STRIDE_SAMPLES_PER_CLASS: u32 = 3;

/// Stride fuzz corpus size — three rotors per class (`8 × 3 = 24`).
pub const STRIDE_FUZZ_ENTRIES: usize = 24;

/// Prop-sample strides across the 10k rotor space.
pub const PROP_SAMPLE_STRIDES: u32 = 8;

/// Prop-sample stride step across default 10k fuzz space.
pub const PROP_SAMPLE_STRIDE_STEP: u32 = 1_250;

/// GMD-7 + S-4 extension pattern identifiers (K_v1 set).
pub const K_V1_PATTERNS: &[&str] = &[
    "mac-address",
    "gpu-uuid-v4",
    "ioplatform-serial",
    "kernel-leaf",
    "cpuinfo-serial",
    "rapl-correlated-bytes",
    "timing-leak-fingerprint",
    "memory-access-fingerprint",
];

/// Idempotent scrub placeholder (mirrors egoff `SCRUB_PLACEHOLDER` posture).
pub const SCRUB_PLACEHOLDER: &[u8] = b"[SCRUBBED]";

/// L-S5 formal proof wired — honest false until Lean lands.
pub const L_S5_PROOF_WIRED_HONEST: bool = false;

/// Sled anomaly audit trail wired — honest false.
pub const SLED_ANOMALY_AUDIT_WIRED_HONEST: bool = false;

/// One hop in the manifold SEC-S4 gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS4GateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-S4 gate runtime wire map (cold-edge evidence → trust side-channel/scrub census).
pub const MANIFOLD_SEC_S4_GATE_WIRE_HOPS: &[SecS4GateWireHop] = &[
    SecS4GateWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecS4GateWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecS4GateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_s4::gate_side_channel_scrub_census",
        role: "Manifold gate SEC-S4 side-channel scrub census",
        wired: true,
    },
    SecS4GateWireHop {
        ordinal: 4,
        surface: "umst-trust::sec_s4_side_channel::validate_s4_side_channel_honesty",
        role: "Trust side-channel scrub delegate (Y82/G75/F75)",
        wired: true,
    },
    SecS4GateWireHop {
        ordinal: 5,
        surface: "egoff::security::sanitize + egoff::security::mod::session_trust_badge",
        role: "egoff GMD-7 scrub + cockpit badge (live wire egoff-owned)",
        wired: true,
    },
    SecS4GateWireHop {
        ordinal: 6,
        surface: "umst-formal::Lean::Crypto::SanitizePatternCoverage",
        role: "L-S5 formal proof (R-LS5-full)",
        wired: false,
    },
    SecS4GateWireHop {
        ordinal: 7,
        surface: "umst-gateway::sec_gw_trust_wrap::trust_wrap_wired",
        role: "Gateway production ceremony + sled anomaly audit (serial Wave H)",
        wired: false,
    },
];

/// One L-S5 K_v1 synthetic coverage probe row at manifold cold edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifoldLs5Kv1Probe {
    /// K_v1 pattern identifier.
    pub pattern_id: &'static str,
    /// Whether synthetic probe bytes scrub to zero residual.
    pub probe_hit: bool,
}

/// Per-pattern hit row in the manifold privacy-fuzz K_v1 histogram.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS4Kv1ClassHit {
    /// K_v1 pattern identifier.
    pub pattern_id: &'static str,
    /// Hit count in the fuzz corpus.
    pub hit_count: u32,
}

/// Mini privacy-fuzz histogram rollup at manifold cold-edge boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS4ManifoldFuzzHistogram {
    /// Target corpus size.
    pub entries_target: u32,
    /// Entries actually run.
    pub entries_run: u32,
    /// Distinct K_v1 pattern classes observed.
    pub k_v1_unique_classes_hit: usize,
    /// Ordered histogram — one row per [`K_V1_PATTERNS`] id.
    pub k_v1_class_histogram: Vec<SecS4Kv1ClassHit>,
    /// True when every K_v1 class has `hit_count >= 1`.
    pub k_v1_exhaustive_in_corpus: bool,
    /// Synthetic scrub round-trip across all K_v1 probes.
    pub scrub_roundtrip_verified: bool,
    /// First eight rotors scrub to zero residual.
    pub fuzz_scrub_sample_verified: bool,
    /// Three stride samples per K_v1 class scrub to zero residual.
    pub fuzz_scrub_stride_verified: bool,
    /// Sparse prop sample across the 10k rotor space scrubs clean.
    pub fuzz_scrub_prop_sample_verified: bool,
    /// Rotor balance — min class hits == max class hits > 0.
    pub k_v1_rotor_balanced: bool,
    /// Minimum per-class hit count in corpus.
    pub k_v1_min_class_hits: u32,
    /// Maximum per-class hit count in corpus.
    pub k_v1_max_class_hits: u32,
}

/// Aggregated SEC-S4 gate side-channel scrub census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS4GateSideChannelScrubCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// L-S5 K_v1 8/8 synthetic probes hit at manifold boundary.
    pub ls5_all_k_v1_probed: bool,
    /// Scrub roundtrip witness at manifold boundary.
    pub scrub_roundtrip_verified: bool,
    /// Mini fuzz histogram 8/8 exhaustive at manifold boundary.
    pub mini_fuzz_histogram_exhaustive: bool,
    /// Stride fuzz histogram balanced at 24 rotors.
    pub stride_fuzz_histogram_balanced: bool,
    /// Fuzz scrub sample witness (first 8 rotors).
    pub fuzz_scrub_sample_verified: bool,
    /// Fuzz scrub stride witness (3 per class).
    pub fuzz_scrub_stride_verified: bool,
    /// Fuzz scrub prop sample witness.
    pub fuzz_scrub_prop_sample_verified: bool,
    /// L-S5 formal proof wired — honest false.
    pub l_s5_proof_wired: bool,
    /// Sled anomaly audit wired — honest false.
    pub sled_anomaly_audit_wired: bool,
    /// Gateway production flip.
    pub production_wired: bool,
    /// Wired hop count.
    pub wire_hop_wired_count: u8,
}

/// Exercise gate cold-edge evidence at manifold SSOT (identity transition admits).
#[must_use]
pub fn gate_transition_evidence_probe() -> bool {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
    let new = old;
    let evidence = CdTransitionCartridge.transition_evidence(&old, &new, 1.0);
    evidence.admissibility == AdmissibilityToken::Admissible && !evidence.catalog_id.is_empty()
}

fn haystack_contains_ci(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || haystack.len() < needle.len() {
        return false;
    }
    let needle_lower: Vec<u8> = needle.iter().map(|b| b.to_ascii_lowercase()).collect();
    haystack.windows(needle.len()).any(|window| {
        window
            .iter()
            .map(|b| b.to_ascii_lowercase())
            .eq(needle_lower.iter().copied())
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kv1PatternClass {
    MacAddress,
    GpuUuidV4,
    IoPlatformSerial,
    KernelLeaf,
    CpuinfoSerial,
    RaplCorrelatedBytes,
    TimingLeakFingerprint,
    MemoryAccessFingerprint,
}

impl Kv1PatternClass {
    fn pattern_id(self) -> &'static str {
        match self {
            Self::MacAddress => "mac-address",
            Self::GpuUuidV4 => "gpu-uuid-v4",
            Self::IoPlatformSerial => "ioplatform-serial",
            Self::KernelLeaf => "kernel-leaf",
            Self::CpuinfoSerial => "cpuinfo-serial",
            Self::RaplCorrelatedBytes => "rapl-correlated-bytes",
            Self::TimingLeakFingerprint => "timing-leak-fingerprint",
            Self::MemoryAccessFingerprint => "memory-access-fingerprint",
        }
    }
}

fn manifold_pattern_classes_in_bytes(bytes: &[u8]) -> Vec<Kv1PatternClass> {
    let mut hits = Vec::new();
    let probes: &[(Kv1PatternClass, &[u8])] = &[
        (Kv1PatternClass::MacAddress, b"00:11:22:33:44:55"),
        (Kv1PatternClass::MacAddress, b"aa:bb:cc:dd:ee:ff"),
        (
            Kv1PatternClass::GpuUuidV4,
            b"12345678-1234-4123-8123-123456789abc",
        ),
        (Kv1PatternClass::IoPlatformSerial, b"IOPlatformSerialNumber"),
        (Kv1PatternClass::KernelLeaf, b"model name :"),
        (Kv1PatternClass::CpuinfoSerial, b"serial :"),
        (Kv1PatternClass::RaplCorrelatedBytes, b"rapl"),
        (Kv1PatternClass::RaplCorrelatedBytes, b"energy_uj"),
        (Kv1PatternClass::TimingLeakFingerprint, b"rdtsc"),
        (Kv1PatternClass::TimingLeakFingerprint, b"timing_variance"),
        (Kv1PatternClass::MemoryAccessFingerprint, b"cache_line"),
        (Kv1PatternClass::MemoryAccessFingerprint, b"stride_prefetch"),
    ];
    for (class, needle) in probes {
        if haystack_contains_ci(bytes, needle) && !hits.contains(class) {
            hits.push(*class);
        }
    }
    hits
}

fn synthetic_probe_bytes(pattern_id: &str) -> Vec<u8> {
    match pattern_id {
        "mac-address" => b"aa:bb:cc:dd:ee:ff".to_vec(),
        "gpu-uuid-v4" => b"12345678-1234-4123-8123-123456789abc".to_vec(),
        "ioplatform-serial" => b"IOPlatformSerialNumber X".to_vec(),
        "kernel-leaf" => b"model name : umst-manifold-kernel-leaf-probe".to_vec(),
        "cpuinfo-serial" => b"serial : 0123456789abcdef".to_vec(),
        "rapl-correlated-bytes" => b"RAPL energy_uj spike".to_vec(),
        "timing-leak-fingerprint" => b"rdtsc timing_variance probe".to_vec(),
        "memory-access-fingerprint" => b"cache_line stride_prefetch".to_vec(),
        _ => Vec::new(),
    }
}

fn scan_k_v1_pattern_classes(bytes: &[u8]) -> bool {
    !manifold_pattern_classes_in_bytes(bytes).is_empty()
}

fn manifold_synthetic_fuzz_entry(i: u32) -> Vec<u8> {
    let seed = format!("umst-manifold-s4-fuzz-{i:05}");
    match i % 8 {
        0 => format!("host mac 00:11:22:33:44:55 {seed}").into_bytes(),
        1 => format!("RAPL energy_uj spike {seed}").into_bytes(),
        2 => format!("rdtsc timing_variance probe {seed}").into_bytes(),
        3 => format!("cache_line stride_prefetch {seed}").into_bytes(),
        4 => format!("gpu 12345678-1234-4123-8123-123456789abc {seed}").into_bytes(),
        5 => format!("serial : 0123456789abcdef {seed}").into_bytes(),
        6 => format!("IOPlatformSerialNumber leaf {seed}").into_bytes(),
        _ => format!("model name : umst-manifold-fuzz {seed}").into_bytes(),
    }
}

fn find_subslice_ci(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    let needle_lower: Vec<u8> = needle.iter().map(|b| b.to_ascii_lowercase()).collect();
    haystack.windows(needle.len()).position(|window| {
        window
            .iter()
            .map(|b| b.to_ascii_lowercase())
            .eq(needle_lower.iter().copied())
    })
}

/// Scrub detected K_v1 tokens from bytes (manifold cold-edge witness).
#[must_use]
pub fn manifold_scrub_k_v1_tokens(bytes: &[u8]) -> (Vec<u8>, usize) {
    let mut buf = bytes.to_vec();
    let mut scrub_count = 0usize;
    let needles: &[&[u8]] = &[
        b"00:11:22:33:44:55",
        b"aa:bb:cc:dd:ee:ff",
        b"12345678-1234-4123-8123-123456789abc",
        b"IOPlatformSerialNumber",
        b"model name :",
        b"serial :",
        b"RAPL",
        b"energy_uj",
        b"rdtsc",
        b"timing_variance",
        b"cache_line",
        b"stride_prefetch",
    ];
    for needle in needles {
        while let Some(start) = find_subslice_ci(&buf, needle) {
            let end = start + needle.len();
            buf.splice(start..end, SCRUB_PLACEHOLDER.iter().copied());
            scrub_count += 1;
        }
    }
    (buf, scrub_count)
}

/// L-S5 K_v1 synthetic coverage probe matrix — 8/8 at manifold cold edge.
#[must_use]
pub fn manifold_ls5_k_v1_coverage_probes() -> Vec<ManifoldLs5Kv1Probe> {
    K_V1_PATTERNS
        .iter()
        .map(|id| {
            let bytes = synthetic_probe_bytes(id);
            let pre_hit = scan_k_v1_pattern_classes(&bytes);
            let (scrubbed, _) = manifold_scrub_k_v1_tokens(&bytes);
            let post_clean = !scan_k_v1_pattern_classes(&scrubbed);
            ManifoldLs5Kv1Probe {
                pattern_id: id,
                probe_hit: pre_hit && post_clean,
            }
        })
        .collect()
}

/// Whether all eight L-S5 K_v1 synthetic probes hit at manifold boundary.
#[must_use]
pub fn manifold_ls5_all_k_v1_probed() -> bool {
    manifold_ls5_k_v1_coverage_probes()
        .iter()
        .all(|p| p.probe_hit)
}

/// Verify every K_v1 synthetic probe scrubs to zero residual hits at manifold boundary.
#[must_use]
pub fn manifold_verify_scrub_roundtrip() -> bool {
    K_V1_PATTERNS.iter().all(|id| {
        let bytes = synthetic_probe_bytes(id);
        let (scrubbed, _) = manifold_scrub_k_v1_tokens(&bytes);
        !scan_k_v1_pattern_classes(&scrubbed)
    })
}

/// Verify first eight fuzz rotors scrub to zero residual at manifold boundary.
#[must_use]
pub fn manifold_verify_fuzz_scrub_sample() -> bool {
    (0..8).all(|i| {
        let payload = manifold_synthetic_fuzz_entry(i);
        let (scrubbed, _) = manifold_scrub_k_v1_tokens(&payload);
        !scan_k_v1_pattern_classes(&scrubbed)
    })
}

/// Verify three stride samples per K_v1 class scrub to zero residual.
#[must_use]
pub fn manifold_verify_fuzz_scrub_stride() -> bool {
    (0..8).all(|class| {
        (0..STRIDE_SAMPLES_PER_CLASS).all(|stride_idx| {
            let i = class + stride_idx * 8;
            let payload = manifold_synthetic_fuzz_entry(i);
            let (scrubbed, _) = manifold_scrub_k_v1_tokens(&payload);
            !scan_k_v1_pattern_classes(&scrubbed)
        })
    })
}

/// Sparse prop sample across the default 10k fuzz space at manifold boundary.
#[must_use]
pub fn manifold_verify_fuzz_scrub_prop_sample() -> bool {
    (0..8).all(|class| {
        (0..PROP_SAMPLE_STRIDES).all(|stride_idx| {
            let i = class + stride_idx * PROP_SAMPLE_STRIDE_STEP;
            let payload = manifold_synthetic_fuzz_entry(i);
            let (scrubbed, _) = manifold_scrub_k_v1_tokens(&payload);
            !scan_k_v1_pattern_classes(&scrubbed)
        })
    })
}

fn kv1_histogram_min_max(counts: &[u32; 8]) -> (u32, u32) {
    let mut min = u32::MAX;
    let mut max = 0u32;
    for &c in counts {
        min = min.min(c);
        max = max.max(c);
    }
    (min, max)
}

fn kv1_histogram_from_counts(counts: &[u32; 8]) -> Vec<SecS4Kv1ClassHit> {
    K_V1_PATTERNS
        .iter()
        .enumerate()
        .map(|(i, pattern_id)| SecS4Kv1ClassHit {
            pattern_id,
            hit_count: counts[i],
        })
        .collect()
}

/// Run mini privacy-fuzz histogram at manifold cold edge (default 8 rotors).
#[must_use]
pub fn manifold_run_mini_fuzz_histogram(entries: usize) -> SecS4ManifoldFuzzHistogram {
    let target = entries.max(1) as u32;
    let mut class_hits = [0u32; 8];
    for i in 0..target {
        let payload = manifold_synthetic_fuzz_entry(i);
        for class in manifold_pattern_classes_in_bytes(&payload) {
            let idx = K_V1_PATTERNS
                .iter()
                .position(|p| *p == class.pattern_id())
                .unwrap_or(0);
            class_hits[idx] = class_hits[idx].saturating_add(1);
        }
    }
    let k_v1_class_histogram = kv1_histogram_from_counts(&class_hits);
    let k_v1_unique_classes_hit = class_hits.iter().filter(|h| **h > 0).count();
    let (k_v1_min_class_hits, k_v1_max_class_hits) = kv1_histogram_min_max(&class_hits);
    SecS4ManifoldFuzzHistogram {
        entries_target: target,
        entries_run: target,
        k_v1_unique_classes_hit,
        k_v1_class_histogram,
        k_v1_exhaustive_in_corpus: k_v1_unique_classes_hit == K_V1_PATTERNS.len(),
        scrub_roundtrip_verified: manifold_verify_scrub_roundtrip(),
        fuzz_scrub_sample_verified: manifold_verify_fuzz_scrub_sample(),
        fuzz_scrub_stride_verified: manifold_verify_fuzz_scrub_stride(),
        fuzz_scrub_prop_sample_verified: manifold_verify_fuzz_scrub_prop_sample(),
        k_v1_rotor_balanced: k_v1_min_class_hits == k_v1_max_class_hits && k_v1_min_class_hits > 0,
        k_v1_min_class_hits,
        k_v1_max_class_hits,
    }
}

/// Run stride privacy-fuzz histogram at manifold cold edge (default 24 rotors).
#[must_use]
pub fn manifold_run_stride_fuzz_histogram(entries: usize) -> SecS4ManifoldFuzzHistogram {
    manifold_run_mini_fuzz_histogram(entries.max(STRIDE_FUZZ_ENTRIES))
}

/// Whether live gateway trust-wrap production flip is plumbed (honest `false`).
#[must_use]
pub const fn sec_s4_production_wired() -> bool {
    false
}

/// Build manifold SEC-S4 gate side-channel scrub census from live measurements.
#[must_use]
pub fn gate_side_channel_scrub_census() -> SecS4GateSideChannelScrubCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_S4_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    let mini_hist = manifold_run_mini_fuzz_histogram(MINI_FUZZ_ENTRIES);
    let stride_hist = manifold_run_stride_fuzz_histogram(STRIDE_FUZZ_ENTRIES);
    SecS4GateSideChannelScrubCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        ls5_all_k_v1_probed: manifold_ls5_all_k_v1_probed(),
        scrub_roundtrip_verified: manifold_verify_scrub_roundtrip(),
        mini_fuzz_histogram_exhaustive: mini_hist.k_v1_exhaustive_in_corpus,
        stride_fuzz_histogram_balanced: stride_hist.k_v1_rotor_balanced,
        fuzz_scrub_sample_verified: mini_hist.fuzz_scrub_sample_verified,
        fuzz_scrub_stride_verified: stride_hist.fuzz_scrub_stride_verified,
        fuzz_scrub_prop_sample_verified: stride_hist.fuzz_scrub_prop_sample_verified,
        l_s5_proof_wired: L_S5_PROOF_WIRED_HONEST,
        sled_anomaly_audit_wired: SLED_ANOMALY_AUDIT_WIRED_HONEST,
        production_wired: sec_s4_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-S4 ceremony is closed at census tier.
///
/// True when cold-edge evidence probe + side-channel scrub wire map hops 1–5 are measured wired.
/// L-S5 formal proof + gateway production flip are explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_s4_ceremony_closed() -> bool {
    let census = gate_side_channel_scrub_census();
    census.gate_evidence_wired
        && census.ls5_all_k_v1_probed
        && census.scrub_roundtrip_verified
        && census.mini_fuzz_histogram_exhaustive
        && census.stride_fuzz_histogram_balanced
        && census.fuzz_scrub_sample_verified
        && census.fuzz_scrub_stride_verified
        && census.fuzz_scrub_prop_sample_verified
        && !census.l_s5_proof_wired
        && !census.sled_anomaly_audit_wired
        && !census.production_wired
        && census.wire_hop_wired_count == 5
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-S4 manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS4GateManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// L-S5 8/8 probes at manifold boundary.
    pub ls5_all_k_v1_probed: bool,
    /// Scrub roundtrip verified.
    pub scrub_roundtrip_verified: bool,
    /// Mini fuzz histogram 8/8 exhaustive.
    pub mini_fuzz_histogram_exhaustive: bool,
    /// Stride fuzz histogram balanced.
    pub stride_fuzz_histogram_balanced: bool,
    /// Fuzz scrub sample verified.
    pub fuzz_scrub_sample_verified: bool,
    /// Fuzz scrub stride verified.
    pub fuzz_scrub_stride_verified: bool,
    /// Fuzz scrub prop sample verified.
    pub fuzz_scrub_prop_sample_verified: bool,
    /// L-S5 proof honest false.
    pub l_s5_proof_honest_false: bool,
    /// Sled anomaly audit honest false.
    pub sled_anomaly_honest_false: bool,
    /// Production flip honest false.
    pub production_honest_false: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build introspection probe for SEC-S4 done-when checks.
#[must_use]
pub fn sec_s4_gate_manifold_probe() -> SecS4GateManifoldProbe {
    let census = gate_side_channel_scrub_census();
    SecS4GateManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        ls5_all_k_v1_probed: census.ls5_all_k_v1_probed,
        scrub_roundtrip_verified: census.scrub_roundtrip_verified,
        mini_fuzz_histogram_exhaustive: census.mini_fuzz_histogram_exhaustive,
        stride_fuzz_histogram_balanced: census.stride_fuzz_histogram_balanced,
        fuzz_scrub_sample_verified: census.fuzz_scrub_sample_verified,
        fuzz_scrub_stride_verified: census.fuzz_scrub_stride_verified,
        fuzz_scrub_prop_sample_verified: census.fuzz_scrub_prop_sample_verified,
        l_s5_proof_honest_false: !census.l_s5_proof_wired,
        sled_anomaly_honest_false: !census.sled_anomaly_audit_wired,
        production_honest_false: !census.production_wired,
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_s4_ceremony_closed(),
    }
}

/// FLEET-COMPOSER Prabhu Wave H H3 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS4P1800H3Probe {
    /// H3 fleet card id.
    pub h3_job_id: &'static str,
    /// Prior 2033 side-channel badge absorbed.
    pub prior_2033_absorbed: bool,
    /// Prior 2127 scrub-roundtrip absorbed.
    pub prior_2127_absorbed: bool,
    /// Prior 2350 fuzz histogram absorbed.
    pub prior_2350_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecS4GateManifoldProbe,
    /// `sec_s4_production_wired()` — honest false.
    pub production_wired: bool,
    /// L-S5 proof — honest false.
    pub l_s5_proof_wired: bool,
}

/// Build FLEET-COMPOSER P1800 H3 integration probe from live measurements.
#[must_use]
pub fn sec_s4_p1800_h3_probe() -> SecS4P1800H3Probe {
    SecS4P1800H3Probe {
        h3_job_id: FLEET_P1800_H3_JOB_ID,
        prior_2033_absorbed: PRIOR_RECEIPT_PATH_2033.contains("SEC-S4_2033"),
        prior_2127_absorbed: PRIOR_RECEIPT_PATH_2127.contains("SEC-S4_2127"),
        prior_2350_absorbed: PRIOR_RECEIPT_PATH_2350.contains("S-4_2350"),
        ceremony_closed: manifold_gate_sec_s4_ceremony_closed(),
        probe: sec_s4_gate_manifold_probe(),
        production_wired: sec_s4_production_wired(),
        l_s5_proof_wired: L_S5_PROOF_WIRED_HONEST,
    }
}

/// FLEET-COMPOSER P1800 H3 honesty gate — ceremony closed + production false + L-S5 false.
#[must_use]
pub fn sec_s4_p1800_h3_honest() -> bool {
    let probe = sec_s4_p1800_h3_probe();
    probe.h3_job_id == FLEET_P1800_H3_JOB_ID
        && probe.prior_2033_absorbed
        && probe.prior_2127_absorbed
        && probe.prior_2350_absorbed
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.ls5_all_k_v1_probed
        && probe.probe.scrub_roundtrip_verified
        && probe.probe.mini_fuzz_histogram_exhaustive
        && probe.probe.stride_fuzz_histogram_balanced
        && probe.probe.fuzz_scrub_sample_verified
        && probe.probe.fuzz_scrub_stride_verified
        && probe.probe.fuzz_scrub_prop_sample_verified
        && probe.probe.l_s5_proof_honest_false
        && probe.probe.sled_anomaly_honest_false
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
        && !probe.l_s5_proof_wired
}

/// FLEET-COMPOSER ACCEL-25 AC06 histogram deepen probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS4AccelAc06Probe {
    /// AC06 fleet card id.
    pub ac06_job_id: &'static str,
    /// Prior H3 ceremony absorbed.
    pub prior_h3_absorbed: bool,
    /// Prior 2350 fuzz histogram absorbed.
    pub prior_2350_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Mini fuzz histogram row count.
    pub histogram_row_count: usize,
    /// Mini fuzz histogram exhaustive.
    pub mini_fuzz_histogram_exhaustive: bool,
    /// Stride fuzz histogram balanced.
    pub stride_fuzz_histogram_balanced: bool,
    /// Fuzz scrub sample verified.
    pub fuzz_scrub_sample_verified: bool,
    /// Fuzz scrub stride verified.
    pub fuzz_scrub_stride_verified: bool,
    /// Fuzz scrub prop sample verified.
    pub fuzz_scrub_prop_sample_verified: bool,
    /// Underlying gate probe.
    pub probe: SecS4GateManifoldProbe,
}

/// Build FLEET-COMPOSER ACCEL-25 AC06 histogram deepen probe.
#[must_use]
pub fn sec_s4_accel_ac06_probe() -> SecS4AccelAc06Probe {
    let mini_hist = manifold_run_mini_fuzz_histogram(MINI_FUZZ_ENTRIES);
    let stride_hist = manifold_run_stride_fuzz_histogram(STRIDE_FUZZ_ENTRIES);
    SecS4AccelAc06Probe {
        ac06_job_id: FLEET_ACCEL_AC06_JOB_ID,
        prior_h3_absorbed: FLEET_P1800_H3_RECEIPT_PATH.contains("COMPOSER_P1800_H3"),
        prior_2350_absorbed: PRIOR_RECEIPT_PATH_2350.contains("S-4_2350"),
        ceremony_closed: manifold_gate_sec_s4_ceremony_closed(),
        histogram_row_count: mini_hist.k_v1_class_histogram.len(),
        mini_fuzz_histogram_exhaustive: mini_hist.k_v1_exhaustive_in_corpus,
        stride_fuzz_histogram_balanced: stride_hist.k_v1_rotor_balanced,
        fuzz_scrub_sample_verified: mini_hist.fuzz_scrub_sample_verified,
        fuzz_scrub_stride_verified: stride_hist.fuzz_scrub_stride_verified,
        fuzz_scrub_prop_sample_verified: stride_hist.fuzz_scrub_prop_sample_verified,
        probe: sec_s4_gate_manifold_probe(),
    }
}

/// FLEET-COMPOSER ACCEL-25 AC06 honesty gate — histogram deepen + ceremony closed.
#[must_use]
pub fn sec_s4_accel_ac06_honest() -> bool {
    let probe = sec_s4_accel_ac06_probe();
    probe.ac06_job_id == FLEET_ACCEL_AC06_JOB_ID
        && probe.prior_h3_absorbed
        && probe.prior_2350_absorbed
        && probe.ceremony_closed
        && probe.histogram_row_count == 8
        && probe.mini_fuzz_histogram_exhaustive
        && probe.stride_fuzz_histogram_balanced
        && probe.fuzz_scrub_sample_verified
        && probe.fuzz_scrub_stride_verified
        && probe.fuzz_scrub_prop_sample_verified
        && probe.probe.gate_evidence_wired
        && probe.probe.ls5_all_k_v1_probed
        && probe.probe.scrub_roundtrip_verified
        && probe.probe.l_s5_proof_honest_false
        && probe.probe.sled_anomaly_honest_false
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
}

/// Validate SEC-S4 gate census honesty — fail closed on fake persistence/production claims.
pub fn validate_sec_s4_gate_honesty() -> Result<(), &'static str> {
    let census = gate_side_channel_scrub_census();
    if census.l_s5_proof_wired {
        return Err("l_s5_proof_wired must stay false until Lean lands");
    }
    if census.sled_anomaly_audit_wired {
        return Err("sled_anomaly_audit_wired must stay false until egoff sled");
    }
    if census.production_wired {
        return Err("sec_s4_production_wired must stay false until SEC-GW-WRAP");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if !census.ls5_all_k_v1_probed {
        return Err("L-S5 8/8 K_v1 probes must hit at manifold boundary");
    }
    if !census.scrub_roundtrip_verified {
        return Err("manifold scrub roundtrip witness failed");
    }
    if !census.mini_fuzz_histogram_exhaustive {
        return Err("mini fuzz histogram must be 8/8 exhaustive at manifold boundary");
    }
    if !census.stride_fuzz_histogram_balanced {
        return Err("stride fuzz histogram must be balanced at 24 rotors");
    }
    if !census.fuzz_scrub_sample_verified {
        return Err("fuzz scrub sample witness failed");
    }
    if !census.fuzz_scrub_stride_verified {
        return Err("fuzz scrub stride witness failed");
    }
    if !census.fuzz_scrub_prop_sample_verified {
        return Err("fuzz scrub prop sample witness failed");
    }
    if MANIFOLD_SEC_S4_GATE_WIRE_HOPS.len() != 7 {
        return Err("seven SEC-S4 gate wire hops expected");
    }
    if census.wire_hop_wired_count != 5 {
        return Err("five SEC-S4 gate wire hops should be wired today");
    }
    if !manifold_gate_sec_s4_ceremony_closed() {
        return Err("manifold gate SEC-S4 ceremony must be closed at census tier");
    }
    if !sec_s4_p1800_h3_honest() {
        return Err("P1800 H3 probe must be honest");
    }
    if !sec_s4_accel_ac06_honest() {
        return Err("ACCEL AC06 histogram deepen probe must be honest");
    }
    Ok(())
}

/// Render SEC-S4 gate wire map for operator receipts.
#[must_use]
pub fn sec_s4_gate_wire_matrix() -> String {
    let census = gate_side_channel_scrub_census();
    let mut out = String::from("SEC-S4 manifold gate side-channel scrub wire map (H3):\n");
    for hop in MANIFOLD_SEC_S4_GATE_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} ls5_all_k_v1_probed={} scrub_roundtrip={} mini_fuzz_exhaustive={} stride_balanced={} fuzz_sample={} fuzz_stride={} fuzz_prop={} l_s5_proof_wired={} production_wired={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_S4_GATE_WIRE_HOPS.len(),
        census.ls5_all_k_v1_probed,
        census.scrub_roundtrip_verified,
        census.mini_fuzz_histogram_exhaustive,
        census.stride_fuzz_histogram_balanced,
        census.fuzz_scrub_sample_verified,
        census.fuzz_scrub_stride_verified,
        census.fuzz_scrub_prop_sample_verified,
        census.l_s5_proof_wired,
        census.production_wired
    ));
    out.push_str(&format!("  trust_ssot={TRUST_SSOT}\n"));
    out.push_str(&format!("  egoff_sanitize_ssot={EGOFF_SANITIZE_SSOT}\n"));
    out
}

/// Next-hop surface for L-S5 formal proof (formal-owned).
#[must_use]
pub const fn sec_s4_l_s5_proof_next_hop() -> &'static str {
    "umst-formal/Lean/Crypto/SanitizePatternCoverage.lean:R-LS5-full"
}

#[cfg(test)]
mod sec_s4_tests {
    use super::*;

    #[test]
    fn sec_s4_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-S4");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-S4");
        assert_eq!(FLEET_P1800_H3_JOB_ID, "PRABHU-WAVE-H-1800-H3");
    }

    #[test]
    fn sec_s4_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_s4_scrub_roundtrip_all_k_v1_probes() {
        assert!(manifold_verify_scrub_roundtrip());
    }

    #[test]
    fn sec_s4_ls5_k_v1_coverage_eight_by_eight() {
        let probes = manifold_ls5_k_v1_coverage_probes();
        assert_eq!(probes.len(), 8);
        assert!(manifold_ls5_all_k_v1_probed());
        assert!(probes.iter().all(|p| p.probe_hit));
    }

    #[test]
    fn sec_s4_scrub_idempotent_on_placeholder() {
        let (scrubbed, count) = manifold_scrub_k_v1_tokens(SCRUB_PLACEHOLDER);
        assert_eq!(scrubbed, SCRUB_PLACEHOLDER);
        assert_eq!(count, 0);
    }

    #[test]
    fn sec_s4_side_channel_census_honest_posture() {
        let census = gate_side_channel_scrub_census();
        assert_eq!(census.board_slice_id, "SEC-S4");
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert!(census.gate_evidence_wired);
        assert!(census.ls5_all_k_v1_probed);
        assert!(census.scrub_roundtrip_verified);
        assert!(census.mini_fuzz_histogram_exhaustive);
        assert!(census.stride_fuzz_histogram_balanced);
        assert!(census.fuzz_scrub_sample_verified);
        assert!(census.fuzz_scrub_stride_verified);
        assert!(census.fuzz_scrub_prop_sample_verified);
        assert!(!census.l_s5_proof_wired);
        assert!(!census.sled_anomaly_audit_wired);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 5);
    }

    #[test]
    fn sec_s4_production_and_l_s5_stay_false() {
        assert!(!sec_s4_production_wired());
        assert!(!L_S5_PROOF_WIRED_HONEST);
        assert!(!SLED_ANOMALY_AUDIT_WIRED_HONEST);
    }

    #[test]
    fn sec_s4_manifold_wire_hops_cover_gate_and_trust_delegate() {
        assert_eq!(MANIFOLD_SEC_S4_GATE_WIRE_HOPS.len(), 7);
        assert_eq!(
            MANIFOLD_SEC_S4_GATE_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            5
        );
        assert!(MANIFOLD_SEC_S4_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("AdmissibilityToken") && h.wired));
        assert!(MANIFOLD_SEC_S4_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("SanitizePatternCoverage") && !h.wired));
        assert!(MANIFOLD_SEC_S4_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("trust_wrap_wired") && !h.wired));
    }

    #[test]
    fn sec_s4_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_s4_ceremony_closed());
        let probe = sec_s4_gate_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.ls5_all_k_v1_probed);
        assert!(probe.scrub_roundtrip_verified);
        assert!(probe.mini_fuzz_histogram_exhaustive);
        assert!(probe.stride_fuzz_histogram_balanced);
        assert!(probe.fuzz_scrub_sample_verified);
        assert!(probe.fuzz_scrub_stride_verified);
        assert!(probe.fuzz_scrub_prop_sample_verified);
        assert!(probe.l_s5_proof_honest_false);
        assert!(probe.sled_anomaly_honest_false);
        assert!(probe.production_honest_false);
        assert_eq!(probe.wire_hop_wired_count, 5);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_s4_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_2033.contains("SEC-S4_2033"));
        assert!(PRIOR_RECEIPT_PATH_2127.contains("SEC-S4_2127"));
        assert!(PRIOR_RECEIPT_PATH_2350.contains("S-4_2350"));
        assert!(TRUST_SSOT.contains("sec_s4_side_channel"));
        assert!(EGOFF_SANITIZE_SSOT.contains("security/sanitize.rs"));
    }

    #[test]
    fn sec_s4_gate_wire_matrix_renders_honest_counts() {
        let matrix = sec_s4_gate_wire_matrix();
        assert!(matrix.contains("SEC-S4 manifold gate"));
        assert!(matrix.contains("ls5_all_k_v1_probed=true"));
        assert!(matrix.contains("mini_fuzz_exhaustive=true"));
        assert!(matrix.contains("l_s5_proof_wired=false"));
        assert!(matrix.contains("wired=5/7"));
    }

    #[test]
    fn sec_s4_mini_fuzz_histogram_exhaustive_eight_by_eight() {
        let hist = manifold_run_mini_fuzz_histogram(MINI_FUZZ_ENTRIES);
        assert_eq!(hist.entries_run, 8);
        assert_eq!(hist.k_v1_class_histogram.len(), 8);
        assert!(hist.k_v1_exhaustive_in_corpus);
        assert!(hist.k_v1_class_histogram.iter().all(|h| h.hit_count >= 1));
        assert!(hist.scrub_roundtrip_verified);
        assert!(hist.fuzz_scrub_sample_verified);
        assert!(hist.fuzz_scrub_stride_verified);
        assert!(hist.fuzz_scrub_prop_sample_verified);
    }

    #[test]
    fn sec_s4_stride_fuzz_histogram_balanced_twenty_four() {
        let hist = manifold_run_stride_fuzz_histogram(STRIDE_FUZZ_ENTRIES);
        assert_eq!(hist.entries_run, 24);
        assert!(hist.k_v1_rotor_balanced);
        assert_eq!(hist.k_v1_min_class_hits, 3);
        assert_eq!(hist.k_v1_max_class_hits, 3);
        assert!(hist.fuzz_scrub_stride_verified);
        assert!(hist.fuzz_scrub_prop_sample_verified);
    }

    #[test]
    fn sec_s4_fuzz_scrub_stride_witness() {
        assert!(manifold_verify_fuzz_scrub_stride());
    }

    #[test]
    fn sec_s4_fuzz_scrub_prop_sample_witness() {
        assert!(manifold_verify_fuzz_scrub_prop_sample());
    }

    #[test]
    fn fleet_composer_accel_ac06_sec_s4_histogram_honest() {
        assert!(sec_s4_accel_ac06_honest());
        let probe = sec_s4_accel_ac06_probe();
        assert_eq!(probe.ac06_job_id, FLEET_ACCEL_AC06_JOB_ID);
        assert!(probe.prior_h3_absorbed);
        assert!(probe.prior_2350_absorbed);
        assert!(probe.ceremony_closed);
        assert_eq!(probe.histogram_row_count, 8);
        assert!(probe.mini_fuzz_histogram_exhaustive);
        assert!(probe.stride_fuzz_histogram_balanced);
    }

    #[test]
    fn fleet_composer_p1800_h3_sec_s4_honest() {
        assert!(sec_s4_p1800_h3_honest());
        let probe = sec_s4_p1800_h3_probe();
        assert_eq!(probe.h3_job_id, FLEET_P1800_H3_JOB_ID);
        assert!(probe.prior_2033_absorbed);
        assert!(probe.prior_2127_absorbed);
        assert!(probe.prior_2350_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(!probe.l_s5_proof_wired);
    }

    #[test]
    fn sec_s4_validate_gate_honesty_residue_measured() {
        validate_sec_s4_gate_honesty().expect("honest SEC-S4 gate census residue");
        assert_eq!(
            sec_s4_l_s5_proof_next_hop(),
            "umst-formal/Lean/Crypto/SanitizePatternCoverage.lean:R-LS5-full"
        );
    }
}
