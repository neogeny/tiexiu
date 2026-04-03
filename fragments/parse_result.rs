// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

/// The standard result for any model's parse attempt.
///
/// Success: Returns the updated Context and the resulting CST node.
/// Failure: Returns the Context (to preserve pos and cut_seen) and an error message.
pub type ParseResult<C> = Result<(Ctx<C>, Cst), (Ctx<C>, String)>;
