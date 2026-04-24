// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::json::error::JsonError;
use crate::json::tree_json::TreeJsonError;
use crate::peg::ParseError;
use crate::peg::error::CompileError;
use crate::peg::nope::Nope;

pub type Result<T> = std::result::Result<T, Error>;

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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON import/export failed: {0}")]
    Regex(#[from] crate::util::pyre::Error),

    #[error("JSON import/export failed: {0}")]
    JsonModel(#[from] JsonError),

    #[error("tree JSON mapping failed: {0}")]
    TreeJson(#[from] TreeJsonError),

    #[error("grammar compilation failed: {0}")]
    Compile(#[from] CompileError),

    #[error("parse failure: {0}")]
    ParseFailure(#[from] Nope),

    #[error("parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("failed to serialize JSON output: {0}")]
    Json(#[from] serde_json::Error),

    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Library failure: {0}")]
    Library(#[from] crate::util::Error),

    #[error("And now a message from your friendly test:\n{0}")]
    AndNowAMessageFromYourFriendlyTest(String),
}
