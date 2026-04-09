// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/strtools_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::util::strtools;

#[test]
fn test_visible_width() {
    assert_eq!(strtools::unicode_display_len("abc"), 3);
    assert_eq!(strtools::unicode_display_len("蛇"), 2);
    assert_eq!(strtools::unicode_display_len("🐍 Py"), 5);
}

#[test]
fn test_visual_linecount() {
    assert_eq!(strtools::linecount(""), 1);
    assert_eq!(strtools::linecount("hello"), 1);
    assert_eq!(strtools::linecount("hello\n"), 2);
    assert_eq!(strtools::linecount("\n\n"), 3);
    assert_eq!(strtools::linecount("win\r\nline"), 2);
    assert_eq!(strtools::linecount("mac\rline"), 2);
}

#[test]
fn test_linecount_delta() {
    assert_eq!(strtools::linecount("") - 1, 0);
    assert_eq!(strtools::linecount("hello\n") - 1, 1);
    assert_eq!(strtools::linecount("win\r\n") - 1, 1);
}

#[test]
fn test_ismultiline() {
    assert!(!strtools::ismultiline(""));
    assert!(!strtools::ismultiline("hello"));
    assert!(strtools::ismultiline("hello\n"));
    assert!(strtools::ismultiline("line1\nline2"));
}

#[test]
fn test_sloc_consistency() {
    let result = strtools::countlines("");
    assert_eq!(result.totl, 0);
    let result = strtools::countlines("x=1\n");
    assert_eq!(result.totl, 1);
    let result = strtools::countlines("\n\n");
    assert_eq!(result.totl, 2);
}
