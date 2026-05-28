// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Tree;
use crate::types::Str;
use crate::util::ensure::Ensure;
use thiserror::Error;

/// A result type for grammar compilation.
pub type CompileResult<T> = Result<T, CompileError>;

impl From<Ensure> for ParseFailure {
    fn from(err: Ensure) -> Self {
        ParseFailure::Ensure(err.condition)
    }
}

/// Errors that can occur during parsing.
#[derive(Error, Default, Debug, Clone, PartialEq)]
pub enum ParseFailure {
    /// Corresponds to Self::Fail
    #[error("Fail")]
    #[default]
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
    #[error("Expecting: '{0}'")]
    ExpectedToken(Str),

    /// Corresponds to Self::Pattern
    #[error("Expecting: /{0}/")]
    ExpectedPattern(String),

    /// Corresponds to Self::NegativeLookahead
    #[error("! not expecting: {0}")]
    NotExpecting(Str),

    /// Corresponds to Self::Choice fallback
    #[error("Expecting: {0:#?}")]
    NoViableOption(Str),

    /// Corresponds is_keyword() validations
    #[error("'{0}' is a reserved word")]
    ReservedWord(Str),

    /// Corresponds memos that are Tree::Bottom
    #[error("Failed parsing '{0}'")]
    FailedParse(Str),

    /// Corresponds memos that are Tree::Bottom
    #[error("Failed parsing '{0}' start {1} end {2}")]
    FailedRecursion(Str, usize, usize, Box<Tree>),

    /// Corresponds memos that are Tree::Bottom
    #[error("UNBOUND LEFT RECURSION OF {0} AT {1}@{2}")]
    UnboundLeftRecursion(usize, Str, usize),

    /// Corresponds memos that are Tree::Bottom
    #[error("Closure matched Void")]
    ClosureMatchedVoid(),

    /// Corresponds rule names not in map
    #[error("Rule not found: '{0}'")]
    RuleNotFound(Str),

    /// Corresponds rule names without a Rule reference
    #[error("Rule not linked: '{0}'")]
    RuleNotLinked(Str),

    #[error("There are no rules in the grammar")]
    NoRulesInGrammar,

    #[error("Alt not captured by a choice")]
    AltWithNoChoice,

    #[error("Cut not captured by a Sequence")]
    CutWithNoSequence,

    #[error("Choice without Alt")]
    ChoiceOptionWithNoAlt,

    #[error("!({0})")]
    Ensure(&'static str),

    /// Left recursion is disabled but the grammar has left-recursive rules.
    #[error("left recursion is disabled but the grammar has left-recursive rules")]
    LeftRecursionDisabled,
}

/// Errors that can occur during grammar compilation/linking.
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

    #[error("Linker error: {0}")]
    Linker(String),
}

impl From<ParseFailure> for CompileError {
    fn from(err: ParseFailure) -> Self {
        CompileError::Linker(err.to_string())
    }
}
