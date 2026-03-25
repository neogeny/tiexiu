pub struct Ctx<'a> {
    pub rest: &'a str,
    pub full_text: &'a str,
    pub max_offset: usize, // The furthest we've EVER successfully matched
}

impl<'a> Ctx<'a> {
    #[inline]
    pub fn advance(mut self, n: usize) -> Self {
        let new_rest = &self.rest[n..];
        let current_offset = self.full_text.len() - new_rest.len();

        // Update the high-water mark
        if current_offset > self.max_offset {
            self.max_offset = current_offset;
        }

        self.rest = new_rest;
        self
    }
}
