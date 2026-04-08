// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Cursor;
use crate::input::StrCursor;
use crate::json::boot::boot_grammar;
use crate::peg::{Grammar, Parser, S};
use crate::state::corectx::CoreCtx;
use crate::trees::Tree;
use crate::{Error, Result};

pub fn parse(grammar: &str) -> Result<Tree> {
    parse_with(StrCursor::new(grammar))
}

pub fn parse_with<U>(cursor: U) -> Result<Tree>
where
    U: Cursor + Clone,
{
    let boot = boot_grammar()?;
    let ctx = CoreCtx::new(cursor);

    match boot.parse(ctx) {
        Ok(S(_, tree)) => Ok(tree),
        Err(failure) => Err(Error::from(failure.error)),
    }
}

pub fn compile(grammar: &str) -> Result<Grammar> {
    compile_with(StrCursor::new(grammar))
}

pub fn compile_with<U>(cursor: U) -> Result<Grammar>
where
    U: Cursor + Clone,
{
    let tree = parse_with(cursor)?;
    Ok(Grammar::compile(&tree)?)
}

pub fn load(json: &str) -> Result<Grammar> {
    Ok(Grammar::from_json(json)?)
}
