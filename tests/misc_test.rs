// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Misc tests

use tiexiu::Result;

#[test]
fn test_mapping() -> Result<()> {
    use std::collections::HashMap;
    fn _check_mapping<V>(_: &HashMap<String, V>) {}
    Ok(())
}
