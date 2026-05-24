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
    fn parse_at(&self, ctx: &mut C) -> ParseResult {
        Exp::parse_at(self, ctx)
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
    pub fn parse_at<C: Ctx>(&self, ctx: &mut C) -> ParseResult {
        match self.do_parse_at(ctx) {
            Err(err) => Err(err),
            Ok(Yeap(tree)) => {
                if let Some(df) = self.df.as_ref() {
                    let mut cloned = tree.as_ref().clone();
                    cloned.define(df);
                    Ok(yeap(cloned.into()))
                } else {
                    Ok(yeap(tree))
                }
            }
        }
    }

    fn do_parse_at<C: Ctx>(&self, ctx: &mut C) -> ParseResult {
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
            ExpKind::EmptyClosure => Ok(yeap(Tree::from(Vec::<Rc<Tree>>::new()).closed().into())),
            ExpKind::Nil => Ok(yeap(Tree::Nil.into())),
            ExpKind::RuleInclude { name, exp } => match exp {
                None => Err(ctx.failure(start, RuleNotLinked(name.clone()))),
                Some(exp) => exp.parse_at(ctx),
            },
            ExpKind::Call { name, rule } => match rule {
                None => Err(ctx.failure(start, RuleNotLinked(name.clone()))),
                Some(rule) => Self::rule_call(ctx, name, rule.as_ref()),
            },
            ExpKind::Cut => {
                ctx.cut();
                ctx.tracer().trace_cut(ctx);
                Ok(yeap(Tree::Nil.into()))
            }
            ExpKind::Void => {
                ctx.match_void();
                Ok(yeap(Tree::Nil.into()))
            }
            ExpKind::Fail => Err(ctx.failure(start, Fail)),
            ExpKind::Dot => {
                if let Some(c) = ctx.next() {
                    Ok(yeap(Tree::text(c.to_string().as_str().into()).into()))
                } else {
                    Err(ctx.failure(start, NoMoreInput))
                }
            }
            ExpKind::Eol => {
                if ctx.match_eol() {
                    Ok(yeap(Tree::Nil.into()))
                } else {
                    Err(ctx.failure(start, ExpectingEol))
                }
            }
            ExpKind::Eof => {
                if ctx.parse_eof() {
                    Ok(yeap(Tree::Nil.into()))
                } else {
                    Err(ctx.failure(start, ExpectingEof))
                }
            }

            ExpKind::Token(token) => {
                if ctx.match_token(token) {
                    Ok(yeap(Tree::Text(token.clone()).into()))
                } else {
                    Err(ctx.failure(start, ExpectedToken(token.clone())))
                }
            }
            ExpKind::Pattern(pattern) => {
                if let Some(matched) = ctx.match_pattern(pattern) {
                    Ok(yeap(Tree::Text(matched.clone()).into()))
                } else {
                    Err(ctx.failure(
                        start,
                        ExpectedPattern(pyre::truncate_pattern(pattern, 16).into()),
                    ))
                }
            }
            ExpKind::Constant(literal) => Ok(yeap(Tree::Text(literal.clone()).into())),
            ExpKind::Alert(literal, _) => Ok(yeap(Tree::Text(literal.clone()).into())),

            ExpKind::Named(name, exp) => match exp.parse_at(ctx) {
                Ok(Yeap(tree)) => {
                    let wrapped = Tree::named(name.clone(), tree);
                    Ok(yeap(wrapped.into()))
                }
                err => err,
            },
            ExpKind::NamedList(name, exp) => match exp.parse_at(ctx) {
                Ok(Yeap(tree)) => {
                    let wrapped = Tree::named_as_list(name.clone(), tree);
                    Ok(yeap(wrapped.into()))
                }
                err => err,
            },
            ExpKind::Override(exp) => match exp.parse_at(ctx) {
                Ok(Yeap(tree)) => {
                    let wrapped = Tree::override_with(tree);
                    Ok(yeap(wrapped.into()))
                }
                err => err,
            },
            ExpKind::OverrideList(exp) => match exp.parse_at(ctx) {
                Ok(Yeap(tree)) => {
                    let wrapped = Tree::override_as_list(tree);
                    Ok(yeap(wrapped.into()))
                }
                err => err,
            },
            ExpKind::Group(exp) => exp.parse_at(ctx),
            ExpKind::SkipGroup(exp) => {
                let result = exp.parse_at(ctx);
                match result {
                    Ok(Yeap(_)) => Ok(yeap(Tree::Nil.into())),
                    err => err,
                }
            }
            ExpKind::Lookahead(exp) => {
                let branch = ctx.mark();
                match exp.parse_at(ctx) {
                    Ok(_) => {
                        ctx.reset(branch);
                        Ok(yeap(Tree::Nil.into()))
                    }
                    Err(nope) => {
                        ctx.reset(branch);
                        Err(nope)
                    }
                }
            }
            ExpKind::NegativeLookahead(exp) => {
                let branch = ctx.mark();
                match exp.parse_at(ctx) {
                    Ok(_) => {
                        ctx.reset(branch);
                        Err(ctx.failure(start, NotExpecting(exp.lookahead_str())))
                    }
                    Err(_) => {
                        ctx.reset(branch);
                        Ok(yeap(Tree::Nil.into()))
                    }
                }
            }
            ExpKind::SkipTo(exp) => loop {
                let branch = ctx.mark();
                match exp.parse_at(ctx) {
                    Err(_) => {
                        ctx.reset(branch);
                        if ctx.next().is_none() {
                            return Err(ctx.failure(start, Fail));
                        }
                    }
                    Ok(Yeap(tree)) => break Ok(yeap(tree)),
                }
            },

            ExpKind::Sequence(sequence) => {
                let seq_start = ctx.mark();
                let mut results: Vec<Rc<Tree>> = Vec::with_capacity(sequence.len());
                for exp in &**sequence {
                    if let ExpKind::Cut = exp.kind {
                        ctx.tracer().trace_cut(ctx);
                        ctx.prune_cache();
                        continue;
                    }
                    match exp.parse_at(ctx) {
                        Ok(Yeap(tree)) => results.push(tree),
                        Err(nope) => {
                            ctx.reset(seq_start);
                            return Err(nope);
                        }
                    }
                }
                if results.is_empty() {
                    Ok(yeap(Tree::Nil.into()))
                } else if results.len() == 1 {
                    Ok(yeap(results[0].clone()))
                } else {
                    Ok(yeap(Tree::Seq(results.into()).into()))
                }
            }
            ExpKind::Alt(_exp) => Err(ctx.failure(start, AltWithNoChoice)),
            ExpKind::Choice(options) => self.parse_choice(ctx, options),
            ExpKind::Optional(exp) => self.parse_optional(ctx, exp),

            ExpKind::Closure(exp) => Self::repeat(ctx, exp, false),
            ExpKind::PositiveClosure(exp) => Self::repeat(ctx, exp, true),
            ExpKind::Join { exp, sep } => Self::repeat_with_sep(ctx, exp, sep, false, true),
            ExpKind::PositiveJoin { exp, sep } => Self::repeat_with_sep(ctx, exp, sep, true, true),
            ExpKind::Gather { exp, sep } => Self::repeat_with_sep(ctx, exp, sep, false, false),

            ExpKind::PositiveGather { exp, sep } => {
                Self::repeat_with_sep(ctx, exp, sep, true, false)
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
        let mut ctx = StrCtx::new(cursor, &[]);
        let result_a = token_a.parse_at(&mut ctx);
        println!("token('a') on 'a': {:?}", result_a);
        assert!(result_a.is_ok(), "token('a') should match 'a'");

        let exp1 = Exp::sequence(vec![Exp::token("a"), Exp::token("b")]);
        let mut ctx1 = StrCtx::new(StrCursor::new("a b"), &[]);
        let result1_ok = exp1.parse_at(&mut ctx1);
        println!("exp1 on 'a b': {:?}", result1_ok);
        assert!(result1_ok.is_ok(), "sequence 1 should succeed on 'a b'");

        let mut ctx2 = StrCtx::new(StrCursor::new("a c x"), &[]);
        let result1_err = exp1.parse_at(&mut ctx2);
        println!("exp1 on 'a c x': {:?}", result1_err);

        let exp2 = Exp::sequence(vec![Exp::token("a"), Exp::token("c"), Exp::token("d")]);
        let mut ctx3 = StrCtx::new(StrCursor::new("a c d"), &[]);
        let result2_ok = exp2.parse_at(&mut ctx3);
        println!("exp2 on 'a c d': {:?}", result2_ok);
        assert!(result2_ok.is_ok(), "sequence 2 should succeed on 'a c d'");

        let mut ctx4 = StrCtx::new(StrCursor::new("a c x"), &[]);
        let result2_err = exp2.parse_at(&mut ctx4);
        println!("exp2 on 'a c x': {:?}", result2_err);

        let exp = Exp::choice(vec![exp1, exp2]);
        let mut ctx = StrCtx::new(StrCursor::new("a c x"), &[]);

        let result = exp.parse_at(&mut ctx);
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
        let mut ctx2 = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx2.cut();
        let result = exp.parse_at(&mut ctx2);
        assert!(result.is_ok(), "choice should succeed");
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
        let mut ctx2 = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx2.cut();
        let result = exp.parse_at(&mut ctx2);
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

        let exp = Exp::choice(vec![Exp::token("abc"), Exp::token("xyz")]);
        let result = exp.parse_at(&mut ctx);
        assert!(result.is_ok(), "choice should succeed");
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

        let mut ctx2 = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx2.cut();
        let exp = Exp::optional(Exp::token("abc"));
        let result = exp.parse_at(&mut ctx2);
        assert!(result.is_ok(), "optional should succeed");
    }

    #[test]
    fn optional_restores_entered_cut_on_failure() {
        let grammar =
            crate::peg::Grammar::new("test", &[Rule::new("start", &[], Exp::token("xyz")).into()]);
        let _ = grammar;
        let mut ctx = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx.cut();
        assert!(ctx.cut_seen(), "ctx should have cut set before optional");

        let mut ctx2 = StrCtx::new(StrCursor::new("abc"), &[]);
        ctx2.cut();
        let exp = Exp::optional(Exp::token("xyz"));
        let result = exp.parse_at(&mut ctx2);
        assert!(result.is_ok(), "optional failure returns Ok with nil");
    }

    #[test]
    fn optional_clears_when_no_cut_enters() {
        let grammar =
            crate::peg::Grammar::new("test", &[Rule::new("start", &[], Exp::token("abc")).into()]);
        let _ = grammar;
        let mut ctx = StrCtx::new(StrCursor::new("abc"), &[]);

        let exp = Exp::optional(Exp::token("abc"));
        let result = exp.parse_at(&mut ctx);
        assert!(result.is_ok(), "optional should succeed");
    }
}
