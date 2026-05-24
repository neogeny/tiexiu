// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::failure::ParseFailure;
use crate::Tree;
use crate::context::CtxI;
use crate::input::memento::Memento;
use std::fmt::Debug;
use std::panic::Location;
use std::rc::Rc;

/// Result of a PEG parse attempt: success (Yeap) or failure (Nope).
pub type ParseResult = Result<Yeap, Nope>;

/// A successful parse result containing the parse tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Yeap(pub Rc<Tree>);

/// Creates a Yeap success value from a tree.
pub fn yeap(tree: Rc<Tree>) -> Yeap {
    Yeap(tree)
}

/// A parse failure (no details — details are in DisasterReport).
#[derive(thiserror::Error, Debug, Default, Clone, PartialEq)]
pub struct Nope {}

/// A detailed report of a parse failure with position and error context.
#[derive(Clone, Debug)]
pub struct DisasterReport {
    /// Source location where the error was created.
    pub location: &'static Location<'static>,
    /// Whether a cut operator was seen before this error.
    pub cutseen: bool,
    /// The underlying parse failure.
    pub error: Rc<ParseFailure>,
    /// Memento with the error position and message.
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
    /// Creates a new disaster report from parsing context and failure.
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

    /// Returns the error start position.
    pub fn start(&self) -> usize {
        self.memento.start
    }

    /// Returns the error mark position.
    pub fn mark(&self) -> usize {
        self.memento.mark
    }

    /// Marks that a cut was seen.
    pub fn setcut(&mut self) {
        self.cutseen = true;
    }

    /// Returns whether a cut was seen and resets the flag.
    pub fn take_cut(&mut self) -> bool {
        let was_cut = self.cutseen;
        self.cutseen = false;
        was_cut
    }
}

impl Yeap {
    /// Consumes the Yeap and returns the inner tree.
    #[inline]
    pub fn tree(self) -> Rc<Tree> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::peg::error::Yeap;
    use crate::peg::error::nope::Nope;

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
}
