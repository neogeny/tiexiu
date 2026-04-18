// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::json::error::JsonError;
use crate::json::tree_json::TreeJsonError;
use crate::peg::error::CompileError;
use crate::peg::{Nope, ParseError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON import/export failed: {0}")]
    JsonModel(#[from] JsonError),

    #[error("tree JSON mapping failed: {0}")]
    TreeJson(#[from] TreeJsonError),

    #[error("grammar compilation failed: {0}")]
    Compile(#[from] CompileError),

    #[error("parse failed: {0}")]
    ParseFailure(#[from] Nope),

    #[error("parse failed: {0}")]
    ParseError(#[from] ParseError),

    #[error("failed to serialize JSON output: {0}")]
    Json(#[from] serde_json::Error),

    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Library failure: {0}")]
    Library(#[from] crate::util::Error),
}
