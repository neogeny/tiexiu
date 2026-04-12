// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

pub type Result<T> = std::result::Result<T, JsonError>;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("JSON Import error at {0}: {1}")]
    JsonPath(String, #[source] serde_json::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("JSON Export error: {0}")]
    JsonExport(#[from] std::fmt::Error),

    #[error("Root node must be a Grammar")]
    InvalidRoot,

    #[error("Invalid field: {0}")]
    InvalidField(String),

    #[error("Unsupported model variant: {0}")]
    UnsupportedModel(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for JsonError {
    fn from(s: String) -> Self {
        JsonError::Other(s)
    }
}
