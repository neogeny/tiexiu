// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use super::parser::S;
use crate::state::Ctx;
use crate::trees::Tree;

impl Exp {
    pub fn skip_exp<C: Ctx>(ctx: C, exp: &Exp) -> C {
        match exp.parse(ctx.clone()) {
            Ok(S(new_ctx, _)) => new_ctx,
            Err(_) => ctx,
        }
    }

    pub fn add_exp<C: Ctx>(ctx: C, exp: &Exp, res: &mut Vec<Tree>) -> Result<C, C> {
        match exp.parse(ctx.clone()) {
            Ok(S(new_ctx, tree)) => {
                res.push(tree);
                Ok(new_ctx)
            }
            Err(_) => Err(ctx),
        }
    }

    pub fn repeat<C: Ctx>(mut ctx: C, exp: &Exp, res: &mut Vec<Tree>) -> C {
        // WARNING
        // TODO: Cut management needs to be implemented
        loop {
            match Self::add_exp(ctx.clone(), exp, res) {
                Ok(new_ctx) => ctx = new_ctx,
                Err(ctx) => return ctx,
            }
        }
    }

    pub fn repeat_with_pre<C: Ctx>(
        mut ctx: C,
        exp: &Exp,
        pre: &Exp,
        res: &mut Vec<Tree>,
        keep_pre: bool,
    ) -> C {
        // WARNING
        // TODO: Cut management needs to be implemented
        loop {
            match pre.parse(ctx.clone()) {
                Err(_) => return ctx,
                Ok(S(new_ctx, pre_cst)) => match exp.parse(new_ctx) {
                    Err(_) => return ctx,
                    Ok(S(repeat_ctx, exp_cst)) => {
                        if keep_pre {
                            res.push(pre_cst);
                        }
                        res.push(exp_cst);
                        ctx = repeat_ctx;
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::strcursor::StrCursor;
    use crate::peg::Grammar;
    use crate::state::corectx::CoreCtx;

    fn setup(input: &str) -> CoreCtx<'_, StrCursor<'_>> {
        let grammar = Box::leak(Box::new(Grammar::default()));
        let _ = grammar;
        CoreCtx::new(StrCursor::new(input))
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
        let ctx = setup("abcabcabc");
        let exp = Exp::token("abc");
        let mut res = Vec::new();
        let final_ctx = Exp::repeat(ctx, &exp, &mut res);
        assert_eq!(res.len(), 3);
        assert_eq!(final_ctx.cursor().mark(), 9);
    }

    #[test]
    fn test_repeat_with_pre() {
        let ctx = setup(",abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = Vec::new();
        let final_ctx = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, true);
        assert_eq!(res.len(), 4); // [",", "abc", ",", "abc"]
        assert_eq!(final_ctx.cursor().mark(), 8);
    }

    #[test]
    fn test_repeat_with_pre_no_keep() {
        let ctx = setup(",abc,abc");
        let exp = Exp::token("abc");
        let pre = Exp::token(",");
        let mut res = Vec::new();
        let final_ctx = Exp::repeat_with_pre(ctx, &exp, &pre, &mut res, false);
        assert_eq!(res.len(), 2); // ["abc", "abc"]
        assert_eq!(final_ctx.cursor().mark(), 8);
    }
}
