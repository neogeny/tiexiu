// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};


pub struct Closure<M> {
    pub exp: Box<M>,
}

pub struct PositiveClosure<M> {
    pub exp: Box<M>,
}

pub struct Join<M> {
    pub exp: Box<M>,
    pub sep: Box<M>,
}



fn add_result<M, C>(results: &mut Vec<Cst>, exp: &M, ctx: Ctx<C>) -> Result<Ctx<C>, (Ctx<C>, String)>
where
    M: CanParse<C>,
    C: Cursor,
{
        match exp.parse(ctx) {
            Ok((new_ctx, cst)) => {
                results.push(cst);
                Ok(new_ctx)
            }
            Err(err) => Err(err)
        }
}


fn repeat<M, C>(results: &mut Vec<Cst>, exp: &M, mut ctx: Ctx<C>) -> Ctx<C>
where
    M: CanParse<C>,
    C: Cursor,
{
    loop {
        match add_result(results, exp, ctx) {
            Ok(new_ctx) => ctx = new_ctx,
            Err((new_ctx, _)) => return new_ctx
        }
    }
}


// fn repeat_with_prefix<M, C>(results: &mut Vec<Cst>, pre: &M, exp: &M, mut
// ctx: Ctx<C>) -> ParseResult<C>
// where
//     M: CanParse<C>,
//     C: Cursor,
// {
//     loop {
//         match pre.parse(ctx.clone()) {
//             Ok((new_ctx, _)) => {
//                 ctx = new_ctx;
//             },
//             Err(_) => {
//                 return Ok((ctx, Cst::from(results)))
//             }
//         };
//
//         match add_result(&mut results, exp, ctx) {
//             Ok(new_ctx) => ctx = new_ctx,
//             Err(new_ctx) => return Ok((new_ctx, Cst::from(results)))
//         }
//     }
// }
//
//

impl<M, C> CanParse<C> for Closure<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        let mut results = Vec::new();
        let new_ctx = repeat(&mut results, &*self.exp, ctx);
        Ok((new_ctx, Cst::from(results)))
    }
}

impl<M, C> CanParse<C> for PositiveClosure<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        let mut results: Vec<Cst> = Vec::new();

        match self.exp.parse(ctx.clone()) {
            Ok((new_ctx, cst)) => {
                ctx = new_ctx;
                results.push(cst);
            },
            err => return err
        };

        let new_ctx = repeat(&mut results, &*self.exp, ctx.clone());
        Ok((new_ctx, Cst::from(results)))
    }
}

// impl<M, C> CanParse<C> for Join<M>
// where
//     M: CanParse<C>,
//     C: Cursor,
// {
//     fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
//         repeat(&*self.exp, ctx)
//     }
// }
