// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Exp;
use crate::context::Ctx;
use crate::peg::error::*;
use crate::trees::TreeList;
use crate::trees::short::NIL;

impl Exp {
    pub fn skip_exp<C: Ctx>(mut ctx: C, exp: &Exp) -> C {
        let skip_ctx = ctx.clone();
        match exp.parse_at(skip_ctx) {
            Ok(Yeap(new_ctx, _)) => {
                ctx.merge(&new_ctx);
                ctx
            }
            Err(_) => ctx,
        }
    }

    pub fn add_exp<C: Ctx>(mut ctx: C, exp: &Exp, res: &mut TreeList) -> Result<C, (C, Nope)> {
        match exp.parse_at(ctx.clone()) {
            Ok(Yeap(new_ctx, tree)) => {
                res.push_back(tree);
                ctx.merge(&new_ctx);
                Ok(ctx)
            }
            Err(nope) => Err((ctx, nope)),
        }
    }

    pub fn repeat<C: Ctx>(mut ctx: C, exp: &Exp, res: &mut TreeList) -> ParseResult {
        loop {
            let mark = ctx.mark();
            match exp.parse_at(ctx.push()) {
                Ok(Yeap(snap, tree)) => {
                    if snap.mark == mark {
                        return Err(ctx.failure(mark, ParseFailure::ClosureMatchedVoid()));
                    }
                    res.push_back(tree);
                    ctx.merge(&snap);
                }
                Err(_nope) => {
                    return Ok(yeap(ctx.into(), NIL.into()));
                }
            }
        }
    }

    pub fn repeat_with_pre<C: Ctx>(
        mut ctx: C,
        exp: &Exp,
        pre: &Exp,
        res: &mut TreeList,
        keep_pre: bool,
    ) -> ParseResult {
        loop {
            let mark = ctx.mark();
            match pre.parse_at(ctx.push()) {
                Err(mut nope) => {
                    if nope.take_cut() {
                        return Err(nope);
                    }
                    return Ok(yeap(ctx.into(), NIL.into()));
                }
                Ok(Yeap(snap, pre_cst)) => {
                    if snap.mark == mark {
                        return Err(ctx.failure(mark, ParseFailure::ClosureMatchedVoid()));
                    }
                    ctx.merge(&snap);
                    let mut inner_ctx = ctx.push();
                    inner_ctx.cut();
                    match exp.parse_at(inner_ctx) {
                        Ok(Yeap(repeat_ctx, exp_cst)) => {
                            if keep_pre {
                                res.push_back(pre_cst);
                            }
                            res.push_back(exp_cst);
                            ctx.merge(&repeat_ctx);
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
    use crate::context::CtxI;
    use crate::context::new_ctx;
    use crate::input::strcursor::StrCursor;

    fn setup(input: &str) -> impl Ctx {
        new_ctx(StrCursor::new(input), &[])
    }

    #[test]
    fn test_skip_exp() {
        let ctx = setup("abc");
        let exp = Exp::token("abc");
        let new_ctx = Exp::skip_exp(ctx.push(), &exp);
        assert_eq!(new_ctx.cursor().mark(), 3);

        let ctx = setup("def");
        let new_ctx = Exp::skip_exp(ctx.push(), &exp);
        assert_eq!(new_ctx.cursor().mark(), 0);
    }

    #[test]
    fn test_add_exp() {
        let ctx = setup("abc");
        let exp = Exp::token("abc");
        let mut res = TreeList::new();
        let result = Exp::add_exp(ctx, &exp, &mut res);
        assert!(result.is_ok());
        assert_eq!(res.len(), 1);
        assert_eq!(result.unwrap().cursor().mark(), 3);
    }

    #[test]
    fn test_repeat() {
        let ctx = setup("abcabcabc");
        let exp = Exp::token("abc");
        let mut res = TreeList::new();
        if let Ok(Yeap(_snap, _)) = Exp::repeat(ctx.push(), &exp, &mut res) {
            assert_eq!(res.len(), 3);
            assert_eq!(ctx.cursor().mark(), 9);
        } else {
            panic!("repeat  failed")
        }
    }

    #[test]
    fn test_repeat_with_pre() {
        let ctx = setup(",abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = TreeList::new();
        if let Ok(Yeap(_snap, _)) = Exp::repeat_with_pre(ctx.push(), &exp, &pre, &mut res, true) {
            assert_eq!(res.len(), 4);
            assert_eq!(ctx.cursor().mark(), 8);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_with_pre_no_keep() {
        let ctx = setup(",abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = TreeList::new();
        if let Ok(Yeap(_snap, _)) = Exp::repeat_with_pre(ctx.push(), &exp, &pre, &mut res, false) {
            assert_eq!(res.len(), 2);
            assert_eq!(ctx.cursor().mark(), 8);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_restores_entered_cut() {
        let mut ctx = setup("abcabcabc");
        ctx.cut();
        assert!(ctx.cut_seen(), "ctx should have cut set before repeat");

        let exp = Exp::token("abc");
        let mut res = TreeList::new();
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
        let mut res = TreeList::new();
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
        let mut res = TreeList::new();
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
