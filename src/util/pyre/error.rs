// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

/// Pyre regex errors.
#[derive(Debug, Error)]
pub enum Error {
    /// An error from the underlying fancy-regex engine.
    #[error("fancy-regex error: {0}")]
    PyReFancy(#[from] fancy_regex::Error),
    /// An invalid regex pattern string.
    #[error("invalid regex pattern: {0}")]
    InvalidPattern(String),
}

/// Pyre result type.
pub type Result<T> = std::result::Result<T, Error>;
