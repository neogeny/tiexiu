pub struct Rule<C: Cursor> {
    pub name: &'static str,
    pub exp: Arc<dyn CanParse<C>>,
}

pub struct Grammar<C: Cursor> {
    // The entire model in one contiguous chunk
    pub rules: Vec<Rule<C>>,
}

impl<C: Cursor> Grammar<C> {
    pub fn find(&self, name: &str) -> Option<Arc<dyn CanParse<C>>> {
        self.rules.iter()
            .find(|r| r.name == name)
            .map(|r| Arc::clone(&r.exp))
    }
}
