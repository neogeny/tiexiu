// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::util::pyre::Error as ReError;
use thiserror::Error;

/// Input-related errors (e.g. invalid regex patterns).
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid {kind} regex pattern `{pattern}`: {source}")]
    InvalidRegex {
        kind: &'static str,
        pattern: String,
        #[source]
        source: ReError,
    },

    #[error(
        "pattern `{pattern}` for {kind} matches the empty string"
    )]
    RegexMatchesEmpty {
        kind: &'static str,
        pattern: String,
    }
}
