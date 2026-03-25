trait Model {
    // Every class implements this specifically
    fn children(&self) -> &[Box<dyn Model>];
}
