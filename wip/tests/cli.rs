// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's cli_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("tatsu")
        .arg("--help")
        .output()
        .expect("Failed to run tatsu");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let pattern = regex::Regex::new(r"(?ms)竜TatSu takes a grammar .*GRAMMAR").unwrap();
    assert!(pattern.is_match(&stdout));
}

#[test]
fn test_cli_python() {
    let output = Command::new("tatsu")
        .arg("./grammar/tatsu.tatsu")
        .output()
        .expect("Failed to run tatsu");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let pattern = regex::Regex::new(r"(?ms)CAVEAT UTILITOR.*?竜TatSu.*?KEYWORDS = \(").unwrap();
    assert!(pattern.is_match(&stdout));
}

#[test]
fn test_cli_model() {
    let output = Command::new("tatsu")
        .arg("-g")
        .arg("./grammar/tatsu.tatsu")
        .output()
        .expect("Failed to run tatsu");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let pattern = regex::Regex::new(r"(?ms)CAVEAT UTILITOR.*?竜TatSu").unwrap();
    assert!(pattern.is_match(&stdout));
}
