// peg/mod.rs

pub enum ModelImpl {
    Sequence(Sequence),
    Choice(Choice),
    Token(Token),
    // ... other 31 variants
}

// The dispatcher: delegating to the inner struct's trait implementation
impl Model for ModelImpl {
    fn parse<C: Cursor>(&self, ctx: Ctx<C>) -> ParseResult<C> {
        match self {
            Self::Sequence(m) => m.parse(ctx),
            Self::Choice(m) => m.parse(ctx),
            Self::Token(m) => m.parse(ctx),
            // ...
        }
    }
}

// peg/sequence.rs
pub struct Sequence {
    pub children: Vec<Box<ModelImpl>>,
}

impl Model for Sequence {
    fn parse<C: Cursor>(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        let mut result = Cst::Nil;
        for child in &self.children {
            let (next_ctx, child_cst) = child.parse(ctx)?;
            ctx = next_ctx;
            result = result.merge(child_cst);
        }
        Ok((ctx, result))
    }
}
