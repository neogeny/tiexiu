// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's util/safe_name_test.py

use tiexiu::Result;
use tiexiu::util::{safe_name, to_snake_case};

#[test]
fn test_safe_name_valid_cases() -> Result<()> {
    let cases = [
        ("valid_name", "_", "valid_name"),
        ("123invalid", "_", "_123invalid"),
        ("name-with-dash", "_", "name_with_dash"),
        ("name with space", "_", "name_with_space"),
        ("let", "_", "let_"),
        ("fn", "_", "fn_"),
        ("mut", "_", "mut_"),
        ("pub", "_", "pub_"),
        ("use", "_", "use_"),
    ];

    for (name, plug, expected) in cases {
        let result = safe_name(name, plug)?;
        assert_eq!(
            result, expected,
            "safe_name({}, {}) = {} expected {}",
            name, plug, result, expected
        );
    }
    Ok(())
}

#[test]
fn test_safe_name_unicode_cases() -> Result<()> {
    let cases = [("name", "_", "name")];

    for (name, plug, expected) in cases {
        let result = safe_name(name, plug)?;
        assert_eq!(result, expected, "safe_name({}, {})", name, plug);
    }
    Ok(())
}

#[test]
fn test_to_snake_case() -> Result<()> {
    let cases = [
        ("someName", "some_name"),
        ("SomeName", "some_name"),
        ("XMLHttpRequest", "xml_http_request"),
        ("some_Name", "some__name"),
        ("NAME", "name"),
    ];

    for (name, expected) in cases {
        let result = to_snake_case(name)?;
        assert_eq!(result, expected, "to_snake_case({})", name);
    }
    Ok(())
}
