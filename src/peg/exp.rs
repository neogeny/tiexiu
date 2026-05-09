// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::rule::RuleRef;
use crate::cfg::types::{Define, Str};
use derivative::Derivative;
use std::fmt;
use std::sync::Arc;

pub type ERef = Box<Exp>;
pub type ERefArr = Box<[Exp]>;

#[derive(Derivative)]
#[derivative(Clone, Debug, Default)]
pub struct Exp {
    pub kind: ExpKind,
    #[derivative(Debug(format_with = "debug_none"))]
    pub la: Option<Arc<[Str]>>, // the lookahead set
    #[derivative(Debug(format_with = "debug_none"))]
    pub df: Option<Arc<[Define]>>, // the defines set
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

#[derive(Derivative)]
#[derivative(Clone, Debug, Default)]
pub enum ExpKind {
    #[derivative(Default)]
    Nil,
    Cut,
    Void,
    Fail,
    Dot,
    Eof,
    Eol,
    EmptyClosure,

    Call {
        name: Str,

        // NOTE
        //   This hacked field makes the structure recursive
        #[derivative(Debug(format_with = "debug_none"))]
        rule: Option<RuleRef>,
    },

    Token(Str),
    Pattern(Str),
    Constant(Str),
    Alert(Str, u8),

    Named(Str, ERef),
    NamedList(Str, ERef),
    Override(ERef),
    OverrideList(ERef),

    Group(ERef),
    SkipGroup(ERef),

    Lookahead(ERef),
    NegativeLookahead(ERef),
    SkipTo(ERef),

    Sequence(ERefArr),
    Choice(ERefArr),
    Alt(ERef),
    Optional(ERef),
    Closure(ERef),
    PositiveClosure(ERef),

    Join {
        exp: ERef,
        sep: ERef,
    },
    PositiveJoin {
        exp: ERef,
        sep: ERef,
    },
    Gather {
        exp: ERef,
        sep: ERef,
    },
    PositiveGather {
        exp: ERef,
        sep: ERef,
    },
    RuleInclude {
        name: Str,

        // NOTE
        //   No recursion, but a repetition that may be large
        // #[derivative(Debug(format_with = "debug_none"))]
        exp: Option<ERef>,
    },
}

#[cfg(test)]
mod tests {
    use crate::exp::*;
    use std::mem::size_of;

    const TARGET: usize = 64;

    #[test]
    fn test_exp_size() {
        let size = size_of::<Exp>();
        assert!(size <= TARGET, "Exp size is {} > {} bytes", size, TARGET);
    }
}
