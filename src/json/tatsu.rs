// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "__class__")]
pub enum TatSuModel {
    Grammar {
        name: String,
        rules: Vec<TatSuModel>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        directives: std::collections::HashMap<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        keywords: Vec<String>,
    },
    Rule {
        name: String,
        exp: Box<TatSuModel>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_name: bool,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_lrec: bool,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_memo: bool,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_tokn: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        params: Vec<String>,
    },
    Sequence {
        sequence: Vec<TatSuModel>,
    },
    Choice {
        options: Vec<TatSuModel>,
    },
    Option {
        exp: Box<TatSuModel>,
    },
    Group {
        exp: Box<TatSuModel>,
    },
    Token {
        token: String,
    },
    Pattern {
        pattern: String,
    },
    Constant {
        value: serde_json::Value,
    },
    Call {
        name: String,
    },
    Void {
        exp: Box<TatSuModel>,
    },
    Cut,
    EOF,
    Optional {
        exp: Box<TatSuModel>,
    },
    Closure {
        exp: Box<TatSuModel>,
    },
    PositiveClosure {
        exp: Box<TatSuModel>,
    },
    Gather {
        exp: Box<TatSuModel>,
        joiner: Box<TatSuModel>,
    },
    PositiveGather {
        exp: Box<TatSuModel>,
        joiner: Box<TatSuModel>,
    },
    LeftJoin {
        exp: Box<TatSuModel>,
        joiner: Box<TatSuModel>,
    },
    RightJoin {
        exp: Box<TatSuModel>,
        joiner: Box<TatSuModel>,
    },
    PositiveLookahead {
        exp: Box<TatSuModel>,
    },
    NegativeLookahead {
        exp: Box<TatSuModel>,
    },
    Named {
        name: String,
        exp: Box<TatSuModel>,
    },
    NamedList {
        name: String,
        exp: Box<TatSuModel>,
    },
    Override {
        exp: Box<TatSuModel>,
    },
    SkipTo {
        exp: Box<TatSuModel>,
    },
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_tatsumodel_round_trip() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("grammar");
        path.push("calc.json");

        let original_json = fs::read_to_string(&path)
            .expect("Unable to read original grammar file");

        // 1. First Eat: JSON -> Rust
        let first_model: TatSuModel = serde_json::from_str(&original_json)
            .expect("First deserialization failed: Original JSON is missing exhaustive fields");

        // 2. The Spit: Rust -> JSON String
        // We use to_string_pretty to make any diffing easier if it fails
        let serialized_json = serde_json::to_string_pretty(&first_model)
            .expect("Serialization failed");

        // 3. Second Eat: New JSON -> New Rust
        let second_model: TatSuModel = serde_json::from_str(&serialized_json)
            .expect("Second deserialization failed: Rust 'spit' something it couldn't 'eat'!");

        // 4. The Final Bite: Compare the two Rust structures
        // Note: This requires #[derive(PartialEq)] on TatSuModel and its children
        assert_eq!(first_model, second_model, "Models differ after round-trip!");

        println!("The snake bit its tail! Round-trip successful.");
    }
}