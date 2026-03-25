impl<'a> From<Cst<'a>> for Box<Cst<'a>> {
    #[inline(always)]
    fn from(cst: Cst<'a>) -> Self {
        Box::new(cst)
    }
}
