//! MI-weighted ranking of candidate epistemic proxies (TytoLabs prototype selector port, Phase K1).
//!
//! Structural analogue: utility-ranked provider keys (`ranked_zeroclaw_completion_keys` in cockpit) —
//! **not** imported here; this module is self-contained for `umst-math`.
//!
//! # Host-adaptive hooks (§0.3 I-B)
//! [`SelectorParams`] holds weights a future meta-loop may supply; defaults are neutral and
//! **Landauer-compatible** in the sense that scoring uses bounded epistemic bits (clamped) —
//! no extra physical assumptions beyond the formal MI bounds cited below.
//!
//! Proof: `InformationCostIdentity/residualCoherence_eq_one_minus_epistemic_bits` (RCC weights MI relevance);
//! `EpistemicMI/epistemicMIBits_le_one` (per-step epistemic MI clamp in \([0,1]\) for ranking scores);
//! `Gate/gate_check` (admissible candidates only).
//! DOI: 10.5281/zenodo.19159660

use ordered_float::NotNan;

/// Tunable weights for MI ranking — **pure data**; callers (Phase N3+) may construct per host.
///
/// Proof: policy parameters do not alter formal statements — they specialise runtime choice only
/// (`EpistemicMI/epistemicMIBits_nonneg`).
/// DOI: 10.5281/zenodo.19159660
#[derive(Clone, Debug, PartialEq)]
pub struct SelectorParams {
    /// Positive scale on clamped MI; default `1.0`.
    ///
    /// Proof: linear positive scaling preserves strict order on nonnegative scores.
    /// DOI: 10.5281/zenodo.19159660
    pub mi_scale: NotNan<f64>,
    /// Optional residual coherence capacity \( \in [0,1] \) — when `Some`, scores are multiplied.
    ///
    /// Proof: `InformationCostIdentity/residualCoherence_eq_one_minus_epistemic_bits` — RCC couples
    /// epistemic bits to measurable coherence.
    /// DOI: 10.5281/zenodo.19159660
    pub residual_coherence: Option<NotNan<f64>>,
}

impl Default for SelectorParams {
    fn default() -> Self {
        SelectorParams {
            mi_scale: NotNan::new(1.0).expect("1.0"),
            residual_coherence: None,
        }
    }
}

impl SelectorParams {
    /// RCC factor in \([0,1]\), or identity `1.0` when unset.
    ///
    /// Proof: product of nonnegative bounded factors stays bounded — compatible with gate semantics.
    /// DOI: 10.5281/zenodo.19159660
    #[must_use]
    pub fn rcc_factor(&self) -> NotNan<f64> {
        self.residual_coherence
            .map(|r| NotNan::new(r.into_inner().clamp(0.0, 1.0)).expect("clamp"))
            .unwrap_or_else(|| NotNan::new(1.0).expect("1.0"))
    }
}

/// One candidate epistemic proxy (e.g. trajectory integrator, density probe, estimator).
///
/// Proof: `Gate/gate_check` — only `admissible` rows participate in ranking.
/// DOI: 10.5281/zenodo.19159660
#[derive(Clone, Debug, PartialEq)]
pub struct EpistemicProxyCandidate<'a> {
    /// Stable identifier for deterministic tie-break (lexicographic).
    pub id: &'a str,
    /// Epistemic MI estimate (bits). Non-finite values are rejected.
    pub mi_bits: f64,
    /// Gate bit — mirrors thermodynamic / structural admissibility.
    pub admissible: bool,
}

/// Output row: candidate + computed ranking score (for telemetry / tests).
///
/// Proof: ordering is total on scores then `id` — stable sort key.
/// DOI: 10.5281/zenodo.19159660
#[derive(Clone, Debug, PartialEq)]
pub struct RankedProxy<'a> {
    /// Ranked candidate (admissible, finite MI).
    pub candidate: EpistemicProxyCandidate<'a>,
    /// Score = clamp\(_{[0,1]}\)(`mi_bits`) × `mi_scale` × RCC factor.
    pub score: NotNan<f64>,
}

#[inline]
fn clamp01_epistemic(mi: f64) -> Option<NotNan<f64>> {
    if !mi.is_finite() {
        return None;
    }
    let c = mi.clamp(0.0, 1.0);
    Some(NotNan::new(c).expect("finite clamp"))
}

/// Rank admissible proxies by descending MI-weighted score; tie-break by ascending `id`.
///
/// Empty input → empty output. All non-admissible or non-finite-MI rows are skipped.
///
/// Proof: `EpistemicMI/epistemicMIBits_le_one` motivates clamping MI contributions to \([0,1]\) for
/// comparability across estimators; `EpistemicMI/epistemicMIBits_nonneg` ensures nonnegative scores.
/// DOI: 10.5281/zenodo.19159660
#[must_use]
pub fn rank_epistemic_proxies_by_mi<'a>(
    candidates: &'a [EpistemicProxyCandidate<'a>],
    params: &SelectorParams,
) -> Vec<RankedProxy<'a>> {
    let rcc = params.rcc_factor();
    let scale = params.mi_scale;
    let mut rows: Vec<RankedProxy<'a>> = candidates
        .iter()
        .filter(|c| c.admissible)
        .filter_map(|c| {
            let mi = clamp01_epistemic(c.mi_bits)?;
            let score =
                NotNan::new((mi.into_inner() * scale.into_inner() * rcc.into_inner()).max(0.0))
                    .expect("finite score");
            Some(RankedProxy {
                candidate: c.clone(),
                score,
            })
        })
        .collect();
    rows.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.candidate.id.cmp(b.candidate.id))
    });
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p<'a>(id: &'a str, mi: f64, adm: bool) -> EpistemicProxyCandidate<'a> {
        EpistemicProxyCandidate {
            id,
            mi_bits: mi,
            admissible: adm,
        }
    }

    #[test]
    fn empty_input_yields_empty() {
        let c: &[EpistemicProxyCandidate] = &[];
        let r = rank_epistemic_proxies_by_mi(c, &SelectorParams::default());
        assert!(r.is_empty());
    }

    #[test]
    fn single_candidate_ranks() {
        let c = [p("a", 0.5, true)];
        let r = rank_epistemic_proxies_by_mi(&c, &SelectorParams::default());
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].candidate.id, "a");
    }

    #[test]
    fn higher_mi_ranks_first() {
        let c = [p("lo", 0.2, true), p("hi", 0.8, true)];
        let r = rank_epistemic_proxies_by_mi(&c, &SelectorParams::default());
        assert_eq!(r[0].candidate.id, "hi");
        assert_eq!(r[1].candidate.id, "lo");
    }

    #[test]
    fn tie_break_lexicographic_id() {
        let c = [p("b", 0.5, true), p("a", 0.5, true)];
        let r = rank_epistemic_proxies_by_mi(&c, &SelectorParams::default());
        assert_eq!(r[0].candidate.id, "a");
        assert_eq!(r[1].candidate.id, "b");
    }

    #[test]
    fn nan_and_inf_mi_skipped() {
        let c = [
            p("ok", 0.3, true),
            p("nan", f64::NAN, true),
            p("inf", f64::INFINITY, true),
        ];
        let r = rank_epistemic_proxies_by_mi(&c, &SelectorParams::default());
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].candidate.id, "ok");
    }

    #[test]
    fn inadmissible_skipped() {
        let c = [p("bad", 1.0, false), p("good", 0.1, true)];
        let r = rank_epistemic_proxies_by_mi(&c, &SelectorParams::default());
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].candidate.id, "good");
    }

    #[test]
    fn monotonic_in_mi() {
        let c1 = [p("x", 0.4, true)];
        let c2 = [p("x", 0.9, true)];
        let s1 = rank_epistemic_proxies_by_mi(&c1, &SelectorParams::default())[0].score;
        let s2 = rank_epistemic_proxies_by_mi(&c2, &SelectorParams::default())[0].score;
        assert!(s2 > s1);
    }

    #[test]
    fn mi_above_one_clamped_for_score() {
        let c = [p("z", 2.0, true)];
        let r = rank_epistemic_proxies_by_mi(&c, &SelectorParams::default());
        assert_eq!(r[0].score.into_inner(), 1.0);
    }

    #[test]
    fn residual_coherence_scales_score() {
        let c = [p("q", 1.0, true)];
        let params = SelectorParams {
            residual_coherence: Some(NotNan::new(0.5).expect("0.5")),
            ..SelectorParams::default()
        };
        let r = rank_epistemic_proxies_by_mi(&c, &params);
        assert!((r[0].score.into_inner() - 0.5).abs() < 1e-12);
    }
}
