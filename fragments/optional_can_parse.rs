pub trait CanParse<C: Cursor> {
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C>;
}

impl<M, C> CanParse<C> for Optional<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        // We clone the state to backtrack if the body fails
        match self.body.parse(ctx.clone()) {
            Ok(success) => Ok(success),
            Err((err_ctx, _)) => {
                if err_ctx.cut_seen {
                    // Even an Optional fails if it hits a Cut
                    return Err((err_ctx, "Cut in optional".to_string()));
                }
                // Otherwise, Optional "succeeds" with Nil at the original position
                Ok((ctx, Cst::Nil))
            }
        }
    }
}
