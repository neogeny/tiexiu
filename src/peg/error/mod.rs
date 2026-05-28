// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Parse failure and compile error types.
pub mod failure;
/// Parse result  and disaster report types.
pub mod nope;

pub use failure::*;
pub use nope::*;
