use crate::grammars::Parser;
use crate::contexts::Ctx;
use crate::contexts::memo::Cache;
use crate::grammars::Rule;
use crate::input::Cursor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct CoreCtx<'c, C>
where
    C: Cursor,
{
    pub cursor: C,
    pub cutseen: bool,
    pub rulemap: &'c HashMap<&'c str, Rule<'c>>,
    pub cache: Rc<RefCell<Cache>>,
}

impl<'c, C> CoreCtx<'c, C>
where
    C: Cursor,
{
    pub fn new(cursor: C, rulemap: &'c HashMap<&'c str, Rule<'c>>) -> Self {
        Self {
            cursor,
            cutseen: false,
            rulemap,
            cache: Rc::new(RefCell::new(Cache::new())),
        }
    }
}

impl<'c, C> Ctx for CoreCtx<'c, C>
where
    C: Cursor + Clone,
{
    fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Cache) -> R,
    {
        let mut cache = self.cache.borrow_mut();
        f(&mut cache)
    }

    fn cursor(&self) -> &dyn Cursor {
        &self.cursor
    }

    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        &mut self.cursor
    }

    fn cut_seen(&self) -> bool {
        self.cutseen
    }

    fn cut(&mut self) {
        self.cutseen = true;
        self.prune_cache();
    }

    fn uncut(&mut self) {
        self.cutseen = false;
    }

    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_cache_mut(|cache| cache.prune(cutpoint));
    }

    fn parser_for(self, name: &str) -> (Self, &dyn Parser<Self>) {
        let rule = self.rulemap
            .get(name)
            .unwrap_or_else(|| panic!("rule '{}' not found", name));
        (self, rule.rhs)
    }
}