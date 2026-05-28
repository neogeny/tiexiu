// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::failure::ParseFailure;
use crate::context::Ctx;
use crate::input::memento::Memento;
use crate::trees::TreeRef;
use crate::types::Ref;
use std::fmt::Debug;
use std::panic::Location;

/// Result of a PEG parse attempt
pub type ParseResult = Result<TreeRef, Nope>;

/// A parse failure carrying a disaster report.
#[derive(thiserror::Error, Debug, Clone)]
pub struct Nope {
    /// The underlying disaster report with the error details.
    pub report: Ref<DisasterReport>,
}

/// A detailed report of a parse failure with position and error context.
#[derive(Clone, Debug)]
pub struct DisasterReport {
    /// Source location where the error was created.
    pub location: &'static Location<'static>,
    /// The underlying parse failure.
    pub error: Ref<ParseFailure>,
    /// Memento with the error position and message.
    pub memento: Ref<Memento>,
}

impl std::fmt::Display for Nope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.report, f)
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
    pub fn new(start: usize, _cutseen: bool, ctx: &dyn Ctx, error: &ParseFailure) -> Self {
        let memento = Memento::new(start, ctx, error.to_string().as_str());
        Self {
            memento: memento.into(),
            error: error.clone().into(),
            location: Location::caller(),
        }
    }

    pub fn mark(&self) -> usize {
        self.memento.mark
    }

    /// Returns the error start position.
    pub fn start(&self) -> usize {
        self.memento.start
    }
}

#[cfg(test)]
mod tests {
    use crate::peg::error::nope::Nope;

    #[test]
    fn test_nope_size() {
        // Nope contains a Box<DisasterReport>.
        let size = size_of::<Nope>();
        assert_eq!(
            size, 8,
            "Nope should be pointer-sized (Box), got {} bytes",
            size
        );
    }
}
