// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ensure::EnsureError;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

/// Error states for the global grammar cache.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Internal Cache Lock was poisoned by a previous panic")]
    LockPoisoned,

    #[error("Failed to compile grammar: {0}")]
    CompileFailure(String),

    #[error("Invariant failed in cache logic: {0}")]
    Invariant(#[from] EnsureError),
}

impl<T> From<PoisonError<T>> for CacheError {
    fn from(_: PoisonError<T>) -> Self {
        CacheError::LockPoisoned
    }
}

pub type CacheResult<T> = Result<T, CacheError>;

// The cache now stores Arcs for zero-copy retrieval.
static CACHE: OnceLock<Mutex<HashMap<u64, Arc<CompiledGrammar>>>> = OnceLock::new();

#[derive(Debug)]
pub struct CompiledGrammar {
    pub name: String,
    pub rules_count: usize,
    // Add your internal compiled representation here
}

/// Very fast semi-unique hash.
pub fn compute_hash(text: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

/// Retrieves a reference-counted handle to the grammar.
pub fn get(text: &str) -> CacheResult<Option<Arc<CompiledGrammar>>> {
    let hash = compute_hash(text);
    let lock = CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    let map = lock.lock()?;
    Ok(map.get(&hash).cloned()) // Clones the Arc (increment ref count), not the grammar.
}

/// The "Atomic" Entry Pattern:
/// Ensures we don't hold the lock during the actual compilation.
pub fn get_or_compile<F>(text: &str, compile_fn: F) -> CacheResult<Arc<CompiledGrammar>>
where
    F: FnOnce(&str) -> CacheResult<CompiledGrammar>
{
    crate::ensure!(!text.is_empty())?;
    let hash = compute_hash(text);
    let lock = CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    // 1. First check: Fast path with a short-lived lock
    {
        let map = lock.lock()?;
        if let Some(existing) = map.get(&hash) {
            return Ok(Arc::clone(existing));
        }
    }

    // 2. Compilation path: Perform the work WITHOUT holding the lock
    // This prevents blocking other threads that might be accessing OTHER grammars.
    let compiled = Arc::new(compile_fn(text)?);

    // 3. Final insert: Re-acquire lock to save the result
    let mut map = lock.lock()?;
    // Use or_insert to handle the race condition where another thread compiled
    // the same grammar while we were working.
    let final_grammar = map.entry(hash).or_insert(compiled);

    Ok(Arc::clone(final_grammar))
}