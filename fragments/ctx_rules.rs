impl<C: Cursor> Ctx<C> {
    /// Creates a new state for a specific input string (via cursor)
    /// and a pre-loaded set of rules.
    pub fn new(cursor: C, rules: RuleMap<C>) -> Self {
        Self { cursor, rules }
    }

    /// A helper to create a "Child" state (e.g., for branching)
    /// that shares the same rules.
    pub fn branch(&self, new_cursor: C) -> Self {
        Self {
            cursor: new_cursor,
            rules: Arc::clone(&self.rules),
        }
    }
}
