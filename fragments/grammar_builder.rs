use std::collections::HashMap;
use std::sync::Arc;

pub struct GrammarBuilder<C: Cursor> {
    rules: HashMap<&'static str, Arc<dyn CanParse<C>>>,
}

impl<C: Cursor> GrammarBuilder<C> {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Adds a rule to the registry.
    /// The rule is wrapped in an Arc here so the rest of the system can share it.
    pub fn add<M>(&mut self, name: &'static str, rule: M)
    where
        M: CanParse<C> + 'static,
    {
        self.rules.insert(name, Arc::new(rule));
    }

    /// "Seals" the grammar into a RuleMap that Ctx can carry.
    pub fn finish(self) -> RuleMap<C> {
        Arc::new(self.rules)
    }
}
