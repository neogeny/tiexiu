// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/safe_name_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::util::{pythonize_name, safe_name};

#[test]
fn test_safe_name_valid_cases() {
    assert_eq!(safe_name("valid_name", "_"), "valid_name");
    assert_eq!(safe_name("123invalid", "_"), "_123invalid");
    assert_eq!(safe_name("name-with-dash", "_"), "name_with_dash");
    assert_eq!(safe_name("name with space", "_"), "name_with_space");
    assert_eq!(safe_name("class", "_"), "class_");
    assert_eq!(safe_name("name!", "_"), "name_");
    assert_eq!(safe_name("_leading", "_"), "_leading");
    assert_eq!(safe_name("123", "_"), "_123");
    assert_eq!(safe_name("if", "_"), "if_");
    assert_eq!(safe_name("type", "_"), "type_");
    assert_eq!(safe_name("name@domain", "_"), "name_domain");
    assert_eq!(safe_name("a-b_c.d", "_"), "a_b_c_d");
}

#[test]
fn test_safe_name_unicode_cases() {
    assert_eq!(safe_name("naïve", "_"), "naïve");
    assert_eq!(safe_name("café", "_"), "café");
    assert_eq!(safe_name("Γαμμα", "_"), "Γαμμα");
}

#[test]
fn test_pythonize_name() {
    assert_eq!(pythonize_name(""), "");
    assert_eq!(pythonize_name("someName"), "some_name");
    assert_eq!(pythonize_name("SomeName"), "some_name");
    assert_eq!(pythonize_name("XMLHttpRequest"), "xml_http_request");
    assert_eq!(pythonize_name("some_Name"), "some__name");
    assert_eq!(pythonize_name("NAME"), "name");
}

#[test]
fn test_pythonize_name_unicode() {
    assert_eq!(pythonize_name("naïve"), "naïve");
    assert_eq!(pythonize_name("SomeΓαμμα"), "some_γαμμα");
}
