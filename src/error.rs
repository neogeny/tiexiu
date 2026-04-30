// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::api::_globalcache::CacheError;
use crate::json::error::JsonError;
use crate::peg::ParseFailure;
use crate::peg::error::{CompileError, Nope};
use crate::util::ensure::Ensure;

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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON import/export failed: {0}")]
    Regex(#[from] crate::util::pyre::Error),

    #[error("JSON import/export failed: {0}")]
    JsonModel(#[from] JsonError),

    // FIXME
    // #[error("tree JSON mapping failed: {0}")]
    // TreeJson(#[from] TreeJsonError),
    #[error("grammar compilation failed: {0}")]
    Compile(#[from] CompileError),

    #[error("parse failure: {0}")]
    ParseError(#[from] Nope),

    #[error("parse error: {0}")]
    Parse(#[from] ParseFailure),

    #[error("failed to serialize JSON output: {0}")]
    Json(#[from] serde_json::Error),

    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("I/O failed: {0}")]
    Cache(#[from] CacheError),

    #[error("Library failure: {0}")]
    Library(#[from] crate::util::Error),

    #[error("And now a message from your friendly test:\n{0}")]
    AndNowAMessageFromYourFriendlyTest(String),
}
