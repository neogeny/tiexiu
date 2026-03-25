impl Model for Closure {
    fn parse(&self, ctx: Ctx) -> ParseResult {
        // Pass the child's parse method as a closure
        let (next_ctx, results) = ctx.repeat(|c| self.exp.parse(c));

        Ok((next_ctx, Cst::Closed(results)))
    }
}
