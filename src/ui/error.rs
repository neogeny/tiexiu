// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::json::error::ImportError;
use crate::peg::error::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON import/export failed: {0}")]
    JsonModel(#[from] ImportError),

    #[error("failed to serialize JSON output: {0}")]
    Json(#[from] serde_json::Error),

    #[error("parse failed: {0}")]
    Parse(#[from] ParseError),

    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
}
