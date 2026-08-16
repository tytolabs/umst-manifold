// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! `HardwareUnit` — the §14bis.f-H-8 *object* morphism interface (category **𝓗**; FORWARD-PLAN v1.2 §0.2).
//! Every method is a typed *capability* on a physical unit; backends (H-9, H-9-mac) will implement.
//!
//! # Morphism / composition story (identity; FORWARD-PLAN §0.2)
//! Workload *routing* morphisms `U → V` and their composition are modeled in
//! [`super::laws`]; the trait here is the per-unit *observation + allocation* API that any
//! routed workload ultimately lands on. **Identity** on a unit is “stay on the same
//! `UnitKind` lane”; see [`super::laws::route_id`] and [`super::laws::compose`].
//! **Inverse** is *not* guaranteed for allocate/infer (irreversible kernel launches); only
//! `deallocate` reclaims explicit handles — matching physical reality (NED §0.5).

use std::string::String;
use std::vec::Vec;

/// ZCI-EXEMPT: one workload class in the PPO / B-Arc story (B-2.5 placeholder for H-8)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorkloadClass {
    /// THEOREM-BOUND: one-shot / batched forward inference
    Inference,
    /// Training or fine-tuning
    Training,
    /// ZCI-EXEMPT: transfer / distillation
    Transfer,
}

/// §14bis.f-H-9: workload kinds wired through [`InferenceBatch::op`]; extend in B-2. Must match
/// `REGISTRY` `hal_workload_smoke_infer_op` (`SMOKE_INFER_OP`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorkloadKind {
    /// 1 KiB host alloc + memcpy + free; every H-9 backend implements or returns [`HalError::NotYetImplemented`].
    Smoke,
}

/// Magic `InferenceBatch.op` for [`WorkloadKind::Smoke`] (Tier-3 REGISTRY: `hal_workload_smoke_infer_op`).
pub const SMOKE_INFER_OP: u32 = 0x0053_4D4B; // "SMK" + tag

/// MEASUREMENT: supported numeric precisions (capability, not a measured constant)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComputePrecision {
    /// 32-bit float
    Fp32,
    /// 64-bit float
    Fp64,
    /// 16-bit float
    Fp16,
    /// 32-bit integer
    I32,
    /// 64-bit integer
    I64,
    /// 8-bit integer
    Int8,
}

/// MEASUREMENT: power-state probe result (read-only; no RAPL/IOReport here in H-8)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerStateKind {
    /// P-style active
    P0,
    P1,
    P2,
    P3,
}

/// Theorem-bound: first-class `PowerState` handle for trait returns
#[derive(Clone, Debug, PartialEq)]
pub struct PowerState {
    /// ZCI-EXEMPT: host-normalized 0.0 = min, 1.0 = full capability (interpretation in H-9)
    pub headroom: f64,
    /// Theorem: discrete bucket for guards
    pub kind: PowerStateKind,
    /// NED: optional probe note, e.g. `permission_denied: /path` (H-9)
    pub reason: Option<String>,
}

/// ZCI-EXEMPT: abstract allocation / inference handles (H-8 carries only structure)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AllocationId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InferenceId(pub u64);

/// MEASUREMENT: allocation / inference *spec* envelopes (H-8 stub-friendly)
#[derive(Clone, Debug, PartialEq)]
pub struct AllocationSpec {
    /// Requested working-set bytes
    pub bytes: u64,
    /// Opaque class tag for backends
    pub class: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InferenceBatch {
    /// Batch size (logical)
    pub batch: u32,
    /// Opaque op id (H-8 does not fix IR)
    pub op: u32,
}

/// NED: explicit HAL error (no `unwrap` in API surface; backends map faults here)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HalError {
    /// ZCI-EXEMPT: not applicable on this architecture
    NotApplicable,
    /// NED: missing capability / not configured
    NotConfigured,
    /// NED: backend reported fault
    Backend { code: u32, reason: String },
    /// H-9: workload not available yet (B-2 may lift); never silent no-op
    NotYetImplemented {
        unit: String,
        workload: String,
        reason: String,
    },
}

/// The hardware-unit capability trait — exactly **7** methods (H-8 Q5/G5; REGISTRY
/// `hal_trait_method_count` = 7; FORWARD-PLAN §3.1 H-8).
///
/// # Categorical / λ notes (I2, I3)
/// Each method is a typed arrow from this unit’s *state* to a **pure** result
/// in the `Result` / `Vec` algebras, modulo future IO in backends (H-9); the
/// *specification* here is total on the happy path for mocks.
pub trait HardwareUnit {
    /// Enumerate model identifiers visible to the scheduler (empty when unknown honestly).
    fn enumerate_models(&self) -> Vec<String>;
    /// Supported precisions for this unit (may be empty = honest “none”).
    fn supported_precisions(&self) -> Vec<ComputePrecision>;
    /// Best-effort allocation; [`Err`] on fault or absence.
    fn allocate(&self, spec: &AllocationSpec) -> Result<AllocationId, HalError>;
    /// Run one inference step on an allocation; [`Err`] when inapplicable.
    fn infer(&self, alloc: AllocationId, batch: &InferenceBatch) -> Result<InferenceId, HalError>;
    /// Reclaim a prior [`AllocationId`] (groupoid leg toward *revert* in later MTP-2.5; FORWARD-PLAN §0.2).
    fn deallocate(&self, id: AllocationId) -> Result<(), HalError>;
    /// Sample power state (H-8 returns synthetic values for [`super::mocks`])
    fn power_state(&self) -> Result<PowerState, HalError>;
    /// Drift window in observation ticks (integer width; B-3 ε story hooks here in H-10+)
    fn drift_window(&self) -> u64;
}
