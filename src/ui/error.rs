// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::error as liberror;
use crate::json::error::JsonError;
use crate::peg::ParseError;
use crate::peg::error::CompileError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON import/export failed: {0}")]
    JsonModel(#[from] JsonError),

    #[error("failed to serialize JSON output: {0}")]
    Json(#[from] serde_json::Error),

    #[error("parse failed: {0}")]
    Parse(#[from] ParseError),

    #[error("grammar compilation failed: {0}")]
    Compile(#[from] CompileError),

    #[error("library operation failed: {0}")]
    Library(#[from] liberror::Error),

    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
}
