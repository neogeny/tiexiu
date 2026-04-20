// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use super::parser::{Nope, ParseResult, Parser, Succ};
use super::rule::RuleRef;
use crate::state::Ctx;
use crate::trees::tree::Define;
use crate::trees::{Str, Tree};
use crate::util::pyre;
use derivative::Derivative;
use std::fmt;
use std::ops::Deref;

pub type ERef = Box<Exp>;
pub type ERefArr = Box<[Exp]>;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct Exp {
    pub kind: ExpKind,
    pub la: Box<[Str]>,    // the lookahead set
    pub df: Box<[Define]>, // the defines set
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
#[derivative(Debug)]
#[derive(Clone)]
pub enum ExpKind {
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
        #[derivative(Debug(format_with = "debug_none"))]
        exp: Option<ERef>,
    },
}

impl<C: Ctx> Parser<C> for Exp {
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.parse(ctx)
    }
}

impl Exp {
    pub fn initialize_caches(&mut self) {
        self.cache_lookahead();
        self.cache_defines();
    }

    pub fn lookahead_str(&self) -> Box<str> {
        self.la
            .iter()
            .map(|s| &**s) // Deref Box<str> to &str
            .collect::<Vec<_>>() // Join needs a slice
            .join(" ")
            .into_boxed_str()
    }

    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        match self.do_parse(ctx) {
            Err(err) => Err(err),
            Ok(Succ(ctx, mut tree)) => {
                tree.define(&self.df);
                Ok(Succ(ctx, tree))
            }
        }
    }

    // #[track_caller]
    fn do_parse<C: Ctx>(&self, mut ctx: C) -> ParseResult<C> {
        let start = ctx.mark();
        let was_cut = ctx.cut_seen();
        match &self.kind {
            ExpKind::EmptyClosure => Ok(Succ(ctx, Tree::from(vec![]).closed())),
            ExpKind::Nil => Ok(Succ(ctx, Tree::Nil)),
            ExpKind::RuleInclude { name, exp } => match exp {
                None => Err(ctx.failure(start, ParseError::RuleNotLinked(name.clone()))),
                Some(exp) => exp.parse(ctx),
            },
            ExpKind::Call { name, rule } => match rule {
                None => Err(ctx.failure(start, ParseError::RuleNotLinked(name.clone()))),
                Some(rule) => match ctx.call(name, rule.as_ref()) {
                    Ok(Succ(mut ctx, tree)) => {
                        ctx.restore_cut(was_cut);
                        Ok(Succ(ctx, tree))
                    }
                    Err(mut f) => {
                        f.take_cut();
                        Err(f)
                    }
                },
            },
            ExpKind::Cut => {
                ctx.setcut();
                Ok(Succ(ctx, Tree::Nil))
            }
            ExpKind::Void => {
                ctx.match_void();
                Ok(Succ(ctx, Tree::Nil))
            }
            ExpKind::Fail => Err(ctx.failure(start, ParseError::Fail)),
            ExpKind::Dot => {
                if let Some(c) = ctx.next() {
                    Ok(Succ(ctx, Tree::text(c.to_string().as_str())))
                } else {
                    Err(ctx.failure(start, ParseError::NoMoreInput))
                }
            }
            ExpKind::Eol => {
                if ctx.match_eol() {
                    Ok(Succ(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure(start, ParseError::ExpectingEol))
                }
            }
            ExpKind::Eof => {
                if ctx.eof_check() {
                    Ok(Succ(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure(start, ParseError::ExpectingEof))
                }
            }

            ExpKind::Token(token) => {
                if ctx.match_token(token) {
                    Ok(Succ(ctx, Tree::Text(token.deref().into())))
                } else {
                    Err(ctx.failure(start, ParseError::ExpectedToken(token.deref().into())))
                }
            }
            ExpKind::Pattern(pattern) => {
                if let Some(matched) = ctx.match_pattern(pattern) {
                    Ok(Succ(ctx, Tree::Text(matched.into())))
                } else {
                    Err(ctx.failure(
                        start,
                        ParseError::ExpectedPattern(
                            pyre::truncate_pattern(pattern, 16).to_string(),
                        ),
                    ))
                }
            }
            ExpKind::Constant(literal) => Ok(Succ(ctx, Tree::Text(literal.deref().into()))),
            ExpKind::Alert(literal, _) => Ok(Succ(ctx, Tree::Text(literal.deref().into()))),

            ExpKind::Named(name, exp) => match exp.parse(ctx) {
                Ok(Succ(ctx, mut tree)) => {
                    tree = Tree::named(name, tree);
                    Ok(Succ(ctx, tree))
                }
                err => err,
            },
            ExpKind::NamedList(name, exp) => match exp.parse(ctx) {
                Ok(Succ(ctx, mut tree)) => {
                    tree = Tree::named_as_list(name, tree);
                    Ok(Succ(ctx, tree))
                }
                err => err,
            },
            ExpKind::Override(exp) => match exp.parse(ctx) {
                Ok(Succ(ctx, mut tree)) => {
                    tree = Tree::override_with(tree);
                    Ok(Succ(ctx, tree))
                }
                err => err,
            },
            ExpKind::OverrideList(exp) => match exp.parse(ctx) {
                Ok(Succ(ctx, mut tree)) => {
                    tree = Tree::override_as_list(tree);
                    Ok(Succ(ctx, tree))
                }
                err => err,
            },
            ExpKind::Group(exp) => exp.parse(ctx),
            ExpKind::SkipGroup(exp) => {
                let Succ(new_ctx, _) = exp.parse(ctx)?;
                Ok(Succ(new_ctx, Tree::Nil))
            }
            ExpKind::Lookahead(exp) => {
                let _ = exp.parse(ctx.clone())?;
                Ok(Succ(ctx, Tree::Nil))
            }
            ExpKind::NegativeLookahead(exp) => {
                if let Ok(Succ(_, _)) = exp.parse(ctx.clone()) {
                    Err(ctx.failure(start, ParseError::NotExpecting(self.lookahead_str())))
                } else {
                    Ok(Succ(ctx, Tree::Nil))
                }
            }
            ExpKind::SkipTo(exp) => loop {
                match exp.parse(ctx.clone()) {
                    Err(f) => {
                        if !ctx.dot() {
                            return Err(f);
                        }
                    }
                    ok => break ok,
                }
            },

            ExpKind::Sequence(sequence) => {
                let mut results = Vec::new();
                for exp in sequence.iter() {
                    match exp.parse(ctx) {
                        Ok(Succ(new_ctx, tree)) => {
                            results.push(tree);
                            ctx = new_ctx;
                        }
                        err => return err,
                    }
                }
                Ok(Succ(ctx, Tree::from(results)))
            }
            ExpKind::Alt(exp) => exp.parse(ctx),
            ExpKind::Choice(options) => {
                let mut furthest: Option<Nope> = None;
                for option in options.iter() {
                    ctx.uncut();
                    match option.parse(ctx.clone()) {
                        Ok(Succ(mut new_ctx, tree)) => {
                            new_ctx.restore_cut(was_cut);
                            return Ok(Succ(new_ctx, tree));
                        }
                        Err(mut f) => {
                            if f.take_cut() {
                                return Err(f);
                            }

                            if furthest.as_ref().is_none_or(|prev| f.start >= prev.start) {
                                furthest = Some(f);
                            }
                        }
                    }
                }
                Err(furthest
                    .unwrap_or(ctx.failure(start, ParseError::NoViableOption(self.la.clone()))))
            }

            ExpKind::Optional(exp) => {
                ctx.uncut();
                match exp.parse(ctx.clone()) {
                    Ok(Succ(mut new_ctx, tree)) => {
                        new_ctx.restore_cut(was_cut);
                        Ok(Succ(new_ctx, tree))
                    }
                    Err(mut f) => {
                        if f.take_cut() {
                            return Err(f);
                        }
                        ctx.restore_cut(was_cut);
                        Ok(Succ(ctx, Tree::Nil))
                    }
                }
            }

            ExpKind::Closure(exp) => {
                let mut res = Vec::new();
                match Self::repeat(ctx, exp, &mut res) {
                    Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res).closed())),
                    err => err,
                }
            }
            ExpKind::PositiveClosure(exp) => {
                let mut res: Vec<Tree> = Vec::new();
                match exp.parse(ctx) {
                    Ok(Succ(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                match Self::repeat(ctx, exp, &mut res) {
                    Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res).closed())),
                    err => err,
                }
            }
            ExpKind::Join { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => match Self::repeat_with_pre(new_ctx, exp, sep, &mut res, true) {
                        Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res).closed())),
                        err => err,
                    },
                    Err((_actx, f)) => Err(f),
                }
            }
            ExpKind::PositiveJoin { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(Succ(new_ctx, tree)) => {
                        res.push(tree);
                        ctx = new_ctx;
                    }
                    err => return err,
                };

                match Self::repeat_with_pre(ctx, exp, sep, &mut res, true) {
                    Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res).closed())),
                    err => err,
                }
            }
            ExpKind::Gather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();
                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => {
                        match Self::repeat_with_pre(new_ctx, exp, sep, &mut res, false) {
                            Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res).closed())),
                            err => err,
                        }
                    }
                    Err((_actx, f)) => Err(f),
                }
            }
            ExpKind::PositiveGather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(Succ(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                match Self::repeat_with_pre(ctx, exp, sep, &mut res, false) {
                    Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res).closed())),
                    err => err,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::StrCursor;
    use crate::peg::Rule;
    use crate::state::prelude::*;
    use crate::state::strctx::StrCtx;

    const TARGET: usize = 64;

    #[test]
    fn test_exp_size() {
        let size = size_of::<Exp>();
        assert!(size <= TARGET, "Exp size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn choice_keeps_furthest_failure() {
        let token_a = Exp::token("a");
        let cursor = StrCursor::new("a");
        println!("cursor on 'a': {:?}", cursor);
        let result_a = token_a.parse(StrCtx::new(cursor, &[]));
        println!("token('a') on 'a': {:?}", result_a);
        assert!(result_a.is_ok(), "token('a') should match 'a'");

        let exp1 = Exp::sequence(vec![Exp::token("a"), Exp::token("b")]);
        let result1_ok = exp1.parse(StrCtx::new(StrCursor::new("a b"), &[]));
        println!("exp1 on 'a b': {:?}", result1_ok);
        assert!(result1_ok.is_ok(), "sequence 1 should succeed on 'a b'");

        let result1_err = exp1.parse(StrCtx::new(StrCursor::new("a c x"), &[]));
        println!("exp1 on 'a c x': {:?}", result1_err);
        let err1 = result1_err.unwrap_err();
        assert_eq!(err1.mark, 2);
        assert_eq!(err1.source, ParseError::ExpectedToken("b".into()).into());

        let exp2 = Exp::sequence(vec![Exp::token("a"), Exp::token("c"), Exp::token("d")]);
        let result2_ok = exp2.parse(StrCtx::new(StrCursor::new("a c d"), &[]));
        println!("exp2 on 'a c d': {:?}", result2_ok);
        assert!(result2_ok.is_ok(), "sequence 2 should succeed on 'a c d'");

        let result2_err = exp2.parse(StrCtx::new(StrCursor::new("a c x"), &[]));
        println!("exp2 on 'a c x': {:?}", result2_err);
        let err2 = result2_err.unwrap_err();
        assert_eq!(err2.mark, 4);
        assert_eq!(err2.source, ParseError::ExpectedToken("d".into()).into());

        let exp = Exp::choice(vec![exp1, exp2]);
        let ctx = StrCtx::new(StrCursor::new("a c x"), &[]);

        let result = exp.parse(ctx);
        println!("choice on 'a c x': {:?}", result);
        let err = result.unwrap_err();

        assert_eq!(err.mark, 4);
        assert_eq!(err.source, ParseError::ExpectedToken("d".into()).into());
    }

    #[test]
    fn choice_restores_entered_cut_on_success() {
        let grammar = crate::peg::Grammar::new(
            "test",
            &[RuleRef::from(Rule::new("start", &[], Exp::token("abc")))],
        );
        let _ = grammar;
        let mut ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx.setcut();
        assert!(ctx.cut_seen(), "ctx should have cut set before choice");

        let exp = Exp::choice(vec![Exp::token("abc"), Exp::token("xyz")]);
        let result = exp.parse(ctx);
        assert!(result.is_ok(), "choice should succeed");
        let succ = result.unwrap();
        assert!(
            succ.0.cut_seen(),
            "cut should be restored after choice success"
        );
    }

    #[test]
    fn choice_returns_err_when_all_options_fail() {
        let grammar = crate::peg::Grammar::new(
            "test",
            &[RuleRef::from(Rule::new("start", &[], Exp::token("xyz")))],
        );
        let _ = grammar;
        let mut ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx.setcut();
        assert!(ctx.cut_seen(), "ctx should have cut set before choice");

        let exp = Exp::choice(vec![Exp::token("xyz"), Exp::token("123")]);
        let result = exp.parse(ctx);
        assert!(
            result.is_err(),
            "choice should return Err when all options fail"
        );
    }

    #[test]
    fn choice_clears_when_no_cut_enters() {
        let grammar = crate::peg::Grammar::new(
            "test",
            &[RuleRef::from(Rule::new("start", &[], Exp::token("abc")))],
        );
        let _ = grammar;
        let ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        assert!(!ctx.cut_seen(), "ctx should not have cut set");

        let exp = Exp::choice(vec![Exp::token("abc"), Exp::token("xyz")]);
        let result = exp.parse(ctx);
        assert!(result.is_ok(), "choice should succeed");
        let succ = result.unwrap();
        assert!(
            !succ.0.cut_seen(),
            "cut should be cleared when not set on entry"
        );
    }

    #[test]
    fn optional_restores_entered_cut_on_success() {
        let grammar = crate::peg::Grammar::new(
            "test",
            &[RuleRef::from(Rule::new("start", &[], Exp::token("abc")))],
        );
        let _ = grammar;
        let mut ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx.setcut();
        assert!(ctx.cut_seen(), "ctx should have cut set before optional");

        let exp = Exp::optional(Exp::token("abc"));
        let result = exp.parse(ctx);
        assert!(result.is_ok(), "optional should succeed");
        let succ = result.unwrap();
        assert!(
            succ.0.cut_seen(),
            "cut should be restored after optional success"
        );
    }

    #[test]
    fn optional_restores_entered_cut_on_failure() {
        let grammar =
            crate::peg::Grammar::new("test", &[Rule::new("start", &[], Exp::token("xyz")).into()]);
        let _ = grammar;
        let mut ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx.setcut();
        assert!(ctx.cut_seen(), "ctx should have cut set before optional");

        let exp = Exp::optional(Exp::token("xyz"));
        let result = exp.parse(ctx);
        assert!(result.is_ok(), "optional failure returns Ok with nil");
        let succ = result.unwrap();
        assert!(
            succ.0.cut_seen(),
            "cut should be restored after optional failure"
        );
    }

    #[test]
    fn optional_clears_when_no_cut_enters() {
        let grammar =
            crate::peg::Grammar::new("test", &[Rule::new("start", &[], Exp::token("abc")).into()]);
        let _ = grammar;
        let ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        assert!(!ctx.cut_seen(), "ctx should not have cut set");

        let exp = Exp::optional(Exp::token("abc"));
        let result = exp.parse(ctx);
        assert!(result.is_ok(), "optional should succeed");
        let succ = result.unwrap();
        assert!(
            !succ.0.cut_seen(),
            "cut should be cleared when not set on entry"
        );
    }
}
