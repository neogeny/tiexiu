impl Model for Choice {
    fn parse(&self, ctx: Ctx) -> Result<(Ctx, Cst), (bool, usize, String)> {
        let mut furthest_err: (bool, usize, String) = (false, 0, String::new());

        for option in &self.options {
            match option.parse(ctx) {
                Ok(res) => return Ok(res),
                Err((cut, offset, msg)) => {
                    // 1. If we hit a CUT, we stop everything and bubble up.
                    if cut {
                        // We reset 'cut' to false here because this Choice
                        // has now "consumed" the cut signal for its parent.
                        return Err((false, offset, msg));
                    }

                    // 2. Otherwise, we track the furthest error for reporting.
                    if offset >= furthest_err.1 {
                        furthest_err = (false, offset, msg);
                    }
                }
            }
        }
        Err(furthest_err)
    }
}
