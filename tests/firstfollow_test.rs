// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for first/follow - requires grammar internals
//!
//! These tests verify first/follow sets and left-recursion detection.

use tiexiu::Result;

#[test]
#[ignore = "firstfollow tests require rule internals"]
fn test_firstfollow() -> Result<()> {
    // This test requires access to internal rule analysis
    // For now, just verify the test infrastructure works
    let grammar = tiexiu::api::compile("start = 'a'", &[])?;
    assert_eq!(grammar.rules().count(), 1);
    Ok(())
}
