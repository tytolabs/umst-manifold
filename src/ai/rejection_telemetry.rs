// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Pure rejection / acceptance accumulator for warm-layer gateway telemetry (Wave 10 T3).
//!
//! Commutative monoid: [`Default`] is identity; [`Self::merge`] folds parallel workers.

use serde::{Deserialize, Serialize};

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
    pub fn rejection_rate(&self) -> f64 {
        if self.total_proposals == 0 {
            return 0.0;
        }
        f64::from(self.hard_rejects as u32) / self.total_proposals as f64
    }

    #[must_use]
    pub fn acceptance_rate(&self) -> f64 {
        if self.total_proposals == 0 {
            return 0.0;
        }
        f64::from(self.committed as u32) / self.total_proposals as f64
    }

    #[must_use]
    pub fn mean_slack_at_commit(&self) -> f64 {
        if self.committed == 0 {
            return 0.0;
        }
        self.cumulative_slack / self.committed as f64
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
        assert_eq!(t.total_proposals, 100);
        assert!((t.rejection_rate() - 0.30).abs() < 1e-9);
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
}
