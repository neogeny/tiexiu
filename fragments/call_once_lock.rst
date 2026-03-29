use std::sync::OnceLock;

pub struct Call<M> {
    pub name: &'static str,
    // OnceLock allows us to set the Box<M> even with a &self reference
    pub exp: OnceLock<Box<M>>,
}

impl<M> Call<M> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            exp: OnceLock::new(),
        }
    }
}

impl<M, C> CanParse<C> for Call<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        // 1. Try to get the cached rule.
        // 2. If it's empty, use the closure to resolve it from Ctx and save it.
        let rule = self.exp.get_or_init(|| {
            ctx.resolve_rule_to_box::<M>(self.name)
        });

        rule.parse(ctx)
    }
}
