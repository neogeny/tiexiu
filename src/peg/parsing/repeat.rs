// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Exp;
use crate::engine::Ctx;
use crate::peg::error::*;
use crate::trees::Tree;
use std::rc::Rc;

impl Exp {
    pub fn skip_exp<C: Ctx>(ctx: C, exp: &Exp) -> C {
        let skip_ctx = ctx.clone();
        match exp.parse(skip_ctx) {
            Ok(Yeap(new_ctx, _)) => ctx.merge(*new_ctx),
            Err(_) => ctx,
        }
    }

    pub fn add_exp<C: Ctx>(ctx: C, exp: &Exp, res: &mut Vec<Rc<Tree>>) -> Result<C, (C, Nope)> {
        match exp.parse(ctx.clone()) {
            Ok(Yeap(new_ctx, tree)) => {
                res.push(tree);
                Ok(ctx.merge(*new_ctx))
            }
            Err(nope) => Err((ctx, nope)),
        }
    }

    pub fn repeat<C: Ctx>(mut ctx: C, exp: &Exp, res: &mut Vec<Rc<Tree>>) -> ParseResult<C> {
        loop {
            let mark = ctx.mark();
            match exp.parse(ctx.push()) {
                Ok(Yeap(new_ctx, tree)) => {
                    if new_ctx.mark() == mark {
                        return Err(ctx.failure(mark, ParseFailure::ClosureMatchedVoid()));
                    }
                    res.push(tree);
                    ctx = ctx.merge(*new_ctx);
                }
                Err(_nope) => {
                    return Ok(Yeap(ctx.into(), Tree::Nil.into()));
                }
            }
        }
    }

    pub fn repeat_with_pre<C: Ctx>(
        mut ctx: C,
        exp: &Exp,
        pre: &Exp,
        res: &mut Vec<Rc<Tree>>,
        keep_pre: bool,
    ) -> ParseResult<C> {
        loop {
            let mark = ctx.mark();
            match pre.parse(ctx.push()) {
                Err(mut nope) => {
                    if nope.take_cut() {
                        return Err(nope);
                    }
                    return Ok(Yeap(ctx.into(), Tree::Nil.into()));
                }
                Ok(Yeap(mut new_ctx, pre_cst)) => {
                    if new_ctx.mark() == mark {
                        return Err(ctx.failure(mark, ParseFailure::ClosureMatchedVoid()));
                    }
                    new_ctx.cut();
                    let inner_ctx = *new_ctx;
                    match exp.parse(inner_ctx) {
                        Ok(Yeap(repeat_ctx, exp_cst)) => {
                            if keep_pre {
                                res.push(pre_cst);
                            }
                            res.push(exp_cst);
                            ctx = ctx.merge(*repeat_ctx);
                        }
                        Err(mut nope) => {
                            nope.take_cut();
                            return Err(nope);
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
    use crate::engine::CtxI;
    use crate::engine::new_ctx;
    use crate::input::strcursor::StrCursor;

    fn setup(input: &str) -> impl Ctx {
        new_ctx(StrCursor::new(input), &[])
    }

    #[test]
    fn test_skip_exp() {
        let mut ctx = setup("abc");
        let exp = Exp::token("abc");
        let new_ctx = Exp::skip_exp(ctx.push(), &exp);
        assert_eq!(new_ctx.cursor().mark(), 3);

        let mut ctx = setup("def");
        let new_ctx = Exp::skip_exp(ctx.push(), &exp);
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
        if let Ok(Yeap(final_ctx, _)) = Exp::repeat(ctx, &exp, &mut res) {
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
        if let Ok(Yeap(final_ctx, _)) = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, true) {
            assert_eq!(res.len(), 4);
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
        if let Ok(Yeap(final_ctx, _)) = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, false) {
            assert_eq!(res.len(), 2);
            assert_eq!(final_ctx.cursor().mark(), 8);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_restores_entered_cut() {
        let mut ctx = setup("abc abc abc");
        ctx.cut();
        assert!(ctx.cut_seen(), "ctx should have cut set before repeat");

        let exp = Exp::token("abc");
        let mut res = Vec::new();
        if let Ok(Yeap(final_ctx, _)) = Exp::repeat(ctx, &exp, &mut res) {
            assert_eq!(res.len(), 3);
            assert!(final_ctx.cut_seen(), "cut should be restored after repeat");
        } else {
            panic!("repeat failed")
        }
    }

    #[test]
    fn test_repeat_with_pre_restores_entered_cut() {
        let mut ctx = setup(",abc,abc");
        ctx.cut();
        assert!(
            ctx.cut_seen(),
            "ctx should have cut set before repeat_with_pre"
        );

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = Vec::new();
        if let Ok(Yeap(final_ctx, _)) = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, true) {
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
        if let Ok(Yeap(final_ctx, _)) = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, true) {
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
