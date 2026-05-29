// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! God-grade CI profile law: release manifest strict lane is on unless explicitly disabled.

use umst_manifold::manifest::{GroundingContract, UmstManifestBuilder};

#[test]
fn verify_stack_script_defaults_release_manifest_profile_to_strict_lane() {
    let script = include_str!("../scripts/verify_umst_stack.sh");
    assert!(
        script.contains("UMST_RELEASE_MANIFEST_PROFILE:-1}"),
        "verify_umst_stack.sh must default UMST_RELEASE_MANIFEST_PROFILE to 1 (strict witness lane)"
    );
}

/// CI guard: epistemic G.2 / G.3 must stay wired in `verify_umst_stack.sh` (god-grade rows 14–15).
#[test]
fn verify_stack_script_includes_epistemic_g2_g3_steps() {
    let script = include_str!("../scripts/verify_umst_stack.sh");
    for needle in [
        "epistemic trace schema G.2",
        "trace calibration G.3",
        "--test epistemic_trace_schema",
        "--test trace_calibration",
    ] {
        assert!(
            script.contains(needle),
            "verify_umst_stack.sh must include epistemic wiring: {needle}"
        );
    }
}

#[test]
fn default_and_release_profile_both_strict_when_profile_enabled() {
    let dev = UmstManifestBuilder::default().build();
    let release = UmstManifestBuilder::default().for_release_profile().build();
    assert!(UmstManifestBuilder::release_manifest_profile_enabled());
    assert_eq!(
        dev.grounding_contract,
        GroundingContract::StrictCatalogMatch,
        "UMST_RELEASE_MANIFEST_PROFILE=1 makes UmstManifestBuilder::default() strict in debug"
    );
    assert_eq!(
        release.grounding_contract,
        GroundingContract::StrictCatalogMatch
    );
}

#[test]
fn for_staging_restores_catalog_pinned_ros2() {
    let staging = UmstManifestBuilder::default().for_staging().build();
    assert_eq!(
        staging.grounding_contract,
        GroundingContract::CatalogPinnedRos2
    );
}
