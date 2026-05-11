// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Exp;
use crate::context::Ctx;
use crate::peg::error::*;
use crate::trees::TreeList;
use crate::trees::short::NIL;

impl Exp {
    pub fn add_exp<C: Ctx>(mut ctx: C, exp: &Exp, res: &mut TreeList) -> ParseResult {
        match exp.parse_at(ctx.clone()) {
            Ok(Yeap(mark, tree)) => {
                res.push_back(tree.clone());
                ctx.merge(mark);
                Ok(yeap(ctx.into(), tree))
            }
            err => err,
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
                    ctx.merge(snap);
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
                    ctx.merge(snap);
                    let inner_ctx = ctx.push();
                    // inner_ctx.cut();
                    match exp.parse_at(inner_ctx) {
                        Ok(Yeap(mark, exp_cst)) => {
                            if keep_pre {
                                res.push_back(pre_cst);
                            }
                            res.push_back(exp_cst);
                            ctx.merge(mark);
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
    fn test_add_exp() {
        let mut ctx = setup("abc");
        let exp = Exp::token("abc");
        let mut res = TreeList::new();
        let result = Exp::add_exp(ctx.clone(), &exp, &mut res);
        assert!(result.is_ok());
        if let Ok(Yeap(mark, _)) = result {
            ctx.merge(mark);
        }
        assert_eq!(res.len(), 1);
        assert_eq!(ctx.cursor().mark(), 3);
    }

    #[test]
    fn test_repeat() {
        let mut ctx = setup("abcabcabc");
        let exp = Exp::token("abc");
        let mut res = TreeList::new();
        if let Ok(Yeap(mark, _)) = Exp::repeat(ctx.push(), &exp, &mut res) {
            ctx.merge(mark);
            assert_eq!(res.len(), 3);
            assert_eq!(ctx.cursor().mark(), 9);
        } else {
            panic!("repeat  failed")
        }
    }

    #[test]
    fn test_repeat_with_pre() {
        let mut ctx = setup(",abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = TreeList::new();
        if let Ok(Yeap(mark, _)) = Exp::repeat_with_pre(ctx.push(), &exp, &pre, &mut res, true) {
            ctx.merge(mark);
            assert_eq!(res.len(), 4);
            assert_eq!(ctx.cursor().mark(), 8);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_with_pre_no_keep() {
        let mut ctx = setup(",abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = TreeList::new();
        if let Ok(Yeap(mark, _)) = Exp::repeat_with_pre(ctx.push(), &exp, &pre, &mut res, false) {
            ctx.merge(mark);
            assert_eq!(res.len(), 2);
            assert_eq!(ctx.cursor().mark(), 8);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    #[ignore = "Ctx.cutseen is being removed"]
    fn test_repeat_restores_entered_cut() {
        let mut ctx = setup("abcabcabc");
        ctx._cut();
        // FIXME
        // assert!(ctx.cut_seen(), "ctx should have cut set before repeat");

        let exp = Exp::token("abc");
        let mut res = TreeList::new();
        if let Ok(Yeap(snap, _)) = Exp::repeat(ctx.clone(), &exp, &mut res) {
            ctx.merge(snap.clone());
            assert_eq!(res.len(), 3);
            assert!(snap.cut_seen(), "cut should be restored after repeat");
        } else {
            panic!("repeat failed")
        }
    }

    #[test]
    #[ignore = "Ctx.cutseen is being removed"]
    fn test_repeat_with_pre_restores_entered_cut() {
        let mut ctx = setup(",abc,abc");
        ctx._cut();
        // FIXME
        // assert!(
        //     ctx.cut_seen(),
        //     "ctx should have cut set before repeat_with_pre"
        // );

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = TreeList::new();
        if let Ok(Yeap(snap, _)) = Exp::repeat_with_pre(ctx.clone(), &exp, &pre, &mut res, true) {
            ctx.merge(snap.clone());
            assert_eq!(res.len(), 4);
            assert!(
                snap.cut_seen(),
                "cut should be restored after repeat_with_pre"
            );
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_with_pre_no_cut_enters_clears() {
        let mut ctx = setup(",abc,abc");
        // FIXME
        // assert!(
        //     !ctx.cut_seen(),
        //     "ctx should not have cut set before repeat_with_pre"
        // );

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = TreeList::new();
        if let Ok(Yeap(snap, _)) = Exp::repeat_with_pre(ctx.clone(), &exp, &pre, &mut res, true) {
            ctx.merge(snap.clone());
            assert_eq!(res.len(), 4);
            assert!(
                !snap.cut_seen(),
                "cut should be cleared when not set on entry"
            );
        } else {
            panic!("repeat_with_pre failed")
        }
    }
}
