// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::CtxSem;
use crate::peg::error::*;
use crate::trees::TreeList;
use crate::trees::tree::NIL;
use crate::{Exp, Tree};

impl Exp {
    /// Parses zero or more (`closure`) or one or more (`positive_closure`) repetitions.
    pub fn repeat<C: CtxSem>(ctx: &mut C, exp: &Exp, positive: bool) -> ParseResult {
        let mut res = TreeList::new();
        if positive {
            ctx.push_cut();
            let result = exp.parse_at(ctx);
            ctx.take_cut();
            match result {
                Err(nope) => return Err(nope),
                Ok(tree) => {
                    res.push_back(tree);
                }
            }
        }

        loop {
            let mark = ctx.mark();
            ctx.push_cut();
            let result = exp.parse_at(ctx);
            let cutseen = ctx.take_cut();
            match result {
                Ok(tree) => {
                    if ctx.mark() == mark {
                        return Err(ctx.failure(mark, ParseFailure::ClosureMatchedVoid()));
                    }
                    res.push_back(tree);
                }
                Err(nope) => {
                    if cutseen {
                        return Err(nope);
                    }
                    return Ok(Tree::from(res).into());
                }
            }
        }
    }

    /// Parses repetitions separated by a separator expression (`join` / `gather`).
    pub fn repeat_with_sep<C: CtxSem>(
        ctx: &mut C,
        exp: &Exp,
        sep: &Exp,
        positive: bool,
        keep_sep: bool,
    ) -> ParseResult {
        let mut res = TreeList::new();
        ctx.push_cut();
        let result = exp.parse_at(ctx);
        let cutseen = ctx.take_cut();
        match result {
            Err(nope) => {
                if positive || cutseen {
                    return Err(nope);
                }
                return Ok(NIL.into());
            }
            Ok(tree) => {
                res.push_back(tree);
            }
        }
        loop {
            let mark = ctx.mark();
            ctx.push_cut();
            let result = sep.parse_at(ctx);
            let cutseen = ctx.take_cut();
            match result {
                Err(nope) => {
                    if cutseen {
                        return Err(nope);
                    }
                    return Ok(Tree::from(res).into());
                }
                Ok(pre_tree) => {
                    if ctx.mark() == mark {
                        return Err(ctx.failure(mark, ParseFailure::ClosureMatchedVoid()));
                    }
                    ctx.push_cut();
                    let result = exp.parse_at(ctx);
                    ctx.take_cut();
                    match result {
                        Ok(tree) => {
                            if keep_sep {
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
    use crate::context::Ctx;
    use crate::context::new_ctx;
    use crate::input::strcursor::StrCursor;

    fn setup(input: &str) -> impl CtxSem {
        let cursor = StrCursor::new(input);
        new_ctx(cursor, &[])
    }

    #[test]
    fn test_repeat() {
        let mut ctx = setup("abcabcabc");
        assert!(!ctx.cursor().name_guard());
        let exp = Exp::token("abc");
        if let Ok(_tree) = Exp::repeat(&mut ctx, &exp, false) {
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
        if let Ok(tree) = Exp::repeat_with_sep(&mut ctx, &exp, &sep, false, true) {
            assert_eq!(tree.width(), 11);
            assert_eq!(ctx.cursor().mark(), 11);
        } else {
            panic!("repeat_with_sep failed")
        }
    }

    #[test]
    fn test_repeat_with_sep_no_keep() {
        let mut ctx = setup("abc,abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        if let Ok(tree) = Exp::repeat_with_sep(&mut ctx, &exp, &pre, false, false) {
            assert_eq!(tree.width(), 9);
            assert_eq!(ctx.cursor().mark(), 11);
        } else {
            panic!("repeat_with_sep failed")
        }
    }

    #[test]
    #[ignore = "Ctx.cutseen is being removed"]
    fn test_repeat_restores_entered_cut() {
        let mut ctx = setup("abcabcabc");
        ctx.cut();
        assert!(ctx.cut_seen(), "ctx should have cut set before repeat");

        let exp = Exp::token("abc");
        assert!(Exp::repeat(&mut ctx, &exp, false).is_ok());
        assert!(ctx.cut_seen(), "cut should be restored after repeat");
    }

    #[test]
    #[ignore = "Ctx.cutseen is being removed"]
    fn test_repeat_with_sep_restores_entered_cut() {
        let mut ctx = setup(",abc,abc");
        ctx.cut();

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        assert!(Exp::repeat_with_sep(&mut ctx, &exp, &pre, false, true).is_ok());
        assert!(
            ctx.cut_seen(),
            "cut should be restored after repeat_with_sep"
        );
    }

    #[test]
    fn test_repeat_with_sep_no_cut_enters_clears() {
        let mut ctx = setup(",abc,abc");
        assert!(
            !ctx.cut_seen(),
            "ctx should not have cut set before repeat_with_sep"
        );

        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        assert!(Exp::repeat_with_sep(&mut ctx, &exp, &pre, false, true).is_ok());
        assert!(
            !ctx.cut_seen(),
            "cut should be cleared when not set on entry"
        );
    }
}
