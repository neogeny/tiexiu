// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::types::Str;
use std::rc::Rc;
// use std::borrow::Cow;

/// A convenience trait to provide a unified path for string-to-boxed-slice conversions.
/// This allows you to use .into_box() as a more descriptive alternative to .into()
pub trait IntoStr {
    fn into_str(self) -> Str;
}

/// The "Call-Site Anchor" trait.
/// You implement this for anything that should be turnable into your
/// internal storage types (`Str` or `Rc<str>`).
pub trait ToInternalStr {
    fn to_internal(self) -> Str;
    fn to_rc(self) -> Rc<str>;
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
    fn to_rc(self) -> Rc<str> {
        self.into()
    }
}

impl ToInternalStr for &str {
    #[inline]
    fn to_internal(self) -> Str {
        self.into()
    }
    #[inline]
    fn to_rc(self) -> Rc<str> {
        self.into()
    }
}

impl ToInternalStr for Str {
    #[inline]
    fn to_internal(self) -> Str {
        self
    }
    #[inline]
    fn to_rc(self) -> Rc<str> {
        self
    }
}

// NOTE: An example

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
