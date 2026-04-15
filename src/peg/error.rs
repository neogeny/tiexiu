// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Corresponds to Self::Fail
    #[error("Fail")]
    Fail,

    /// Corresponds to Self::Dot (No more input)
    #[error("no more input")]
    NoMoreInput,

    /// Corresponds to Self::Eof
    #[error("expecting EOF/EOT")]
    ExpectingEof,

    /// Corresponds to Self::Token
    #[error("{0}")]
    ExpectedToken(Box<str>),

    /// Corresponds to Self::Pattern
    #[error("expected pattern: {0}")]
    ExpectedPattern(String),

    /// Corresponds to Self::NegativeLookahead
    #[error("!{0}")]
    NotExpecting(Box<str>),

    /// Corresponds to Self::Choice fallback
    #[error("no viable option")]
    NoViableOption(Box<[Box<str>]>),

    /// Corresponds memos that are Tree::Bottom
    #[error("Failed parsing '{0}'")]
    FailedParse(Box<str>),

    /// Corresponds rule names not in map
    #[error("Rule not found: '{0}'")]
    RuleNotFound(Box<str>),

    /// Corresponds rule names without a Rule reference
    #[error("Rule not linked: '{0}'")]
    RuleNotLinked(Box<str>),
    
    #[error("There are no rules in the grammar")]
    NoRulesInGrammar,
}
