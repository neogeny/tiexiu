// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Implementation of `call` logic for `Exp`.
//! Moved from `Ctx` trait to decouple parsing logic from context management.

use crate::context::CtxSem;
use crate::peg::Exp;
use crate::peg::error::{Nope, ParseFailure, ParseResult};
use crate::peg::rule::Rule;
use crate::trees::TreeRef;
use crate::trees::tree::Tree;

impl Exp {
    /// Core entry point for calling a rule.
    /// Handles setup, tracing, token skipping, and delegation to `do_call`.
    pub fn rule_call<C: CtxSem>(ctx: &mut C, name: &str, rule: &Rule) -> ParseResult {
        let start = ctx.mark();
        let key = ctx.key(name, rule.is_memoizable());

        if !rule.is_token() {
            ctx.next_token();
        }

        if rule.should_trace() {
            ctx.enter(name);
            ctx.tracer().trace_entry(ctx);
        }

        ctx.push_cut();
        let result = Self::do_call(ctx, name, rule);
        let _cutseen = ctx.take_cut();
        match result {
            Ok(tree) => {
                if rule.should_trace() {
                    ctx.leave();
                }
                if rule.is_name()
                    && let Tree::Text(name) = tree.as_ref()
                    && ctx.is_keyword(name)
                {
                    ctx.reset(start);
                    ctx.memoize(&key, &Tree::Bottom.into(), ctx.mark());
                    let error = ParseFailure::ReservedWord(name.clone());
                    ctx.tracer().trace_failure(ctx, name);
                    return Err(ctx.failure(start, error));
                }
                ctx.tracer().trace_success(ctx);
                ctx.memoize(&key, &tree, ctx.mark());
                ctx.heartbeat_tick();
                Ok(tree)
            }
            Err(nope) => {
                ctx.reset(start);
                if rule.should_trace() {
                    ctx.leave();
                }
                ctx.tracer().trace_failure(ctx, name);
                ctx.memoize(&key, &Tree::Bottom.into(), ctx.mark());
                Err(nope)
            }
        }
    }

    /// Internal dispatch for a call, handling memoization and left recursion.
    /// This mirrors the logic previously in `Ctx::do_call`.
    fn do_call<C: CtxSem>(ctx: &mut C, name: &str, rule: &Rule) -> ParseResult {
        let start = ctx.mark();
        let key = ctx.key(name, rule.is_memoizable());

        if let Some(memo) = ctx.memo(&key) {
            return match memo.tree.as_ref() {
                Tree::Bottom => {
                    let err = ParseFailure::FailedParse(name.into());
                    Err(ctx.failure(start, err))
                }
                _ => {
                    ctx.reset(memo.mark);
                    Ok(memo.tree.clone())
                }
            };
        }

        if rule.is_left_recursive() {
            Self::call_recursive(ctx, &key, rule)
        } else {
            rule.parse_at(ctx)
        }
    }

    /// Handles left-recursive rule invocations using the iterative bootstrapping approach.
    fn call_recursive<C: CtxSem>(
        ctx: &mut C,
        key: &crate::context::memo::MemoKey,
        rule: &Rule,
    ) -> ParseResult {
        ctx.tracer().trace_recursion(ctx);
        let start = ctx.mark();
        if !rule.is_left_recursive() {
            return Err(ctx.failure(start, ParseFailure::FailedParse(rule.name.clone())));
        }
        let mut lastmark = start;
        let mut lasttree: TreeRef = Tree::Null.into();
        let mut lastnope: Option<Nope> = None;

        ctx.memoize(key, &Tree::Bottom.into(), start);

        loop {
            ctx.reset(start);
            ctx.track_recursion_depth(key)?;

            let result = rule.parse_at(ctx);
            ctx.untrack(key);

            match result {
                Err(nope) => {
                    lastnope = Some(nope);
                    break;
                }
                Ok(tree) => {
                    let endmark = ctx.mark();
                    if endmark <= lastmark {
                        break;
                    }
                    lastmark = endmark;
                    lasttree = tree.clone();
                    ctx.memoize(key, &lasttree.clone(), lastmark);
                }
            }
        }

        ctx.reset(lastmark);
        ctx.memoize(key, &lasttree.clone(), lastmark);

        if *lasttree == Tree::Bottom {
            let nope = lastnope.unwrap_or_else(|| {
                ctx.failure(
                    start,
                    ParseFailure::FailedRecursion(
                        rule.name.clone(),
                        start,
                        lastmark,
                        lasttree.clone(),
                    ),
                )
            });
            return Err(nope);
        }
        Ok(lasttree)
    }
}
