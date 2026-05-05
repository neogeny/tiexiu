// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub const TATSU_TYPE_TAG: &str = "__class__";
pub const ENV_PREFIX: &str = "TIEXIU";

// Paths for unit tests

pub const PATH_TATSU_GRAMMAR_EBNF: &str = "grammar/tatsu.ebnf";
pub const PATH_TATSU_GRAMMAR_JSON: &str = "grammar/tatsu.json";
pub const PATH_CALC_GRAMMAR_JSON: &str = "grammar/calc.json";
pub const PATH_CALC_GRAMMAR_EBNF: &str = "grammar/calc.ebnf";

// Grammar directives (from TatSu)
pub const STR_GRAMMAR_NAME: &str = "grammar";
pub const STR_WHITESPACE: &str = "whitespace";
pub const STR_COMMENTS: &str = "comments";
pub const STR_EOL_COMMENTS: &str = "eol_comments";
pub const STR_IGNORECASE: &str = "ignorecase";
pub const STR_LEFTREC: &str = "left_recursion";
pub const STR_PARSEINFO: &str = "parseinfo";
pub const STR_MEMOIZATION: &str = "memoization";
pub const STR_NAMECHARS: &str = "namechars";
pub const STR_NAMEGUARD: &str = "nameguard";
pub const STR_KEYWORD: &str = "keyword";
pub const STR_START: &str = "start";

// Symbols

pub const SYM_ETX: &str = "＄";
pub const SYM_EOL: &str = "⏎";
