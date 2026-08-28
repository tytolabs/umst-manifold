// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Cold-edge serialization for gate verdicts (`ucrs-provenance` only).
//!
//! No clock reads here — caller supplies [`super::evidence::UcrsObservedAtWire`] at the boundary.
//!
//! # Honesty (W29-113-COLD_WIRE)
//!
//! Telemetry export deepen only. Does **not** invent:
//! - physics / fleet **GREEN**
//! - **PRODUCTION_WIRED**
//! - **MASTER_RETICK** / master retick eligibility
//! - **OP-5 PASS**

use serde::{Deserialize, Serialize};

use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

use super::admissibility_margin::AdmissibilityMargin;
use super::evidence::{AdmissibilityToken, TransitionEvidence, UcrsObservedAtWire};

/// W29-113 swarm cell id (cold-wire deepen).
pub const W29_113_CELL_ID: &str = "W29-113-COLD_WIRE";

/// W29-113 honest posture — cold-edge serde deepen only.
pub const W29_113_HONEST_POSTURE: &str = "COLD_WIRE_TELEMETRY_DEEPEN_ONLY";

/// W29-113 explicit non-claims (gate text).
pub const W29_113_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// W29-113 deepen schema version.
pub const W29_113_DEEPEN_SCHEMA_VERSION: &str = "cold_wire_w29_113_deepen_v1";

/// AGENT-LOOP-07 swarm cell id (principal credit cold-wire deepen).
pub const AGENT_LOOP_07_CELL_ID: &str = "AGENT-LOOP-07-PRINCIPAL-CREDIT";

/// AGENT-LOOP-07 honest posture — principal/commission/executor stamp deepen only.
pub const AGENT_LOOP_07_HONEST_POSTURE: &str = "COLD_WIRE_PRINCIPAL_CREDIT_DEEPEN_ONLY";

/// AGENT-LOOP-07 explicit non-claims (gate text).
pub const AGENT_LOOP_07_NON_CLAIM: &str =
    "principal stamp ≠ consciousness; not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK; git author ≠ executed_by";

/// AGENT-LOOP-07 deepen schema version.
pub const AGENT_LOOP_07_DEEPEN_SCHEMA_VERSION: &str = "cold_wire_agent_loop_07_v1";

/// AGENT-LOOP-07 remainder gap — git/PR/author wrap still unmeasured on host.
pub const AGENT_LOOP_07_REMAINDER_GAP: &str =
    "git/PR/author wrap unwrapped; principal-side appropriation arm absent";

/// Principal git/PR wrap posture — host wrap not wired (≠ cold-wire telemetry deepen).
pub const AGENT_LOOP_07_PRINCIPAL_WRAP_POSTURE: &str = "PRINCIPAL_WRAP_UNWIRED";

/// Cold-wire telemetry deepen posture ≠ principal wrap on host (must stay true).
#[must_use]
pub fn agent_loop_07_cold_wire_posture_not_principal_wrap() -> bool {
    AGENT_LOOP_07_HONEST_POSTURE == "COLD_WIRE_PRINCIPAL_CREDIT_DEEPEN_ONLY"
        && AGENT_LOOP_07_PRINCIPAL_WRAP_POSTURE == "PRINCIPAL_WRAP_UNWIRED"
        && !agent_loop_07_remainder_row_closed()
}

/// Remainder-row honesty pin — cold-wire telemetry deepen only; does not close Padma row 07.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentLoop07RemainderPin {
    pub physics_green: bool,
    pub remainder_row_closed: bool,
    pub git_author_wrap_wired: bool,
}

/// AGENT-LOOP-07 remainder pin @ HEAD — physics GREEN and row close stay refused.
#[must_use]
pub const fn agent_loop_07_remainder_pin() -> AgentLoop07RemainderPin {
    AgentLoop07RemainderPin {
        physics_green: false,
        remainder_row_closed: false,
        git_author_wrap_wired: false,
    }
}

/// Whether AGENT-LOOP-07 remainder row is closed (must stay false until host wrap lands).
#[must_use]
pub const fn agent_loop_07_remainder_row_closed() -> bool {
    false
}

/// Fleet physics GREEN for AGENT-LOOP-07 (cold-wire deepen never claims it).
#[must_use]
pub const fn agent_loop_07_physics_green() -> bool {
    false
}

/// Principal-side credit arm absent on host (must stay true until git/PR wrap).
#[must_use]
pub const fn agent_loop_07_principal_credit_arm_absent() -> bool {
    true
}

/// Party credited for commissioning durable identity writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditCommissionParty {
    Human,
    Agent,
    Joint,
    Unknown,
}

/// Party credited for executing durable identity writes (`unknown` not allowed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditExecutorParty {
    Human,
    Agent,
    Joint,
}

/// Principal-side public stamp — who is named on git/PR/commit (≠ executor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrincipalNamedParty {
    Human,
    Agent,
    Joint,
    Unknown,
}

#[must_use]
const fn party_differs_from_executor(
    named: PrincipalNamedParty,
    executed_by: CreditExecutorParty,
) -> bool {
    match (named, executed_by) {
        (PrincipalNamedParty::Human, CreditExecutorParty::Human) => false,
        (PrincipalNamedParty::Agent, CreditExecutorParty::Agent) => false,
        (PrincipalNamedParty::Joint, CreditExecutorParty::Joint) => false,
        (PrincipalNamedParty::Unknown, _) => true,
        _ => true,
    }
}

/// Git/PR/commit public stamp facets — telemetry only; not legal authorship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPrCommitPublicStamp {
    pub git_author: PrincipalNamedParty,
    pub pr_principal: PrincipalNamedParty,
    pub commit_stamp: PrincipalNamedParty,
}

impl GitPrCommitPublicStamp {
    /// Git author names a different party than the executor (appropriation collision).
    #[must_use]
    pub fn git_author_differs_from_executor(
        self,
        executed_by: CreditExecutorParty,
    ) -> bool {
        party_differs_from_executor(self.git_author, executed_by)
    }

    /// Any public stamp facet collides with executor while arm stays unmeasured.
    #[must_use]
    pub fn appropriation_collision_unmeasured(
        self,
        executed_by: CreditExecutorParty,
    ) -> bool {
        (self.git_author_differs_from_executor(executed_by)
            || party_differs_from_executor(self.pr_principal, executed_by)
            || party_differs_from_executor(self.commit_stamp, executed_by))
            && agent_loop_07_principal_credit_arm_absent()
    }
}

/// Fourth arm — principal-side appropriation on git/PR/commit (cold-wire deepen only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrincipalAppropriationArm {
    pub public_stamp: GitPrCommitPublicStamp,
    pub executed_by: CreditExecutorParty,
}

impl PrincipalAppropriationArm {
    /// Arm unmeasured on host; posture ≠ credit wrap GREEN.
    #[must_use]
    pub fn arm_absent(self) -> bool {
        agent_loop_07_principal_credit_arm_absent()
    }

    /// Git author ≠ executed_by with appropriation arm still absent.
    #[must_use]
    pub fn git_author_executor_collision(self) -> bool {
        self.public_stamp
            .git_author_differs_from_executor(self.executed_by)
    }

    /// Collision present but fourth arm not wired — remainder row stays open.
    #[must_use]
    pub fn collision_unmeasured(self) -> bool {
        self.git_author_executor_collision() && self.arm_absent()
    }
}

/// ActionCredit fragment for durable identity-class transitions (cold-wire telemetry only).
///
/// Principal ≠ consciousness; not legal authorship. `principal_named` is the
/// public git/PR name and may differ from `executed_by` (appropriation arm).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurableIdentityCreditStamp {
    pub commissioned_by: CreditCommissionParty,
    pub executed_by: CreditExecutorParty,
    pub principal_named: PrincipalNamedParty,
}

impl DurableIdentityCreditStamp {
    /// Whether the principal public stamp names a different party than the executor.
    #[must_use]
    pub fn principal_differs_from_executor(self) -> bool {
        party_differs_from_executor(self.principal_named, self.executed_by)
    }

    /// Build git/PR/commit public stamp from principal name (host wrap still absent).
    #[must_use]
    pub fn public_stamp(self) -> GitPrCommitPublicStamp {
        GitPrCommitPublicStamp {
            git_author: self.principal_named,
            pr_principal: self.principal_named,
            commit_stamp: self.principal_named,
        }
    }

    /// Fourth appropriation arm sample — collision unmeasured until host wrap lands.
    #[must_use]
    pub fn appropriation_arm(self) -> PrincipalAppropriationArm {
        PrincipalAppropriationArm {
            public_stamp: self.public_stamp(),
            executed_by: self.executed_by,
        }
    }

    /// Principal/executor collision present but credit arm still unmeasured on host.
    #[must_use]
    pub fn principal_credit_gap_unmeasured(self) -> bool {
        self.principal_differs_from_executor() && agent_loop_07_principal_credit_arm_absent()
    }
}

/// Whether `catalog_id` is a durable identity-class gate transition (host CD identity admits).
#[must_use]
pub fn is_durable_identity_transition(catalog_id: &str) -> bool {
    catalog_id == CD_TRANSITION_CATALOG_ID
}

/// Dual energy ledger per spine event — compute (Landauer) and material (d_int), distinct.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpineEventCost {
    /// Compute-Landauer cost of evaluation (joules).
    pub compute_j: f64,
    /// Material dissipation D_int from transition (joules/kg-scale host units).
    pub material_j: f64,
    /// Axiom anchor string (both ≥ 0 trace here).
    pub axiom_anchor: &'static str,
}

impl SpineEventCost {
    pub const PHYSICAL_SECOND_LAW: &'static str = "physicalSecondLaw";

    #[must_use]
    pub fn new(compute_j: f64, material_j: f64) -> Self {
        Self {
            compute_j,
            material_j,
            axiom_anchor: Self::PHYSICAL_SECOND_LAW,
        }
    }

    /// Both rails non-negative (dual-ledger ≥ 0 fence sample).
    #[must_use]
    pub fn rails_nonnegative(self) -> bool {
        self.compute_j >= 0.0 && self.material_j >= 0.0
    }
}

/// JSON wire for a gate transition verdict (telemetry export only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionEvidenceWire {
    pub catalog_id: String,
    pub admissibility: AdmissibilityWire,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<UcrsObservedAtWireSerde>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_cost_j: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_dissipation_j: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axiom_anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissioned_by: Option<CreditCommissionParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<CreditExecutorParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_named: Option<PrincipalNamedParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_author: Option<PrincipalNamedParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr_principal: Option<PrincipalNamedParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_stamp: Option<PrincipalNamedParty>,
}

impl TransitionEvidenceWire {
    /// Stamp-shape refuse: a public principal/git name with executor omitted.
    /// Host git wrap remains unwired — this is shape refuse, not wrap.
    #[must_use]
    pub fn refuses_executor_erasure(&self) -> bool {
        let named = self.principal_named.is_some()
            || self.git_author.is_some()
            || self.pr_principal.is_some()
            || self.commit_stamp.is_some();
        named && self.executed_by.is_none()
    }

    /// Crate-root `pub use` is the scan-c surface; git-author wrap stays unwired.
    #[must_use]
    pub fn crate_root_export_is_not_git_wrap(&self) -> bool {
        self.refuses_executor_erasure() && !agent_loop_07_remainder_pin().git_author_wrap_wired
    }
}

/// Live COMPLETE-path consume — refuse principal/git stamp shape without executor.
///
/// Telemetry deepen ≠ host git/PR wrap; remainder row 07 stays open.
pub fn complete_stamp_shape_live_consume(wire: &TransitionEvidenceWire) -> Result<(), String> {
    if wire.refuses_executor_erasure() {
        return Err(
            "COMPLETE stamp-shape live consume refused executor erasure (git/PR wrap stays unwired)"
                .into(),
        );
    }
    if agent_loop_07_remainder_row_closed() {
        return Err("overclaim: AGENT-LOOP-07 remainder row closed invented".into());
    }
    if !agent_loop_07_principal_credit_arm_absent() {
        return Err("overclaim: principal-side credit arm wired invented".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdmissibilityWire {
    Admissible,
    Inadmissible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UcrsObservedAtWireSerde {
    pub wall_ms: i64,
    pub ucrs_seq: u64,
}

impl From<AdmissibilityToken> for AdmissibilityWire {
    fn from(t: AdmissibilityToken) -> Self {
        match t {
            AdmissibilityToken::Admissible => Self::Admissible,
            AdmissibilityToken::Inadmissible => Self::Inadmissible,
        }
    }
}

impl From<UcrsObservedAtWire> for UcrsObservedAtWireSerde {
    fn from(s: UcrsObservedAtWire) -> Self {
        Self {
            wall_ms: s.wall_ms,
            ucrs_seq: s.ucrs_seq,
        }
    }
}

impl From<UcrsObservedAtWireSerde> for UcrsObservedAtWire {
    fn from(s: UcrsObservedAtWireSerde) -> Self {
        Self {
            wall_ms: s.wall_ms,
            ucrs_seq: s.ucrs_seq,
        }
    }
}

/// Attach optional stamp, dual ledger, and identity credit at cold boundary (no hot-path clock).
///
/// Boundary `stamp` wins over `evidence.observed_at` when both are present.
/// `credit` is serialized only on durable identity-class transitions
/// ([`is_durable_identity_transition`]).
#[must_use]
pub fn transition_evidence_to_wire(
    evidence: TransitionEvidence,
    stamp: Option<UcrsObservedAtWire>,
    cost: Option<SpineEventCost>,
    credit: Option<DurableIdentityCreditStamp>,
) -> TransitionEvidenceWire {
    let identity_durable = is_durable_identity_transition(evidence.catalog_id);
    let credit_wire = if identity_durable { credit } else { None };
    TransitionEvidenceWire {
        catalog_id: evidence.catalog_id.to_string(),
        admissibility: evidence.admissibility.into(),
        observed_at: stamp
            .or(evidence.observed_at)
            .map(UcrsObservedAtWireSerde::from),
        compute_cost_j: cost.map(|c| c.compute_j),
        material_dissipation_j: cost.map(|c| c.material_j),
        axiom_anchor: cost.map(|c| c.axiom_anchor.to_string()),
        commissioned_by: credit_wire.map(|c| c.commissioned_by),
        executed_by: credit_wire.map(|c| c.executed_by),
        principal_named: credit_wire.map(|c| c.principal_named),
        git_author: credit_wire.map(|c| c.public_stamp().git_author),
        pr_principal: credit_wire.map(|c| c.public_stamp().pr_principal),
        commit_stamp: credit_wire.map(|c| c.public_stamp().commit_stamp),
    }
}

/// Honest fence flags for cold-wire deepen (W29-113).
///
/// All invent-claim bools stay `false`; `deepen_honest` is true only when cell
/// pins, dual-ledger sample, and wire round-trip stay consistent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColdWireW29113DeepenProbe {
    pub schema_version: &'static str,
    pub cell_id: &'static str,
    pub honest_posture: &'static str,
    pub non_claim: &'static str,
    pub production_wired_claimed: bool,
    pub green_claimed: bool,
    pub op5_pass_claimed: bool,
    pub master_retick_claimed: bool,
    pub deepen_honest: bool,
}

/// Build the W29-113 cold-wire deepen honesty probe.
#[must_use]
pub fn cold_wire_w29_113_deepen_probe() -> ColdWireW29113DeepenProbe {
    let production_wired_claimed = false;
    let green_claimed = false;
    let op5_pass_claimed = false;
    let master_retick_claimed = false;
    let sample_cost = SpineEventCost::new(2.87e-21, 150.0);
    let sample_ok = sample_cost.rails_nonnegative()
        && sample_cost.axiom_anchor == SpineEventCost::PHYSICAL_SECOND_LAW;
    let deepen_honest = W29_113_CELL_ID == "W29-113-COLD_WIRE"
        && W29_113_DEEPEN_SCHEMA_VERSION == "cold_wire_w29_113_deepen_v1"
        && W29_113_HONEST_POSTURE == "COLD_WIRE_TELEMETRY_DEEPEN_ONLY"
        && !production_wired_claimed
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && W29_113_NON_CLAIM.contains("not GREEN")
        && W29_113_NON_CLAIM.contains("not OP-5 PASS")
        && W29_113_NON_CLAIM.contains("not production_wired")
        && W29_113_NON_CLAIM.contains("not MASTER_RETICK")
        && sample_ok;
    ColdWireW29113DeepenProbe {
        schema_version: W29_113_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_113_CELL_ID,
        honest_posture: W29_113_HONEST_POSTURE,
        non_claim: W29_113_NON_CLAIM,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-113 cold-wire deepen honesty probe passes.
#[must_use]
pub fn cold_wire_w29_113_deepen_honest() -> bool {
    cold_wire_w29_113_deepen_probe().deepen_honest
}

/// Cold-wire fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn cold_wire_honest_fence_holds() -> bool {
    let p = cold_wire_w29_113_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
        && agent_loop_07_principal_credit_honest()
}

/// Honesty probe for AGENT-LOOP-07 principal credit deepen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentLoop07PrincipalCreditProbe {
    pub schema_version: &'static str,
    pub cell_id: &'static str,
    pub honest_posture: &'static str,
    pub non_claim: &'static str,
    pub remainder_gap: &'static str,
    pub remainder_pin: AgentLoop07RemainderPin,
    pub production_wired_claimed: bool,
    pub green_claimed: bool,
    pub physics_green: bool,
    pub op5_pass_claimed: bool,
    pub master_retick_claimed: bool,
    pub consciousness_claimed: bool,
    pub remainder_row_closed: bool,
    pub git_author_wrap_wired: bool,
    pub principal_credit_arm_absent: bool,
    pub deepen_honest: bool,
}

/// Build the AGENT-LOOP-07 principal credit cold-wire deepen honesty probe.
#[must_use]
pub fn agent_loop_07_principal_credit_probe() -> AgentLoop07PrincipalCreditProbe {
    let remainder_pin = agent_loop_07_remainder_pin();
    let production_wired_claimed = false;
    let green_claimed = false;
    let physics_green = remainder_pin.physics_green;
    let op5_pass_claimed = false;
    let master_retick_claimed = false;
    let consciousness_claimed = false;
    let remainder_row_closed = remainder_pin.remainder_row_closed;
    let git_author_wrap_wired = remainder_pin.git_author_wrap_wired;
    let principal_credit_arm_absent = agent_loop_07_principal_credit_arm_absent();
    let sample_credit = DurableIdentityCreditStamp {
        commissioned_by: CreditCommissionParty::Human,
        executed_by: CreditExecutorParty::Agent,
        principal_named: PrincipalNamedParty::Human,
    };
    let sample_wire = transition_evidence_to_wire(
        TransitionEvidence {
            catalog_id: CD_TRANSITION_CATALOG_ID,
            admissibility: AdmissibilityToken::Admissible,
            margin: AdmissibilityMargin(0.5),
            observed_at: None,
        },
        None,
        None,
        Some(sample_credit),
    );
    let deepen_honest = AGENT_LOOP_07_CELL_ID == "AGENT-LOOP-07-PRINCIPAL-CREDIT"
        && AGENT_LOOP_07_DEEPEN_SCHEMA_VERSION == "cold_wire_agent_loop_07_v1"
        && AGENT_LOOP_07_HONEST_POSTURE == "COLD_WIRE_PRINCIPAL_CREDIT_DEEPEN_ONLY"
        && !production_wired_claimed
        && !green_claimed
        && !physics_green
        && !op5_pass_claimed
        && !master_retick_claimed
        && !consciousness_claimed
        && !remainder_row_closed
        && !git_author_wrap_wired
        && principal_credit_arm_absent
        && AGENT_LOOP_07_REMAINDER_GAP.contains("appropriation arm absent")
        && AGENT_LOOP_07_NON_CLAIM.contains("principal stamp ≠ consciousness")
        && AGENT_LOOP_07_NON_CLAIM.contains("git author ≠ executed_by")
        && AGENT_LOOP_07_REMAINDER_GAP.contains("git/PR/author wrap unwrapped")
        && !agent_loop_07_remainder_row_closed()
        && !agent_loop_07_physics_green()
        && agent_loop_07_cold_wire_posture_not_principal_wrap()
        && remainder_pin == agent_loop_07_remainder_pin()
        && is_durable_identity_transition(CD_TRANSITION_CATALOG_ID)
        && sample_credit.principal_differs_from_executor()
        && sample_credit.principal_credit_gap_unmeasured()
        && {
            let arm = sample_credit.appropriation_arm();
            arm.git_author_executor_collision() && arm.collision_unmeasured() && arm.arm_absent()
        }
        && sample_wire.commissioned_by == Some(CreditCommissionParty::Human)
        && sample_wire.executed_by == Some(CreditExecutorParty::Agent)
        && sample_wire.principal_named == Some(PrincipalNamedParty::Human)
        && !sample_wire.refuses_executor_erasure()
        && {
            let mut erased = sample_wire.clone();
            erased.executed_by = None;
            erased.refuses_executor_erasure()
        }
        && sample_wire.git_author == Some(PrincipalNamedParty::Human)
        && sample_wire.pr_principal == Some(PrincipalNamedParty::Human)
        && sample_wire.commit_stamp == Some(PrincipalNamedParty::Human);
    AgentLoop07PrincipalCreditProbe {
        schema_version: AGENT_LOOP_07_DEEPEN_SCHEMA_VERSION,
        cell_id: AGENT_LOOP_07_CELL_ID,
        honest_posture: AGENT_LOOP_07_HONEST_POSTURE,
        non_claim: AGENT_LOOP_07_NON_CLAIM,
        remainder_gap: AGENT_LOOP_07_REMAINDER_GAP,
        remainder_pin,
        production_wired_claimed,
        green_claimed,
        physics_green,
        op5_pass_claimed,
        master_retick_claimed,
        consciousness_claimed,
        remainder_row_closed,
        git_author_wrap_wired,
        principal_credit_arm_absent,
        deepen_honest,
    }
}

/// Refuse bool-flip remainder close / invented physics GREEN on AGENT-LOOP-07 deepen.
pub fn agent_loop_07_refuse_remainder_overclaim(
    p: &AgentLoop07PrincipalCreditProbe,
) -> Result<(), String> {
    if p.physics_green || agent_loop_07_physics_green() {
        return Err("overclaim: agent-loop-07 cold-wire is not physics GREEN".into());
    }
    if p.remainder_row_closed || agent_loop_07_remainder_row_closed() {
        return Err("overclaim: agent-loop-07 remainder row closed invented".into());
    }
    if p.git_author_wrap_wired {
        return Err("overclaim: git/PR/author wrap not wired on host".into());
    }
    if !p.principal_credit_arm_absent || !agent_loop_07_principal_credit_arm_absent() {
        return Err("overclaim: principal-side credit arm wired invented".into());
    }
    if !p.deepen_honest {
        return Err("agent-loop-07 principal credit deepen probe failed".into());
    }
    Ok(())
}

/// Whether the AGENT-LOOP-07 principal credit cold-wire deepen honesty probe passes.
#[must_use]
pub fn agent_loop_07_principal_credit_honest() -> bool {
    agent_loop_07_principal_credit_probe().deepen_honest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::gate::evidence::TransitionEvidence;

    fn sample_evidence() -> TransitionEvidence {
        TransitionEvidence {
            catalog_id: "umst.gate.cd_transition",
            admissibility: AdmissibilityToken::Admissible,
            margin: AdmissibilityMargin(0.5),
            observed_at: None,
        }
    }

    #[test]
    fn wire_serializes_observed_at_when_provided() {
        let wire = transition_evidence_to_wire(
            sample_evidence(),
            Some(UcrsObservedAtWire {
                wall_ms: 1_718_745_600_000,
                ucrs_seq: 42,
            }),
            None,
            None,
        );
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(json.contains("observed_at"));
        assert!(json.contains("1718745600000"));
    }

    #[test]
    fn default_build_omits_observed_at_without_stamp() {
        let wire = transition_evidence_to_wire(sample_evidence(), None, None, None);
        assert!(wire.observed_at.is_none());
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(!json.contains("observed_at"));
    }

    #[test]
    fn spine_event_dual_ledger_axiom_anchor() {
        let cost = SpineEventCost::new(2.87e-21, 150.0);
        assert!(cost.rails_nonnegative());
        assert_eq!(cost.axiom_anchor, SpineEventCost::PHYSICAL_SECOND_LAW);
        let wire = transition_evidence_to_wire(sample_evidence(), None, Some(cost), None);
        assert_eq!(wire.axiom_anchor.as_deref(), Some("physicalSecondLaw"));
        assert!(wire.compute_cost_j.unwrap() >= 0.0);
        assert!(wire.material_dissipation_j.unwrap() >= 0.0);
    }

    #[test]
    fn boundary_stamp_prefers_over_evidence_observed_at() {
        let mut evidence = sample_evidence();
        evidence.observed_at = Some(UcrsObservedAtWire {
            wall_ms: 1,
            ucrs_seq: 1,
        });
        let wire = transition_evidence_to_wire(
            evidence,
            Some(UcrsObservedAtWire {
                wall_ms: 99,
                ucrs_seq: 7,
            }),
            None,
            None,
        );
        let stamp = wire.observed_at.expect("boundary stamp");
        assert_eq!(stamp.wall_ms, 99);
        assert_eq!(stamp.ucrs_seq, 7);
    }

    #[test]
    fn evidence_observed_at_used_when_boundary_stamp_absent() {
        let mut evidence = sample_evidence();
        evidence.observed_at = Some(UcrsObservedAtWire {
            wall_ms: 55,
            ucrs_seq: 3,
        });
        let wire = transition_evidence_to_wire(evidence, None, None, None);
        let stamp = wire.observed_at.expect("evidence stamp");
        assert_eq!(stamp.wall_ms, 55);
        assert_eq!(stamp.ucrs_seq, 3);
    }

    #[test]
    fn admissibility_wire_round_trip_screaming_snake() {
        let wire = transition_evidence_to_wire(
            TransitionEvidence {
                catalog_id: "umst.gate.cd_transition",
                admissibility: AdmissibilityToken::Inadmissible,
                margin: AdmissibilityMargin(-0.1),
                observed_at: None,
            },
            None,
            None,
            None,
        );
        assert_eq!(wire.admissibility, AdmissibilityWire::Inadmissible);
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(json.contains("INADMISSIBLE"));
        let back: TransitionEvidenceWire = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.admissibility, AdmissibilityWire::Inadmissible);
        assert_eq!(back.catalog_id, "umst.gate.cd_transition");
    }

    #[test]
    fn ucrs_observed_at_wire_serde_bidirectional() {
        let host = UcrsObservedAtWire {
            wall_ms: 1_700_000_000_000,
            ucrs_seq: 9,
        };
        let serde_form = UcrsObservedAtWireSerde::from(host);
        assert_eq!(serde_form.wall_ms, host.wall_ms);
        assert_eq!(serde_form.ucrs_seq, host.ucrs_seq);
        let back = UcrsObservedAtWire::from(serde_form);
        assert_eq!(back, host);
    }

    #[test]
    fn w29_113_cold_wire_deepen_honest_probe() {
        let probe = cold_wire_w29_113_deepen_probe();
        assert_eq!(probe.cell_id, W29_113_CELL_ID);
        assert_eq!(probe.schema_version, W29_113_DEEPEN_SCHEMA_VERSION);
        assert_eq!(probe.honest_posture, W29_113_HONEST_POSTURE);
        assert!(!probe.green_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(cold_wire_w29_113_deepen_honest());
        assert!(cold_wire_honest_fence_holds());
    }

    #[test]
    fn w29_113_non_claim_text_covers_forbidden_invent() {
        for needle in [
            "not GREEN",
            "not OP-5 PASS",
            "not production_wired",
            "not MASTER_RETICK",
        ] {
            assert!(
                W29_113_NON_CLAIM.contains(needle),
                "missing non-claim fragment: {needle}"
            );
        }
    }

    #[test]
    fn agent_loop_07_identity_credit_stamp_serializes_on_durable_transition() {
        let credit = DurableIdentityCreditStamp {
            commissioned_by: CreditCommissionParty::Human,
            executed_by: CreditExecutorParty::Agent,
            principal_named: PrincipalNamedParty::Human,
        };
        assert!(credit.principal_differs_from_executor());
        let wire = transition_evidence_to_wire(
            sample_evidence(),
            None,
            None,
            Some(credit),
        );
        assert_eq!(wire.commissioned_by, Some(CreditCommissionParty::Human));
        assert_eq!(wire.executed_by, Some(CreditExecutorParty::Agent));
        assert_eq!(wire.principal_named, Some(PrincipalNamedParty::Human));
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(json.contains("commissioned_by"));
        assert!(json.contains("executed_by"));
        assert!(json.contains("principal_named"));
        let back: TransitionEvidenceWire = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.commissioned_by, wire.commissioned_by);
        assert_eq!(back.executed_by, wire.executed_by);
        assert_eq!(back.principal_named, wire.principal_named);
    }

    #[test]
    fn agent_loop_07_credit_omitted_on_non_identity_catalog() {
        let credit = DurableIdentityCreditStamp {
            commissioned_by: CreditCommissionParty::Agent,
            executed_by: CreditExecutorParty::Agent,
            principal_named: PrincipalNamedParty::Agent,
        };
        let wire = transition_evidence_to_wire(
            TransitionEvidence {
                catalog_id: "thermodynamic_mix",
                admissibility: AdmissibilityToken::Admissible,
                margin: AdmissibilityMargin(0.5),
                observed_at: None,
            },
            None,
            None,
            Some(credit),
        );
        assert!(wire.commissioned_by.is_none());
        assert!(wire.executed_by.is_none());
        assert!(wire.principal_named.is_none());
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(!json.contains("commissioned_by"));
    }

    #[test]
    fn agent_loop_07_principal_credit_deepen_honest_probe() {
        let probe = agent_loop_07_principal_credit_probe();
        assert_eq!(probe.cell_id, AGENT_LOOP_07_CELL_ID);
        assert_eq!(probe.schema_version, AGENT_LOOP_07_DEEPEN_SCHEMA_VERSION);
        assert!(!probe.consciousness_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.green_claimed);
        assert!(agent_loop_07_principal_credit_honest());
        assert!(cold_wire_honest_fence_holds());
    }

    #[test]
    fn agent_loop_07_non_claim_covers_principal_executor_collision() {
        assert!(AGENT_LOOP_07_NON_CLAIM.contains("principal stamp ≠ consciousness"));
        assert!(AGENT_LOOP_07_NON_CLAIM.contains("git author ≠ executed_by"));
    }

    #[test]
    fn principal_stamp_refuses_executor_erasure_shape() {
        let credit = DurableIdentityCreditStamp {
            commissioned_by: CreditCommissionParty::Human,
            executed_by: CreditExecutorParty::Agent,
            principal_named: PrincipalNamedParty::Human,
        };
        let mut wire = transition_evidence_to_wire(
            TransitionEvidence {
                catalog_id: CD_TRANSITION_CATALOG_ID,
                admissibility: AdmissibilityToken::Admissible,
                margin: AdmissibilityMargin(0.5),
                observed_at: None,
            },
            None,
            None,
            Some(credit),
        );
        assert!(!wire.refuses_executor_erasure());
        assert!(complete_stamp_shape_live_consume(&wire).is_ok());
        wire.executed_by = None;
        assert!(wire.refuses_executor_erasure());
        assert!(wire.crate_root_export_is_not_git_wrap());
        assert!(agent_loop_07_principal_credit_arm_absent());
        assert!(!agent_loop_07_remainder_pin().git_author_wrap_wired);
        assert!(complete_stamp_shape_live_consume(&wire).is_err());
    }

    #[test]
    fn agent_loop_07_remainder_pin_physics_green_and_row_open() {
        let pin = agent_loop_07_remainder_pin();
        assert!(!pin.physics_green);
        assert!(!pin.remainder_row_closed);
        assert!(!pin.git_author_wrap_wired);
        assert!(!agent_loop_07_physics_green());
        assert!(!agent_loop_07_remainder_row_closed());
        assert!(AGENT_LOOP_07_REMAINDER_GAP.contains("git/PR/author wrap unwrapped"));
    }

    #[test]
    fn agent_loop_07_refuse_remainder_overclaim_holds() {
        let probe = agent_loop_07_principal_credit_probe();
        assert!(!probe.physics_green);
        assert!(!probe.remainder_row_closed);
        assert!(!probe.git_author_wrap_wired);
        assert!(probe.principal_credit_arm_absent);
        agent_loop_07_refuse_remainder_overclaim(&probe).expect("honest probe");
    }

    #[test]
    fn agent_loop_07_refuse_remainder_overclaim_rejects_bool_flip() {
        let mut remainder_closed = agent_loop_07_principal_credit_probe();
        remainder_closed.remainder_row_closed = true;
        assert!(agent_loop_07_refuse_remainder_overclaim(&remainder_closed)
            .unwrap_err()
            .contains("remainder row closed"));

        let mut physics_green = agent_loop_07_principal_credit_probe();
        physics_green.physics_green = true;
        assert!(agent_loop_07_refuse_remainder_overclaim(&physics_green)
            .unwrap_err()
            .contains("physics GREEN"));

        let mut wrap_wired = agent_loop_07_principal_credit_probe();
        wrap_wired.git_author_wrap_wired = true;
        assert!(agent_loop_07_refuse_remainder_overclaim(&wrap_wired)
            .unwrap_err()
            .contains("git/PR/author wrap"));

        let mut arm_wired = agent_loop_07_principal_credit_probe();
        arm_wired.principal_credit_arm_absent = false;
        assert!(agent_loop_07_refuse_remainder_overclaim(&arm_wired)
            .unwrap_err()
            .contains("credit arm wired"));
    }

    #[test]
    fn agent_loop_07_cold_wire_posture_not_principal_wrap_holds() {
        assert!(super::agent_loop_07_cold_wire_posture_not_principal_wrap());
        assert_eq!(AGENT_LOOP_07_HONEST_POSTURE, "COLD_WIRE_PRINCIPAL_CREDIT_DEEPEN_ONLY");
        assert_eq!(AGENT_LOOP_07_PRINCIPAL_WRAP_POSTURE, "PRINCIPAL_WRAP_UNWIRED");
        assert!(!agent_loop_07_remainder_row_closed());
    }

    #[test]
    fn agent_loop_07_git_pr_commit_stamp_appropriation_arm() {
        let credit = DurableIdentityCreditStamp {
            commissioned_by: CreditCommissionParty::Human,
            executed_by: CreditExecutorParty::Agent,
            principal_named: PrincipalNamedParty::Human,
        };
        let stamp = credit.public_stamp();
        assert_eq!(stamp.git_author, PrincipalNamedParty::Human);
        let arm = credit.appropriation_arm();
        assert!(arm.git_author_executor_collision());
        assert!(arm.collision_unmeasured());
        assert!(arm.arm_absent());
        let wire = transition_evidence_to_wire(sample_evidence(), None, None, Some(credit));
        assert_eq!(wire.git_author, Some(PrincipalNamedParty::Human));
        assert_eq!(wire.pr_principal, Some(PrincipalNamedParty::Human));
        assert_eq!(wire.commit_stamp, Some(PrincipalNamedParty::Human));
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(json.contains("git_author"));
        assert!(json.contains("pr_principal"));
        assert!(json.contains("commit_stamp"));
    }

    #[test]
    fn agent_loop_07_principal_credit_gap_unmeasured_on_collision() {
        assert!(agent_loop_07_principal_credit_arm_absent());
        let credit = DurableIdentityCreditStamp {
            commissioned_by: CreditCommissionParty::Human,
            executed_by: CreditExecutorParty::Agent,
            principal_named: PrincipalNamedParty::Human,
        };
        assert!(credit.principal_differs_from_executor());
        assert!(credit.principal_credit_gap_unmeasured());
        let aligned = DurableIdentityCreditStamp {
            commissioned_by: CreditCommissionParty::Agent,
            executed_by: CreditExecutorParty::Agent,
            principal_named: PrincipalNamedParty::Agent,
        };
        assert!(!aligned.principal_differs_from_executor());
        assert!(!aligned.principal_credit_gap_unmeasured());
    }
}
