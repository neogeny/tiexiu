pub enum Model<'m, C: Ctx> {
    Group { exp: &'m Model<'m, C> },
    Sequence { list: Vec<&'m Model<'m, C>> },
    Token { val: &'static str },
    // ... all 32 variants here
}
