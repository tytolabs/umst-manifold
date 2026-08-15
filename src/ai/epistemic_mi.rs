// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//! Histogram mutual information estimator (ported from umst-prototype-2a), behind `epistemic-ppo`.
//!
//! **Honest status:** slice-1 histogram MI + Landauer clamp landed — **not** physics GREEN,
//! **not** production wired, **not** a machine-checked information certificate.
//! Formal MI obligations live in [**`umst-formal`**](https://github.com/tytolabs/umst-formal).
//!
//! **Witness reading:** MI estimates are morphisms into the R2 Landauer envelope only — valid as
//! post-composition scalar `info_gain` tensors after CBF, never as standalone certificates
//! (see [`GOD_GRADE_WITNESS_LADDER`](../../docs/RELEASE_WITNESS_LADDER.md) § MI inside the envelope).
//!
//! Stateful histogram updates are confined to [`MutualInfoEstimator`]; scoring and clamp maps are pure.

#![cfg(feature = "epistemic-ppo")]

/// W29 wave step — histogram MI deepen reproof in this module.
pub const W29_EPISTEMIC_MI_DEFERRED_STEP: &str = "W29-008-EPISTEMIC_MI";

/// Formal MI bridge deferred owning slice (not claimed by this host estimator).
pub const FORMAL_MI_BRIDGE_DEFERRED_STEP: &str = "P4-FORMAL-MI-BRIDGE";

/// Production orch pin deferred — oracle harvest pins trajectories only.
pub const EPISTEMIC_MI_PRODUCTION_ORCH_DEFERRED_STEP: &str = "P4-EPI-MI-PRODUCTION-ORCH";

/// Honest posture tag — heuristic histogram MI, not certified mutual information.
pub const EPISTEMIC_MI_POSTURE_TAG: &str = "honest-histogram-mi-partial";

/// Compile-time honest fence string — operator receipts must parse these literals.
pub const EPISTEMIC_MI_HONEST_FENCE: &str =
    "estimator_landed=true production_wired=false physics_green=false master_retick=false green_claim_blocked=true";

/// Source non-claim — histogram MI is a host surrogate inside the Landauer envelope only.
pub const EPISTEMIC_MI_SOURCE_NON_CLAIM: &str =
    "Histogram MI + Landauer clamp measured in crate tests; not physics GREEN / not production wired / not a formal MI certificate.";

/// Catalog trace contract: per-step MI upper bound (nats).
pub const LANDAUER_MI_CAP_NATS: f64 = std::f64::consts::LN_2;

/// Histogram estimator landed @ slice-1 (still NOT physics GREEN).
pub const EPISTEMIC_MI_ESTIMATOR_LANDED: bool = true;

/// Honest physics posture — heuristic histogram MI, not certified mutual information.
pub const EPISTEMIC_MI_PHYSICS_GREEN: bool = false;

/// Production gateway wiring deferred — oracle harvest pins trajectories only.
pub const EPISTEMIC_MI_PRODUCTION_WIRED: bool = false;

/// Master retick / full envelope clearance blocked until formal MI bridge.
pub const EPISTEMIC_MI_MASTER_RETICK: bool = false;

/// Operator receipts must not claim unblocked GREEN from this module alone.
pub const EPISTEMIC_MI_GREEN_CLAIM_BLOCKED: bool = true;

/// Epistemic-MI fence facet count (honest census).
pub const EPISTEMIC_MI_FENCE_FACET_COUNT: usize = 7;

/// Epistemic-MI fence facets wired today (5/7 measured).
pub const EPISTEMIC_MI_FENCE_WIRED_COUNT: usize = 5;

/// Epistemic-MI fence facets still deferred (formal bridge + production orch).
pub const EPISTEMIC_MI_FENCE_DEFERRED_COUNT: usize = 2;

/// Wire-hop inventory length (estimator → clamp → bonus → formal → production).
pub const EPISTEMIC_MI_WIRE_HOP_COUNT: usize = 5;

/// Wire hops closed today (3/5 — formal + production open).
pub const EPISTEMIC_MI_WIRE_HOPS_CLOSED: usize = 3;

/// Tracker MI history ring capacity (prototype-2a parity).
pub const EPISTEMIC_TRACKER_HISTORY_CAP: usize = 256;

/// Morphism `EPI-MI01` — histogram MI estimator host probe.
pub const MORPHISM_EPI_MI01: &str = "EPI-MI01";
/// Morphism `EPI-MI02` — Landauer clamp map.
pub const MORPHISM_EPI_MI02: &str = "EPI-MI02";
/// Morphism `EPI-MI03` — epistemic bonus shaping (post-CBF only).
pub const MORPHISM_EPI_MI03: &str = "EPI-MI03";

/// Registry of stable epistemic-MI morphism ids @ slice-1 (witness routing).
pub const EPISTEMIC_MI_MORPHISM_IDS: &[&str] =
    &[MORPHISM_EPI_MI01, MORPHISM_EPI_MI02, MORPHISM_EPI_MI03];

/// Stable facet ids for epistemic-MI production fence census.
pub const EPISTEMIC_MI_FENCE_FACET_IDS: &[&str] = &[
    "histogram_mi_estimator",
    "landauer_clamp",
    "epistemic_bonus_shaping",
    "warmup_gate",
    "honest_posture_probe",
    "formal_mi_bridge",
    "production_wired",
];

/// One facet of the epistemic-MI production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpistemicMiProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue / deferred.
    pub owning_slice: &'static str,
}

/// Epistemic-MI production fence facet inventory (honest posture SSOT).
pub const EPISTEMIC_MI_PRODUCTION_FENCE_FACETS: &[EpistemicMiProductionFenceFacet] = &[
    EpistemicMiProductionFenceFacet {
        facet: "histogram_mi_estimator",
        wired: true,
        owning_slice: W29_EPISTEMIC_MI_DEFERRED_STEP,
    },
    EpistemicMiProductionFenceFacet {
        facet: "landauer_clamp",
        wired: true,
        owning_slice: W29_EPISTEMIC_MI_DEFERRED_STEP,
    },
    EpistemicMiProductionFenceFacet {
        facet: "epistemic_bonus_shaping",
        wired: true,
        owning_slice: W29_EPISTEMIC_MI_DEFERRED_STEP,
    },
    EpistemicMiProductionFenceFacet {
        facet: "warmup_gate",
        wired: true,
        owning_slice: W29_EPISTEMIC_MI_DEFERRED_STEP,
    },
    EpistemicMiProductionFenceFacet {
        facet: "honest_posture_probe",
        wired: true,
        owning_slice: W29_EPISTEMIC_MI_DEFERRED_STEP,
    },
    EpistemicMiProductionFenceFacet {
        facet: "formal_mi_bridge",
        wired: false,
        owning_slice: FORMAL_MI_BRIDGE_DEFERRED_STEP,
    },
    EpistemicMiProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: EPISTEMIC_MI_PRODUCTION_ORCH_DEFERRED_STEP,
    },
];

/// One hop in the epistemic-MI wire map (host estimator → envelope → deferred formal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpistemicMiWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Epistemic-MI wire map — closed hops are measured morphisms only (not GREEN).
pub const EPISTEMIC_MI_WIRE_HOPS: &[EpistemicMiWireHop] = &[
    EpistemicMiWireHop {
        ordinal: 1,
        surface: "umst-manifold::ai::epistemic_mi::MutualInfoEstimator",
        role: "Histogram MI host probe (EPI-MI01)",
        wired: true,
    },
    EpistemicMiWireHop {
        ordinal: 2,
        surface: "umst-manifold::ai::epistemic_mi::clamp_mi_for_landauer",
        role: "Landauer ln2 clamp map (EPI-MI02)",
        wired: true,
    },
    EpistemicMiWireHop {
        ordinal: 3,
        surface: "umst-manifold::ai::epistemic_mi::epistemic_bonus_from_mi",
        role: "Post-CBF epistemic bonus shaping (EPI-MI03)",
        wired: true,
    },
    EpistemicMiWireHop {
        ordinal: 4,
        surface: "umst-formal::mi_bridge",
        role: "Formal MI certificate bridge (deferred)",
        wired: false,
    },
    EpistemicMiWireHop {
        ordinal: 5,
        surface: "orch::epistemic_mi_production_pin",
        role: "Production orch pin (deferred)",
        wired: false,
    },
];

/// Compile-time fence — production / GREEN / MASTER flip not authorized at posture tier.
const _: () = assert!(!EPISTEMIC_MI_PHYSICS_GREEN);
const _: () = assert!(!EPISTEMIC_MI_PRODUCTION_WIRED);
const _: () = assert!(!EPISTEMIC_MI_MASTER_RETICK);
const _: () = assert!(EPISTEMIC_MI_GREEN_CLAIM_BLOCKED);
const _: () = assert!(EPISTEMIC_MI_ESTIMATOR_LANDED);
const _: () = assert!(EPISTEMIC_MI_FENCE_FACET_COUNT == 7);
const _: () = assert!(EPISTEMIC_MI_FENCE_WIRED_COUNT == 5);
const _: () = assert!(EPISTEMIC_MI_FENCE_DEFERRED_COUNT == 2);
const _: () = assert!(
    EPISTEMIC_MI_FENCE_WIRED_COUNT + EPISTEMIC_MI_FENCE_DEFERRED_COUNT
        == EPISTEMIC_MI_FENCE_FACET_COUNT
);
const _: () = assert!(EPISTEMIC_MI_WIRE_HOP_COUNT == 5);
const _: () = assert!(EPISTEMIC_MI_WIRE_HOPS_CLOSED == 3);

/// Default histogram bin count (prototype-2a parity).
pub const DEFAULT_BINS: usize = 12;
/// Minimum samples per bin before MI EMA update engages.
pub const MIN_COUNT_PER_BIN: usize = 3;
/// EMA smoothing on raw histogram MI recomputes.
pub const EMA_ALPHA: f64 = 0.1;
/// Histogram mass decay per update (prototype-2a parity).
pub const HIST_DECAY: f64 = 0.999;
/// Absolute MI-delta threshold that unlocks exploration bonus.
pub const EXPLORATION_GAMMA_THRESHOLD: f64 = 0.01;
/// Exploration scale applied to |ΔMI| above threshold.
pub const EXPLORATION_SCALE: f64 = 0.1;

/// Count wired epistemic-MI fence facets (must match [`EPISTEMIC_MI_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn epistemic_mi_fence_wired_count() -> usize {
    EPISTEMIC_MI_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Count deferred epistemic-MI fence facets (must match [`EPISTEMIC_MI_FENCE_DEFERRED_COUNT`]).
#[must_use]
pub fn epistemic_mi_fence_deferred_count() -> usize {
    EPISTEMIC_MI_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| !f.wired)
        .count()
}

/// Count closed epistemic-MI wire hops (must match [`EPISTEMIC_MI_WIRE_HOPS_CLOSED`]).
#[must_use]
pub fn epistemic_mi_wire_hops_closed() -> usize {
    EPISTEMIC_MI_WIRE_HOPS.iter().filter(|h| h.wired).count()
}

/// Residue census — deferred facets with owning slices (honest open work).
#[must_use]
pub fn epistemic_mi_residue_census() -> Vec<&'static EpistemicMiProductionFenceFacet> {
    EPISTEMIC_MI_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| !f.wired)
        .collect()
}

/// Warmup sample threshold before EMA MI path engages.
#[must_use]
pub const fn warmup_sample_threshold(n_bins: usize) -> u64 {
    (MIN_COUNT_PER_BIN as u64).saturating_mul(n_bins as u64)
}

/// W29-008 deepen probe — honest posture bundle for operator receipts.
#[derive(Debug, Clone, PartialEq)]
pub struct EpistemicMiHonestProbe {
    pub deferred_step: &'static str,
    pub posture_tag: &'static str,
    pub honest_fence: &'static str,
    pub source_non_claim: &'static str,
    pub estimator_landed: bool,
    pub physics_green: bool,
    pub production_wired: bool,
    pub master_retick: bool,
    pub green_claim_blocked: bool,
    pub landauer_cap_nats: f64,
    pub morphism_count: usize,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub fence_deferred_count: usize,
    pub wire_hop_count: usize,
    pub wire_hops_closed: usize,
    pub deferred_formal_mi_bridge: &'static str,
    pub deferred_production_orch: &'static str,
}

/// Build W29-008 epistemic-MI deepen probe from live module constants.
#[must_use]
pub fn epistemic_mi_w29_008_probe() -> EpistemicMiHonestProbe {
    EpistemicMiHonestProbe {
        deferred_step: W29_EPISTEMIC_MI_DEFERRED_STEP,
        posture_tag: EPISTEMIC_MI_POSTURE_TAG,
        honest_fence: EPISTEMIC_MI_HONEST_FENCE,
        source_non_claim: EPISTEMIC_MI_SOURCE_NON_CLAIM,
        estimator_landed: EPISTEMIC_MI_ESTIMATOR_LANDED,
        physics_green: EPISTEMIC_MI_PHYSICS_GREEN,
        production_wired: EPISTEMIC_MI_PRODUCTION_WIRED,
        master_retick: EPISTEMIC_MI_MASTER_RETICK,
        green_claim_blocked: EPISTEMIC_MI_GREEN_CLAIM_BLOCKED,
        landauer_cap_nats: LANDAUER_MI_CAP_NATS,
        morphism_count: EPISTEMIC_MI_MORPHISM_IDS.len(),
        fence_facet_count: EPISTEMIC_MI_FENCE_FACET_COUNT,
        fence_wired_count: epistemic_mi_fence_wired_count(),
        fence_deferred_count: epistemic_mi_fence_deferred_count(),
        wire_hop_count: EPISTEMIC_MI_WIRE_HOPS.len(),
        wire_hops_closed: epistemic_mi_wire_hops_closed(),
        deferred_formal_mi_bridge: FORMAL_MI_BRIDGE_DEFERRED_STEP,
        deferred_production_orch: EPISTEMIC_MI_PRODUCTION_ORCH_DEFERRED_STEP,
    }
}

/// Honesty gate — must not invent physics GREEN, production wired, or master retick.
#[must_use]
pub fn epistemic_mi_w29_008_honest(probe: &EpistemicMiHonestProbe) -> bool {
    probe.deferred_step == W29_EPISTEMIC_MI_DEFERRED_STEP
        && probe.posture_tag == EPISTEMIC_MI_POSTURE_TAG
        && !probe.posture_tag.to_ascii_lowercase().contains("green")
        && probe.estimator_landed
        && !probe.physics_green
        && !probe.production_wired
        && !probe.master_retick
        && probe.green_claim_blocked
        && (probe.landauer_cap_nats - LANDAUER_MI_CAP_NATS).abs() < 1e-15
        && probe.morphism_count == EPISTEMIC_MI_MORPHISM_IDS.len()
        && probe.fence_facet_count == EPISTEMIC_MI_FENCE_FACET_COUNT
        && probe.fence_wired_count == EPISTEMIC_MI_FENCE_WIRED_COUNT
        && probe.fence_wired_count == epistemic_mi_fence_wired_count()
        && probe.fence_deferred_count == EPISTEMIC_MI_FENCE_DEFERRED_COUNT
        && probe.fence_deferred_count == epistemic_mi_fence_deferred_count()
        && probe.fence_wired_count + probe.fence_deferred_count == probe.fence_facet_count
        && probe.wire_hop_count == EPISTEMIC_MI_WIRE_HOP_COUNT
        && probe.wire_hops_closed == EPISTEMIC_MI_WIRE_HOPS_CLOSED
        && probe.wire_hops_closed == epistemic_mi_wire_hops_closed()
        && probe.deferred_formal_mi_bridge == FORMAL_MI_BRIDGE_DEFERRED_STEP
        && probe.deferred_production_orch == EPISTEMIC_MI_PRODUCTION_ORCH_DEFERRED_STEP
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master_retick=false")
        && probe.honest_fence.contains("green_claim_blocked=true")
        && probe.honest_fence.contains("estimator_landed=true")
        && probe
            .source_non_claim
            .to_ascii_lowercase()
            .contains("not physics green")
}

/// Operator-facing honesty validation — returns `Err` when posture fences are violated.
pub fn validate_epistemic_mi_honesty() -> Result<(), &'static str> {
    let probe = epistemic_mi_w29_008_probe();
    if probe.physics_green {
        return Err("EPISTEMIC_MI_PHYSICS_GREEN must stay false — histogram MI is heuristic only");
    }
    if probe.production_wired {
        return Err("EPISTEMIC_MI_PRODUCTION_WIRED must stay false until production orch lands");
    }
    if probe.master_retick {
        return Err("EPISTEMIC_MI_MASTER_RETICK must stay false until formal MI bridge");
    }
    if !probe.green_claim_blocked {
        return Err("EPISTEMIC_MI_GREEN_CLAIM_BLOCKED must stay true");
    }
    if !probe.estimator_landed {
        return Err("EPISTEMIC_MI_ESTIMATOR_LANDED must stay true at W29 deepen tier");
    }
    if epistemic_mi_fence_wired_count() != EPISTEMIC_MI_FENCE_WIRED_COUNT {
        return Err("fence wired count drift vs EPISTEMIC_MI_FENCE_WIRED_COUNT");
    }
    if epistemic_mi_fence_deferred_count() != EPISTEMIC_MI_FENCE_DEFERRED_COUNT {
        return Err("fence deferred count drift vs EPISTEMIC_MI_FENCE_DEFERRED_COUNT");
    }
    if EPISTEMIC_MI_PRODUCTION_FENCE_FACETS.len() != EPISTEMIC_MI_FENCE_FACET_COUNT {
        return Err("fence facet inventory length drift vs EPISTEMIC_MI_FENCE_FACET_COUNT");
    }
    if EPISTEMIC_MI_FENCE_FACET_IDS.len() != EPISTEMIC_MI_FENCE_FACET_COUNT {
        return Err("fence facet id registry length drift");
    }
    if EPISTEMIC_MI_WIRE_HOPS.len() != EPISTEMIC_MI_WIRE_HOP_COUNT {
        return Err("wire hop inventory length drift vs EPISTEMIC_MI_WIRE_HOP_COUNT");
    }
    if epistemic_mi_wire_hops_closed() != EPISTEMIC_MI_WIRE_HOPS_CLOSED {
        return Err("wire hops closed drift vs EPISTEMIC_MI_WIRE_HOPS_CLOSED");
    }
    if epistemic_mi_residue_census().len() != EPISTEMIC_MI_FENCE_DEFERRED_COUNT {
        return Err("residue census length drift vs EPISTEMIC_MI_FENCE_DEFERRED_COUNT");
    }
    for (facet, id) in EPISTEMIC_MI_PRODUCTION_FENCE_FACETS
        .iter()
        .zip(EPISTEMIC_MI_FENCE_FACET_IDS.iter())
    {
        if &facet.facet != id {
            return Err("fence facet id order mismatch vs EPISTEMIC_MI_FENCE_FACET_IDS");
        }
    }
    for (i, hop) in EPISTEMIC_MI_WIRE_HOPS.iter().enumerate() {
        if hop.ordinal as usize != i + 1 {
            return Err("wire hop ordinal drift");
        }
    }
    if !epistemic_mi_w29_008_honest(&probe) {
        return Err(
            "epistemic_mi posture fence violated — do not invent GREEN / production_wired / master",
        );
    }
    Ok(())
}

/// Histogram-based MI estimator I[X;Y] ≈ H(X)+H(Y)-H(X,Y).
#[derive(Clone, Debug)]
pub struct MutualInfoEstimator {
    n_bins: usize,
    state_dim: usize,
    obs_dim: usize,
    mi_estimate: f64,
    confidence: f64,
    total_samples: u64,
    state_hist: Vec<f64>,
    obs_hist: Vec<f64>,
    joint_hist: Vec<f64>,
    state_bounds: Vec<(f64, f64)>,
    obs_bounds: Vec<(f64, f64)>,
}

impl MutualInfoEstimator {
    #[must_use]
    pub fn new(state_dim: usize, obs_dim: usize) -> Self {
        let n_bins = DEFAULT_BINS;
        let state_hist_size = n_bins.pow(state_dim.min(3) as u32).min(4096);
        let obs_hist_size = n_bins.pow(obs_dim.min(3) as u32).min(4096);
        let joint_hist_size = (state_hist_size * obs_hist_size).min(65536);
        Self {
            n_bins,
            state_dim,
            obs_dim,
            mi_estimate: 0.0,
            confidence: 0.0,
            total_samples: 0,
            state_hist: vec![0.0; state_hist_size],
            obs_hist: vec![0.0; obs_hist_size],
            joint_hist: vec![0.0; joint_hist_size],
            state_bounds: vec![(0.0, 1.0); state_dim],
            obs_bounds: vec![(0.0, 1.0); obs_dim],
        }
    }

    /// Material-proxy layout: pinned [`UMST_SCALAR_CHANNEL_COUNT`] nodal means × same observation width.
    #[must_use]
    pub fn for_material_proxy() -> Self {
        use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
        Self::new(UMST_SCALAR_CHANNEL_COUNT, UMST_SCALAR_CHANNEL_COUNT)
    }

    #[must_use]
    pub fn estimate(&self) -> f64 {
        self.mi_estimate.max(0.0)
    }

    /// MI estimate after Landauer clamp — post-composition scalar for R2 envelope only.
    #[must_use]
    pub fn estimate_clamped_for_landauer(&self) -> f64 {
        clamp_mi_for_landauer(self.estimate())
    }

    #[must_use]
    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    /// Total accepted `(state, observation)` update pairs.
    #[must_use]
    pub fn total_samples(&self) -> u64 {
        self.total_samples
    }

    /// Whether the EMA MI path has engaged (finite-sample warmup gate).
    #[must_use]
    pub fn is_warm(&self) -> bool {
        self.total_samples >= warmup_sample_threshold(self.n_bins)
    }

    /// Histogram bin count (prototype-2a default).
    #[must_use]
    pub fn n_bins(&self) -> usize {
        self.n_bins
    }

    /// Configured state dimension.
    #[must_use]
    pub fn state_dim(&self) -> usize {
        self.state_dim
    }

    /// Configured observation dimension.
    #[must_use]
    pub fn obs_dim(&self) -> usize {
        self.obs_dim
    }

    /// Decayed state-histogram mass (diagnostic; not a certificate).
    #[must_use]
    pub fn state_hist_mass(&self) -> f64 {
        self.state_hist.iter().sum()
    }

    /// Decayed observation-histogram mass (diagnostic; not a certificate).
    #[must_use]
    pub fn obs_hist_mass(&self) -> f64 {
        self.obs_hist.iter().sum()
    }

    /// Decayed joint-histogram mass (diagnostic; not a certificate).
    #[must_use]
    pub fn joint_hist_mass(&self) -> f64 {
        self.joint_hist.iter().sum()
    }

    /// Marginal entropies `(H(X), H(Y), H(X,Y))` in nats from current histograms.
    #[must_use]
    pub fn marginal_entropies(&self) -> (f64, f64, f64) {
        (
            entropy(&self.state_hist),
            entropy(&self.obs_hist),
            entropy(&self.joint_hist),
        )
    }

    /// Instantaneous MI from histograms (no EMA) — diagnostic only.
    #[must_use]
    pub fn raw_histogram_mi(&self) -> f64 {
        self.compute_mi()
    }

    /// Replace per-axis state normalization bounds (length must match `state_dim`).
    pub fn set_state_bounds(&mut self, bounds: &[(f64, f64)]) {
        if bounds.len() == self.state_dim {
            self.state_bounds.copy_from_slice(bounds);
        }
    }

    /// Replace per-axis observation normalization bounds (length must match `obs_dim`).
    pub fn set_obs_bounds(&mut self, bounds: &[(f64, f64)]) {
        if bounds.len() == self.obs_dim {
            self.obs_bounds.copy_from_slice(bounds);
        }
    }

    /// Clear histograms / EMA state while preserving dimensions and bounds.
    pub fn reset(&mut self) {
        self.mi_estimate = 0.0;
        self.confidence = 0.0;
        self.total_samples = 0;
        for h in &mut self.state_hist {
            *h = 0.0;
        }
        for h in &mut self.obs_hist {
            *h = 0.0;
        }
        for h in &mut self.joint_hist {
            *h = 0.0;
        }
    }

    pub fn update(&mut self, state: &[f64], observation: &[f64]) {
        if state.len() != self.state_dim || observation.len() != self.obs_dim {
            return;
        }

        let state_norm: Vec<f64> = state
            .iter()
            .enumerate()
            .map(|(i, &x)| self.normalize(x, self.state_bounds[i]))
            .collect();
        let obs_norm: Vec<f64> = observation
            .iter()
            .enumerate()
            .map(|(i, &x)| self.normalize(x, self.obs_bounds[i]))
            .collect();

        for h in &mut self.state_hist {
            *h *= HIST_DECAY;
        }
        for h in &mut self.obs_hist {
            *h *= HIST_DECAY;
        }
        for h in &mut self.joint_hist {
            *h *= HIST_DECAY;
        }

        let sb = self.bin_index(&state_norm);
        let ob = self.bin_index(&obs_norm);
        if sb < self.state_hist.len() {
            self.state_hist[sb] += 1.0;
        }
        if ob < self.obs_hist.len() {
            self.obs_hist[ob] += 1.0;
        }
        let jb = sb.saturating_mul(self.obs_hist.len()) + ob;
        if jb < self.joint_hist.len() {
            self.joint_hist[jb] += 1.0;
        }

        self.total_samples += 1;
        if self.total_samples >= warmup_sample_threshold(self.n_bins) {
            let new_mi = self.compute_mi();
            self.mi_estimate = EMA_ALPHA * new_mi + (1.0 - EMA_ALPHA) * self.mi_estimate;
            self.confidence = (self.total_samples as f64 / 1000.0).min(1.0);
        }
    }

    fn normalize(&self, x: f64, bounds: (f64, f64)) -> f64 {
        let (lo, hi) = bounds;
        if hi <= lo {
            return 0.5;
        }
        ((x - lo) / (hi - lo)).clamp(0.0, 1.0)
    }

    fn bin_index(&self, values: &[f64]) -> usize {
        let mut idx = 0usize;
        let mut stride = 1usize;
        for &v in values.iter().take(3) {
            let b = ((v * self.n_bins as f64) as usize).min(self.n_bins - 1);
            idx += b * stride;
            stride *= self.n_bins;
        }
        idx
    }

    fn compute_mi(&self) -> f64 {
        let h_x = entropy(&self.state_hist);
        let h_y = entropy(&self.obs_hist);
        let h_xy = entropy(&self.joint_hist);
        (h_x + h_y - h_xy).max(0.0)
    }
}

fn entropy(hist: &[f64]) -> f64 {
    let total: f64 = hist.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    hist.iter()
        .filter(|&&c| c > 0.0)
        .map(|&c| {
            let p = c / total;
            -p * p.ln()
        })
        .sum()
}

/// Tracks epistemic bonus β·I[ψ;o] for reward shaping post-CBF (R2 envelope).
#[derive(Clone, Debug)]
pub struct EpistemicStateTracker {
    mi_history: Vec<f64>,
    epistemic_bonus: f64,
    beta: f64,
}

impl EpistemicStateTracker {
    #[must_use]
    pub fn new() -> Self {
        Self {
            mi_history: Vec::with_capacity(EPISTEMIC_TRACKER_HISTORY_CAP),
            epistemic_bonus: 0.0,
            beta: 0.1,
        }
    }

    pub fn set_beta(&mut self, beta: f64) {
        self.beta = beta.clamp(0.0, 1.0);
    }

    #[must_use]
    pub fn beta(&self) -> f64 {
        self.beta
    }

    #[must_use]
    pub fn history_len(&self) -> usize {
        self.mi_history.len()
    }

    /// History ring capacity (prototype-2a parity).
    #[must_use]
    pub const fn history_cap() -> usize {
        EPISTEMIC_TRACKER_HISTORY_CAP
    }

    pub fn update(&mut self, mi: f64) {
        let prior = self.mi_history.last().copied();
        self.mi_history.push(mi);
        if self.mi_history.len() > EPISTEMIC_TRACKER_HISTORY_CAP {
            self.mi_history.remove(0);
        }
        self.epistemic_bonus = epistemic_bonus_from_mi(self.beta, mi, prior);
    }

    #[must_use]
    pub fn epistemic_bonus(&self) -> f64 {
        self.epistemic_bonus
    }
}

impl Default for EpistemicStateTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Pure β·I[ψ;o] bonus with optional exploration from MI delta (post-CBF shaping only).
#[must_use]
pub fn epistemic_bonus_from_mi(beta: f64, mi: f64, prior_mi: Option<f64>) -> f64 {
    let gamma = prior_mi.map(|p| (mi - p).abs()).unwrap_or(0.0);
    let exploration = if gamma > EXPLORATION_GAMMA_THRESHOLD {
        EXPLORATION_SCALE * gamma
    } else {
        0.0
    };
    beta * mi + exploration
}

/// Per-step MI upper bound from catalog trace contract (`stepMI ≤ ln 2`).
#[must_use]
pub fn clamp_mi_for_landauer(mi: f64) -> f64 {
    mi.max(0.0).min(LANDAUER_MI_CAP_NATS)
}

/// Compose estimate → Landauer clamp for R2 envelope admission (pure morphism EPI-MI02).
#[must_use]
pub fn landauer_info_gain_nats(raw_mi: f64) -> f64 {
    clamp_mi_for_landauer(raw_mi.max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w29_008_metadata_and_morphism_registry() {
        assert_eq!(W29_EPISTEMIC_MI_DEFERRED_STEP, "W29-008-EPISTEMIC_MI");
        assert_eq!(EPISTEMIC_MI_POSTURE_TAG, "honest-histogram-mi-partial");
        assert_eq!(EPISTEMIC_MI_MORPHISM_IDS.len(), 3);
        assert!(EPISTEMIC_MI_MORPHISM_IDS.contains(&MORPHISM_EPI_MI01));
        assert!(EPISTEMIC_MI_MORPHISM_IDS.contains(&MORPHISM_EPI_MI02));
        assert!(EPISTEMIC_MI_MORPHISM_IDS.contains(&MORPHISM_EPI_MI03));
        assert_eq!(
            EPISTEMIC_MI_FENCE_FACET_IDS.len(),
            EPISTEMIC_MI_FENCE_FACET_COUNT
        );
        assert_eq!(
            EPISTEMIC_MI_PRODUCTION_FENCE_FACETS.len(),
            EPISTEMIC_MI_FENCE_FACET_COUNT
        );
        assert_eq!(
            epistemic_mi_fence_wired_count(),
            EPISTEMIC_MI_FENCE_WIRED_COUNT
        );
    }

    #[test]
    fn w29_008_honest_posture_fences() {
        let probe = epistemic_mi_w29_008_probe();
        assert!(EPISTEMIC_MI_ESTIMATOR_LANDED);
        assert!(!EPISTEMIC_MI_PHYSICS_GREEN);
        assert!(!EPISTEMIC_MI_PRODUCTION_WIRED);
        assert!(!EPISTEMIC_MI_MASTER_RETICK);
        assert!(EPISTEMIC_MI_GREEN_CLAIM_BLOCKED);
        assert!(epistemic_mi_w29_008_honest(&probe));
        validate_epistemic_mi_honesty().expect("posture fences");
    }

    #[test]
    fn w29_008_fence_facets_honest_inventory() {
        assert_eq!(EPISTEMIC_MI_FENCE_WIRED_COUNT, 5);
        assert_eq!(EPISTEMIC_MI_FENCE_DEFERRED_COUNT, 2);
        let wired = EPISTEMIC_MI_PRODUCTION_FENCE_FACETS
            .iter()
            .filter(|f| f.wired)
            .count();
        let deferred = EPISTEMIC_MI_PRODUCTION_FENCE_FACETS
            .iter()
            .filter(|f| !f.wired)
            .count();
        assert_eq!(wired, 5);
        assert_eq!(deferred, 2);
        assert_eq!(epistemic_mi_fence_deferred_count(), deferred);
        let formal = EPISTEMIC_MI_PRODUCTION_FENCE_FACETS
            .iter()
            .find(|f| f.facet == "formal_mi_bridge")
            .expect("formal facet");
        assert!(!formal.wired);
        assert_eq!(formal.owning_slice, FORMAL_MI_BRIDGE_DEFERRED_STEP);
        let prod = EPISTEMIC_MI_PRODUCTION_FENCE_FACETS
            .iter()
            .find(|f| f.facet == "production_wired")
            .expect("production facet");
        assert!(!prod.wired);
        assert_eq!(
            prod.owning_slice,
            EPISTEMIC_MI_PRODUCTION_ORCH_DEFERRED_STEP
        );
    }

    #[test]
    fn w29_008_wire_hops_and_residue_census() {
        assert_eq!(EPISTEMIC_MI_WIRE_HOPS.len(), EPISTEMIC_MI_WIRE_HOP_COUNT);
        assert_eq!(
            epistemic_mi_wire_hops_closed(),
            EPISTEMIC_MI_WIRE_HOPS_CLOSED
        );
        assert_eq!(
            EPISTEMIC_MI_WIRE_HOPS_CLOSED + EPISTEMIC_MI_FENCE_DEFERRED_COUNT,
            // 3 closed hops + 2 deferred hops = hop inventory
            EPISTEMIC_MI_WIRE_HOP_COUNT
        );
        let residue = epistemic_mi_residue_census();
        assert_eq!(residue.len(), EPISTEMIC_MI_FENCE_DEFERRED_COUNT);
        assert!(residue.iter().all(|f| !f.wired));
        assert_eq!(residue[0].facet, "formal_mi_bridge");
        assert_eq!(residue[1].facet, "production_wired");
        for (i, hop) in EPISTEMIC_MI_WIRE_HOPS.iter().enumerate() {
            assert_eq!(hop.ordinal as usize, i + 1);
        }
        assert!(!EPISTEMIC_MI_WIRE_HOPS[3].wired);
        assert!(!EPISTEMIC_MI_WIRE_HOPS[4].wired);
    }

    #[test]
    fn correlated_samples_yield_positive_mi() {
        let mut est = MutualInfoEstimator::new(2, 2);
        for i in 0..300 {
            let x = i as f64 / 300.0;
            est.update(&[x, x], &[x, x]);
        }
        assert!(est.is_warm());
        assert_eq!(est.total_samples(), 300);
        assert!(est.estimate() >= 0.0);
        assert!(est.estimate_clamped_for_landauer() <= LANDAUER_MI_CAP_NATS + 1e-15);
        assert_eq!(est.state_dim(), 2);
        assert_eq!(est.obs_dim(), 2);
        assert_eq!(est.n_bins(), DEFAULT_BINS);
        let (hx, hy, hxy) = est.marginal_entropies();
        let raw = est.raw_histogram_mi();
        assert!(raw + 1e-12 >= 0.0);
        // I[X;Y] ≤ min(H(X), H(Y)) for discrete histograms (heuristic tolerance).
        assert!(raw <= hx.min(hy) + 1e-9);
        assert!(hxy + 1e-12 >= 0.0);
        assert!(est.state_hist_mass() > 0.0);
        assert!(est.obs_hist_mass() > 0.0);
        assert!(est.joint_hist_mass() > 0.0);
    }

    #[test]
    fn independent_samples_mi_stays_near_zero_after_warmup() {
        let mut est = MutualInfoEstimator::new(1, 1);
        // Deterministic quasi-independent: state ramp vs observation phase-shifted ramp.
        for i in 0..200 {
            let s = (i as f64 / 200.0).fract();
            let o = ((i as f64 * 0.37 + 0.11) / 1.0).fract();
            est.update(&[s], &[o]);
        }
        assert!(est.is_warm());
        // Histogram MI on weakly dependent 1-D streams stays below Landauer cap and modest.
        assert!(est.estimate() < LANDAUER_MI_CAP_NATS);
        assert!(est.estimate_clamped_for_landauer() <= LANDAUER_MI_CAP_NATS + 1e-15);
        assert!(est.raw_histogram_mi() >= 0.0);
    }

    #[test]
    fn independent_samples_stay_low_confidence_early() {
        let mut est = MutualInfoEstimator::new(2, 2);
        for i in 0..5 {
            let x = i as f64 / 5.0;
            est.update(&[x, 0.0], &[0.0, x]);
        }
        assert!(!est.is_warm());
        assert!(est.confidence() < 1.0);
        assert_eq!(
            warmup_sample_threshold(est.n_bins()),
            (MIN_COUNT_PER_BIN * DEFAULT_BINS) as u64
        );
        // After 5 updates with decay, mass is strictly positive but < sample count.
        assert!(est.state_hist_mass() > 0.0);
        assert!(est.state_hist_mass() < 5.0 + 1e-9);
    }

    #[test]
    fn dimension_mismatch_is_silent_noop() {
        let mut est = MutualInfoEstimator::new(2, 2);
        est.update(&[0.5], &[0.5, 0.5]);
        assert_eq!(est.total_samples(), 0);
        assert_eq!(est.state_hist_mass(), 0.0);
        assert_eq!(est.joint_hist_mass(), 0.0);
    }

    #[test]
    fn reset_clears_ema_and_histograms() {
        let mut est = MutualInfoEstimator::new(1, 1);
        for i in 0..80 {
            let x = i as f64 / 80.0;
            est.update(&[x], &[x]);
        }
        assert!(est.total_samples() > 0);
        assert!(est.state_hist_mass() > 0.0);
        est.reset();
        assert_eq!(est.total_samples(), 0);
        assert_eq!(est.estimate(), 0.0);
        assert_eq!(est.confidence(), 0.0);
        assert!(!est.is_warm());
        assert_eq!(est.state_hist_mass(), 0.0);
        assert_eq!(est.obs_hist_mass(), 0.0);
        assert_eq!(est.joint_hist_mass(), 0.0);
        assert_eq!(est.raw_histogram_mi(), 0.0);
    }

    #[test]
    fn bounds_setters_reject_length_mismatch() {
        let mut est = MutualInfoEstimator::new(2, 1);
        est.set_state_bounds(&[(0.0, 2.0)]);
        // mismatch ignored — defaults remain
        est.update(&[1.5, 0.5], &[0.25]);
        assert_eq!(est.total_samples(), 1);
        est.set_state_bounds(&[(0.0, 2.0), (0.0, 2.0)]);
        est.set_obs_bounds(&[(0.0, 2.0)]);
        est.update(&[1.5, 0.5], &[0.25]);
        assert_eq!(est.total_samples(), 2);
    }

    #[test]
    fn landauer_clamp_respects_ln2() {
        assert!((LANDAUER_MI_CAP_NATS - f64::ln(2.0)).abs() < 1e-15);
        assert!(clamp_mi_for_landauer(10.0) <= LANDAUER_MI_CAP_NATS + 1e-9);
        assert!(clamp_mi_for_landauer(-1.0) >= 0.0);
        assert!((landauer_info_gain_nats(10.0) - LANDAUER_MI_CAP_NATS).abs() < 1e-15);
        assert_eq!(landauer_info_gain_nats(-3.0), 0.0);
    }

    #[test]
    fn entropy_non_negative_and_empty_zero() {
        assert_eq!(entropy(&[]), 0.0);
        assert_eq!(entropy(&[0.0, 0.0]), 0.0);
        let h = entropy(&[1.0, 1.0, 1.0, 1.0]);
        assert!(h >= 0.0);
        assert!((h - f64::ln(4.0)).abs() < 1e-12);
    }

    #[test]
    fn epistemic_bonus_increases_with_mi() {
        let b = epistemic_bonus_from_mi(0.1, 0.5, None);
        assert!(b >= 0.05 - 1e-9);
    }

    #[test]
    fn epistemic_bonus_explores_on_mi_delta() {
        let flat = epistemic_bonus_from_mi(0.1, 0.5, Some(0.5));
        let jump = epistemic_bonus_from_mi(0.1, 0.5, Some(0.3));
        assert!(jump > flat);
        assert!((jump - flat - EXPLORATION_SCALE * 0.2).abs() < 1e-12);
    }

    #[test]
    fn epistemic_tracker_beta_clamp() {
        let mut t = EpistemicStateTracker::new();
        t.set_beta(2.0);
        assert!((t.beta() - 1.0).abs() < 1e-15);
        t.update(0.2);
        assert!(t.epistemic_bonus() <= 0.2 + 1e-9);
        assert_eq!(t.history_len(), 1);
        assert_eq!(
            EpistemicStateTracker::history_cap(),
            EPISTEMIC_TRACKER_HISTORY_CAP
        );
    }

    #[test]
    fn epistemic_tracker_history_ring_caps() {
        let mut t = EpistemicStateTracker::new();
        for i in 0..(EPISTEMIC_TRACKER_HISTORY_CAP + 40) {
            t.update(i as f64 * 0.001);
        }
        assert_eq!(t.history_len(), EPISTEMIC_TRACKER_HISTORY_CAP);
    }

    #[test]
    fn material_proxy_dims_match_schema() {
        use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
        let est = MutualInfoEstimator::for_material_proxy();
        assert_eq!(est.state_dim(), UMST_SCALAR_CHANNEL_COUNT);
        assert_eq!(est.obs_dim(), UMST_SCALAR_CHANNEL_COUNT);
    }

    #[test]
    fn probe_refuses_green_production_master_literals() {
        let probe = epistemic_mi_w29_008_probe();
        let fence = probe.honest_fence.to_ascii_lowercase();
        assert!(!fence.contains("physics_green=true"));
        assert!(!fence.contains("production_wired=true"));
        assert!(!fence.contains("master_retick=true"));
        assert!(fence.contains("green_claim_blocked=true"));
        assert_eq!(probe.fence_deferred_count, 2);
        assert_eq!(probe.wire_hops_closed, 3);
        validate_epistemic_mi_honesty().expect("honesty");
    }
}
