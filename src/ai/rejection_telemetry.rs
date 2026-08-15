// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Pure rejection / acceptance accumulator for warm-layer gateway telemetry (Wave 10 T3).
//!
//! Commutative monoid: [`Default`] is identity; [`Self::merge`] folds parallel workers.
//!
//! # Honest boundary
//!
//! This module is an **in-memory commutative accumulator** — no I/O, no logging sink, no
//! fleet-wide GREEN claim. [`rejection_telemetry_production_wired`] stays **false** until a
//! measured export path (e.g. P4 witness JSON) is explicitly authorized at the integration
//! boundary. Rates here are **descriptive statistics** over recorded events, not certificates
//! of thermodynamic admissibility.
//!
//! Witness cross-refs: [`crate::ai::ppo::ManifoldGateway::telemetry`],
//! `artifacts/training/rejection_baseline.json`.

use serde::{Deserialize, Serialize};

/// Wave 10 T3 telemetry slice id.
pub const SLICE_ID: &str = "wave-10-t3";

/// P4 exit witness catalog cross-ref (cold-edge baseline JSON).
pub const P4_WITNESS_ARTIFACT: &str = "artifacts/training/rejection_baseline.json";

/// Honest posture — accumulator landed; production export path **open**.
pub const POSTURE_TAG: &str = "REJECTION_ACCUMULATOR_PARTIAL";

/// Whether the in-memory accumulator API is landed.
pub const ACCUMULATOR_LANDED: bool = true;

/// Whether a production telemetry export sink is wired.
pub const EXPORT_SINK_LANDED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const HONEST_FENCE: &str = "accumulator_landed=true production_wired=false";

/// Honest production telemetry export — **false** until measured live sink.
#[must_use]
pub const fn rejection_telemetry_production_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at posture tier.
const _: () = assert!(!rejection_telemetry_production_wired());

/// Typed probe for rejection telemetry posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RejectionTelemetryPostureProbe {
    pub slice_id: &'static str,
    pub posture_tag: &'static str,
    pub accumulator_landed: bool,
    pub export_sink_landed: bool,
    pub production_wired: bool,
    pub honest_fence: &'static str,
    pub p4_witness_artifact: &'static str,
}

/// Build introspection probe for done-when / fleet checks.
#[must_use]
pub const fn rejection_telemetry_posture_probe() -> RejectionTelemetryPostureProbe {
    RejectionTelemetryPostureProbe {
        slice_id: SLICE_ID,
        posture_tag: POSTURE_TAG,
        accumulator_landed: ACCUMULATOR_LANDED,
        export_sink_landed: EXPORT_SINK_LANDED,
        production_wired: rejection_telemetry_production_wired(),
        honest_fence: HONEST_FENCE,
        p4_witness_artifact: P4_WITNESS_ARTIFACT,
    }
}

/// Posture honesty gate — accumulator real; production export blocked.
#[must_use]
pub fn rejection_telemetry_posture_honest(probe: &RejectionTelemetryPostureProbe) -> bool {
    probe.slice_id == SLICE_ID
        && probe.posture_tag == POSTURE_TAG
        && probe.accumulator_landed
        && !probe.export_sink_landed
        && !probe.production_wired
        && probe.honest_fence.contains("accumulator_landed=true")
        && probe.honest_fence.contains("production_wired=false")
}

/// Validate posture honesty — fail closed on fake production claims.
pub fn validate_rejection_telemetry_posture_honesty() -> Result<(), &'static str> {
    let probe = rejection_telemetry_posture_probe();
    if probe.production_wired {
        return Err("rejection_telemetry_production_wired must stay false until export sink");
    }
    if !probe.accumulator_landed {
        return Err("accumulator_landed must stay true at wave-10-t3");
    }
    if !rejection_telemetry_posture_honest(&probe) {
        return Err("rejection_telemetry_posture_honest failed");
    }
    Ok(())
}

/// Immutable serializable witness of accumulator state (cold edge).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RejectionTelemetrySnapshot {
    pub total_proposals: u64,
    pub hard_rejects: u64,
    pub soft_penalties: u64,
    pub committed: u64,
    pub cumulative_slack: f64,
    pub rejection_rate: f64,
    pub soft_penalty_rate: f64,
    pub acceptance_rate: f64,
    pub mean_slack_at_commit: f64,
}

/// Immutable-value event log — no I/O, no logging, no hot-path coupling.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RejectionTelemetry {
    total_proposals: u64,
    hard_rejects: u64,
    soft_penalties: u64,
    committed: u64,
    cumulative_slack: f64,
}

impl RejectionTelemetry {
    pub fn record_reject(&mut self) {
        self.total_proposals = self.total_proposals.saturating_add(1);
        self.hard_rejects = self.hard_rejects.saturating_add(1);
    }

    pub fn record_soft_penalty(&mut self, cd_slack: f64) {
        self.total_proposals = self.total_proposals.saturating_add(1);
        self.soft_penalties = self.soft_penalties.saturating_add(1);
        if cd_slack.is_finite() {
            self.cumulative_slack += cd_slack;
        }
    }

    pub fn record_commit(&mut self, cd_slack: f64) {
        self.total_proposals = self.total_proposals.saturating_add(1);
        self.committed = self.committed.saturating_add(1);
        if cd_slack.is_finite() {
            self.cumulative_slack += cd_slack;
        }
    }

    #[must_use]
    pub fn total_proposals(&self) -> u64 {
        self.total_proposals
    }

    #[must_use]
    pub fn hard_rejects(&self) -> u64 {
        self.hard_rejects
    }

    #[must_use]
    pub fn soft_penalties(&self) -> u64 {
        self.soft_penalties
    }

    #[must_use]
    pub fn committed(&self) -> u64 {
        self.committed
    }

    #[must_use]
    pub fn cumulative_slack(&self) -> f64 {
        self.cumulative_slack
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total_proposals == 0
    }

    #[must_use]
    pub fn rejection_rate(&self) -> f64 {
        if self.total_proposals == 0 {
            return 0.0;
        }
        f64::from(self.hard_rejects as u32) / self.total_proposals as f64
    }

    #[must_use]
    pub fn soft_penalty_rate(&self) -> f64 {
        if self.total_proposals == 0 {
            return 0.0;
        }
        f64::from(self.soft_penalties as u32) / self.total_proposals as f64
    }

    #[must_use]
    pub fn acceptance_rate(&self) -> f64 {
        if self.total_proposals == 0 {
            return 0.0;
        }
        f64::from(self.committed as u32) / self.total_proposals as f64
    }

    /// Fraction of proposals that were neither hard-rejected nor committed (soft-penalized only).
    #[must_use]
    pub fn unresolved_rate(&self) -> f64 {
        if self.total_proposals == 0 {
            return 0.0;
        }
        let accounted = self.hard_rejects + self.soft_penalties + self.committed;
        if accounted >= self.total_proposals {
            return 0.0;
        }
        (self.total_proposals - accounted) as f64 / self.total_proposals as f64
    }

    #[must_use]
    pub fn mean_slack_at_commit(&self) -> f64 {
        if self.committed == 0 {
            return 0.0;
        }
        self.cumulative_slack / self.committed as f64
    }

    /// Cold-edge witness with precomputed rates for JSON export stubs.
    #[must_use]
    pub fn snapshot(&self) -> RejectionTelemetrySnapshot {
        RejectionTelemetrySnapshot {
            total_proposals: self.total_proposals,
            hard_rejects: self.hard_rejects,
            soft_penalties: self.soft_penalties,
            committed: self.committed,
            cumulative_slack: self.cumulative_slack,
            rejection_rate: self.rejection_rate(),
            soft_penalty_rate: self.soft_penalty_rate(),
            acceptance_rate: self.acceptance_rate(),
            mean_slack_at_commit: self.mean_slack_at_commit(),
        }
    }

    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        Self {
            total_proposals: self.total_proposals.saturating_add(other.total_proposals),
            hard_rejects: self.hard_rejects.saturating_add(other.hard_rejects),
            soft_penalties: self.soft_penalties.saturating_add(other.soft_penalties),
            committed: self.committed.saturating_add(other.committed),
            cumulative_slack: self.cumulative_slack + other.cumulative_slack,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejection_rate_matches_hundred_proposals() {
        let mut t = RejectionTelemetry::default();
        for _ in 0..30 {
            t.record_reject();
        }
        for _ in 0..20 {
            t.record_soft_penalty(0.0);
        }
        for i in 0..50_u64 {
            t.record_commit(i as f64 * 0.01);
        }
        assert_eq!(t.total_proposals(), 100);
        assert!((t.rejection_rate() - 0.30).abs() < 1e-9);
        assert!((t.soft_penalty_rate() - 0.20).abs() < 1e-9);
        assert!((t.acceptance_rate() - 0.50).abs() < 1e-9);
        let expected_mean = (0..50_u64).map(|i| i as f64 * 0.01).sum::<f64>() / 50.0;
        assert!((t.mean_slack_at_commit() - expected_mean).abs() < 1e-9);
    }

    #[test]
    fn merge_is_commutative_monoid() {
        let mut a = RejectionTelemetry::default();
        a.record_reject();
        a.record_commit(0.5);
        let mut b = RejectionTelemetry::default();
        b.record_soft_penalty(0.25);
        b.record_commit(1.0);
        let left = a.clone().merge(b.clone());
        let right = b.merge(a);
        assert_eq!(left, right);
    }

    #[test]
    fn snapshot_round_trips_rates() {
        let mut t = RejectionTelemetry::default();
        t.record_reject();
        t.record_soft_penalty(0.1);
        t.record_commit(0.5);
        let snap = t.snapshot();
        assert_eq!(snap.total_proposals, 3);
        assert!((snap.rejection_rate - t.rejection_rate()).abs() < 1e-12);
        assert!((snap.soft_penalty_rate - t.soft_penalty_rate()).abs() < 1e-12);
        assert!((snap.acceptance_rate - t.acceptance_rate()).abs() < 1e-12);
    }

    #[test]
    fn non_finite_slack_ignored() {
        let mut t = RejectionTelemetry::default();
        t.record_commit(f64::NAN);
        t.record_soft_penalty(f64::INFINITY);
        assert_eq!(t.cumulative_slack(), 0.0);
        assert_eq!(t.committed(), 1);
        assert_eq!(t.soft_penalties(), 1);
    }

    #[test]
    fn posture_metadata_locked() {
        assert_eq!(SLICE_ID, "wave-10-t3");
        assert_eq!(POSTURE_TAG, "REJECTION_ACCUMULATOR_PARTIAL");
        assert!(ACCUMULATOR_LANDED);
        assert!(!EXPORT_SINK_LANDED);
        assert!(!rejection_telemetry_production_wired());
        assert_eq!(
            HONEST_FENCE,
            "accumulator_landed=true production_wired=false"
        );
    }

    #[test]
    fn posture_probe_honest_not_production() {
        let probe = rejection_telemetry_posture_probe();
        assert!(rejection_telemetry_posture_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.export_sink_landed);
        validate_rejection_telemetry_posture_honesty().expect("posture must validate");
    }

    #[test]
    fn default_is_empty() {
        let t = RejectionTelemetry::default();
        assert!(t.is_empty());
        assert_eq!(t.rejection_rate(), 0.0);
        assert_eq!(t.unresolved_rate(), 0.0);
    }
}
