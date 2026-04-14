use indexmap::IndexMap;

struct Grammar {
    // The key is the rule name, the index is the "ID"
    rule_registry: IndexMap<Box<str>, Rule>,
}

impl Grammar {
    fn get_id(&self, name: &str) -> Option<usize> {
        // Returns the index (the ID) without a string search later
        self.rule_registry.get_index_of(name)
    }

    fn get_by_id(&self, id: usize) -> Option<(&Box<str>, &Rule)> {
        // Resolution is just an array lookup
        self.rule_registry.get_index(id)
    }
}
