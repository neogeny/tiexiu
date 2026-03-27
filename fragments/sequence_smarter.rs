// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

pub struct Sequence<M, C>
where
    M: Model<C>,
    C: Cursor
{
    pub children: Vec<Box<M>>,
    _phantom: std::marker::PhantomData<C>,
}

impl<M, C> Model<C> for Sequence<M, C>
where
    M: Model<C>,
    C: Cursor
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        let mut elements = Vec::with_capacity(self.children.len());

        for child in &self.children {
            let (next_ctx, child_cst) = child.parse(ctx)?;
            ctx = next_ctx;

            match child_cst {
                Cst::Nil => continue,
                other => elements.push(Box::new(other)),
            }
        }

        // Always return a List.
        // Rule::parse() will decide if this needs to be an Ast or a singleton.
        Ok((ctx, Cst::List(elements)))
    }
}
