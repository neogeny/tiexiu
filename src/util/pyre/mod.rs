// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Pyre error types.
pub mod error;
/// Core regex pattern type.
pub mod pattern;
/// Pyre traits for pattern matching.
pub mod traits;

use crate::types::Str;
pub use error::*;
pub use pattern::*;

mod pyre_regex;

pub use traits::*;

pub type Pattern = pyre_regex::Pattern;

/// Compiles a regex pattern string into a Pattern.
pub fn compile(pattern: &str) -> Result<pyre_regex::Pattern> {
    pyre_regex::compile(pattern)
}

/// Escapes special regex characters for use as a literal.
pub fn escape(pattern: &str) -> Str {
    pyre_regex::escape(pattern)
}

/// Truncates a pattern string to the given limit.
pub fn truncate_pattern(pattern: &str, limit: usize) -> &str {
    if pattern.len() <= limit {
        pattern
    } else {
        &pattern[..limit]
    }
}
