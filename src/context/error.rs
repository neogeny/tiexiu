// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::util::pyre::Error as ReError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("rule not found in grammar: `{0}`")]
    MissingRule(String),

    #[error("invalid regex pattern `{pattern}` in parser state: {source}")]
    InvalidRegexPattern {
        pattern: String,
        #[source]
        source: ReError,
    },

    #[error("recursive parse entered for non-left-recursive rule `{0}`")]
    NonLeftRecursiveCall(String),
}
