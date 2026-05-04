// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::ctx::{Ctx, CtxI};
use super::memo::{Key, Memo};
use crate::cfg::*;
use crate::engine::state::CallStack;
use crate::engine::trace::Tracer;
use crate::input::Cursor;
use crate::trees::Tree;
use crate::util::pyre::Pattern;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct CtxProxy<C: Ctx> {
    pub inner: Rc<RefCell<C>>,
    frozen: Option<usize>,
}

impl<C: Ctx> Clone for CtxProxy<C> {
    fn clone(&self) -> Self {
        let mut cloned = Self {
            inner: self.inner.clone(),
            frozen: None,
        };
        cloned.freeze();
        cloned
    }
}

impl<C: Ctx> CtxProxy<C> {
    pub fn new(inner: C) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner)),
            frozen: None,
        }
    }

    pub fn inner(&self) -> Ref<'_, C> {
        self.inner.borrow()
    }

    pub fn inner_mut(&mut self) -> RefMut<'_, C> {
        self.frozen = None;
        self.inner.borrow_mut()
    }

    pub fn freeze(&mut self) {
        // todo!("freeze not resolved")
    }
}

impl<C: Ctx> Drop for CtxProxy<C> {
    fn drop(&mut self) {
        // WARNING This unbalances the state stack
        // self.undo_unmerged();
    }
}

impl<C: Ctx> CtxI for CtxProxy<C> {
    fn cursor(&self) -> &dyn Cursor {
        //     SAFETY: We obtain a shared reference to the cursor from the inner context.
        //     This is safe as long as we don't perform any mutable operations on the RefCell
        //     while this reference is alive. In the parser, cursor() is usually called
        //     for read-only access between state-changing calls.
        unsafe {
            let ptr = self.inner.as_ptr();
            (*ptr).cursor()
        }
    }

    fn callstack(&self) -> CallStack {
        if let Some(_id) = &self.frozen {}
        self.inner.borrow().callstack()
    }

    fn mark(&self) -> usize {
        if let Some(_id) = &self.frozen {}
        self.inner.borrow().mark()
    }

    fn cut_seen(&self) -> bool {
        if let Some(_id) = &self.frozen {}
        self.inner.borrow().cut_seen()
    }
}

impl<C: Ctx> Configurable for CtxProxy<C> {
    fn configure(&mut self, cfg: &CfgBox) {
        self.inner_mut().configure(cfg);
    }
}

impl<C: Ctx> Ctx for CtxProxy<C> {
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        // SAFETY: Similar to cursor(), we obtain a mutable reference.
        unsafe {
            let ptr = self.inner.as_ptr();
            (*ptr).cursor_mut()
        }
    }

    fn enter(&mut self, name: &str) {
        self.inner_mut().enter(name);
    }

    fn leave(&mut self) {
        self.inner_mut().leave();
    }

    fn tracer(&self) -> &dyn Tracer {
        // SAFETY: Similar to cursor().
        unsafe {
            let ptr = self.inner.as_ptr();
            (*ptr).tracer()
        }
    }

    fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.inner_mut().get_pattern(pattern)
    }

    fn memo(&mut self, key: &Key) -> Option<Memo> {
        self.inner_mut().memo(key)
    }

    fn memoize(&mut self, key: &Key, tree: &Tree) {
        self.inner_mut().memoize(key, tree);
    }

    fn cut(&mut self) {
        self.inner_mut().cut();
    }

    fn prune_cache(&mut self) {
        self.inner_mut().prune_cache();
    }

    fn is_keyword(&self, name: &str) -> bool {
        self.inner.borrow().is_keyword(name)
    }

    fn set_keywords(&mut self, keywords: &[Str]) {
        self.inner_mut().set_keywords(keywords);
    }

    fn merge(self, other: Self) -> Self {
        Self {
            inner: other.inner.clone(),
            frozen: None,
        }
    }

    fn push_state(&mut self) {
        self.inner_mut().push_state();
    }

    fn done(&self) -> bool {
        self.inner.borrow().done()
    }

    fn pop(&mut self) {
        self.inner_mut().pop();
    }

    fn undo(&mut self) {
        self.inner_mut().undo();
    }
}
