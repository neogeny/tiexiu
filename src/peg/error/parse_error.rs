// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Tree;
use crate::types::Str;
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

    /// Corresponds to Self::Eol
    #[error("expecting EOL")]
    ExpectingEol,

    /// Corresponds to Self::Token
    #[error("{0}")]
    ExpectedToken(Str),

    /// Corresponds to Self::Pattern
    #[error("/{0}/")]
    ExpectedPattern(String),

    /// Corresponds to Self::NegativeLookahead
    #[error("! not expecting: {0}")]
    NotExpecting(Str),

    /// Corresponds to Self::Choice fallback
    #[error("no viable option")]
    NoViableOption(Box<[Str]>),

    /// Corresponds is_keyword() validations
    #[error("'{0}' is a reserved word")]
    ReservedWord(Str),

    /// Corresponds memos that are Tree::Bottom
    #[error("Failed parsing '{0}'")]
    FailedParse(Str),

    /// Corresponds memos that are Tree::Bottom
    #[error("Failed parsing '{0}' start {1} end {2}")]
    FailedRecursion(Str, usize, usize, Box<Tree>),

    /// Corresponds rule names not in map
    #[error("Rule not found: '{0}'")]
    RuleNotFound(Str),

    /// Corresponds rule names without a Rule reference
    #[error("Rule not linked: '{0}'")]
    RuleNotLinked(Str),

    #[error("There are no rules in the grammar")]
    NoRulesInGrammar,
}

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CompileError {
    #[error("expected {0} to be a Tree::Node")]
    ExpectedNode(String),

    #[error("expected {0} to contain a Tree::Map")]
    ExpectedMap(String),

    #[error("expected {0} to be Tree::Text")]
    ExpectedText(&'static str),

    #[error("expected {0} to be Tree::List")]
    ExpectedList(String),

    #[error("expected {0} to be Tree::List or Tree::Nil")]
    ExpectedListOrNil(&'static str),

    #[error("expected {0} to be Tree::Text or Tree::Nil")]
    ExpectedTextOrNil(&'static str),

    #[error("expected {context} to contain key '{key}'")]
    MissingKey {
        context: String,
        key: &'static str,
        tree: Box<Tree>,
    },

    #[error("expected {0}")]
    ExpectedField(&'static str),

    #[error("expected {expected}, found '{found}'")]
    UnexpectedNodeName { expected: &'static str, found: Str },

    #[error("expected {expected}, found '{found}'")]
    UnexpectedTypeName { expected: Str, found: Str },

    #[error("{0} is not implemented")]
    NotImplemented(&'static str),

    #[error("Unknown expression type '{0}'")]
    UnknownExpressionType(Str),
}
