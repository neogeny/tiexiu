// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::json::error::JsonError;
use crate::peg::ParseFailure;
use crate::peg::error::{CompileError, DisasterReport, Nope};
use crate::util::ensure::Ensure;

/// Result type alias with `Error` as the error variant.
pub type Result<T> = std::result::Result<T, Error>;

impl From<Ensure> for Error {
    fn from(e: Ensure) -> Self {
        Error::AndNowAMessageFromYourFriendlyTest(e.to_string())
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::AndNowAMessageFromYourFriendlyTest(msg.to_string())
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::AndNowAMessageFromYourFriendlyTest(msg)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AndNowAMessageFromYourFriendlyTest(msg) => {
                write!(f, "{}", msg)
            }
            // Uses your #[error("...")] definitions. Safely breaks the loop!
            other => std::fmt::Display::fmt(other, f),
        }
    }
}

/// Unified error type for all TieXiu operations.
#[derive(thiserror::Error)]
pub enum Error {
    /// Regex compilation error.
    #[error("JSON import/export failed: {0}")]
    Regex(#[from] crate::util::pyre::Error),

    /// JSON model conversion error.
    #[error("JSON import/export failed: {0}")]
    JsonModel(#[from] JsonError),

    // FIXME
    // #[error("tree JSON mapping failed: {0}")]
    // TreeJson(#[from] TreeJsonError),
    /// Grammar compilation failed.
    #[error("grammar compilation failed: {0}")]
    Compile(#[from] CompileError),

    /// Parsing disaster (unrecoverable error).
    #[error("{0}")]
    ParseDisaster(#[from] DisasterReport),

    /// Parse failure.
    #[error("!! {0}")]
    Parse(#[from] ParseFailure),

    /// Escaped internal error variant.
    #[error("!! {0}")]
    EscapedNope(#[from] Nope),

    /// JSON serialization error.
    #[cfg(feature = "serde_json")]
    #[error("failed to serialize JSON output: {0}")]
    Json(#[from] serde_json::Error),

    /// I/O error.
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),

    /// Format error.
    #[error("Fmt failed: {0}")]
    Fmt(#[from] std::fmt::Error),

    // #[error("I/O failed: {0}")]
    // Cache(#[from] CacheError),
    /// Internal library error.
    #[error("Library failure: {0}")]
    Library(#[from] crate::util::Error),

    /// Test-purpose error with a custom message.
    #[error("And now a message from your friendly test:\n{0}")]
    AndNowAMessageFromYourFriendlyTest(String),
}
