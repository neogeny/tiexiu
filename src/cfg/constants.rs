// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Prefix for recognized environment variables
pub(crate) const ENV_PREFIX: &str = "TIEXIU";

// Maximum number of memos in Memo cache
pub(crate) const MEMO_CACHE_CAPACITY: usize = 8 * 1024;

/// Maximum allowed recursion depth for parsing.
pub const MAX_RECURSION_DEPTH: usize = 64;

/// Tag use for nodes in JSON versions of grammars
pub(crate) const TATSU_TYPE_TAG: &str = "__class__";

/// Path to the TatSu EBNF grammar file.
pub const PATH_TATSU_GRAMMAR_EBNF: &str = "grammar/tatsu.ebnf";
/// Path to the TatSu JSON grammar file.
pub const PATH_TATSU_GRAMMAR_JSON: &str = "grammar/tatsu.json";
/// Path to the Calc JSON grammar file.
pub const PATH_CALC_GRAMMAR_JSON: &str = "grammar/calc.json";
/// Path to the Calc EBNF grammar file.
pub const PATH_CALC_GRAMMAR_EBNF: &str = "grammar/calc.ebnf";

// Grammar directives (from TatSu)
pub(crate) const STR_GRAMMAR_NAME: &str = "grammar";
pub(crate) const STR_WHITESPACE: &str = "whitespace";
pub(crate) const STR_COMMENTS: &str = "comments";
pub(crate) const STR_EOL_COMMENTS: &str = "eol_comments";
pub(crate) const STR_IGNORECASE: &str = "ignorecase";
pub(crate) const STR_LEFTREC: &str = "left_recursion";
pub(crate) const STR_PARSEINFO: &str = "parseinfo";
pub(crate) const STR_MEMOIZATION: &str = "memoization";
pub(crate) const STR_NAMECHARS: &str = "namechars";
pub(crate) const STR_NAMEGUARD: &str = "nameguard";
pub(crate) const STR_KEYWORD: &str = "keyword";
pub(crate) const STR_START: &str = "start";
pub(crate) const STR_SOURCE: &str = "source";
pub(crate) const STR_FILENAME: &str = "filename";

pub(crate) const STR_TRACE: &str = "trace";
pub(crate) const STR_DEBUG: &str = "debug";
pub(crate) const STR_VERBOSE: &str = "verbose";

// Symbols

pub(crate) const SYM_ETX: &str = "＄";
pub(crate) const SYM_EOL: &str = "⏎";
