# GPT54 Study Summary

## Purpose

TieXiu is a Rust port of TatSu, a PEG parser generator, with Python bindings via PyO3 and maturin. The project aims to preserve TatSu grammar syntax and semantics while reimplementing the engine in Rust with a smaller, more cache-friendly runtime model.

## Current Project Position

The project is still in alpha. The core execution engine, grammar model, parsing engine, JSON grammar import path, and left-recursion support are in place. The main unfinished area is the higher-level tree/model path needed for full TatSu-compatible grammar bootstrap from EBNF text.

The roadmap centers on self-hosting:

- parse TatSu JSON grammar exports today
- parse TatSu EBNF text directly next
- use that to support full grammar compilation and eventual bootstrap

## Core Architecture

The Rust codebase is structured into clear subsystems:

- `src/api.rs`: high-level public parsing/compilation/loading entry points
- `src/peg/`: grammar structures, parser logic, left recursion, linking, nullability, repetition, lookahead, pretty-printing
- `src/state/`: parser context, memoization, tracing, cut handling
- `src/input/`: cursor abstractions for parsing text and token streams
- `src/trees/`: CST/AST tree representation and folding/build utilities
- `src/json/`: TatSu JSON import/export and bootstrap grammar loading
- `src/python/`: Python-facing wrappers and tree conversion
- `src/ui/`: CLI-facing code
- `tests/`: Rust test suite and fixtures

The project summary in `corpus/SUMMARY.md` matches the code layout reasonably well: grammar ownership is centralized, parsing state is copy-on-write, and memoization is shared to preserve packrat behavior.

## Parsing Model

TieXiu is designed around a lean parsing context:

- parser state changes are intentionally small
- backtracking is lazy and based on copy-on-write state
- memoization is shared across parse branches
- cut state participates in branch control and memo pruning

This is meant to preserve PEG behavior while keeping runtime overhead low. The README emphasizes small handles, stack-friendly state transitions, and bounded memory through cut-aware pruning.

## Grammar Semantics

The local `SYNTAX.md` and TatSu docs establish the semantic target:

- PEG with ordered choice
- sequence, grouping, optionals, closures, joins, gathers
- lookahead and negative lookahead
- cut semantics scoped to enclosing rule/choice/group constructs
- named fields and named-list fields affecting AST shape
- override operators changing the produced AST
- directives such as `@@grammar`, `@@whitespace`, `@@comments`, `@@keyword`, `@@left_recursion`
- uppercase rule names behave lexically and do not auto-skip whitespace/comments

The resulting parse tree model is not “just a CST”. TatSu semantics drive when output is a scalar, list, mapping, or rule node with metadata. That distinction matters for Rust implementation work because preserving output shape is part of compatibility.

## TatSu Reference Mapping

The Python reference implementation is extensive and useful for alignment:

- `tatsu/tatsu/grammars/`: grammar model types and semantics
- `tatsu/tatsu/contexts/`: parse context behavior
- `tatsu/tatsu/parser/`: bootstrap/parser machinery
- `tatsu/tatsu/input/`: text buffering and lexical behavior
- `tatsu/tatsu/railroads/`: diagram generation support
- `tatsu/tatsu/objectmodel/` and `semantics.py`: model-building semantics

The Python tests are also organized in a way that maps well to the Rust work:

- `tatsu/tests/grammar/*.py`: grammar-language behavior
- `tatsu/tests/util/*.py`: utility behavior
- top-level tests: parser/model/bootstrap/diagram/walker behavior

The `wip/tests/` Rust area appears to be a translation staging area for TatSu tests, while `tests/` contains active Rust tests.

## Documentation Takeaways

From the TatSu docs and README:

- TatSu is PEG-first, not regex-first
- left recursion is supported and important
- semantic actions are intentionally separate from grammar text
- AST/model shape is a first-class feature
- directives and parser configuration are part of the public grammar contract
- translation, walking, pretty-printing, and diagrams are not side features; they are part of the intended ecosystem

The docs also reinforce that TatSu compatibility is broader than “can parse input”. It includes grammar directives, AST conventions, parseinfo behavior, model building, tracing, and bootstrap behavior.

## Sessions and Recent Trajectory

The session history shows several recurring themes:

- prior agents frequently made unauthorized changes and were corrected
- the user expects strict compliance with planning and authorization rules
- explicit source-directory scoping for `rg` is important
- the `wip/tests/` area has been used for translated TatSu tests
- some prior work involved parser bugs around token/whitespace behavior, repetition, and bootstrap tests
- there was confusion in earlier sessions about file renames and non-project directories such as `fragments/`

The most important procedural lesson from the corpus is that local rules matter as much as technical correctness.

## Working Rules To Respect

From `AGENTS.md` and `RULES.md`:

- study first, change later
- never change files without a plan and user authorization
- always consult before multi-file changes
- do not act on assumptions; verify with the user when needed
- do not use `sed`, `awk`, or similar bulk text tools to modify code
- when using `rg`, explicitly name source directories such as `src tests benches`
- keep temporary files under `./tmp/`
- do not access or modify files outside the project

Rust-specific conventions:

- return `Result` for invalid user-facing parameters
- reserve `panic!` for internal invariants and unrecoverable bugs
- prefer `Box<str>` for dynamic error strings inside error enums
- prefer `.into()` over explicit heap constructors where practical
- use pattern matching to avoid noisy dereferencing in loops

Test conventions:

- inline EBNF strings should use `unindent`
- use `tests/fixtures.rs` helpers such as `boot_grammar()` and `parse_ebnf()`
- tests may be ignored, but they must compile
- project guidance says `cargo test --verbose`; AGENTS also highlights `just fmt`, `just clippy`, and `just test`

## Practical Understanding Of The Codebase

At this point, the project can be understood as:

1. a Rust PEG engine already capable of executing grammars and handling left recursion
2. a TatSu-compatibility effort, not a greenfield parser with arbitrary semantics
3. a staged bootstrap effort where JSON import is ahead of native EBNF parsing
4. a repository with strong process discipline expectations from the user

## Implications For Future Work

Any future code change should be evaluated against four constraints:

- Does it preserve TatSu semantics?
- Does it fit the Rust ownership/state architecture already in place?
- Does it respect the project’s local coding conventions?
- Has the user authorized the scope of the change?

That combination is the real working definition of “correct” in this repository.
