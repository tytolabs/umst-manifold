// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! PPO gateway and reward wiring.
//!
//! ## IO barrier (lazy solver cores, **fp-categorical-v04**)
//!
//! Treat [`ManifoldGateway`] as the **policy-facing boundary** between differentiable
//! physics (cartridge / solver stacks) and scalar **host** decisions:
//!
//! - **On-device reductions**: `dissipation.sum_dim(1)`, `free_energy` / reward reductions use
//!   `sum_dim` / `mean_dim` and return [`Tensor`]s — no `.into_scalar()` on the reward path here.
//! - **Deliberate scalar sync**: [`ThermodynamicCBF::verify_tensor_update`](crate::ai::cbf::ThermodynamicCBF::verify_tensor_update)
//!   sums `info_gain` and batch-summed `d_int`, then performs the **two** `.into_scalar()` reductions
//!   per topology step so Landauer erasure, Clausius–Duhem dissipation credit, and energy bookkeeping
//!   run in ordinary `f64` control flow (see that method’s docs). That is the canonical **bits +
//!   dissipation → host** read for this stack; keep additional `.into_scalar()` out of inner solver
//!   iterations unless required for numerics or convergence tests.
//! - **File / JSON**: this crate does not load UMST from disk; any serialization or filesystem
//!   I/O belongs in cartridges or upstream runners — keep solver kernels free of `std::fs` so
//!   they stay composable and lazy-friendly.
//!
//! Nodal diagnostics: [`crate::core::emergence::nodal_defect_tensor`],
//! [`crate::core::emergence::combine_nodal_for_reward`]; grid hotspots:
//! [`crate::core::emergence::EmergenceMonitor`].
//!
//! Optional structural-margin shaping: [`ManifoldGateway::zeta`] scales a per-batch
//! **mean** of [`PhysicalResult::safety_margin`](crate::core::traits::PhysicalResult::safety_margin)
//! added to the scalar reward. Default **ζ = 0** keeps the legacy reward and leaves the
//! thermodynamic CBF gate unchanged.
//!
//! With the **`information_density`** crate feature, [`ManifoldGateway::eta`] adds
//! **η · mean(information_density)** from the optional `information_density` field on [`PhysicalResult`]
//! the same way. Default **η = 0** preserves the reward without that term.
//!
//! With **`formal-witness`**, [`Self::evaluate_topology_step`] (and [`Self::evaluate_topology_step_formal`])
//! may reject transitions when gateway and UMST disagree on [`crate::core::tensors::UnifiedMaterialStateTensor::catalog_schema_digest`]
//! (both sides must opt in via `Some(digest)`; with **`formal-witness`**, [`Self::new`] pins the
//! gateway expectation to compiled `catalog.lock.json` — UMST still needs
//! [`UnifiedMaterialStateTensor::with_lock_catalog_schema_digest`] or an explicit digest).

use crate::ai::cbf::ThermodynamicCBF;
use crate::ai::constraint_loss::scaled_clausius_duhem_violation;
#[cfg(feature = "thmc-coupled")]
use crate::ai::constraint_loss::scaled_constraint_violation_penalty;
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
use crate::ai::constraint_loss::scaled_landauer_slack_violation;
use crate::ai::formal::FormalReject;
use crate::core::traits::{IScienceCartridge, PhysicalResult};
use burn::tensor::{backend::Backend, Tensor};

/// The Gateway interface for Thermodynamic Topology Optimization.
///
/// It wraps physical cartridges and enforces the Thermodynamic CBF. As an **IO barrier**:
/// [`Self::evaluate_topology_step`] keeps spatial economics on the tensor graph and performs the
/// only required **host scalar** extractions for mutual-information bits and batch-summed `d_int`
/// inside [`ThermodynamicCBF::verify_tensor_update`](crate::ai::cbf::ThermodynamicCBF::verify_tensor_update)
/// (not in the cartridge’s Newton / CG inner loops).
pub struct ManifoldGateway<B: Backend, C: IScienceCartridge<B>> {
    pub cartridge: C,
    pub cbf: ThermodynamicCBF,
    /// Safety-margin reward weight **ζ** (dimensionless). When non-zero, the scalar reward adds
    /// `ζ * mean_voxels(safety_margin)` per batch row (positive ζ rewards higher structural margin).
    /// **Default 0** in [`Self::new`] — no effect on CBF admissibility checks or legacy reward.
    pub zeta: f32,
    /// Information-density reward weight **η** (dimensionless). With the **`information_density`**
    /// feature enabled, when non-zero the scalar reward adds `η * mean_voxels(information_density)` per
    /// batch row (same reduction pattern as [`Self::zeta`] on `safety_margin`).
    /// **Default 0** in [`Self::new`]; ignored when the feature is off (field is not compiled into
    /// [`PhysicalResult`]).
    pub eta: f32,
    /// Performance reward weight **α** (free-energy term). Default **1.0**.
    pub alpha: f32,
    /// Dissipation penalty weight **β**. Default **0.5**.
    pub beta: f32,
    /// Carbon / cost penalty weight **γ**. Default **2.0**.
    pub gamma: f32,
    /// Clausius–Duhem constraint slack weight **λ_cd** (epistemic / Kleisli hot-bind paths).
    /// When non-zero with **`epistemic-ppo`** or **`kleisli-ppo-hot-bind`**, [`Self::constraint_loss_penalty`]
    /// scales [`crate::ai::constraint_loss::clausius_duhem_violation`] for soft training penalties.
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    pub lambda_cd: f32,
    /// Landauer erasure slack weight **λ_landauer** (same feature gate as [`Self::lambda_cd`]).
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    pub lambda_landauer: f32,
    /// Optional catalog/schema digest asserted against the incoming UMST when **`formal-witness`** is on.
    /// [`Self::new`] defaults to compiled lock bytes; set `None` explicitly to skip the witness.
    #[cfg(feature = "formal-witness")]
    pub expected_catalog_schema_digest: Option<[u8; 32]>,
    /// Phase 4 exit witnesses — warm orchestrator only ([`crate::ai::rejection_telemetry::RejectionTelemetry`]).
    pub rejection_telemetry: crate::ai::rejection_telemetry::RejectionTelemetry,
    /// Kleisli penalize drain from [`crate::physics::solvers::ThmcSolver::drain_gate_evidence`].
    #[cfg(feature = "thmc-coupled")]
    pub thmc_gate_evidence: Vec<crate::physics::solvers::thmc_step::ThmcStepGateEvidence>,
    /// Mechanics solve witnesses from [`crate::physics::solvers::ThmcSolver::drain_mechanics_solve_reports`].
    #[cfg(all(feature = "thmc-coupled", feature = "mechanics-adjoint"))]
    pub mechanics_solve_reports: Vec<crate::solve_report::SolveReport>,
    _backend: std::marker::PhantomData<B>,
}

impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> ManifoldGateway<B, C> {
    pub fn new(cartridge: C, temperature_k: f64, initial_credit: f64) -> Self {
        Self {
            cartridge,
            cbf: ThermodynamicCBF::new(temperature_k, initial_credit),
            zeta: 0.0_f32,
            eta: 0.0_f32,
            alpha: 1.0_f32,
            beta: 0.5_f32,
            gamma: 2.0_f32,
            #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
            lambda_cd: 0.0_f32,
            #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
            lambda_landauer: 0.0_f32,
            #[cfg(feature = "formal-witness")]
            expected_catalog_schema_digest: Some(
                crate::runtime::catalog::lock_upstream_catalog_digest_bytes(),
            ),
            rejection_telemetry: crate::ai::rejection_telemetry::RejectionTelemetry::default(),
            #[cfg(feature = "thmc-coupled")]
            thmc_gate_evidence: Vec::new(),
            #[cfg(all(feature = "thmc-coupled", feature = "mechanics-adjoint"))]
            mechanics_solve_reports: Vec::new(),
            _backend: std::marker::PhantomData,
        }
    }

    /// Inject host-parsed constraint slack weights (see [`crate::runtime::ppo_host`]).
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    #[must_use]
    pub fn with_constraint_weights(
        mut self,
        weights: crate::runtime::ppo_host::PpoConstraintWeights,
    ) -> Self {
        self.lambda_cd = weights.lambda_cd;
        self.lambda_landauer = weights.lambda_landauer;
        self
    }

    /// Apply [`crate::runtime::ppo_host::ppo_constraint_weights_from_env`] at the IO boundary.
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    #[must_use]
    pub fn with_constraint_weights_from_env(self) -> Self {
        self.with_constraint_weights(crate::runtime::ppo_host::ppo_constraint_weights_from_env())
    }

    /// Phase 4 exit witness accessor (cold edge).
    #[must_use]
    pub fn telemetry(&self) -> &crate::ai::rejection_telemetry::RejectionTelemetry {
        &self.rejection_telemetry
    }

    /// Absorb THMC post-step gate evidence from solver drain into Kleisli penalize path.
    #[cfg(feature = "thmc-coupled")]
    pub fn absorb_thmc_gate_evidence(
        &mut self,
        batch: impl IntoIterator<Item = crate::physics::solvers::thmc_step::ThmcStepGateEvidence>,
    ) {
        use crate::runtime::gate::evidence::AdmissibilityToken;
        for evidence in batch {
            if evidence.transition.admissibility == AdmissibilityToken::Inadmissible {
                self.rejection_telemetry
                    .record_soft_penalty(f64::from(evidence.constraint.margin.violation()));
            } else {
                self.rejection_telemetry
                    .record_commit(f64::from(evidence.constraint.margin.violation()));
            }
            self.thmc_gate_evidence.push(evidence);
        }
    }

    /// Differentiable penalize morphism from drained THMC gate evidence → [`Self::constraint_loss_penalty`].
    #[cfg(feature = "thmc-coupled")]
    pub fn constraint_loss_penalty_from_gate_evidence(&self, device: &B::Device) -> Tensor<B, 1>
    where
        B: Backend<FloatElem = f32>,
    {
        let lambda_cd = {
            #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
            {
                self.lambda_cd
            }
            #[cfg(not(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind")))]
            {
                0.0_f32
            }
        };
        let batch = self.thmc_gate_evidence.len().max(1);
        let violations: Vec<f32> = self
            .thmc_gate_evidence
            .iter()
            .map(|e| e.constraint.margin.violation())
            .collect();
        let data = if violations.is_empty() {
            vec![0.0_f32]
        } else {
            violations
        };
        let violation_tensor = Tensor::<B, 1>::from_floats(data.as_slice(), device);
        let sized = if violation_tensor.dims()[0] == batch {
            violation_tensor
        } else {
            violation_tensor.reshape([batch])
        };
        scaled_constraint_violation_penalty(lambda_cd, sized)
    }

    /// Absorb mechanics [`crate::solve_report::SolveReport`] into warm telemetry (non-converged → soft penalty).
    #[cfg(all(feature = "thmc-coupled", feature = "mechanics-adjoint"))]
    pub fn absorb_mechanics_solve_reports(
        &mut self,
        reports: impl IntoIterator<Item = crate::solve_report::SolveReport>,
    ) {
        for report in reports {
            if report.converged() {
                self.rejection_telemetry
                    .record_commit(f64::from(report.rel_residual));
            } else {
                self.rejection_telemetry
                    .record_soft_penalty(f64::from(report.rel_residual));
            }
            self.mechanics_solve_reports.push(report);
        }
    }

    /// Drain accumulated THMC gate evidence after an episode rollout.
    #[cfg(feature = "thmc-coupled")]
    pub fn drain_thmc_gate_evidence(
        &mut self,
    ) -> Vec<crate::physics::solvers::thmc_step::ThmcStepGateEvidence> {
        std::mem::take(&mut self.thmc_gate_evidence)
    }

    /// Host-side CD slack from drained THMC evidence (inadmissible → unit slack per step).
    #[cfg(feature = "thmc-coupled")]
    pub fn thmc_evidence_penalty_hint(&self) -> f32 {
        use crate::runtime::gate::evidence::AdmissibilityToken;
        let inadmissible = self
            .thmc_gate_evidence
            .iter()
            .filter(|e| e.transition.admissibility == AdmissibilityToken::Inadmissible)
            .count();
        inadmissible as f32
    }

    /// Set η from catalog per-step MI scan ([`crate::ros::calibrate_eta_bound_from_trace`]; Track G.3).
    /// Uses `TraceCalibrationReport::eta_bound_suggested` (post-CBF reward weight only).
    #[cfg(feature = "trace-calibration")]
    pub fn calibrate_eta_from_trace(&mut self, schema: &crate::ros::EmittedTraceSchema) {
        let report = crate::ros::calibrate_eta_bound_from_trace(schema);
        self.eta = report.eta_bound_suggested.clamp(0.0, 1.0) as f32;
    }

    /// Set η from prototype rolled-up ε envelope ([`crate::ros::prototype_eta_from_trace`]; Track G.3).
    #[cfg(feature = "trace-calibration")]
    pub fn calibrate_eta_from_prototype_envelope(
        &mut self,
        schema: &crate::ros::EmittedTraceSchema,
    ) {
        self.eta = crate::ros::prototype_eta_from_trace(schema);
    }

    /// Optional Clausius–Duhem soft penalty (`λ_cd · relu(−margin)` per batch row).
    ///
    /// With penalize features disabled or [`Self::lambda_cd`] = 0, returns a zero `[B]` tensor.
    pub fn constraint_loss_penalty(
        &self,
        old_density: Tensor<B, 1>,
        new_density: Tensor<B, 1>,
        old_free_energy: Tensor<B, 1>,
        new_free_energy: Tensor<B, 1>,
        dt_s: Tensor<B, 1>,
    ) -> Tensor<B, 1>
    where
        B: Backend<FloatElem = f32>,
    {
        let lambda_cd = {
            #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
            {
                self.lambda_cd
            }
            #[cfg(not(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind")))]
            {
                0.0_f32
            }
        };
        scaled_clausius_duhem_violation(
            lambda_cd,
            old_density,
            new_density,
            old_free_energy,
            new_free_energy,
            dt_s,
        )
    }

    /// Optional Landauer erasure soft penalty (`λ_landauer · relu(k_B T ln2 · bits − credit)`).
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    pub fn landauer_constraint_loss_penalty(&self, info_gain_bits: Tensor<B, 1>) -> Tensor<B, 1>
    where
        B: Backend<FloatElem = f32>,
    {
        if self.lambda_landauer == 0.0_f32 {
            let batch = info_gain_bits.dims()[0];
            return Tensor::zeros([batch], &info_gain_bits.device());
        }
        scaled_landauer_slack_violation(
            self.lambda_landauer,
            info_gain_bits,
            self.cbf.temperature_k as f32,
            self.cbf.available_credit_joules as f32,
        )
    }

    /// Combined CD + Landauer soft penalties for Kleisli penalize hooks.
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    pub fn total_constraint_loss_penalty(
        &self,
        old_density: Tensor<B, 1>,
        new_density: Tensor<B, 1>,
        old_free_energy: Tensor<B, 1>,
        new_free_energy: Tensor<B, 1>,
        dt_s: Tensor<B, 1>,
        info_gain_bits: Tensor<B, 1>,
    ) -> Tensor<B, 1>
    where
        B: Backend<FloatElem = f32>,
    {
        self.constraint_loss_penalty(
            old_density,
            new_density,
            old_free_energy,
            new_free_energy,
            dt_s,
        )
        .add(self.landauer_constraint_loss_penalty(info_gain_bits))
    }

    /// Evaluates a proposed topology state; errors are structured as [`FormalReject`].
    ///
    /// Prefer this when rejecting transitions must be classified (CBF vs catalog witness). The
    /// legacy [`Self::evaluate_topology_step`] API maps these cases to [`String`] via [`FormalReject`]'s [`Display`] impl.
    pub fn evaluate_topology_step_formal(
        &mut self,
        raw_state: crate::core::tensors::UnifiedMaterialStateTensor<B>,
        info_gain: Tensor<B, 1>,
    ) -> Result<
        (
            crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
            Tensor<B, 1>,
        ),
        FormalReject,
    > {
        #[cfg(feature = "formal-witness")]
        if let (Some(expected), Some(observed)) = (
            self.expected_catalog_schema_digest,
            raw_state.catalog_schema_digest,
        ) {
            if expected != observed {
                return Err(FormalReject::CatalogSchemaDigestMismatch { expected, observed });
            }
        }

        let staging = raw_state.try_as_verified_dec_bundle(0).map_err(|e| {
            FormalReject::DecTypestateStaging {
                detail: format!("{e:?}"),
            }
        })?;

        // 1. Execute the physics simulation across the topological Cellular Sheaf
        let physical_result: PhysicalResult<B> = self.cartridge.compute_topology(&raw_state);

        let mut secured_state = raw_state;
        crate::core::apply_physics::apply_physics_to_umst(&physical_result, &mut secured_state)
            .map_err(|detail| FormalReject::DecTypestateStaging { detail })?;

        // Keep metrics in Sparse Space [Batch, N_active_voxels]
        let free_energy = physical_result.free_energy.clone();
        let dissipation = physical_result.dissipation.clone();
        let cost = physical_result.cost.clone();

        // 2. Validate against the Thermodynamic Control Barrier Function
        // Sum dissipation across all voxels for the CBF macro check
        let d_int = dissipation.clone().sum_dim(1).squeeze(1);

        match self.cbf.verify_tensor_update::<B>(d_int, info_gain.clone()) {
            Ok(erasure_cost) => {
                self.rejection_telemetry.record_commit(0.0);
                // Spatial Reward = (Alpha * Performance) - (Beta * Dissipation) - (Gamma * CO2) - Erasure Cost
                let performance = free_energy.mul_scalar(self.alpha);
                let penalty = dissipation
                    .mul_scalar(self.beta)
                    .add(cost.mul_scalar(self.gamma));

                // The erasure cost is paid uniformly across the topology
                let final_spatial_reward = performance.sub(penalty).sub_scalar(erasure_cost as f32);

                // Construct the mathematically secured tensor (staging → proof morphism).
                let verified_state = crate::core::tensors::VerifiedUMST::<
                    B,
                    crate::core::tensors::ClausiusDuhemProof,
                >::lift_after_dec_staging_witness(
                    staging, secured_state
                );

                // Flatten the spatial reward to a single scalar [Batch] for the policy gradient (Adjoint Method target)
                let mut total_reward = final_spatial_reward.sum_dim(1).squeeze(1);
                if self.zeta != 0.0_f32 {
                    let margin_mean = physical_result.safety_margin.clone().mean_dim(1).squeeze(1);
                    total_reward = total_reward.add(margin_mean.mul_scalar(self.zeta));
                }
                #[cfg(feature = "information_density")]
                if self.eta != 0.0_f32 {
                    let info_mean = physical_result
                        .information_density
                        .clone()
                        .mean_dim(1)
                        .squeeze(1);
                    total_reward = total_reward.add(info_mean.mul_scalar(self.eta));
                }

                Ok((verified_state, total_reward))
            }
            Err(detail) => {
                self.rejection_telemetry.record_reject();
                Err(FormalReject::ThermodynamicControlBarrier {
                    catalog_id: crate::ai::formal::LANDAUER_CBF_CATALOG_ID,
                    detail,
                })
            }
        }
    }

    /// Evaluates a proposed topology state.
    /// This runs the full Cartridge functor pass and gates the result through the CBF.
    ///
    /// # Arguments
    /// * `raw_state` - The proposed UMST Cellular Sheaf
    /// * `info_gain` - The calculated mutual information resolved by this step.
    ///
    /// # Returns
    /// * Ok(VerifiedUMST, Reward) - The mathematically secured state and the per-batch scalar reward
    ///   (spatial thermodynamic terms plus **ζ · mean(safety_margin)** when [`ManifoldGateway::zeta`] ≠ 0,
    ///   and with **`information_density`**, **η · mean(information_density)** when [`ManifoldGateway::eta`] ≠ 0).
    /// * Err(String) - If the state violates the Clausius-Duhem Thermodynamic gate (or, with **`formal-witness`**, the catalog digest witness).
    pub fn evaluate_topology_step(
        &mut self,
        raw_state: crate::core::tensors::UnifiedMaterialStateTensor<B>,
        info_gain: Tensor<B, 1>,
    ) -> Result<
        (
            crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
            Tensor<B, 1>,
        ),
        String,
    > {
        self.evaluate_topology_step_formal(raw_state, info_gain)
            .map_err(|e| e.to_string())
    }
}
