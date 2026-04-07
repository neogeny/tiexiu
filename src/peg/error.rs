// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Corresponds to Self::Fail
    #[error("Fail")]
    Fail,

    /// Corresponds to Self::Dot (No more input)
    #[error("No more input")]
    NoMoreInput,

    /// Corresponds to Self::Eof
    #[error("Expecting EOF/EOT")]
    ExpectingEof,

    /// Corresponds to Self::Token
    #[error("Expecting '{0}'")]
    ExpectedToken(Box<str>),

    /// Corresponds to Self::Pattern
    #[error("Expecting '{0}'")]
    ExpectedPattern(Box<str>),

    /// Corresponds to Self::NegativeLookahead
    #[error("Not expecting: ???")]
    UnexpectedLookahead,

    /// Corresponds to Self::Choice fallback
    #[error("No viable option")]
    NoViableOption(Box<[Box<str>]>),

    /// Corresponds memos that are Tree::Bottom
    #[error("Failed parsing '{0}'")]
    FailedParse(Box<str>),

    /// Corresponds rule names not in map
    #[error("Rule not found: '{0}'")]
    RuleNotFound(Box<str>),
}
