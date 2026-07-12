// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::peg::error::{CompileError, ParseFailure};
use thiserror::Error;

/// Result type alias for JSON import/export operations.
pub type Result<T> = std::result::Result<T, JsonError>;

/// Errors that can occur during JSON import/export of grammars and trees.
#[derive(Debug, Error)]
pub enum JsonError {
    /// Error encountered at a specific JSON path during import.
    #[cfg(feature = "serde_json")]
    #[error("JSON Import error at {0}: {1}")]
    JsonPath(String, #[source] serde_json::Error),

    /// Wrapper for serde_json errors.
    #[cfg(feature = "serde_json")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Error parsing JSON text.
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] json::Error),

    /// Error during JSON export/formatting.
    #[error("JSON Export error: {0}")]
    JsonExport(#[from] std::fmt::Error),

    /// The root JSON object is not a valid Grammar.
    #[error("Root node must be a Grammar")]
    InvalidRoot,

    /// An unexpected field was encountered in the JSON structure.
    #[error("Invalid field: {0}")]
    InvalidField(String),

    /// An unsupported model variant was encountered.
    #[error("Unsupported model variant: {0}")]
    UnsupportedModel(String),

    /// Generic catch-all error.
    #[error("Other error: {0}")]
    Other(String),

    /// Wrapper for parse failures.
    #[error("Parse failure: {0}")]
    Parse(#[from] ParseFailure),
}

impl From<String> for JsonError {
    fn from(s: String) -> Self {
        JsonError::Other(s)
    }
}

impl From<CompileError> for JsonError {
    fn from(e: CompileError) -> Self {
        JsonError::Other(e.to_string())
    }
}
