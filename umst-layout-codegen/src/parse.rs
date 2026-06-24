// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use serde::Deserialize;
use thiserror::Error;

pub const SCALAR_LAYOUT_SCHEMA_V1: &str = "umst_scalar_layout_v1";

/// Parsed nodal scalar channel map from `artifacts/scalar_layout.lock.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutSpec {
    pub schema: String,
    pub scalar_channel_count: usize,
    pub channel_ids: Vec<String>,
}

#[derive(Debug, Error)]
pub enum LayoutCodegenError {
    #[error("scalar layout lock is not valid JSON: {0}")]
    Json(String),
    #[error("scalar layout lock has unsupported schema `{found}` (expected `{SCALAR_LAYOUT_SCHEMA_V1}`)")]
    UnsupportedSchema { found: String },
    #[error("scalar layout lock missing or invalid scalar_channel_count")]
    InvalidChannelCount,
    #[error("scalar layout lock missing channel_ids array")]
    MissingChannelIds,
    #[error("scalar layout drift: scalar_channel_count={count} != channel_ids.len()={len}")]
    ChannelCountMismatch { count: usize, len: usize },
    #[error("channel_ids[{index}] `{id}` does not match SCALAR_[A-Z0-9_]+")]
    InvalidChannelId { index: usize, id: String },
}

#[derive(Debug, Deserialize)]
struct ScalarLayoutLock {
    schema: String,
    scalar_channel_count: u64,
    channel_ids: Vec<String>,
}

/// Parse pinned scalar layout JSON into a validated [`LayoutSpec`].
pub fn parse_scalar_layout_lock(json: &str) -> Result<LayoutSpec, LayoutCodegenError> {
    let lock: ScalarLayoutLock = serde_json::from_str(json).map_err(|e| LayoutCodegenError::Json(e.to_string()))?;

    if lock.schema != SCALAR_LAYOUT_SCHEMA_V1 {
        return Err(LayoutCodegenError::UnsupportedSchema {
            found: lock.schema,
        });
    }

    if lock.scalar_channel_count < 1 {
        return Err(LayoutCodegenError::InvalidChannelCount);
    }

    let count = lock.scalar_channel_count as usize;
    let len = lock.channel_ids.len();
    if count != len {
        return Err(LayoutCodegenError::ChannelCountMismatch { count, len });
    }

    if lock.channel_ids.is_empty() {
        return Err(LayoutCodegenError::MissingChannelIds);
    }

    for (index, id) in lock.channel_ids.iter().enumerate() {
        if !is_valid_scalar_channel_id(id) {
            return Err(LayoutCodegenError::InvalidChannelId {
                index,
                id: id.clone(),
            });
        }
    }

    Ok(LayoutSpec {
        schema: lock.schema,
        scalar_channel_count: count,
        channel_ids: lock.channel_ids,
    })
}

fn is_valid_scalar_channel_id(id: &str) -> bool {
    let Some(rest) = id.strip_prefix("SCALAR_") else {
        return false;
    };
    !rest.is_empty()
        && rest
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_LOCK: &str = r#"{
  "schema": "umst_scalar_layout_v1",
  "scalar_channel_count": 2,
  "channel_ids": ["SCALAR_CHANNEL0", "SCALAR_HUMIDITY"]
}"#;

    #[test]
    fn parse_rejects_count_mismatch() {
        let json = r#"{
  "schema": "umst_scalar_layout_v1",
  "scalar_channel_count": 3,
  "channel_ids": ["SCALAR_CHANNEL0", "SCALAR_HUMIDITY"]
}"#;
        let err = parse_scalar_layout_lock(json).unwrap_err();
        assert!(matches!(
            err,
            LayoutCodegenError::ChannelCountMismatch { count: 3, len: 2 }
        ));
    }

    #[test]
    fn parse_accepts_minimal_lock() {
        let spec = parse_scalar_layout_lock(SAMPLE_LOCK).unwrap();
        assert_eq!(spec.scalar_channel_count, 2);
        assert_eq!(
            spec.channel_ids,
            vec!["SCALAR_CHANNEL0".to_string(), "SCALAR_HUMIDITY".to_string()]
        );
    }
}
