// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `serde_json` round-trip checks for ROS contract types (`--features serde`).

use umst_manifold::ros::contract::{
    GateDecisionPayload, MixProposalPayload, TelemetryFramePayload,
};

fn sample_hash(byte: u8) -> [u8; 32] {
    let mut h = [0_u8; 32];
    h[0] = byte;
    h[31] = byte.wrapping_add(1);
    h
}

#[test]
fn gate_decision_payload_json_roundtrip() {
    let v = GateDecisionPayload {
        catalog_hash: sample_hash(1),
        gate_lane_id: 42,
        admitted: true,
        residual_margin: 1.0e-6,
    };
    let s = serde_json::to_string(&v)
        .expect("serde_json::to_string GateDecisionPayload ROS contract round-trip harness (FP §6)");
    let back: GateDecisionPayload = serde_json::from_str(&s)
        .expect("serde_json::from_str GateDecisionPayload ROS contract round-trip harness (FP §6)");
    assert_eq!(back, v);
}

#[test]
fn mix_proposal_payload_json_roundtrip() {
    let v = MixProposalPayload {
        catalog_hash: sample_hash(2),
        mix_epoch: 9001,
        proposal_digest: sample_hash(3),
    };
    let s = serde_json::to_string(&v)
        .expect("serde_json::to_string MixProposalPayload ROS contract round-trip harness (FP §6)");
    let back: MixProposalPayload = serde_json::from_str(&s)
        .expect("serde_json::from_str MixProposalPayload ROS contract round-trip harness (FP §6)");
    assert_eq!(back, v);
}

#[test]
fn telemetry_frame_payload_json_roundtrip() {
    let v = TelemetryFramePayload {
        catalog_hash: sample_hash(4),
        frame_seq: 128,
        wall_time_ns: 1_702_000_000_000_000_000_u128,
    };
    let s = serde_json::to_string(&v)
        .expect("serde_json::to_string TelemetryFramePayload ROS contract round-trip harness (FP §6)");
    let back: TelemetryFramePayload = serde_json::from_str(&s)
        .expect("serde_json::from_str TelemetryFramePayload ROS contract round-trip harness (FP §6)");
    assert_eq!(back, v);
}
