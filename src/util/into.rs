// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Str;
// use std::borrow::Cow;

/// A convenience trait for string-to-boxed-slice conversions.
pub trait IntoStr {
    /// Converts this value into a boxed Str.
    fn into_str(self) -> Str;
}

/// Trait for converting values to internal string storage types.
pub trait ToInternalStr {
    /// Converts to a boxed Str.
    fn to_internal(self) -> Str;
    /// Converts to a reference-counted string.
    fn to_ref(self) -> Str;
}

impl IntoStr for String {
    #[inline]
    fn into_str(self) -> Str {
        self.into()
    }
}

impl IntoStr for &str {
    #[inline]
    fn into_str(self) -> Str {
        self.into()
    }
}

impl ToInternalStr for String {
    #[inline]
    fn to_internal(self) -> Str {
        self.into()
    }
    #[inline]
    fn to_ref(self) -> Str {
        self.into()
    }
}

impl ToInternalStr for &str {
    #[inline]
    fn to_internal(self) -> Str {
        self.into()
    }
    #[inline]
    fn to_ref(self) -> Str {
        self.into()
    }
}

impl ToInternalStr for Str {
    #[inline]
    fn to_internal(self) -> Str {
        self
    }
    #[inline]
    fn to_ref(self) -> Str {
        self
    }
}

// NOTE: An example

/// A newtype wrapper for rule name strings.
pub struct RuleName(pub Str);

impl From<String> for RuleName {
    fn from(s: String) -> Self {
        RuleName(s.into())
    }
}

impl From<&str> for RuleName {
    fn from(s: &str) -> Self {
        RuleName(s.into())
    }
}
