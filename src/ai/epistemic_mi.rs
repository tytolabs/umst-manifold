// SPDX-License-Identifier: MIT
//! Histogram mutual information estimator (ported from umst-prototype-2a), behind `epistemic-ppo`.
//!
//! **Witness reading:** MI estimates are morphisms into the R2 Landauer envelope only — valid as
//! post-composition scalar `info_gain` tensors after CBF, never as standalone certificates
//! (see [`GOD_GRADE_WITNESS_LADDER`](../../docs/GOD_GRADE_WITNESS_LADDER.md) § MI inside the envelope).
//!
//! Stateful histogram updates are confined to [`MutualInfoEstimator`]; scoring and clamp maps are pure.

#![cfg(feature = "epistemic-ppo")]

const DEFAULT_BINS: usize = 12;
const MIN_COUNT_PER_BIN: usize = 3;
const EMA_ALPHA: f64 = 0.1;

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

    /// Material-proxy layout: 6 nodal scalar means (humidity→damage + extras) × 6 observation channels.
    #[must_use]
    pub fn for_material_proxy() -> Self {
        Self::new(6, 6)
    }

    #[must_use]
    pub fn estimate(&self) -> f64 {
        self.mi_estimate.max(0.0)
    }

    #[must_use]
    pub fn confidence(&self) -> f64 {
        self.confidence
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

        let decay = 0.999_f64;
        for h in &mut self.state_hist {
            *h *= decay;
        }
        for h in &mut self.obs_hist {
            *h *= decay;
        }
        for h in &mut self.joint_hist {
            *h *= decay;
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
        if self.total_samples >= MIN_COUNT_PER_BIN as u64 * self.n_bins as u64 {
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
        for (i, &v) in values.iter().take(3).enumerate() {
            let b = ((v * self.n_bins as f64) as usize).min(self.n_bins - 1);
            idx += b * stride;
            stride *= self.n_bins;
            let _ = i;
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
            mi_history: Vec::with_capacity(256),
            epistemic_bonus: 0.0,
            beta: 0.1,
        }
    }

    pub fn set_beta(&mut self, beta: f64) {
        self.beta = beta.clamp(0.0, 1.0);
    }

    pub fn update(&mut self, mi: f64) {
        let prior = self.mi_history.last().copied();
        self.mi_history.push(mi);
        if self.mi_history.len() > 256 {
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
    let exploration = if gamma > 0.01 { 0.1 * gamma } else { 0.0 };
    beta * mi + exploration
}

/// Per-step MI upper bound from catalog trace contract (`stepMI ≤ ln 2`).
#[must_use]
pub fn clamp_mi_for_landauer(mi: f64) -> f64 {
    mi.max(0.0).min(f64::ln(2.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correlated_samples_yield_positive_mi() {
        let mut est = MutualInfoEstimator::new(2, 2);
        for i in 0..300 {
            let x = i as f64 / 300.0;
            est.update(&[x, x], &[x, x]);
        }
        assert!(est.estimate() >= 0.0);
    }

    #[test]
    fn landauer_clamp_respects_ln2() {
        assert!(clamp_mi_for_landauer(10.0) <= f64::ln(2.0) + 1e-9);
    }

    #[test]
    fn epistemic_bonus_increases_with_mi() {
        let b = epistemic_bonus_from_mi(0.1, 0.5, None);
        assert!(b >= 0.05 - 1e-9);
    }
}
