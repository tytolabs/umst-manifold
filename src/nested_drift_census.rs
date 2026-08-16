// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! FLEET-COMPOSER-G G15 — nested-repo drift honest census for `umst-manifold/`.
//!
//! Frozen inventory of uncommitted paths in the nested manifold product root. Does **not**
//! claim nested-repo clean, OP-5 PASS, or META-6 clearance. Receipt SSOT:
//! `outputs/.tmp/COMPOSER_G15_MANIFOLD_2143.md`.
//!
//! # Honest boundary (W29-044)
//!
//! [`NestedDriftCensusProbe`] and [`NESTED_DRIFT_SITES`] are **drift inventory SSOT** — frozen
//! path counts and honesty gates. Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`.
//! Live `git status` sync and nested-repo commit closure remain deferred slices.

/// W29 deepen cell — nested drift census honest fence bundle.
pub const W29_NESTED_DRIFT_DEEPEN_CELL: &str = "W29-044-NESTED_DRIFT_CENSUS";

/// Live nested-repo `git status` sync deferred beyond frozen G15 snapshot.
pub const NESTED_DRIFT_LIVE_GIT_SYNC_DEFERRED_STEP: &str = "nested-repo-commit-closure";

/// Nested-repo commit closure deferred beyond census inventory.
pub const NESTED_DRIFT_COMMIT_CLOSURE_DEFERRED_STEP: &str = "nested-repo-clean-push";

/// Honest physics posture — drift census is inventory only; does not certify continuum physics.
pub const NESTED_DRIFT_PHYSICS_GREEN: bool = false;

/// Production deployment wiring — not claimed by drift census module alone.
pub const NESTED_DRIFT_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by drift census module.
pub const NESTED_DRIFT_MASTER: bool = false;

/// Whether frozen G15 drift site inventory is landed.
pub const NESTED_DRIFT_INVENTORY_LANDED: bool = true;

/// Whether row-total reconcile helper is landed.
pub const NESTED_DRIFT_RECONCILE_LANDED: bool = true;

/// Whether G15 honesty gate is landed.
pub const NESTED_DRIFT_HONESTY_GATE_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const NESTED_DRIFT_HONEST_FENCE: &str =
    "inventory_landed=true reconcile_landed=true honesty_gate_landed=true production_wired=false master_composition_wired=false";

/// Nested drift fence facet count (honest census).
pub const NESTED_DRIFT_FENCE_FACET_COUNT: usize = 8;

/// Nested drift fence facets wired today (5/8 measured; live git sync deferred).
pub const NESTED_DRIFT_FENCE_WIRED_COUNT: usize = 5;

/// Stable facet ids for nested drift production fence census.
pub const NESTED_DRIFT_FENCE_FACET_IDS: &[&str] = &[
    "drift_sites_inventory",
    "row_total_reconcile",
    "g15_honesty_gate",
    "op5_meta6_fail_fence",
    "nested_clean_false_fence",
    "j11_cargo_gap_coupling",
    "live_git_status_sync",
    "production_wired",
];

/// FLEET-COMPOSER-G parent fleet id.
pub const FLEET_PARENT: &str = "FLEET-COMPOSER-G";

/// G15 agent job id.
pub const COMPOSER_G15_JOB_ID: &str = "FLEET-COMPOSER-G15-MANIFOLD";

/// G15 receipt path — SSOT for this census pass.
pub const COMPOSER_G15_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_G15_MANIFOLD_2143.md";

/// Prior manifold status receipt absorbed by this pass.
pub const ABSORBED_F15_RECEIPT: &str = "outputs/.tmp/COMPOSER_F15_MANIFOLD_1934.md";

/// Prior manifold drift surface receipt.
pub const ABSORBED_D72_RECEIPT: &str = "outputs/.tmp/COMPOSER_D72_MANIFOLD_STATUS_1741.md";

/// Honest adoption tier for this census module.
pub const POSTURE_TAG: &str = "honest-drift-inventory-only";

/// Nested `umst-manifold/` HEAD @ G15 census time.
pub const NESTED_HEAD_SHA: &str = "2718374";

/// Total dirty paths in nested repo @ G15.
pub const TOTAL_DIRTY_PATHS: usize = 105;

/// Modified (`M`) paths @ G15.
pub const MODIFIED_PATHS: usize = 50;

/// Untracked (`??`) paths @ G15.
pub const UNTRACKED_PATHS: usize = 55;

/// OP-5 checklist status — never PASS in this module.
pub const OP5_STATUS: &str = "FAIL";

/// META-6 stability-axis status — never PASS in this module.
pub const META_6_STATUS: &str = "FAIL";

/// Nested-repo clean status — never true in this module.
pub const NESTED_REPO_CLEAN: bool = false;

/// Drift conjunct theme — semantic lane / web constitutive / embodied orchestration.
pub const THEME_SEMANTIC_EMBODIED: &str = "semantic_lane_embodied_orch";

/// Drift conjunct theme — atoms tensor lift / nalgebra algebra bridges.
pub const THEME_ATOMS_TENSOR_LIFT: &str = "atoms_tensor_lift_bridge";

/// Drift conjunct theme — gate routing / semantic CBF / open system.
pub const THEME_GATE_ROUTING: &str = "gate_routing_semantic_cbf";

/// Drift conjunct theme — umst-math derivation / theorem registry.
pub const THEME_MATH_DERIVATION: &str = "umst_math_derivation_registry";

/// Drift conjunct theme — umst-layout-codegen (OP-5 production edge target).
pub const THEME_LAYOUT_CODEGEN: &str = "umst_layout_codegen_op5";

/// Drift conjunct theme — benchmark artifacts / scripts / root manifest.
pub const THEME_MANIFEST_ARTIFACTS: &str = "manifest_artifacts_scripts";

/// Reproducibility risk tier for a drift inventory row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftRiskTier {
    /// Compile receipts may diverge from clean nested HEAD checkout.
    Reproducibility,
    /// OP-5 production edge target — blocks frozen-core PASS.
    Op5Blocker,
    /// Witness / test harness drift — parity risk.
    WitnessParity,
    /// Documentation / manifest drift — low compile impact.
    DocManifest,
}

impl DriftRiskTier {
    /// Stable tag for receipts / CI introspection.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Reproducibility => "reproducibility",
            Self::Op5Blocker => "op5_blocker",
            Self::WitnessParity => "witness_parity",
            Self::DocManifest => "doc_manifest",
        }
    }
}

/// One row in the nested drift inventory table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NestedDriftSite {
    /// Area prefix under nested `umst-manifold/`.
    pub area: &'static str,
    /// Primary conjunct / theme label.
    pub theme: &'static str,
    /// Modified path count.
    pub modified: usize,
    /// Untracked path count.
    pub untracked: usize,
    /// Risk classification.
    pub risk: DriftRiskTier,
    /// Evidence receipt or wave anchor.
    pub evidence: &'static str,
}

impl NestedDriftSite {
    /// Total dirty paths for this row.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.modified + self.untracked
    }
}

/// Frozen nested drift inventory @ G15 — path counts from `git status --porcelain`.
pub const NESTED_DRIFT_SITES: &[NestedDriftSite] = &[
    NestedDriftSite {
        area: "src/runtime",
        theme: THEME_ATOMS_TENSOR_LIFT,
        modified: 4,
        untracked: 9,
        risk: DriftRiskTier::Reproducibility,
        evidence: "AGAP-2033",
    },
    NestedDriftSite {
        area: "src/embodied",
        theme: THEME_SEMANTIC_EMBODIED,
        modified: 2,
        untracked: 6,
        risk: DriftRiskTier::Reproducibility,
        evidence: "AGAP-2350",
    },
    NestedDriftSite {
        area: "src/gate",
        theme: THEME_GATE_ROUTING,
        modified: 4,
        untracked: 2,
        risk: DriftRiskTier::Reproducibility,
        evidence: "SWARM-C25-0831-93",
    },
    NestedDriftSite {
        area: "src/physics",
        theme: THEME_GATE_ROUTING,
        modified: 4,
        untracked: 0,
        risk: DriftRiskTier::Reproducibility,
        evidence: "thmc_residual_inventory",
    },
    NestedDriftSite {
        area: "src/core",
        theme: THEME_SEMANTIC_EMBODIED,
        modified: 3,
        untracked: 1,
        risk: DriftRiskTier::Reproducibility,
        evidence: "semantic_lane_schema_v1",
    },
    NestedDriftSite {
        area: "src/ai",
        theme: THEME_SEMANTIC_EMBODIED,
        modified: 1,
        untracked: 1,
        risk: DriftRiskTier::Reproducibility,
        evidence: "semantic_evolution_bridge",
    },
    NestedDriftSite {
        area: "src/root",
        theme: THEME_SEMANTIC_EMBODIED,
        modified: 1,
        untracked: 3,
        risk: DriftRiskTier::Reproducibility,
        evidence: "night_residual_deepen",
    },
    NestedDriftSite {
        area: "umst-math",
        theme: THEME_MATH_DERIVATION,
        modified: 8,
        untracked: 11,
        risk: DriftRiskTier::Reproducibility,
        evidence: "theorem_registry_derivation",
    },
    NestedDriftSite {
        area: "crates/umst-runtime",
        theme: THEME_ATOMS_TENSOR_LIFT,
        modified: 3,
        untracked: 9,
        risk: DriftRiskTier::Reproducibility,
        evidence: "A3_alias_posture",
    },
    NestedDriftSite {
        area: "umst-layout-codegen",
        theme: THEME_LAYOUT_CODEGEN,
        modified: 4,
        untracked: 1,
        risk: DriftRiskTier::Op5Blocker,
        evidence: "OP5_EXCEPTION_UMST_ALGEBRA",
    },
    NestedDriftSite {
        area: "tests",
        theme: THEME_SEMANTIC_EMBODIED,
        modified: 6,
        untracked: 10,
        risk: DriftRiskTier::WitnessParity,
        evidence: "embodied_orch_loop_transcript",
    },
    NestedDriftSite {
        area: "artifacts",
        theme: THEME_MANIFEST_ARTIFACTS,
        modified: 3,
        untracked: 0,
        risk: DriftRiskTier::DocManifest,
        evidence: "prabhu_baseline_note",
    },
    NestedDriftSite {
        area: "docs",
        theme: THEME_MANIFEST_ARTIFACTS,
        modified: 2,
        untracked: 1,
        risk: DriftRiskTier::DocManifest,
        evidence: "SEMANTIC_LANE_SCHEMA_V1",
    },
    NestedDriftSite {
        area: "scripts",
        theme: THEME_MANIFEST_ARTIFACTS,
        modified: 2,
        untracked: 1,
        risk: DriftRiskTier::DocManifest,
        evidence: "check_theorem_counts_ssot",
    },
    NestedDriftSite {
        area: "root-manifest",
        theme: THEME_MANIFEST_ARTIFACTS,
        modified: 3,
        untracked: 0,
        risk: DriftRiskTier::DocManifest,
        evidence: "Cargo.toml_umst.toml",
    },
];

/// One facet of the nested drift production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NestedDriftProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue.
    pub owning_slice: &'static str,
}

/// Nested drift production fence facet inventory (honest posture SSOT).
pub const NESTED_DRIFT_PRODUCTION_FENCE_FACETS: &[NestedDriftProductionFenceFacet] = &[
    NestedDriftProductionFenceFacet {
        facet: "drift_sites_inventory",
        wired: true,
        owning_slice: W29_NESTED_DRIFT_DEEPEN_CELL,
    },
    NestedDriftProductionFenceFacet {
        facet: "row_total_reconcile",
        wired: true,
        owning_slice: W29_NESTED_DRIFT_DEEPEN_CELL,
    },
    NestedDriftProductionFenceFacet {
        facet: "g15_honesty_gate",
        wired: true,
        owning_slice: W29_NESTED_DRIFT_DEEPEN_CELL,
    },
    NestedDriftProductionFenceFacet {
        facet: "op5_meta6_fail_fence",
        wired: true,
        owning_slice: W29_NESTED_DRIFT_DEEPEN_CELL,
    },
    NestedDriftProductionFenceFacet {
        facet: "nested_clean_false_fence",
        wired: true,
        owning_slice: W29_NESTED_DRIFT_DEEPEN_CELL,
    },
    NestedDriftProductionFenceFacet {
        facet: "j11_cargo_gap_coupling",
        wired: false,
        owning_slice: "cargo_test_gap_census",
    },
    NestedDriftProductionFenceFacet {
        facet: "live_git_status_sync",
        wired: false,
        owning_slice: NESTED_DRIFT_LIVE_GIT_SYNC_DEFERRED_STEP,
    },
    NestedDriftProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: NESTED_DRIFT_COMMIT_CLOSURE_DEFERRED_STEP,
    },
];

/// Count wired nested drift fence facets (must match [`NESTED_DRIFT_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn nested_drift_fence_wired_count() -> usize {
    NESTED_DRIFT_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Honest production wiring — **false** until nested-repo commit closure measured.
#[must_use]
pub const fn nested_drift_production_wired() -> bool {
    false
}

/// Master composition wiring — **false** until fleet orchestration loop closes.
#[must_use]
pub const fn nested_drift_master_composition_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at posture tier.
const _: () = assert!(!nested_drift_production_wired());

/// Measured honest-posture snapshot for nested drift census (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NestedDriftHonestPosture {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub inventory_landed: bool,
    pub reconcile_landed: bool,
    pub honesty_gate_landed: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub honest_fence: &'static str,
    pub deferred_live_git_sync: &'static str,
    pub deferred_commit_closure: &'static str,
}

/// Honest posture bundle for orchestrator / census probes — no invented GREEN.
#[must_use]
pub fn nested_drift_honest_posture_bundle() -> NestedDriftHonestPosture {
    NestedDriftHonestPosture {
        physics_green: NESTED_DRIFT_PHYSICS_GREEN,
        production_wired: NESTED_DRIFT_PRODUCTION_WIRED,
        master: NESTED_DRIFT_MASTER,
        inventory_landed: NESTED_DRIFT_INVENTORY_LANDED,
        reconcile_landed: NESTED_DRIFT_RECONCILE_LANDED,
        honesty_gate_landed: NESTED_DRIFT_HONESTY_GATE_LANDED,
        fence_facet_count: NESTED_DRIFT_FENCE_FACET_COUNT,
        fence_wired_count: nested_drift_fence_wired_count(),
        honest_fence: NESTED_DRIFT_HONEST_FENCE,
        deferred_live_git_sync: NESTED_DRIFT_LIVE_GIT_SYNC_DEFERRED_STEP,
        deferred_commit_closure: NESTED_DRIFT_COMMIT_CLOSURE_DEFERRED_STEP,
    }
}

/// Typed probe for nested drift posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NestedDriftPostureProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub inventory_landed: bool,
    pub reconcile_landed: bool,
    pub honesty_gate_landed: bool,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub physics_green: bool,
    pub op5_status: &'static str,
    pub meta_6_status: &'static str,
    pub nested_repo_clean: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub honest_fence: &'static str,
}

/// Build introspection probe for nested drift done-when / fleet checks.
#[must_use]
pub fn nested_drift_posture_probe() -> NestedDriftPostureProbe {
    NestedDriftPostureProbe {
        cell_id: W29_NESTED_DRIFT_DEEPEN_CELL,
        posture_tag: POSTURE_TAG,
        inventory_landed: NESTED_DRIFT_INVENTORY_LANDED,
        reconcile_landed: NESTED_DRIFT_RECONCILE_LANDED,
        honesty_gate_landed: NESTED_DRIFT_HONESTY_GATE_LANDED,
        production_wired: nested_drift_production_wired(),
        master_composition_wired: nested_drift_master_composition_wired(),
        physics_green: NESTED_DRIFT_PHYSICS_GREEN,
        op5_status: OP5_STATUS,
        meta_6_status: META_6_STATUS,
        nested_repo_clean: NESTED_REPO_CLEAN,
        fence_facet_count: NESTED_DRIFT_FENCE_FACET_COUNT,
        fence_wired_count: NESTED_DRIFT_FENCE_WIRED_COUNT,
        honest_fence: NESTED_DRIFT_HONEST_FENCE,
    }
}

/// Nested drift SSOT landed with production/master composition honestly open.
#[must_use]
pub fn nested_drift_posture_honest(probe: &NestedDriftPostureProbe) -> bool {
    probe.cell_id == W29_NESTED_DRIFT_DEEPEN_CELL
        && probe.posture_tag == POSTURE_TAG
        && probe.inventory_landed
        && probe.reconcile_landed
        && probe.honesty_gate_landed
        && !probe.physics_green
        && !probe.production_wired
        && !probe.master_composition_wired
        && !probe.nested_repo_clean
        && probe.op5_status == "FAIL"
        && probe.meta_6_status == "FAIL"
        && probe.fence_facet_count == NESTED_DRIFT_FENCE_FACET_COUNT
        && probe.fence_wired_count == NESTED_DRIFT_FENCE_WIRED_COUNT
        && probe.honest_fence.contains("inventory_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe
            .honest_fence
            .contains("master_composition_wired=false")
}

/// Validate nested drift posture honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_nested_drift_posture_honesty() -> Result<(), &'static str> {
    let probe = nested_drift_posture_probe();
    if probe.physics_green {
        return Err("NESTED_DRIFT_PHYSICS_GREEN must stay false — census is inventory only");
    }
    if probe.production_wired {
        return Err("nested_drift_production_wired must stay false until commit closure lands");
    }
    if probe.master_composition_wired {
        return Err(
            "nested_drift_master_composition_wired must stay false until fleet orch closes",
        );
    }
    if nested_drift_fence_wired_count() != NESTED_DRIFT_FENCE_WIRED_COUNT {
        return Err("nested_drift_fence_wired_count drift from NESTED_DRIFT_FENCE_WIRED_COUNT");
    }
    if !nested_drift_posture_honest(&probe) {
        return Err("nested_drift_posture_honest failed");
    }
    Ok(())
}

/// W29 deepen probe — absorbs G15 census + honest posture fence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct W29NestedDriftDeepenProbe {
    pub cell_id: &'static str,
    pub g15_job_id: &'static str,
    pub g15_receipt_path: &'static str,
    pub g15_drift_honest: bool,
    pub posture_honest: bool,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
}

/// Honest W29 nested drift deepen probe.
#[must_use]
pub fn w29_nested_drift_deepen_probe() -> W29NestedDriftDeepenProbe {
    let g15 = nested_drift_census_probe();
    let posture = nested_drift_posture_probe();
    W29NestedDriftDeepenProbe {
        cell_id: W29_NESTED_DRIFT_DEEPEN_CELL,
        g15_job_id: COMPOSER_G15_JOB_ID,
        g15_receipt_path: COMPOSER_G15_RECEIPT_PATH,
        g15_drift_honest: nested_drift_census_honest(&g15),
        posture_honest: nested_drift_posture_honest(&posture),
        production_wired: nested_drift_production_wired(),
        master_composition_wired: nested_drift_master_composition_wired(),
        fence_facet_count: NESTED_DRIFT_FENCE_FACET_COUNT,
        fence_wired_count: nested_drift_fence_wired_count(),
    }
}

/// Honesty gate for W29 deepen — must not invent clean nested repo or production wired.
#[must_use]
pub fn w29_nested_drift_deepen_honest(probe: &W29NestedDriftDeepenProbe) -> bool {
    probe.cell_id == W29_NESTED_DRIFT_DEEPEN_CELL
        && probe.g15_job_id == COMPOSER_G15_JOB_ID
        && probe
            .g15_receipt_path
            .contains("COMPOSER_G15_MANIFOLD_2143")
        && probe.g15_drift_honest
        && probe.posture_honest
        && !probe.production_wired
        && !probe.master_composition_wired
        && probe.fence_facet_count == NESTED_DRIFT_FENCE_FACET_COUNT
        && probe.fence_wired_count == NESTED_DRIFT_FENCE_WIRED_COUNT
}

/// G15 nested drift census probe — honest inventory, no clean claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedDriftCensusProbe {
    pub job_id: &'static str,
    pub receipt_path: &'static str,
    pub nested_head_sha: &'static str,
    pub total_dirty_paths: usize,
    pub modified_paths: usize,
    pub untracked_paths: usize,
    pub site_count: usize,
    pub nested_repo_clean: bool,
    pub op5_status: &'static str,
    pub meta_6_status: &'static str,
}

/// Honest G15 nested drift census probe.
#[must_use]
pub fn nested_drift_census_probe() -> NestedDriftCensusProbe {
    NestedDriftCensusProbe {
        job_id: COMPOSER_G15_JOB_ID,
        receipt_path: COMPOSER_G15_RECEIPT_PATH,
        nested_head_sha: NESTED_HEAD_SHA,
        total_dirty_paths: TOTAL_DIRTY_PATHS,
        modified_paths: MODIFIED_PATHS,
        untracked_paths: UNTRACKED_PATHS,
        site_count: NESTED_DRIFT_SITES.len(),
        nested_repo_clean: NESTED_REPO_CLEAN,
        op5_status: OP5_STATUS,
        meta_6_status: META_6_STATUS,
    }
}

/// Honesty gate — census must not invent clean nested repo or OP-5 PASS.
#[must_use]
pub fn nested_drift_census_honest(probe: &NestedDriftCensusProbe) -> bool {
    probe.job_id == COMPOSER_G15_JOB_ID
        && probe.receipt_path.contains("COMPOSER_G15_MANIFOLD_2143")
        && probe.nested_head_sha == NESTED_HEAD_SHA
        && probe.total_dirty_paths == TOTAL_DIRTY_PATHS
        && probe.modified_paths == MODIFIED_PATHS
        && probe.untracked_paths == UNTRACKED_PATHS
        && probe.site_count == NESTED_DRIFT_SITES.len()
        && !probe.nested_repo_clean
        && probe.op5_status == "FAIL"
        && probe.meta_6_status == "FAIL"
}

/// Sum of row totals — must reconcile with [`TOTAL_DIRTY_PATHS`].
#[must_use]
pub fn nested_drift_sites_total() -> usize {
    NESTED_DRIFT_SITES.iter().map(NestedDriftSite::total).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g15_nested_drift_metadata_pins() {
        assert_eq!(FLEET_PARENT, "FLEET-COMPOSER-G");
        assert_eq!(COMPOSER_G15_JOB_ID, "FLEET-COMPOSER-G15-MANIFOLD");
        assert!(ABSORBED_F15_RECEIPT.contains("F15_MANIFOLD"));
        assert!(ABSORBED_D72_RECEIPT.contains("D72_MANIFOLD"));
    }

    #[test]
    fn nested_drift_sites_reconcile_to_total() {
        let row_sum = nested_drift_sites_total();
        assert_eq!(row_sum, TOTAL_DIRTY_PATHS);
        assert_eq!(MODIFIED_PATHS + UNTRACKED_PATHS, TOTAL_DIRTY_PATHS);
    }

    #[test]
    fn nested_drift_census_honest_not_clean() {
        let probe = nested_drift_census_probe();
        assert!(nested_drift_census_honest(&probe));
        assert!(!probe.nested_repo_clean);
        assert_eq!(probe.op5_status, "FAIL");
        assert_eq!(probe.meta_6_status, "FAIL");
    }

    #[test]
    fn layout_codegen_row_is_op5_blocker() {
        let layout = NESTED_DRIFT_SITES
            .iter()
            .find(|s| s.area == "umst-layout-codegen")
            .expect("layout-codegen row");
        assert_eq!(layout.risk, DriftRiskTier::Op5Blocker);
        assert_eq!(layout.theme, THEME_LAYOUT_CODEGEN);
    }

    #[test]
    fn drift_risk_tier_tags_stable() {
        assert_eq!(DriftRiskTier::Op5Blocker.tag(), "op5_blocker");
        assert_eq!(DriftRiskTier::Reproducibility.tag(), "reproducibility");
    }

    #[test]
    fn w29_nested_drift_metadata_pins() {
        assert_eq!(W29_NESTED_DRIFT_DEEPEN_CELL, "W29-044-NESTED_DRIFT_CENSUS");
        assert_eq!(POSTURE_TAG, "honest-drift-inventory-only");
        assert_eq!(
            NESTED_DRIFT_FENCE_FACET_IDS.len(),
            NESTED_DRIFT_FENCE_FACET_COUNT
        );
    }

    #[test]
    fn w29_nested_drift_fence_wired_count_matches() {
        assert_eq!(
            nested_drift_fence_wired_count(),
            NESTED_DRIFT_FENCE_WIRED_COUNT
        );
        assert_eq!(
            NESTED_DRIFT_PRODUCTION_FENCE_FACETS.len(),
            NESTED_DRIFT_FENCE_FACET_COUNT
        );
    }

    #[test]
    fn w29_nested_drift_posture_honest_not_green() {
        let probe = nested_drift_posture_probe();
        assert!(nested_drift_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master_composition_wired);
        assert!(!probe.nested_repo_clean);
        assert_eq!(probe.op5_status, "FAIL");
        assert_eq!(probe.meta_6_status, "FAIL");
    }

    #[test]
    fn w29_nested_drift_validate_posture_honesty() {
        assert!(validate_nested_drift_posture_honesty().is_ok());
    }

    #[test]
    fn w29_nested_drift_deepen_absorbs_g15() {
        let probe = w29_nested_drift_deepen_probe();
        assert!(w29_nested_drift_deepen_honest(&probe));
        assert!(probe.g15_drift_honest);
        assert!(probe.posture_honest);
        assert!(!probe.production_wired);
        assert!(!probe.master_composition_wired);
    }

    #[test]
    fn w29_nested_drift_honest_posture_bundle() {
        let bundle = nested_drift_honest_posture_bundle();
        assert!(bundle.inventory_landed);
        assert!(bundle.reconcile_landed);
        assert!(bundle.honesty_gate_landed);
        assert!(!bundle.physics_green);
        assert!(!bundle.production_wired);
        assert!(!bundle.master);
        assert!(bundle.honest_fence.contains("production_wired=false"));
    }

    #[test]
    fn w29_production_wired_stays_false() {
        assert!(!nested_drift_production_wired());
        assert!(!nested_drift_master_composition_wired());
    }
}
