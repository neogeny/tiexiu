use crate::astree::Cst;
use crate::context::Ctx;
use crate::context::memo::{Key, Memo, MemoCache};
use crate::input::Cursor;
use crate::model::Grammar;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type RegexCache = HashMap<String, Regex>;

#[derive(Clone, Debug)]
pub struct State<U: Cursor> {
    pub cursor: U,
    pub cutseen: bool,
}

#[derive(Clone, Debug)]
pub struct HeavyState<'c> {
    pub grammar: &'c Grammar,
    pub memos: MemoCache,
    pub regex: RegexCache,
}

#[derive(Clone, Debug)]
pub struct CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub state: Rc<State<U>>,
    pub heavy: Rc<RefCell<HeavyState<'c>>>,
}

impl<'c, U> CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    pub fn new(cursor: U, grammar: &'c Grammar) -> Self {
        Self {
            state: State {
                cursor,
                cutseen: false,
            }
            .into(),
            heavy: RefCell::new(HeavyState {
                grammar,
                memos: MemoCache::new(),
                regex: RegexCache::new(),
            })
            .into(),
        }
    }

    #[inline]
    fn state_mut(&mut self) -> &mut State<U> {
        Rc::make_mut(&mut self.state)
    }

    fn with_memos_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut MemoCache) -> R,
    {
        let mut heavy = self.heavy.borrow_mut();
        f(&mut heavy.memos)
    }

    fn with_regex_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut RegexCache) -> R,
    {
        let mut heavy = self.heavy.borrow_mut();
        f(&mut heavy.regex)
    }
}

impl<'c, U> Ctx for CoreCtx<'c, U>
where
    U: Cursor + Clone,
{
    fn grammar(&self) -> &Grammar {
        self.heavy.borrow().grammar
    }

    #[inline]
    fn cursor(&self) -> &dyn Cursor {
        &self.state.cursor
    }

    #[inline]
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        &mut self.state_mut().cursor
    }

    fn regex(&self, pattern: &str) -> Regex {
        self.with_regex_mut(|regex| {
            regex
                .entry(pattern.to_string())
                .or_insert_with(|| Regex::new(pattern).unwrap())
                .clone()
        })
    }

    fn memo(&mut self, key: &Key) -> Option<Memo> {
        self.with_memos_mut(|cache| cache.memo(key))
    }

    fn memoize(&mut self, key: &Key, cst: &Cst) {
        let mark = self.mark();
        self.with_memos_mut(|cache| {
            cache.memoize(key, cst, mark);
        });
    }

    #[inline]
    fn cut_seen(&self) -> bool {
        self.state.cutseen
    }
    #[inline]
    fn uncut(&mut self) {
        self.state_mut().cutseen = false;
    }

    fn cut(&mut self) {
        self.state_mut().cutseen = true;
        self.prune_cache();
    }

    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_memos_mut(|cache| cache.prune(cutpoint));
    }
}
