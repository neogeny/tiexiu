// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Implementation of `call` logic for `Exp`.
//! Moved from `Ctx` trait to decouple parsing logic from context management.

use crate::context::Ctx;
use crate::peg::Exp;
use crate::peg::error::{Nope, ParseFailure, ParseResult, Yeap};
use crate::peg::rule::Rule;
use crate::trees::tree::Tree;
use std::rc::Rc;

pub const MAX_RECURSION_DEPTH: usize = 64;

impl Exp {
    /// Core entry point for calling a rule.
    /// Handles setup, tracing, token skipping, and delegation to `do_call`.
    pub fn rule_call<C: Ctx>(mut ctx: C, name: &str, rule: &Rule) -> ParseResult<C> {
        let start = ctx.mark();
        let key = ctx.key(name, rule.is_memoizable());

        if !rule.is_token() {
            ctx.next_token();
        }

        if rule.should_trace() {
            ctx.enter(name);
            ctx.tracer().trace_entry(&ctx);
        }

        match Self::do_call(ctx.push(), name, rule) {
            Ok(Yeap(mut ctx, tree)) => {
                if rule.should_trace() {
                    ctx.leave();
                }
                if rule.is_name()
                    && let Tree::Text(name) = tree.as_ref()
                    && ctx.is_keyword(name)
                {
                    ctx.memoize(&key, &Tree::Bottom.into(), ctx.mark());
                    let error = ParseFailure::ReservedWord(name.clone());
                    ctx.tracer().trace_failure(&ctx, name);
                    return Err(ctx.failure(start, error));
                }
                ctx.tracer().trace_success(&ctx);
                ctx.memoize(&key, &tree, ctx.mark());
                ctx.heartbeat_tick();
                Ok(Yeap(ctx, tree))
            }
            Err(mut nope) => {
                if rule.should_trace() {
                    ctx.leave();
                }
                ctx.tracer().trace_failure(&ctx, name);
                ctx.memoize(&key, &Tree::Bottom.into(), ctx.mark());
                nope.take_cut();
                Err(nope)
            }
        }
    }

    /// Internal dispatch for a call, handling memoization and left recursion.
    /// This mirrors the logic previously in `Ctx::do_call`.
    fn do_call<C: Ctx>(mut ctx: C, name: &str, rule: &Rule) -> ParseResult<C> {
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
                    Ok(Yeap(ctx.into(), memo.tree))
                }
            };
        }

        if rule.is_left_recursive() {
            Self::call_recursive(ctx, &key, rule)
        } else {
            rule.parse(ctx)
        }
    }

    /// Handles left-recursive rule invocations using the iterative bootstrapping approach.
    fn call_recursive<C: Ctx>(
        mut ctx: C,
        key: &crate::context::memo::MemoKey,
        rule: &Rule,
    ) -> ParseResult<C> {
        ctx.tracer().trace_recursion(&ctx);
        if !rule.is_left_recursive() {
            panic!("Recursive call on non-LRec rule");
        }

        let start = ctx.mark();
        let mut lastmark = start;
        let mut lasttree: Tree = Tree::Nil;
        let mut lastnope: Option<Nope> = None;

        ctx.memoize(key, &Tree::Bottom.into(), start);

        loop {
            ctx.reset(start);
            Self::track_recursion_depth(&mut ctx, key)?;

            // We need to push a context state here to attempt the parse safely
            let result = rule.parse(ctx.push());
            ctx.untrack(key);

            match result {
                Err(nope) => {
                    lastnope = Some(nope);
                    break;
                }
                Ok(Yeap(inner_ctx, tree)) => {
                    let endmark = inner_ctx.mark();
                    let endtree = tree;
                    if endmark <= lastmark {
                        break;
                    }
                    lastmark = endmark;
                    lasttree = Rc::unwrap_or_clone(endtree);
                    ctx = ctx.merge(*inner_ctx);
                    ctx.memoize(key, &lasttree.clone().into(), lastmark);
                }
            }
        }

        ctx.reset(lastmark);
        ctx.memoize(key, &lasttree.clone().into(), lastmark);

        if lasttree == Tree::Bottom {
            let nope = lastnope.unwrap_or(ctx.failure(
                start,
                ParseFailure::FailedRecursion(
                    key.name.clone(),
                    start,
                    lastmark,
                    lasttree.clone().into(),
                ),
            ));
            return Err(nope);
        }
        Ok(Yeap(ctx.into(), lasttree.into()))
    }

    /// Checks recursion depth to prevent stack overflow.
    fn track_recursion_depth<C: Ctx>(
        ctx: &mut C,
        key: &crate::context::memo::MemoKey,
    ) -> Result<(), Nope> {
        let depth = ctx.track(key);
        if depth > MAX_RECURSION_DEPTH {
            panic!("Recursion depth exceeded")
        } else {
            Ok(())
        }
    }
}
