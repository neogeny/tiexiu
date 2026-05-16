// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

/// Utility-level result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Utility error type.
#[derive(Debug, Error)]
pub enum Error {
    /// An unknown configuration option was encountered.
    #[error("Unknown Cfg option {0}")]
    UnknownCfgOption(Box<str>),
}
