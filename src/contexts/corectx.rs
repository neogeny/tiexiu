use crate::contexts::Ctx;
use crate::contexts::memo::Cache;
use crate::input::Cursor;
use std::cell::RefCell;
use std::rc::Rc;
use crate::grammars::Grammar;

#[derive(Clone, Debug)]
pub struct CoreCtx<'c, C>
where
    C: Cursor,
{
    pub cursor: C,
    pub cutseen: bool,
    pub grammar: &'c Grammar,
    pub cache: Rc<RefCell<Cache>>,
}

impl<'c, C> CoreCtx<'c, C>
where
    C: Cursor,
{
    pub fn new(cursor: C, grammar: &'c Grammar) -> Self {
        Self {
            cursor,
            cutseen: false,
            grammar,
            cache: Rc::new(RefCell::new(Cache::new())),
        }
    }
}

impl<'c, C> Ctx for CoreCtx<'c, C>
where
    C: Cursor + Clone,
{
    fn grammar(&self) -> &Grammar {
        self.grammar
    }

    #[inline]
    fn cursor(&self) -> &dyn Cursor {
        &self.cursor
    }

    #[inline]
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        &mut self.cursor
    }

    fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Cache) -> R,
    {
        let mut cache = self.cache.borrow_mut();
        f(&mut cache)
    }

    #[inline]
    fn cut_seen(&self) -> bool {
        self.cutseen
    }

    #[inline]
    fn uncut(&mut self) {
        self.cutseen = false;
    }

    fn cut(&mut self) {
        self.cutseen = true;
        self.prune_cache();
    }
    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_cache_mut(|cache| cache.prune(cutpoint));
    }
}
