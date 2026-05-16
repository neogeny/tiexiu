// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

/// Tree-related errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("expected tree kind `{expected}`, found `{found}`")]
    UnexpectedKind {
        expected: &'static str,
        found: &'static str,
    },
}
