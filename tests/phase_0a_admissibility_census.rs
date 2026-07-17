// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! # Phase 0a — Admissibility census + RED reconciliation scaffold
//!
//! **Card:** Phase 0a (gate consolidation foundation slice).  
//! **SSOT:** this integration test + `src/gate/admissibility_census.rs` registry.  
//! **Parity anchor:** `umst-concrete-cartridge/crates/umst-mcp/tests/fixtures/gate_parity_v0.json`  
//! SHA256 `149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3` (prefix `149081fa81a6525f…`).  
//! **Next:** Phase 0b — pure `gate<R>` Core = Mass + CD only (§17.3).
//!
//! ## Census — compute sites (predicate producers)
//!
//! | ID | Symbol / entry | Path | Conjuncts evaluated | Role |
//! |----|----------------|------|---------------------|------|
//! | C01 | `transition_outcome` | `src/gate/transition_proposal.rs` | mass, CD (`D_int≥−ε`), strength mono, reaction irreversible | **Primary SSOT cluster** |
//! | C02 | `thermodynamic_transition_admissible_tol` | `src/gate/transition_proposal.rs` | C01 + `new_strength ≤ new_max_strength` | C-ABI / FFI scalar predicate |
//! | C03 | `thermodynamic_transition_admissible` | `src/gate/transition_proposal.rs` | delegates → C02 | C-ABI convenience wrapper |
//! | C04 | `TransitionFilter::check_transition` | `src/gate/transition_proposal.rs` | C01 + telemetry counters | Stateful wrapper (no new math) |
//! | C05 | `ThermodynamicGate::check_transition_host` | `src/gate/evaluator.rs`, `src/gate/thermo_transition.rs` | C01 via host evaluator | `umst.gate.cd_transition` catalog surface |
//! | C06 | `thermo_gate_transition_outcome` | `src/gate/thermo_transition.rs` | C01 | Pure outcome helper |
//! | C07 | `CdTransitionCartridge::transition_evidence` | `src/runtime/gate/cartridge.rs` | C01 → `TransitionEvidence` | Cold witness cartridge |
//! | C08 | `explain_cd_transition_host` | `src/runtime/gate/evidence.rs` | C01 → `ConstraintExplanation` | Cold explain / margin wire |
//! | C09 | `clausius_duhem_violation` / `clausius_duhem_margin` | `src/ai/constraint_loss.rs` | **CD only** (`relu(−D_int)`) | Burn hot-path slack |
//! | C10 | `ThermodynamicCBF::{verify_and_deduct_update,verify_tensor_update}` | `src/ai/cbf.rs` | CD + **Landauer debit** (`P_input` open-system) | RL / topology CBF envelope |
//! | C11 | `http_manifest::evaluate` | `src/gate/http_manifest.rs` | **strength-excess only** (`predicted > bound`) | HTTP shim `umst.gate.http_shim` |
//! | C12 | `gate_sdf` / `clausius_duhem_admissible` | `umst-math/src/manifold/{sdf,csg}.rs` | SDF sign / 2-state CD | Formal identity layer |
//! | C13 | `KleisliUnitEvaluator` | `src/gate/kleisli.rs` | reflexive lift (η) | Prototype monad unit |
//! | C14 | `gate_check_mix` → `gate_recheck` → `thermodynamic_ok` → `predict_with_options` | `umst-concrete-cartridge/.../contribution.rs`, `pipeline/dual_gate.rs`, `facade/mod.rs` | regime envelope + C05 when `manifest-bridge` | **MCP concrete path** (composite) |
//! | C15 | `enforce_manifold_transition_gate` | `umst-concrete-cartridge/.../facade/mod.rs` | C05 on mix-calibrated lift | Cartridge → manifold bridge |
//! | C16 | `thermodynamic_transition_admissible` (FFI) | `egoff/umst-formal/ffi-bridge/src/lib.rs` | C03 | External C consumers |
//!
//! ## Census — consume sites (verdict readers / gates-of-gates)
//!
//! | ID | Symbol / entry | Path | Upstream compute | Role |
//! |----|----------------|------|------------------|------|
//! | K01 | `gate_check_mix_result` | `umst-concrete-cartridge/.../contribution.rs` | C14 | MCP wire + golden fixture bytes |
//! | K02 | `gate_recheck` | `umst-concrete-cartridge/.../contribution.rs` | C14 or cached `gate_summary` | Memory append guard |
//! | K03 | `umst_gate_check` (MCP tool) | `umst-concrete-cartridge/crates/umst-mcp/src/agent_layer.rs` | K01 | Agent session hard gate |
//! | K04 | `umst_gate_check` (C-ABI) | `umst-concrete-cartridge/crates/umst-gate-ffi/src/lib.rs` | K01 | FFI consumers |
//! | K05 | `soft_gate::{smoothstep,soft_lower_gate,…}` | `umst-concrete-cartridge/crates/umst-agent-mcp-core/src/soft_gate.rs` | **none (training templates)** | Differentiable slack only |
//! | K06 | `constraint_loss_penalty` / PPO hooks | `src/ai/ppo.rs`, `src/ai/constraint_loss.rs` | C09 | RL penalize tier |
//! | K07 | manifest orchestrator gate branch | `src/manifest/orchestrator.rs` | C05 | UMST manifest default gate |
//! | K08 | `UmstGateAckV1` | `src/ros/contract.rs` | bridge-defined | ROS DDS ack payload |
//! | K09 | `thermodynamic_admissible` default | `src/ros/epistemic_trace.rs` | telemetry default | Epistemic trace schema |
//! | K10 | `gate_server` / `gate_server_router` | `src/bin/gate_server.rs`, `src/gate_server_router.rs` | C11 | HTTP bulk gate |
//! | K11 | arena hot path | `umst-runtime-arena/` | **no admissibility predicate** | Perf tier only |
//! | K12 | `rank_admissible_proxies` | `umst-math/src/epistemic/selector.rs` | caller-supplied `admissible` bool | MI ranking filter |
//! | K13 | `dual_gate::thermodynamic_ok` | `umst-concrete-cartridge/.../pipeline/dual_gate.rs` | C14 | Track-A print ∧ thermo |
//! | K14 | `admissibility_margin` / cold wire | `src/runtime/gate/admissibility_margin.rs`, `cold_wire.rs` | C08 | Margin telemetry export |
//! | K15 | MCP parity harness | `umst-concrete-cartridge/crates/umst-mcp/tests/gate_parity.rs` | K01 + K03 | CI golden lock (0f precursor) |
//!
//! ## Reconciliation matrix (RED → GREEN targets for 0b–0f)
//!
//! | Pair | Status | Documented delta |
//! |------|--------|----------------|
//! | C01 ↔ C02 | **AGREE** when `new_strength ≤ new_max_strength` | C02 adds explicit strength cap |
//! | C01 ↔ C05/C06/C07/C08 | **AGREE** | Thin wrappers, no alternate math |
//! | C01 ↔ C09 | **PARTIAL** | C09 is CD-only; ignores mass / strength / reaction rejects |
//! | C01 ↔ C10 | **DELTA** | C10 adds Landauer erasure + credit budget (open-system `P_input`) |
//! | C01 ↔ C11 | **DELTA** | C11 is strength-excess shim; no mass/CD/reaction |
//! | C01 ↔ C14/K01 | **TBD (0c)** | MCP path = `predict_with_options` success + regime envelope, not raw C01 on mix JSON |
//! | K01 ↔ K03 (MCP) | **AGREE** | Locked under `gate_parity_v0.json` digest pin |
//! | K05 | **OUT OF SCOPE** | Soft templates; never hard witness |
//! | Manifold goldens ↔ MCP goldens | **TBD (0a→0f)** | Unify fixture family across repos |
//!
//! ## Four-invariant gate (Phase 0a scope)
//!
//! | Invariant | 0a evidence |
//! |-----------|-------------|
//! | Truth | Census + matrix committed; fixture digest pinned |
//! | Thermodynamics | Reconciliation names CD vs strength vs open-system splits |
//! | Trust-in-Process | RED scaffold tests; `#[ignore]` marks not-yet-green cross-repo rows |
//! | Honest-Maximal-Performance | N/A at census layer |

use std::path::PathBuf;

use umst_manifold::ai::cbf::ThermodynamicCBF;
use umst_manifold::ai::constraint_loss::clausius_duhem_margin;
use umst_manifold::gate::http_manifest::{
    evaluate, reaction_extent_from_age, GateManifest, MixProposal,
};
use umst_manifold::gate::transition_proposal::{
    transition_outcome, thermodynamic_transition_admissible_tol, ThermodynamicStateSnapshot,
    TRANSITION_TOLERANCE,
};
use umst_manifold::manifest::UmstManifest;

use umst_manifold::gate::admissibility_census::{
    format_open_deltas, ADMISSIBILITY_COMPUTE_SITES, ADMISSIBILITY_CONSUME_SITES,
    ConjunctFamily, GATE_PARITY_V0_FIXTURE_REL, GATE_PARITY_V0_SHA256 as CENSUS_GATE_SHA256,
    GATE_PARITY_V0_SHA256_PREFIX, OPEN_RECONCILIATION_DELTAS, SiteRole,
};
use umst_manifold::gate::{
    evaluate_http_mix_manifest, HttpGateManifest, HttpMixProposal,
};

/// Live parity fixture — repo wins over stale `a389b838…` shorthand in older docs.
/// Same pin as `gate_parity_v0.json` / census `GATE_PARITY_V0_SHA256`.
pub const GATE_PARITY_V0_SHA256: &str =
    "149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3";

fn gate_parity_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../umst-concrete-cartridge/crates/umst-mcp/tests/fixtures/gate_parity_v0.json")
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

// --- Parity pin (Truth invariant) ---

#[test]
fn gate_parity_v0_fixture_digest_pinned() {
    let path = gate_parity_fixture_path();
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        panic!(
            "missing live fixture at {} — Phase 0a requires gate_parity_v0.json: {e}",
            path.display()
        )
    });
    assert_eq!(
        sha256_hex(&bytes),
        GATE_PARITY_V0_SHA256,
        "gate_parity_v0.json digest drift — update pin only after intentional fixture change"
    );
}

#[test]
fn gate_parity_v0_fixture_schema_version() {
    let path = gate_parity_fixture_path();
    let text = std::fs::read_to_string(&path).expect("fixture readable");
    let v: serde_json::Value = serde_json::from_str(&text).expect("valid json");
    assert_eq!(
        v["schema_version"].as_str(),
        Some("gate_parity_v0"),
        "fixture schema_version must remain gate_parity_v0"
    );
    assert!(
        v["mix_table"].as_object().is_some(),
        "fixture must carry mix_table for MCP/manifold reconciliation family"
    );
}

// --- RED reconciliation scaffold (in-repo pairs) ---

#[test]
fn reconcile_c01_transition_outcome_agrees_c02_tol_when_cap_inactive() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.30, 293.15, 40.0);
    let mut new = old;
    new.reaction_extent = 0.35;
    new.free_energy = old.free_energy - 100.0;
    let dt = 1.0;
    let tol = TRANSITION_TOLERANCE;
    let max_strength = 80.0;
    let outcome = transition_outcome(&old, &new, dt, tol);
    let tol_adm = thermodynamic_transition_admissible_tol(
        old.density,
        old.free_energy,
        old.reaction_extent,
        old.strength,
        new.density,
        new.free_energy,
        new.reaction_extent,
        new.strength,
        max_strength,
        dt,
        tol,
    );
    assert_eq!(
        outcome.accepted, tol_adm,
        "C01 ↔ C02 must agree when strength cap is inactive"
    );
}

#[test]
fn reconcile_c01_c02_delta_strength_cap_can_disagree() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.30, 293.15, 40.0);
    let mut new = old;
    new.strength = 90.0;
    let dt = 1.0;
    let tol = TRANSITION_TOLERANCE;
    let outcome = transition_outcome(&old, &new, dt, tol);
    let tol_adm = thermodynamic_transition_admissible_tol(
        old.density,
        old.free_energy,
        old.reaction_extent,
        old.strength,
        new.density,
        new.free_energy,
        new.reaction_extent,
        new.strength,
        80.0,
        dt,
        tol,
    );
    assert!(
        outcome.accepted && !tol_adm,
        "documented DELTA: C02 rejects when new_strength > new_max_strength even if C01 accepts"
    );
}

#[test]
fn reconcile_c11_http_shim_routes_canonical_transition() {
    let manifest = GateManifest::from(&UmstManifest::default());
    let admit = MixProposal {
        constituent_primary_kg: 400.0,
        constituent_secondary_kg: 0.0,
        constituent_tertiary_kg: 0.0,
        water: 200.0,
        age_days: 28.0,
        predicted_strength_mpa: 25.0,
        temperature_c: 20.0,
    };
    let r = evaluate(&admit, &manifest);
    assert!(
        r.admissible,
        "HTTP shim admits when canonical transition + strength bound pass"
    );
    let mut reject = admit;
    reject.predicted_strength_mpa = 120.0;
    let r2 = evaluate(&reject, &manifest);
    assert!(!r2.is_admissible(), "HTTP shim rejects strength excess");
    assert!(
        r2.codes.iter().any(|c| c.contains("STRENGTH")),
        "HTTP shim still surfaces strength-excess codes"
    );
}

#[test]
fn reconcile_c09_constraint_loss_cd_only_partial_agrees_on_cd_margin() {
    use burn_ndarray::NdArray;
    use burn::tensor::Tensor;

    type B = NdArray<f32>;
    let device = Default::default();
    let old_rho = Tensor::<B, 1>::from_floats([2400.0], &device);
    let new_rho = Tensor::<B, 1>::from_floats([2400.0], &device);
    let old_psi = Tensor::<B, 1>::from_floats([-200_000.0], &device);
    let new_psi = Tensor::<B, 1>::from_floats([-210_000.0], &device);
    let dt = Tensor::<B, 1>::from_floats([1.0], &device);
    let margin = clausius_duhem_margin(old_rho, new_rho, old_psi, new_psi, dt);
    let margin_val = margin.into_data().value[0];

    let old = ThermodynamicStateSnapshot {
        density: 2400.0,
        temperature: 293.15,
        free_energy: -200_000.0,
        entropy: 0.1,
        reaction_extent: 0.3,
        strength: 10.0,
    };
    let new = ThermodynamicStateSnapshot {
        free_energy: -210_000.0,
        ..old
    };
    let host = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
    assert!(
        (margin_val - host.dissipation as f32).abs() < 1e-3,
        "C09 CD margin must track C01 dissipation on the CD leg"
    );
}

#[test]
fn reconcile_c10_thermodynamic_cbf_open_system_delta() {
    let mut cbf = ThermodynamicCBF::new(300.0, 1.0);
    let err = cbf
        .verify_and_deduct_update(0.0, 1e15)
        .expect_err("Landauer debit exceeds finite credit");
    let detail = err.to_string();
    assert!(
        detail.contains("Insufficient") || detail.contains("Credit") || detail.contains("Clausius"),
        "C10 DELTA: open-system debit beyond passive C01 — {detail}"
    );
}

/// Cross-repo: MCP `gate_check_mix_result` bytes vs manifold `transition_outcome` on fixture mixes.
/// Covered by `cargo test -p umst-mcp --test gate_parity gate_check_mix_result_parity_fixture` (Phase 0f lock).
#[test]
#[ignore = "optional cross-crate dev-dep audit; authoritative lock is gate_parity harness (0f)"]
fn reconcile_c14_mcp_gate_path_vs_c01_fixture_family() {
    let _ = gate_parity_fixture_path();
    panic!("intentionally RED — wire manifold snapshot lift for fixture mixes in 0c");
}

/// Cross-repo: `gate-ffi` / `soft_gate` consumers must not introduce second hard predicates.
#[test]
#[ignore = "optional consumer audit; K04 defers to gate_check_mix_result; K05 soft templates out of scope"]
fn reconcile_k04_k05_ffi_and_soft_gate_consumer_audit() {
    panic!("intentionally RED — document consumer audit in 0d routing table");
}

// --- Library census registry (SSOT in `src/gate/admissibility_census.rs`) ---

#[test]
fn phase0a_census_registers_anchor_compute_and_consume_sites() {
    assert!(
        ADMISSIBILITY_COMPUTE_SITES.len() >= 10,
        "expected ≥10 compute sites, got {}",
        ADMISSIBILITY_COMPUTE_SITES.len()
    );
    assert!(
        ADMISSIBILITY_CONSUME_SITES.len() >= 10,
        "expected ≥10 consume sites, got {}",
        ADMISSIBILITY_CONSUME_SITES.len()
    );

    let canonical = ADMISSIBILITY_COMPUTE_SITES
        .iter()
        .find(|s| s.symbol == "transition_outcome")
        .expect("transition_outcome must appear in compute census");
    assert_eq!(canonical.role, SiteRole::Compute);
    assert!(canonical.conjuncts.contains(&ConjunctFamily::ClausiusDuhem));

    let mcp = ADMISSIBILITY_CONSUME_SITES
        .iter()
        .find(|s| s.symbol == "gate_check_mix_result")
        .expect("gate_check_mix_result must appear in consume census");
    assert_eq!(mcp.role, SiteRole::Consume);
}

#[test]
fn phase0a_parity_fixture_path_and_digest_recorded() {
    assert_eq!(
        CENSUS_GATE_SHA256,
        GATE_PARITY_V0_SHA256,
        "test-local pin must match census module pin"
    );
    assert!(CENSUS_GATE_SHA256.starts_with(GATE_PARITY_V0_SHA256_PREFIX));
    assert!(GATE_PARITY_V0_FIXTURE_REL.contains("gate_parity_v0.json"));
}

/// Phase 0d: HTTP shim routes through canonical `transition_outcome` for the hydration lift.
#[test]
fn phase0a_http_shim_aligns_with_canonical_transition() {
    let proposal = HttpMixProposal {
        constituent_primary_kg: 400.0,
        constituent_secondary_kg: 100.0,
        constituent_tertiary_kg: 0.0,
        water: 200.0,
        age_days: 28.0,
        temperature_c: 20.0,
        predicted_strength_mpa: 5.0,
    };
    let http = evaluate_http_mix_manifest(&proposal, &HttpGateManifest::default());
    assert!(
        http.admissible,
        "HTTP shim should pass when canonical transition + strength bound pass"
    );

    let total = proposal.constituent_primary_kg
        + proposal.constituent_secondary_kg
        + proposal.constituent_tertiary_kg;
    let w_c = proposal.water / total;
    let supplementary_ratio =
        (proposal.constituent_secondary_kg + proposal.constituent_tertiary_kg) / total;
    let alpha = reaction_extent_from_age(
        proposal.age_days,
        proposal.temperature_c,
        supplementary_ratio,
    );
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(
        w_c,
        0.0,
        proposal.temperature_c + 273.15,
        80.0,
    );
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(
        w_c,
        alpha,
        proposal.temperature_c + 273.15,
        80.0,
    );
    let outcome = transition_outcome(&old, &new, proposal.age_days * 24.0 * 3600.0, TRANSITION_TOLERANCE);
    assert!(
        outcome.accepted,
        "canonical transition should accept hydration lift for HTTP admit fixture"
    );
}

/// RED: fails until Phase 0b–0f clears every entry in `OPEN_RECONCILIATION_DELTAS`.
#[test]
fn phase0a_reconciliation_matrix_red() {
    assert!(
        OPEN_RECONCILIATION_DELTAS.is_empty(),
        "Phase 0a reconciliation incomplete — {} open delta(s):\n{}\n         Fixture: {} · digest {}…\n         Clear deltas in phases 0b–0f to turn this test GREEN.",
        OPEN_RECONCILIATION_DELTAS.len(),
        format_open_deltas(),
        GATE_PARITY_V0_FIXTURE_REL,
        GATE_PARITY_V0_SHA256_PREFIX,
    );
}

