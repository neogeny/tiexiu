// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod error;
pub mod fancy;
pub mod pattern;
pub mod traits;

pub use error::*;
pub use pattern::*;

pub use fancy::*;

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
