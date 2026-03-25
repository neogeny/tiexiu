impl Model for Choice {
    fn parse(&self, mut ctx: Ctx) -> ParseResult {
        let mut last_error_msg = String::new();

        for option in &self.options {
            match option.parse(ctx) {
                Ok(res) => return Ok(res),
                Err((cut, offset, msg)) => {
                    if cut {
                        return Err((false, offset, msg));
                    }
                    // We don't necessarily need to 'max' here if
                    // ctx.max_offset is already doing the work!
                    last_error_msg = msg;
                }
            }
        }
        // Return the furthest point reached across ALL branches
        Err((false, ctx.max_offset, last_error_msg))
    }
}
