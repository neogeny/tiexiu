// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::failure::ParseFailure;
use crate::Tree;
use crate::context::{CtxI, Snap};
use crate::input::memento::Memento;
use std::fmt::Debug;
use std::panic::Location;
use std::rc::Rc;

pub type ParseResult = Result<Yeap, Nope>;

#[derive(Debug, Clone, PartialEq)]
pub struct Yeap(pub Rc<Snap>, pub Rc<Tree>);

pub fn yeap(snap: &Snap, tree: Rc<Tree>) -> Yeap {
    Yeap(snap.clone().into(), tree)
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub struct Nope {
    pub cutseen: bool,
}

#[derive(Clone, Debug)]
pub struct DisasterReport {
    pub location: &'static Location<'static>,
    pub cutseen: bool,
    pub error: Rc<ParseFailure>,
    pub memento: Rc<Memento>,
}

impl std::fmt::Display for Nope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Display for DisasterReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.memento, f)
    }
}

impl std::error::Error for DisasterReport {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.error.as_ref())
    }
}

impl DisasterReport {
    #[track_caller]
    pub fn new(start: usize, cutseen: bool, ctx: &dyn CtxI, error: &ParseFailure) -> Self {
        let memento = Memento::new(start, ctx, error.to_string().as_str());
        Self {
            cutseen,
            memento: memento.into(),
            error: error.clone().into(),
            location: Location::caller(),
        }
    }

    pub fn start(&self) -> usize {
        self.memento.start
    }

    pub fn mark(&self) -> usize {
        self.memento.mark
    }

    pub fn setcut(&mut self) {
        self.cutseen = true;
    }

    pub fn take_cut(&mut self) -> bool {
        let was_cut = self.cutseen;
        self.cutseen = false;
        was_cut
    }
}

impl Nope {
    #[track_caller]
    pub fn new(cutseen: bool) -> Self {
        Self { cutseen }
    }

    pub fn setcut(&mut self) {
        self.cutseen = true;
    }

    pub fn take_cut(&mut self) -> bool {
        let was_cut = self.cutseen;
        self.cutseen = false;
        was_cut
    }

    pub fn restore_cut(&mut self, was_cut: bool) {
        if !was_cut {
            self.cutseen = false;
        }
    }
}

impl Yeap {
    #[inline]
    pub fn tree(self) -> Rc<Tree> {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use crate::Tree;
    use crate::context::CtxI;
    use crate::peg::error::Yeap;
    use crate::peg::error::nope::{Nope, yeap};
    use std::rc::Rc;

    const TARGET: usize = 32;

    #[test]
    fn test_yeap_size() {
        let size = size_of::<Yeap>();
        assert!(size <= TARGET, "Yeap size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_nope_size() {
        let size = size_of::<Nope>();
        assert!(size <= TARGET, "Nope size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_yeap_tree_returns_rc() {
        use crate::context::strctx::StrCtx;
        use crate::input::StrCursor;

        let tree = Tree::Text("hello".into());
        let ctx = StrCtx::new(StrCursor::new("hello"), &[]);
        let yeap = yeap(&ctx.click(), tree.into());
        let rc: Rc<Tree> = yeap.tree();
        assert!(matches!(rc.as_ref(), Tree::Text(_)));
    }
}
