// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::util::pyre::Error as ReError;
use thiserror::Error;

/// Errors that can occur during context/parsing operations.
#[derive(Debug, Error)]
pub enum Error {
    /// A referenced rule was not found in the grammar.
    #[error("rule not found in grammar: `{0}`")]
    MissingRule(String),

    /// An invalid regex pattern was provided.
    #[error("invalid regex pattern `{pattern}` in parser state: {source}")]
    InvalidRegexPattern {
        pattern: String,
        #[source]
        source: ReError,
    },

    /// A non-left-recursive rule was called recursively.
    #[error("recursive parse entered for non-left-recursive rule `{0}`")]
    NonLeftRecursiveCall(String),
}
