pub enum ModelImpl {
    Sequence(Sequence<ModelImpl>),
    Choice(Choice<ModelImpl>),
    Token(Token),
    // ...
}

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

pub struct Sequence<M: Model> {
    pub children: Vec<Box<M>>,
}

impl<M: Model> Model for Sequence<M> {
    fn parse<C: Cursor>(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        let mut result = Cst::Nil;
        for child in &self.children {
            // Static dispatch through the Box<M>
            let (next_ctx, child_cst) = child.parse(ctx)?;
            ctx = next_ctx;
            result = result.merge(child_cst);
        }

        // Structural reduction (List of 1 -> Item, List of 0 -> Nil)
        let final_cst = match result {
            Cst::List(mut elements) if elements.len() == 1 => {
                *elements.pop().unwrap()
            }
            Cst::List(elements) if elements.is_empty() => {
                Cst::Nil
            }
            other => other,
        };

        Ok((ctx, final_cst))
    }
}
