# 铁修 TieXiu Design

This document described the design criteria used and implementation choices made for the current implementation of the project. Some decisions were arguably not the best for a contexts with the Rust semantics or runtime behavior, but they remain for now.


## Lean Parsing Context

**TieXiu** uses the runtime stack as the parsing state stack. The parsing context (`CoreCtx`) owns both the mutable cursor position and the shared heavyweight state (grammar, memoization cache, tracer) directly, avoiding heap indirection. This allows for deep recursive descent with minimal stack pressure. The `Cursor` implementation for parsing text (`StrCursor`) uses 24 bytes (`Arc<str>` + `usize` offset + `Arc<CursorHeavy>`) and has copy-on-write semantics during a parse (grammar elements that don't advance over the input share the same cursor).


## Mutable Context with Reset

Backtracking in **TieXiu** uses mutable context semantics (`&mut CtxSem`). The context is created once and passed by mutable reference through the parse. Branching at choice points saves the cursor mark and restores it on failure. Failed parses restore the cursor position and register the furthest failure position for error reporting.

## Trees

A CST/Tree uses *32 bytes* per node plus space proportional to the input matched, but CST are only kept for the currently successful path on the parse, and are dropped as soon as an option fails. CST are compacted on the boundary of the successful parse of a grammar rule node.


## Failures

The failure path returns the furthest position reached in the input and a message about the error encountered there. The same error value is passed back during backtracking until a branch point is reached and another path can be tried. At branching, the error value belonging to the furthes position in the input is chosen to pass back. The error value also passes back the _cut_ state so branches can commit to a failed alternative if it was fixed with a cut.

## Left Recursion Support

TieXiu features a complete implementation for handling left-recursive grammars. A pre-pass _analysis_ identifies and marks recursive cycles, while the _runtime_ includes the necessary logic to grow the recursive content by iteration instead of recursion.

## Complete Grammar Model

The building blocks for grammar models are implemented with a clear chain of ownership. The `Grammar` acts as the root container owning the `Rule` map, while each `Rule` owns its `Model` definition. This hierarchy eliminates reference proliferation and simplifies lifetime management.

## Milestone: From CST to AST

 The algebra for creating **Concrete Syntax Trees (CST)** was ported from **TatSu** to **TieXiu**, with optimizations. Instead of computing the resulting CST during parsing, the engine generates unoptimized trees that are normalized into their concrete versions at rule boundaries. **TieXiu** uses the **TatSu** semantics for **Abstract Syntax Tree (AST)**, in which named elements in a rule definition force the result to be a mapping of names to parsed elements. 

## Grammar-Specific ASTs
 
Rust doesn't allow the creation of synthetic types at runtime, so parsing to native types will require code generation for the desired model and deserialization of the JSON-compatible `Tree` result of a parse into the desired model nodes. **TiexSiu**'s own `compiler.rs` may be used a an example of how to navigate a `Tree` to produce an object model (a `Grammar` in the case of `compiler.rs`).  `Tree.to_value()` can be used to objtain a `serde_json::Value` version of a `Tree`, and some may prefer to use that 

## Packrat & Memoization

All branches in a parse use a shared *Memoization Cache* to achieve the `O(N) ` complexity of packrat parsers. The cache is pruned at `cut` points to place a bound on memory use and make the lookups more efficient.

## The Bootstrap Plan

* Renerating Rust code for the bootstrap grammar is already possible with the current feature set but the implementation is pending.

## Features

* [x] **24-byte Cursor**: Optimized for L1 cache and deep recursion.
* [x] **32-byte Tree Nodes**: Compact AST representation with niche optimization.
* [x] **Mutable Context**: Efficient `&mut CtxSem` semantics with mark/reset for backtracking.
* [x] **Complete Parsing Engine**: Core PEG execution logic is fully implemented.
* [x] **Left Recursion**: Both analysis and runtime support are complete.
* [x] **Complete Grammar Model**: Rules and Models are fully defined and owned.
* [x] **Thread-Safe Grammar**: Grammar models are immutable after constructed so theycan be shared across multiple execution threads.
* [x] **Efficient Memoization**: Global cache consistency across backtracking branches.
* [x] **Object-Safe Cursors**: Abstract `Cursor` trait allows for string, byte, or custom stream inputs.
