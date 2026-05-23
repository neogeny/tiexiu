// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::api::error::nope::ParseResult;
use crate::context::Ctx;
use crate::peg::error::Yeap;
use crate::peg::error::nope::yeap;
use crate::peg::{Exp, ExpKind, ParseFailure::*, Parser};
use crate::trees::Tree;
use crate::types::Str;
use crate::util::pyre;
use std::rc::Rc;

impl<C: Ctx> Parser<C> for Exp {
    fn parse_at(&self, ctx: C) -> ParseResult {
        self.parse_at(ctx)
    }
}

impl Exp {
    /// Pre-computes lookahead and defines caches for this expression.
    pub fn initialize_caches(&mut self) {
        self.cache_lookahead();
        self.cache_defines();
    }

    /// Returns the lookahead set joined into a single string for error messages.
    pub fn lookahead_str(&self) -> Str {
        self.la
            .as_ref()
            .map(|la| la.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(" "))
            .unwrap_or_default()
            .into()
    }

    /// Parses at the current context position, applying defines after a successful match.
    pub fn parse_at<C: Ctx>(&self, ctx: C) -> ParseResult {
        match self.do_parse_at(ctx) {
            Err(err) => Err(err),
            Ok(Yeap(snap, tree)) => {
                if let Some(df) = self.df.as_ref() {
                    let mut cloned = tree.as_ref().clone();
                    cloned.define(df);
                    Ok(yeap(&snap, cloned.into()))
                } else {
                    Ok(yeap(&snap, tree))
                }
            }
        }
    }

    fn do_parse_at<C: Ctx>(&self, mut ctx: C) -> ParseResult {
        let start = ctx.mark();
        let mut exp = self;
        while let ExpKind::RuleInclude { .. } | ExpKind::Group(_) = &exp.kind {
            match &exp.kind {
                ExpKind::Group(next) => {
                    exp = next;
                }
                ExpKind::RuleInclude { name, exp: opt_exp } => match opt_exp {
                    None => return Err(ctx.failure(start, RuleNotLinked(name.clone()))),
                    Some(next) => exp = next,
                },
                _ => break,
            }
        }

        match &exp.kind {
            ExpKind::EmptyClosure => Ok(yeap(
                &ctx.click(),
                Tree::from(Vec::<Rc<Tree>>::new()).closed().into(),
            )),
            ExpKind::Nil => Ok(yeap(&ctx.click(), Tree::Nil.into())),
            ExpKind::RuleInclude { name, exp } => match exp {
                None => Err(ctx.failure(start, RuleNotLinked(name.clone()))),
                Some(exp) => exp.parse_at(ctx),
            },
            ExpKind::Call { name, rule } => match rule {
                None => Err(ctx.failure(start, RuleNotLinked(name.clone()))),
                Some(rule) => match Self::rule_call(ctx.push(), name, rule.as_ref()) {
                    Ok(Yeap(snap, tree)) => {
                        ctx.merge(&snap);
                        Ok(yeap(&ctx.click(), tree))
                    }
                    err => err,
                },
            },
            ExpKind::Cut => {
                ctx.cut();
                ctx.tracer().trace_cut(&ctx);
                Ok(yeap(&ctx.click(), Tree::Nil.into()))
            }
            ExpKind::Void => {
                ctx.match_void();
                Ok(yeap(&ctx.click(), Tree::Nil.into()))
            }
            ExpKind::Fail => Err(ctx.failure(start, Fail)),
            ExpKind::Dot => {
                if let Some(c) = ctx.next() {
                    Ok(yeap(
                        &ctx.click(),
                        Tree::text(c.to_string().as_str().into()).into(),
                    ))
                } else {
                    Err(ctx.failure(start, NoMoreInput))
                }
            }
            ExpKind::Eol => {
                if ctx.match_eol() {
                    Ok(yeap(&ctx.click(), Tree::Nil.into()))
                } else {
                    Err(ctx.failure(start, ExpectingEol))
                }
            }
            ExpKind::Eof => {
                if ctx.parse_eof() {
                    Ok(yeap(&ctx.click(), Tree::Nil.into()))
                } else {
                    Err(ctx.failure(start, ExpectingEof))
                }
            }

            ExpKind::Token(token) => {
                if ctx.match_token(token) {
                    Ok(yeap(&ctx.click(), Tree::Text(token.clone()).into()))
                } else {
                    Err(ctx.failure(start, ExpectedToken(token.clone())))
                }
            }
            ExpKind::Pattern(pattern) => {
                if let Some(matched) = ctx.match_pattern(pattern) {
                    Ok(yeap(&ctx.click(), Tree::Text(matched.clone()).into()))
                } else {
                    Err(ctx.failure(
                        start,
                        ExpectedPattern(pyre::truncate_pattern(pattern, 16).into()),
                    ))
                }
            }
            ExpKind::Constant(literal) => {
                Ok(yeap(&ctx.click(), Tree::Text(literal.clone()).into()))
            }
            ExpKind::Alert(literal, _) => {
                Ok(yeap(&ctx.click(), Tree::Text(literal.clone()).into()))
            }

            ExpKind::Named(name, exp) => match exp.parse_at(ctx.clone()) {
                Ok(Yeap(snap, tree)) => {
                    let wrapped = Tree::named(name.clone(), tree);
                    ctx.merge(&snap);
                    Ok(yeap(&ctx.click(), wrapped.into()))
                }
                err => err,
            },
            ExpKind::NamedList(name, exp) => match exp.parse_at(ctx.clone()) {
                Ok(Yeap(snap, tree)) => {
                    let wrapped = Tree::named_as_list(name.clone(), tree);
                    ctx.merge(&snap);
                    Ok(yeap(&ctx.click(), wrapped.into()))
                }
                err => err,
            },
            ExpKind::Override(exp) => match exp.parse_at(ctx.clone()) {
                Ok(Yeap(snap, tree)) => {
                    let wrapped = Tree::override_with(tree);
                    ctx.merge(&snap);
                    Ok(yeap(&ctx.click(), wrapped.into()))
                }
                err => err,
            },
            ExpKind::OverrideList(exp) => match exp.parse_at(ctx.clone()) {
                Ok(Yeap(snap, tree)) => {
                    let wrapped = Tree::override_as_list(tree);
                    ctx.merge(&snap);
                    Ok(yeap(&ctx.click(), wrapped.into()))
                }
                err => err,
            },
            ExpKind::Group(exp) => {
                // NOTE contain cutseen value
                //  The grammar doesn't enforce it, but the only
                //  logical reason to introduce a Group is to
                //  introduce a nested Choice.
                match exp.parse_at(ctx.push()) {
                    Ok(Yeap(snap, tree)) => {
                        ctx.merge(&snap);
                        Ok(yeap(&ctx.click(), tree))
                    }
                    err => err,
                }
            }
            ExpKind::SkipGroup(exp) => {
                let Yeap(snap, _) = exp.parse_at(ctx.clone())?;
                ctx.merge(&snap);
                Ok(yeap(&ctx.click(), Tree::Nil.into()))
            }
            ExpKind::Lookahead(exp) => match exp.parse_at(ctx.push()) {
                Ok(Yeap(_, _)) => Ok(yeap(&ctx.click(), Tree::Nil.into())),
                Err(nope) => Err(nope),
            },
            ExpKind::NegativeLookahead(exp) => {
                if let Ok(Yeap(_, _)) = exp.parse_at(ctx.push()) {
                    Err(ctx.failure(start, NotExpecting(exp.lookahead_str())))
                } else {
                    Ok(yeap(&ctx.click(), Tree::Nil.into()))
                }
            }
            ExpKind::SkipTo(exp) => loop {
                match exp.parse_at(ctx.clone()) {
                    Err(nope) => {
                        if ctx.dot().is_none() {
                            return Err(nope);
                        }
                    }
                    Ok(Yeap(snap, tree)) => {
                        ctx.merge(&snap);
                        break Ok(yeap(&ctx.into(), tree));
                    }
                }
            },

            ExpKind::Sequence(sequence) => {
                let mut results: Vec<Rc<Tree>> = Vec::with_capacity(sequence.len());
                for exp in &**sequence {
                    if let ExpKind::Cut = exp.kind {
                        // ctx.cut();
                        ctx.tracer().trace_cut(&ctx);
                        ctx.prune_cache();
                        continue;
                    }
                    match exp.parse_at(ctx.push()) {
                        Ok(Yeap(snap, tree)) => {
                            results.push(tree);
                            ctx.merge(&snap);
                        }
                        Err(nope) => {
                            return Err(nope);
                        }
                    }
                }
                let snap = ctx.click();
                if results.is_empty() {
                    Ok(yeap(&snap, Tree::Nil.into()))
                } else if results.len() == 1 {
                    Ok(yeap(&snap, results[0].clone()))
                } else {
                    Ok(yeap(&snap, Tree::Seq(results.into()).into()))
                }
            }
            ExpKind::Alt(_exp) => Err(ctx.failure(start, AltWithNoChoice)),
            ExpKind::Choice(options) => self.parse_choice(ctx, options),
            ExpKind::Optional(exp) => self.parse_optional(ctx, exp),

            ExpKind::Closure(exp) => Self::repeat(ctx.push(), exp, false),
            ExpKind::PositiveClosure(exp) => Self::repeat(ctx.push(), exp, true),
            ExpKind::Join { exp, sep } => Self::repeat_with_sep(ctx.push(), exp, sep, false, true),
            ExpKind::PositiveJoin { exp, sep } => {
                Self::repeat_with_sep(ctx.push(), exp, sep, true, true)
            }
            ExpKind::Gather { exp, sep } => {
                Self::repeat_with_sep(ctx.push(), exp, sep, false, false)
            }

            ExpKind::PositiveGather { exp, sep } => {
                Self::repeat_with_sep(ctx.push(), exp, sep, true, false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::context::prelude::*;
    use crate::context::strctx::StrCtx;
    use crate::exp::*;
    use crate::input::StrCursor;
    use crate::peg::Rule;
    use crate::rule::RuleRef;

    #[test]
    #[ignore]
    fn choice_keeps_furthest_failure() {
        let token_a = Exp::token("a");
        let cursor = StrCursor::new("a");
        println!("cursor on 'a': {:?}", cursor);
        let result_a = token_a.parse_at(StrCtx::new(cursor, &[]));
        println!("token('a') on 'a': {:?}", result_a);
        assert!(result_a.is_ok(), "token('a') should match 'a'");

        let exp1 = Exp::sequence(vec![Exp::token("a"), Exp::token("b")]);
        let result1_ok = exp1.parse_at(StrCtx::new(StrCursor::new("a b"), &[]));
        println!("exp1 on 'a b': {:?}", result1_ok);
        assert!(result1_ok.is_ok(), "sequence 1 should succeed on 'a b'");

        let result1_err = exp1.parse_at(StrCtx::new(StrCursor::new("a c x"), &[]));
        println!("exp1 on 'a c x': {:?}", result1_err);

        let exp2 = Exp::sequence(vec![Exp::token("a"), Exp::token("c"), Exp::token("d")]);
        let result2_ok = exp2.parse_at(StrCtx::new(StrCursor::new("a c d"), &[]));
        println!("exp2 on 'a c d': {:?}", result2_ok);
        assert!(result2_ok.is_ok(), "sequence 2 should succeed on 'a c d'");

        let result2_err = exp2.parse_at(StrCtx::new(StrCursor::new("a c x"), &[]));
        println!("exp2 on 'a c x': {:?}", result2_err);

        let exp = Exp::choice(vec![exp1, exp2]);
        let ctx = StrCtx::new(StrCursor::new("a c x"), &[]);

        let result = exp.parse_at(ctx);
        println!("choice on 'a c x': {:?}", result);
    }

    #[test]
    #[ignore = "cutseen is being removed from Ctx"]
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
        let result = exp.parse_at(ctx.clone());
        assert!(result.is_ok(), "choice should succeed");
        let succ = result.unwrap();
        ctx.merge(&succ.0);
        assert!(
            ctx.cut_seen(),
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
        let result = exp.parse_at(ctx);
        assert!(
            result.is_err(),
            "choice should return Err when all options fail"
        );
    }

    #[test]
    fn choice_clears_when_no_cut_enters() -> crate::Result<()> {
        let grammar = crate::peg::Grammar::new(
            "test",
            &[RuleRef::from(Rule::new("start", &[], Exp::token("abc")))],
        );
        let _ = grammar;
        let mut ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        assert!(!ctx.cut_seen(), "ctx should not have cut set");

        let exp = Exp::choice(vec![Exp::token("abc"), Exp::token("xyz")]);
        let result = exp.parse_at(ctx.clone());
        assert!(result.is_ok(), "choice should succeed");
        let succ = result?;
        ctx.merge(&succ.0);
        assert!(
            !ctx.cut_seen(),
            "cut should be cleared when not set on entry"
        );
        Ok(())
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
        let result = exp.parse_at(ctx.clone());
        assert!(result.is_ok(), "optional should succeed");
        let succ = result.unwrap();
        ctx.merge(&succ.0);
        assert!(
            ctx.cut_seen(),
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
        let result = exp.parse_at(ctx.clone());
        assert!(result.is_ok(), "optional failure returns Ok with nil");
        let succ = result.unwrap();
        ctx.merge(&succ.0);
        assert!(
            ctx.cut_seen(),
            "cut should be restored after optional failure"
        );
    }

    #[test]
    fn optional_clears_when_no_cut_enters() {
        let grammar =
            crate::peg::Grammar::new("test", &[Rule::new("start", &[], Exp::token("abc")).into()]);
        let _ = grammar;
        let mut ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        assert!(!ctx.cut_seen(), "ctx should not have cut set");

        let exp = Exp::optional(Exp::token("abc"));
        let result = exp.parse_at(ctx.clone());
        assert!(result.is_ok(), "optional should succeed");
        let succ = result.unwrap();
        ctx.merge(&succ.0);
        assert!(
            !ctx.cut_seen(),
            "cut should be cleared when not set on entry"
        );
    }
}
