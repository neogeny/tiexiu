pub enum Transformer<'a> {
    Named(&'a Str),
    NamedList(&'a Str),
    Override,
    OverrideList,
}

impl<'a> Transformer<'a> {
    /// Apply the transformation to the completed tree.
    pub fn apply(self, tree: Rc<Tree>) -> Rc<Tree> {
        match self {
            Self::Named(name) => Tree::named(name, tree).into(),
            Self::NamedList(name) => Tree::named_as_list(name, tree).into(),
            Self::Override => Tree::override_with(tree).into(),
            Self::OverrideList => Tree::override_as_list(tree).into(),
        }
    }
}

  fn do_parse<C: Ctx>(&self, mut ctx: C) -> ParseResult<C> {
        let start = ctx.mark();
        let mut exp = self;

        // Contiguous, stack-allocated vector of instructions
        let mut transforms = Vec::new();

        loop {
            // Wrapper flattening
            while let ExpKind::RuleInclude { .. } | ExpKind::Group(_) | ExpKind::Alt(_) = &exp.kind {
                match &exp.kind {
                    ExpKind::Group(next) | ExpKind::Alt(next) => exp = next,
                    ExpKind::RuleInclude { name, exp: opt_exp } => match opt_exp {
                        None => return Err(ctx.failure(start, RuleNotLinked(name.clone()))),
                        Some(next) => exp = next,
                    },
                    _ => break,
                }
            }

            match &exp.kind {
                // Instantly construct and push the instruction, then move to inner expression
                ExpKind::Named(name, inner) => {
                    transforms.push(Transformer::Named(name));
                    exp = inner;
                    continue;
                }
                ExpKind::NamedList(name, inner) => {
                    transforms.push(Transformer::NamedList(name));
                    exp = inner;
                    continue;
                }
                ExpKind::Override(inner) => {
                    transforms.push(Transformer::Override);
                    exp = inner;
                    continue;
                }
                ExpKind::OverrideList(inner) => {
                    transforms.push(Transformer::OverrideList);
                    exp = inner;
                    continue;
                }

                leaf_kind => {
                    let mut parse_res = match leaf_kind {
                        // All terminals (Nil, Constant, Token, etc.) or complex recursions
                        _ => exp.parse(ctx),
                    };

                    // On success, unwind the transform stack to build the final tree
                    if let Ok(Yeap(ref mut final_ctx, ref mut tree)) = parse_res {
                        while let Some(transform) = transforms.pop() {
                            *tree = transform.apply(tree.clone());
                        }
                    }

                    return parse_res;
                }
            }
        }
    }
