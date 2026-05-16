// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::Ctx;
use crate::peg::error::*;
use crate::trees::TreeList;
use crate::trees::short::NIL;
use crate::{Exp, Tree};

impl Exp {
    pub fn add_exp<C: Ctx>(mut ctx: C, exp: &Exp, res: &mut TreeList) -> ParseResult {
        match exp.parse_at(ctx.clone()) {
            Ok(Yeap(snap, tree)) => {
                res.push_back(tree.clone());
                ctx.merge(&snap);
                Ok(yeap(&ctx.into(), tree))
            }
            err => err,
        }
    }

    pub fn repeat<C: Ctx>(mut ctx: C, exp: &Exp, positive: bool) -> ParseResult {
        let mut res = TreeList::new();
        if positive {
            ctx.push_cut();
            let result = exp.parse_at(ctx.push());
            ctx.take_cut();
            match result {
                Err(nope) => return Err(nope),
                Ok(Yeap(snap, tree)) => {
                    ctx.merge(&snap);
                    res.push_back(tree);
                }
            }
        }

        loop {
            let mark = ctx.mark();
            ctx.push_cut();
            let result = exp.parse_at(ctx.push());
            let cutseen = ctx.take_cut();
            match result {
                Ok(Yeap(snap, tree)) => {
                    if snap.mark == mark {
                        return Err(ctx.failure(mark, ParseFailure::ClosureMatchedVoid()));
                    }
                    res.push_back(tree);
                    ctx.merge(&snap);
                }
                Err(nope) => {
                    if cutseen {
                        return Err(nope);
                    }
                    return Ok(yeap(&ctx.into(), Tree::from(res).into()));
                }
            }
        }
    }

    pub fn repeat_with_sep<C: Ctx>(
        mut ctx: C,
        exp: &Exp,
        sep: &Exp,
        positive: bool,
        keep_pre: bool,
    ) -> ParseResult {
        let mut res = TreeList::new();
        ctx.push_cut();
        let result = exp.parse_at(ctx.push());
        let cutseen = ctx.take_cut();
        match result {
            Err(nope) => {
                if positive || cutseen {
                    return Err(nope);
                }
                return Ok(yeap(&ctx.click(), NIL.into()));
            }
            Ok(Yeap(snap, tree)) => {
                ctx.merge(&snap);
                res.push_back(tree);
            }
        }
        loop {
            let mark = ctx.mark();
            ctx.push_cut();
            let result = sep.parse_at(ctx.push());
            let cutseen = ctx.take_cut();
            match result {
                Err(nope) => {
                    if cutseen {
                        return Err(nope);
                    }
                    return Ok(yeap(&ctx.into(), Tree::from(res).into()));
                }
                Ok(Yeap(snap, pre_tree)) => {
                    if snap.mark == mark {
                        return Err(ctx.failure(mark, ParseFailure::ClosureMatchedVoid()));
                    }
                    ctx.merge(&snap);
                    ctx.push_cut();
                    let result = exp.parse_at(ctx.push());
                    ctx.take_cut();
                    match result {
                        Ok(Yeap(snap, tree)) => {
                            ctx.merge(&snap);
                            if keep_pre {
                                res.push_back(pre_tree);
                            }
                            res.push_back(tree);
                        }
                        Err(nope) => {
                            // NOTE: must see exp after pre
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
        if let Ok(Yeap(snap, _)) = result {
            ctx.merge(&snap);
        }
        assert_eq!(res.len(), 1);
        assert_eq!(ctx.cursor().mark(), 3);
    }

    #[test]
    fn test_repeat() {
        let mut ctx = setup("abcabcabc");
        let exp = Exp::token("abc");
        if let Ok(Yeap(snap, _tree)) = Exp::repeat(ctx.push(), &exp, false) {
            ctx.merge(&snap);
            // FIXME
            // assert_eq!(res.len(), 3);
            assert_eq!(ctx.cursor().mark(), 9);
        } else {
            panic!("repeat  failed")
        }
    }

    #[test]
    fn test_repeat_with_sep() {
        let mut ctx = setup("abc,abc,abc");
        let exp = Exp::token("abc");
        let sep = Exp::token(",");
        if let Ok(Yeap(snap, tree)) = Exp::repeat_with_sep(ctx.push(), &exp, &sep, false, true) {
            ctx.merge(&snap);
            assert_eq!(tree.width(), 11);
            assert_eq!(ctx.cursor().mark(), 11);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_with_sep_no_keep() {
        let mut ctx = setup("abc,abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        if let Ok(Yeap(snap, tree)) = Exp::repeat_with_sep(ctx.push(), &exp, &pre, false, false) {
            ctx.merge(&snap);
            assert_eq!(tree.width(), 9);
            assert_eq!(ctx.cursor().mark(), 11);
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    #[ignore = "Ctx.cutseen is being removed"]
    fn test_repeat_restores_entered_cut() {
        let mut ctx = setup("abcabcabc");
        ctx.cut();
        assert!(ctx.cut_seen(), "ctx should have cut set before repeat");

        let exp = Exp::token("abc");
        if let Ok(Yeap(snap, _)) = Exp::repeat(ctx.clone(), &exp, false) {
            ctx.merge(&snap);
            // FIXME
            // assert_eq!(res.len(), 3);
            assert!(ctx.cut_seen(), "cut should be restored after repeat");
        } else {
            panic!("repeat failed")
        }
    }

    #[test]
    #[ignore = "Ctx.cutseen is being removed"]
    fn test_repeat_with_pre_restores_entered_cut() {
        let mut ctx = setup(",abc,abc");
        ctx.cut();
        // FIXME
        // assert!(
        //     ctx.cut_seen(),
        //     "ctx should have cut set before repeat_with_pre"
        // );

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        if let Ok(Yeap(snap, _)) = Exp::repeat_with_sep(ctx.clone(), &exp, &pre, false, true) {
            ctx.merge(&snap);
            // FIXME
            // assert_eq!(res.len(), 4);
            assert!(
                ctx.cut_seen(),
                "cut should be restored after repeat_with_pre"
            );
        } else {
            panic!("repeat_with_pre failed")
        }
    }

    #[test]
    fn test_repeat_with_pre_no_cut_enters_clears() {
        let mut ctx = setup(",abc,abc");
        assert!(
            !ctx.cut_seen(),
            "ctx should not have cut set before repeat_with_pre"
        );

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        if let Ok(Yeap(snap, _)) = Exp::repeat_with_sep(ctx.clone(), &exp, &pre, false, true) {
            ctx.merge(&snap);
            // FIXME
            // assert_eq!(res.len(), 4);
            assert!(
                !ctx.cut_seen(),
                "cut should be cleared when not set on entry"
            );
        } else {
            panic!("repeat_with_pre failed")
        }
    }
}
