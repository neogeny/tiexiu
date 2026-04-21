// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for regexpp

use tiexiu::Result;
use tiexiu::util::pyre::pattern::regexpp as r;

#[test]
fn test_regexpp_simple() -> Result<()> {
    let result = r(r"\d+")?;
    assert!(result.starts_with("r\"") || result.starts_with("r'"));
    Ok(())
}

#[test]
fn test_regexpp_with_backslash() -> Result<()> {
    let _ = r(r"\\d")?;
    Ok(())
}

#[test]
fn test_regexpp_with_tab() -> Result<()> {
    let result = r("a\tb")?;
    assert!(result.contains("\\t"));
    Ok(())
}
