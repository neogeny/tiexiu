// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Pyre error types.
pub mod error;
/// Fancy pattern output/display.
pub mod fancy;
/// Core regex pattern type.
pub mod pattern;
/// Pyre traits for pattern matching.
pub mod traits;

pub use error::*;
pub use fancy::*;
pub use pattern::*;

/// Truncates a pattern string to the given limit.
pub fn truncate_pattern(pattern: &str, limit: usize) -> &str {
    if pattern.len() <= limit {
        pattern
    } else {
        &pattern[..limit]
    }
}

pub fn escape(pattern: &str) -> Box<str> {
    fancy::escape(pattern)
}
