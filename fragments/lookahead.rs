impl Model for PositiveLookahead {
    fn parse(&self, ctx: Ctx) -> Result<(Ctx, Cst), String> {
        // We 'try' the inner expression with a copy of the state
        self.expr.parse(ctx)?;

        // If it succeeds, we return the ORIGINAL ctx.
        // We "saw" the match, but we didn't "consume" any input.
        Ok((ctx, Cst::Void))
    }
}
