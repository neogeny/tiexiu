// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::rule::RuleRef;
use crate::cfg::types::{Define, Str};
use derivative::Derivative;
use std::fmt;
use std::sync::Arc;

/// A heap-allocated PEG expression.
pub type ERef = Box<Exp>;
/// A boxed slice of PEG expressions (e.g. for Choice/Sequence children).
pub type ERefArr = Box<[Exp]>;

/// A PEG parsing expression with its kind, lookahead, and defines sets.
#[derive(Derivative)]
#[derivative(Clone, Debug, Default)]
pub struct Exp {
    /// The kind of expression (Nil, Call, Sequence, Choice, etc.).
    pub kind: ExpKind,
    /// Cached FIRST/LOOKAHEAD set (computed during analysis).
    #[derivative(Debug(format_with = "debug_none"))]
    pub la: Option<Arc<[Str]>>,
    /// Set of defines collected for this expression.
    #[derivative(Debug(format_with = "debug_none"))]
    pub df: Option<Arc<[Define]>>,
}

// NOTE
//  For output to reconstruct, Exp.kind cannot be hidden
// impl Debug for Exp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         Debug::fmt(&self.kind, f)
//     }
// }

fn debug_none<T>(_field: &T, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "None")
}

impl Exp {}

/// The kind of the PEG expression, encoding all grammar operators.
#[derive(Derivative)]
#[derivative(Clone, Debug, Default)]
pub enum ExpKind {
    /// No expression (default).
    #[derivative(Default)]
    Nil,
    /// The cut operator (!).
    Cut,
    /// Matches nothing, produces nothing.
    Void,
    /// Always fails.
    Fail,
    /// Matches any single character.
    Dot,
    /// Matches end of input.
    Eof,
    /// Matches end of line.
    Eol,
    /// A closure that matched empty — used for error reporting.
    EmptyClosure,
    /// A call to another rule by name.
    Call {
        name: Str,
        #[derivative(Debug(format_with = "debug_none"))]
        rule: Option<RuleRef>,
    },
    /// Match an exact token string.
    Token(Str),
    /// Match a regex pattern.
    Pattern(Str),
    /// Match a constant value.
    Constant(Str),
    /// Report a parsing alert with severity.
    Alert(Str, u8),
    /// Name the result of an expression.
    Named(Str, ERef),
    /// Name the result as a list.
    NamedList(Str, ERef),
    /// Override the result with a new tree.
    Override(ERef),
    /// Override the result as a list.
    OverrideList(ERef),
    /// Group an expression.
    Group(ERef),
    /// Skip grouping (no node wrapping).
    SkipGroup(ERef),
    /// Positive lookahead (&exp).
    Lookahead(ERef),
    /// Negative lookahead (!exp).
    NegativeLookahead(ERef),
    /// Skip until exp matches.
    SkipTo(ERef),
    /// Sequence of expressions (concatenation).
    Sequence(ERefArr),
    /// Ordered choice (prioritized alternatives).
    Choice(ERefArr),
    /// Alternative within a choice (/).
    Alt(ERef),
    /// Optional expression (?).
    Optional(ERef),
    /// Kleene closure (*).
    Closure(ERef),
    /// Positive closure (+).
    PositiveClosure(ERef),
    /// Join expression with separator.
    Join { exp: ERef, sep: ERef },
    /// Positive join (one or more).
    PositiveJoin { exp: ERef, sep: ERef },
    /// Gather expression (left-fold with separator).
    Gather { exp: ERef, sep: ERef },
    /// Positive gather (one or more).
    PositiveGather { exp: ERef, sep: ERef },
    /// Include rules from another grammar.
    RuleInclude { name: Str, exp: Option<ERef> },
    /// Match a name/identifier (like `@name`).
    NameMeta,
    /// Match a signed integer (like `@int`).
    IntMeta,
    /// Match an unsigned integer (like `@uint`).
    UIntMeta,
    /// Match a floating-point literal (like `@float`).
    FloatMeta,
    /// Match a boolean literal (like `@bool`).
    BoolMeta,
}

#[cfg(test)]
mod tests {
    use crate::exp::*;
    use std::mem::size_of;

    const TARGET: usize = 80;

    #[test]
    fn test_exp_size() {
        let size = size_of::<Exp>();
        assert!(size <= TARGET, "Exp size is {} > {} bytes", size, TARGET);
    }
}
