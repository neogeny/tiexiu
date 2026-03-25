impl Model for Group {
    fn children(&self) -> &[Box<dyn Model>] {
        // Rust idiom: slice::from_ref turns a single item into a 1-element slice
        std::slice::from_ref(&self.exp)
    }
}

impl Model for Sequence {
    fn children(&self) -> &[Box<dyn Model>] {
        &self.exps // Returns the whole slice of children
    }
}
