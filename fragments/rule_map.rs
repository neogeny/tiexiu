use std::collections::HashMap;
use std::sync::Arc;

/// The rule registry. Using Arc allows the Ctx to own it without copying the data.
pub type RuleMap<C> = Arc<HashMap<&'static str, Arc<dyn CanParse<C>>>>;

pub struct Ctx<C: Cursor> {
    pub cursor: C,
    pub rules: RuleMap<C>,
}

impl<C: Cursor> Ctx<C> {
    pub fn resolve(&self, name: &str) -> Arc<dyn CanParse<C>> {
        self.rules.get(name)
            .cloned()
            .unwrap_or_else(|| panic!("Rule {} not found", name))
    }
}
