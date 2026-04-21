// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod error;
pub mod fancy;
pub mod pattern;
pub mod traits;

#[cfg(feature = "pcre2")]
pub mod pcre2;

#[cfg(feature = "regex")]
pub mod regex;

pub use error::*;
pub use pattern::*;

// Selection of the default backend
// Priority: pcre2 > regex > fancy (fallback)

#[cfg(feature = "pcre2")]
pub use pcre2::*;

#[cfg(all(not(feature = "pcre2"), feature = "regex"))]
pub use regex::*;

#[cfg(not(any(feature = "pcre2", feature = "regex")))]
pub use fancy::*;

pub fn truncate_pattern(pattern: &str, limit: usize) -> &str {
    if pattern.len() <= limit {
        pattern
    } else {
        &pattern[..limit]
    }
}

pub fn escape(pattern: &str) -> Box<str> {
    #[cfg(feature = "pcre2")]
    {
        pcre2::escape(pattern)
    }
    #[cfg(all(not(feature = "pcre2"), feature = "regex"))]
    {
        regex::escape(pattern)
    }
    #[cfg(not(any(feature = "pcre2", feature = "regex")))]
    {
        fancy::escape(pattern)
    }
}
