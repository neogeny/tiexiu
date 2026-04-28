// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::api::error::nope::ParseResult;
use crate::engine::Ctx;
use crate::peg::error::nope::Yeap;
use crate::peg::{Exp, ExpKind, ParseError, Parser};
use crate::trees::Tree;
use crate::types::Str;
use crate::util::pyre;

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

    pub fn lookahead_str(&self) -> Str {
        self.la
            .as_ref()
            .map(|la| la.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(" "))
            .unwrap_or_default()
            .into()
    }

    // #[track_caller]
    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        match self.do_parse(ctx) {
            Err(err) => Err(err),
            Ok(Yeap(ctx, mut tree)) => {
                if let Some(df) = self.df.as_ref() {
                    tree.define(df);
                }
                Ok(Yeap(ctx, tree))
            }
        }
    }

    fn do_parse<C: Ctx>(&self, mut ctx: C) -> ParseResult<C> {
        let start = ctx.mark();
        match &self.kind {
            ExpKind::EmptyClosure => Ok(Yeap(ctx, Tree::from(vec![]).closed())),
            ExpKind::Nil => Ok(Yeap(ctx, Tree::Nil)),
            ExpKind::RuleInclude { name, exp } => match exp {
                None => {
                    // Err(ctx.failure(start, ParseError::RuleNotLinked(name.clone())))
                    panic!("Unlinked rule {name}")
                }
                Some(exp) => exp.parse(ctx),
            },
            ExpKind::Call { name, rule } => match rule {
                None => Err(ctx.failure(start, ParseError::RuleNotLinked(name.clone()))),
                Some(rule) => match ctx.call(name, rule.as_ref()) {
                    Ok(ok) => Ok(ok),
                    Err(mut nope) => {
                        nope.take_cut();
                        Err(nope)
                    }
                },
            },
            ExpKind::Cut => {
                ctx.cut();
                Ok(Yeap(ctx, Tree::Nil))
            }
            ExpKind::Void => {
                ctx.match_void();
                Ok(Yeap(ctx, Tree::Nil))
            }
            ExpKind::Fail => Err(ctx.failure(start, ParseError::Fail)),
            ExpKind::Dot => {
                if ctx.next() {
                    Ok(Yeap(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure(start, ParseError::NoMoreInput))
                }
            }
            ExpKind::Eol => {
                if ctx.match_eol() {
                    Ok(Yeap(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure(start, ParseError::ExpectingEol))
                }
            }
            ExpKind::Eof => {
                if ctx.parse_eof() {
                    Ok(Yeap(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure(start, ParseError::ExpectingEof))
                }
            }

            ExpKind::Token(token) => {
                if ctx.match_token(token) {
                    Ok(Yeap(ctx, Tree::Text(token.clone())))
                } else {
                    Err(ctx.failure(start, ParseError::ExpectedToken(token.clone())))
                }
            }
            ExpKind::Pattern(pattern) => {
                if let Some(matched) = ctx.match_pattern(pattern) {
                    Ok(Yeap(ctx, Tree::Text(matched.clone())))
                } else {
                    Err(ctx.failure(
                        start,
                        ParseError::ExpectedPattern(pyre::truncate_pattern(pattern, 16).into()),
                    ))
                }
            }
            ExpKind::Constant(literal) => Ok(Yeap(ctx, Tree::Text(literal.clone()))),
            ExpKind::Alert(literal, _) => Ok(Yeap(ctx, Tree::Text(literal.clone()))),

            ExpKind::Named(name, exp) => match exp.parse(ctx) {
                Ok(Yeap(ctx, mut tree)) => {
                    tree = Tree::named(name, tree);
                    Ok(Yeap(ctx, tree))
                }
                err => err,
            },
            ExpKind::NamedList(name, exp) => match exp.parse(ctx) {
                Ok(Yeap(ctx, mut tree)) => {
                    tree = Tree::named_as_list(name, tree);
                    Ok(Yeap(ctx, tree))
                }
                err => err,
            },
            ExpKind::Override(exp) => match exp.parse(ctx) {
                Ok(Yeap(ctx, mut tree)) => {
                    tree = Tree::override_with(tree);
                    Ok(Yeap(ctx, tree))
                }
                err => err,
            },
            ExpKind::OverrideList(exp) => match exp.parse(ctx) {
                Ok(Yeap(ctx, mut tree)) => {
                    tree = Tree::override_as_list(tree);
                    Ok(Yeap(ctx, tree))
                }
                err => err,
            },
            ExpKind::Group(exp) => exp.parse(ctx),
            ExpKind::SkipGroup(exp) => {
                let Yeap(new_ctx, _) = exp.parse(ctx)?;
                Ok(Yeap(new_ctx, Tree::Nil))
            }
            ExpKind::Lookahead(exp) => match exp.parse(ctx.push()) {
                Ok(Yeap(_, _)) => Ok(Yeap(ctx, Tree::Nil)),
                Err(nope) => Err(nope),
            },
            ExpKind::NegativeLookahead(exp) => {
                if let Ok(Yeap(_, _)) = exp.parse(ctx.push()) {
                    Err(ctx.failure(start, ParseError::NotExpecting(exp.lookahead_str())))
                } else {
                    Ok(Yeap(ctx, Tree::Nil))
                }
            }
            ExpKind::SkipTo(exp) => loop {
                match exp.parse(ctx.push()) {
                    Err(nope) => {
                        if !ctx.dot() {
                            return Err(nope);
                        }
                    }
                    Ok(Yeap(inner_ctx, tree)) => {
                        ctx = ctx.merge(inner_ctx); // Merge the successful state
                        break Ok(Yeap(ctx, tree));
                    }
                }
            },

            ExpKind::Sequence(sequence) => {
                let mut results = Vec::new();
                for exp in &**sequence {
                    match exp.parse(ctx) {
                        Ok(Yeap(new_ctx, tree)) => {
                            results.push(tree);
                            ctx = new_ctx;
                        }
                        err => return err,
                    }
                }
                Ok(Yeap(ctx, Tree::from(results)))
            }
            ExpKind::Alt(exp) => exp.parse(ctx),
            ExpKind::Choice(options) => self.parse_choice(ctx, options),
            ExpKind::Optional(exp) => self.parse_optional(ctx, exp),

            ExpKind::Closure(exp) => {
                let mut res = Vec::new();
                match Self::repeat(ctx, exp, &mut res) {
                    Ok(Yeap(new_ctx, _)) => Ok(Yeap(new_ctx, Tree::from(res).closed())),
                    err => err,
                }
            }
            ExpKind::PositiveClosure(exp) => {
                let mut res: Vec<Tree> = Vec::new();
                match exp.parse(ctx) {
                    Ok(Yeap(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                match Self::repeat(ctx, exp, &mut res) {
                    Ok(Yeap(new_ctx, _)) => Ok(Yeap(new_ctx, Tree::from(res).closed())),
                    err => err,
                }
            }
            ExpKind::Join { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => match Self::repeat_with_pre(new_ctx, exp, sep, &mut res, true) {
                        Ok(Yeap(new_ctx, _)) => Ok(Yeap(new_ctx, Tree::from(res).closed())),
                        err => err,
                    },
                    Err((_actx, nope)) => Err(nope),
                }
            }
            ExpKind::PositiveJoin { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(Yeap(new_ctx, tree)) => {
                        res.push(tree);
                        ctx = new_ctx;
                    }
                    err => return err,
                };

                match Self::repeat_with_pre(ctx, exp, sep, &mut res, true) {
                    Ok(Yeap(new_ctx, _)) => Ok(Yeap(new_ctx, Tree::from(res).closed())),
                    err => err,
                }
            }
            ExpKind::Gather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();
                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => {
                        match Self::repeat_with_pre(new_ctx, exp, sep, &mut res, false) {
                            Ok(Yeap(new_ctx, _)) => Ok(Yeap(new_ctx, Tree::from(res).closed())),
                            err => err,
                        }
                    }
                    Err((_actx, nope)) => Err(nope),
                }
            }
            ExpKind::PositiveGather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(Yeap(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                match Self::repeat_with_pre(ctx, exp, sep, &mut res, false) {
                    Ok(Yeap(new_ctx, _)) => Ok(Yeap(new_ctx, Tree::from(res).closed())),
                    err => err,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::prelude::*;
    use crate::engine::strctx::StrCtx;
    use crate::exp::*;
    use crate::input::StrCursor;
    use crate::peg::ParseError;
    use crate::peg::Rule;
    use crate::rule::RuleRef;

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
        ctx.cut();
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
        ctx.cut();
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
        ctx.cut();
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
        ctx.cut();
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
