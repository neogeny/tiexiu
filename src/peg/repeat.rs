// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use super::parser::Succ;
use crate::peg::{Nope, ParseResult};
use crate::state::Ctx;
use crate::trees::Tree;

impl Exp {
    pub fn skip_exp<C: Ctx>(ctx: C, exp: &Exp) -> C {
        match exp.parse(ctx.clone()) {
            Ok(Succ(new_ctx, _)) => new_ctx,
            Err(_) => ctx,
        }
    }

    pub fn add_exp<C: Ctx>(ctx: C, exp: &Exp, res: &mut Vec<Tree>) -> Result<C, (C, Nope)> {
        match exp.parse(ctx.clone()) {
            Ok(Succ(new_ctx, tree)) => {
                res.push(tree);
                Ok(new_ctx)
            }
            Err(f) => Err((ctx, f)),
        }
    }

    pub fn repeat<C: Ctx>(mut ctx: C, exp: &Exp, res: &mut Vec<Tree>) -> ParseResult<C> {
        let was_cut = ctx.cut_seen();
        loop {
            let mut next_ctx = ctx.clone();
            next_ctx.unset_cut();
            match exp.parse(next_ctx) {
                Ok(Succ(new_ctx, tree)) => {
                    res.push(tree);
                    ctx = new_ctx;
                }
                Err(mut f) => {
                    if f.take_cut() {
                        return Err(f);
                    }
                    ctx.restore_if_was_cut(was_cut);
                    return Ok(Succ(ctx, Tree::Nil));
                }
            }
        }
    }

    pub fn repeat_with_pre<C: Ctx>(
        mut ctx: C,
        exp: &Exp,
        pre: &Exp,
        res: &mut Vec<Tree>,
        keep_pre: bool,
    ) -> ParseResult<C> {
        let was_cut = ctx.cut_seen(); // take to not affect the choice that comes
        loop {
            ctx.unset_cut();
            match pre.parse(ctx.clone()) {
                Err(mut f) => {
                    if f.take_cut() {
                        return Err(f);
                    }
                    // OK to match nothing
                    ctx.restore_if_was_cut(was_cut);
                    return Ok(Succ(ctx, Tree::Nil));
                }
                Ok(Succ(mut new_ctx, pre_cst)) => {
                    new_ctx.unset_cut();
                    match exp.parse(new_ctx.clone()) {
                        // NOTE: pre.parse().is_ok() so exp.parse().is_ok_or(fail)
                        Ok(Succ(repeat_ctx, exp_cst)) => {
                            if keep_pre {
                                res.push(pre_cst);
                            }
                            res.push(exp_cst);
                            ctx = repeat_ctx;
                        }
                        Err(f) => {
                            return Err(f); // the implicit cut after pre.parse()
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::strcursor::StrCursor;
    use crate::peg::Grammar;
    use crate::state::CtxI;
    use crate::state::corectx::CoreCtx;

    fn setup(input: &str) -> CoreCtx<'_, StrCursor> {
        let grammar = Box::leak(Box::new(Grammar::default()));
        let _ = grammar;
        CoreCtx::new(StrCursor::new(input), &[])
    }

    #[test]
    fn test_skip_exp() {
        let ctx = setup("abc");
        let exp = Exp::token("abc");
        let new_ctx = Exp::skip_exp(ctx.clone(), &exp);
        assert_eq!(new_ctx.cursor().mark(), 3);

        let ctx = setup("def");
        let new_ctx = Exp::skip_exp(ctx.clone(), &exp);
        assert_eq!(new_ctx.cursor().mark(), 0);
    }

    #[test]
    fn test_add_exp() {
        let ctx = setup("abc");
        let exp = Exp::token("abc");
        let mut res = Vec::new();
        let result = Exp::add_exp(ctx, &exp, &mut res);
        assert!(result.is_ok());
        assert_eq!(res.len(), 1);
        assert_eq!(result.unwrap().cursor().mark(), 3);
    }

    #[test]
    fn test_repeat() {
        let ctx = setup("abc abc abc");
        let exp = Exp::token("abc");
        let mut res = Vec::new();
        if let Ok(Succ(final_ctx, _)) = Exp::repeat(ctx, &exp, &mut res) {
            assert_eq!(res.len(), 3);
            assert_eq!(final_ctx.cursor().mark(), 11);
        } else {
            panic!("repeat  failed")
        }
    }

    #[test]
    fn test_repeat_with_pre() {
        let ctx = setup(",abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = Vec::new();
        if let Ok(Succ(final_ctx, _)) = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, true) {
            assert_eq!(res.len(), 4); // [",", "abc", ",", "abc"]
            assert_eq!(final_ctx.cursor().mark(), 8);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_with_pre_no_keep() {
        let ctx = setup(",abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = Vec::new();
        if let Ok(Succ(final_ctx, _)) = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, false) {
            assert_eq!(res.len(), 2); // ["abc", "abc"]
            assert_eq!(final_ctx.cursor().mark(), 8);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_restores_entered_cut() {
        let mut ctx = setup("abc abc abc");
        ctx.cut(); // set cut before entering Repeat
        assert!(ctx.cut_seen(), "ctx should have cut set before repeat");

        let exp = Exp::token("abc");
        let mut res = Vec::new();
        if let Ok(Succ(final_ctx, _)) = Exp::repeat(ctx, &exp, &mut res) {
            assert_eq!(res.len(), 3);
            assert!(final_ctx.cut_seen(), "cut should be restored after repeat");
        } else {
            panic!("repeat failed")
        }
    }

    #[test]
    fn test_repeat_with_pre_restores_entered_cut() {
        let mut ctx = setup(",abc,abc");
        ctx.cut(); // set cut before entering repeat_with_pre
        assert!(
            ctx.cut_seen(),
            "ctx should have cut set before repeat_with_pre"
        );

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = Vec::new();
        if let Ok(Succ(final_ctx, _)) = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, true) {
            assert_eq!(res.len(), 4);
            assert!(
                final_ctx.cut_seen(),
                "cut should be restored after repeat_with_pre"
            );
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_with_pre_no_cut_enters_clears() {
        let ctx = setup(",abc,abc");
        assert!(
            !ctx.cut_seen(),
            "ctx should not have cut set before repeat_with_pre"
        );

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = Vec::new();
        if let Ok(Succ(final_ctx, _)) = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, true) {
            assert_eq!(res.len(), 4);
            assert!(
                !final_ctx.cut_seen(),
                "cut should be cleared when not set on entry"
            );
        } else {
            panic!("repeat_with_pre failed")
        }
    }
}
