// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("JSON error at {0}: {1}")]
    JsonPath(String, #[source] serde_json::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Root node must be a Grammar")]
    InvalidRoot,

    #[error("Unsupported model variant: {0}")]
    UnsupportedModel(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for ImportError {
    fn from(s: String) -> Self {
        ImportError::Other(s)
    }
}
