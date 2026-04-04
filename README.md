# TieXiu

[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/neogeny/TieXiu?utm_source=badge)

A high-performance port of **TatSu** to Rust.

**TieXiu** (铁修) is a PEG (Parsing Expression Grammar) engine that implements the flexibility and power of the original **[TatSu][]** lineage into a memory-safe, high-concurrency architecture optimized for modern CPU caches.

[TatSu]: https://tatsu.readthedocs.io/en/stable/

## Current Project Status: Alpha

Implementation is complete for the core execution engine, the grammar model, the parsing engine, and support for left-recursive grammars. Development is currently advancing toward high-level tree construction and model synthesis.

### Lean Parsing Context

**TieXiu** uses the runtime stack as the parsing state stack. A state 
change has a 16-byte stack footprint consisting of two  pointers: one for the *Volatile State* (Input Cursor) and one for the *Heavy Stated* (Grammar + Memoization Cache). This allows for deep recursive descent with 
minimal stack pressure and $O(1)$ branching costs. The `Cursor` 
implementation for parsing text (`StrCursor`) uses 16 bytes (`&str` + 
`usize`) and has copy-on-write semantics during a parse (grammar elements 
that don't advance over the input share the same cursor).


### Copy-on-Write State Transitions

Backtracking in **TieXiu** is *lazy*. Cloning a context/state only increments reference counts. The engine leverages the Rust runtime stack to handle state changes. Branching at choice points is a *16-byte* clone of the state, with a *16-byte* (for text) allocation only when the state is mutated. Failed parses restore the cursor position and register the furthest failure position for error reporting. The state returned occupies the same space as the original state.

A CST may use *64-bytes* per atomic node plus space proportional to the input matched, but CST are only kept for the currently successful path on the parse, and are dropped as soon as an option fails. CST are compacted on the boundary of the successful parse of a grammar rule node.

### Complete Parsing Engine

The core parsing logic is fully implemented, providing a robust execution environment for PEG rules. Choice points, sequences, and lookaheads are handled with high efficiency, leveraging the CoW context for seamless backtracking.

### Left Recursion Support

TieXiu features a complete implementation for handling left-recursive 
grammars. A pre-pass *analysis* identifies and marks recursive cycles, while the *runtime* includes the necessary logic to grow the recursive content by iteration instead of recursion. 

### Complete Grammar Model

The building blocks for grammar models are implemented with a clear chain of ownership. The `Grammar` acts as the root container owning the `Rule` map, while each `Rule` owns its `Model` definition. This hierarchy eliminates reference proliferation and simplifies lifetime management.

### Milestone: From CST to AST

 The algebra for creating **Concrete Syntax Trees (CST)** was ported from 
 **TatSu** to **TieXiu** with optimizations. Instead of computing the 
 resulting CST during parsing, the engine generates unoptimized trees that 
 are destilled into their concrete versions at rule boundaries. **TieXiu** 
 uses the **TatSu** semantics for **Abstract Syntax Tree (AST)**, in which 
 named elements in a rule definition force the result to be a mapping of 
 names to parsed elements. Rust doesn't allow the creation of synthetic 
 types at runtime, so parsing to native types will require code generation 
 for the desired model and deserialization of the JSON-compatible result of 
 a parse into the desired model nodes.

### Packrat & Memoization

All branches in a parse use a shared *Memoization Cache* to achieve the `O(N) ` complexity of packrat parsers. The cache is pruned at `cut` points to place a bound on memory use and make the lookups more efficient.

### The Bootstrap Plan

A critical upcoming milestone is the **Bootstrap Process**. The roadmap
includes **Self-Hosting** through the implementation of a **TieXiu** grammar that describes its own EBNF. Grammars, including the grammar for grammars, will be passed to **TieXiu** using **TatSu**'s JSON export format for deseriallization into models.

After the initial bootstrap (**TieXiu** parsing grammars in its own grammar
language) either **TieXiu** or **TatSu** will generate the Rust code
necessary for faster parser bootstrap.

## Features

* [x] **16-byte Handle Size**: Optimized for L1 cache and deep recursion.
* [x] **32-byte State Changes**: Efficient CoW allocation on the Rust runtime stack.
* [x] **Complete Parsing Engine**: Core PEG execution logic is fully implemented.
* [x] **Left Recursion**: Both analysis and runtime support are complete.
* [x] **Complete Grammar Model**: Rules and Models are fully defined and owned.
* [x] **Thread-Safe Grammar**: Static grammar structure can be shared across multiple threads.
* [x] **Efficient Memoization**: Global cache consistency across backtracking branches.
* [x] **Object-Safe Cursors**: Abstract `Cursor` trait allows for string, byte, or custom stream inputs.
* [ ] **Self-Hosting Bootstrap**: (Planned) Engine parsing its own EBNF.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.