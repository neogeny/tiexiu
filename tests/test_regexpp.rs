// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for regexpp

use tiexiu::util::pyre::pattern::regexpp as r;

#[test]
fn test_regexpp_simple() {
    let result = r(r"\d+").unwrap();
    assert!(result.starts_with("r\"") || result.starts_with("r'"));
}

#[test]
fn test_regexpp_with_backslash() {
    let _ = r(r"\\d").unwrap();
}

#[test]
fn test_regexpp_with_tab() {
    let result = r("a\tb").unwrap();
    assert!(result.contains("\\t"));
}
