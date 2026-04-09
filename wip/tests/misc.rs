// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's misc_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

// ============================================================================
// Misc Tests
// ============================================================================

use std::collections::abc::Mapping;

#[test]
fn test_import_mapping_from_collections_abc() {
    fn check_mapping<M: Mapping>() {}
    check_mapping::<std::collections::HashMap<String, i32>>();
}
