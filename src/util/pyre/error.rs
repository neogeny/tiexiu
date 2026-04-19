// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("fancy-regex error: {0}")]
    Fancy(#[from] fancy_regex::Error),

    #[cfg(feature = "pcre2")]
    #[error("pcre2 error: {0}")]
    Pcre2(#[from] pcre2::Error),

    #[error("invalid regex pattern: {0}")]
    InvalidPattern(String),
}

pub type Result<T> = std::result::Result<T, Error>;
