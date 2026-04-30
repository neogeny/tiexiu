// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("fancy-regex error: {0}")]
    PyReFancy(#[from] fancy_regex::Error),

    #[error("invalid regex pattern: {0}")]
    InvalidPattern(String),
}

pub type Result<T> = std::result::Result<T, Error>;
