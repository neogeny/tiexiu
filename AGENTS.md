---
apply: always
---

# AGENTS

## Project Overview
**TieXiu** is a Rust port of the **TatSu** PEG parser generator, providing Python bindings via **PyO3/maturin**.

## Core Operational Rules
* **Research Phase:** Study [README.md](README.md), [SYNTAX.md](SYNTAX.md), and [./tatsu/README.rst](./tatsu/README.rst). Read all `*.rst` documents in [./tatsu/docs/](./tatsu/docs/) to establish PEG/TatSu domain context.
* **Context Gathering:** Analyze the current Rust project structure. Read all files in [./corpus/sessions/](./corpus/sessions/) to understand the history and current trajectory of the work.
* **Project Summary**: Study [./corpus/SUMMARY.md](./corpus/SUMMARY.md) for a brief project description created for collaborators.
* **Source Mapping:** Cross-reference the Python source in [./tatsu/tatsu/](./tatsu/tatsu/) and [./tatsu/tests/](./tatsu/tests/) to ensure the Rust implementation aligns with TatSu's logic and goals.
* **Code Modification:** Do not use `sed` or `awk` for bulk directory/glob modifications. Target specific files one-by-one only when structural tools are insufficient.
* **Strict Compliance:** Adhere strictly to [RULES.md](RULES.md).

## Rust Implementation Standards
* **Error Handling:** Return `Result` for invalid parameter values. Reserve `panic!` for internal logic bugs or unrecoverable state.
* **String Allocation:** Use `Box<str>` for dynamic error messages within error enums to maintain a small memory footprint for the `Result` type.
* **Future-Proofing:** Always use `.into()` for string-to-heap conversions to ensure compatibility with a future migration to `Cow<'a, str>`.
* **Deref Handling:** When iterating over references, use pattern matching (e.g., `for (&name, _) in pairs`) to simplify access to underlying data.
* **Tests:** Use the `unindent` utility when inlining EBNF strings. Utilize `boot_grammar()` and `parse_ebnf()` from `tests/fixtures.rs`.
* **Other**: Study [RULES.md](RULES.md) for more detailed guidelines.

## Development Workflow

There is a [Justfile](Justfile) defined for the project with targets for common, version control, and integration. The Python environment is managed using `uv`.

```bash
# Quality gates (mandatory before submission)
just fmt                                       # Format check
just clippy                                    # Strict lint
just test                                      # Lint and run tests with nextest

# Full verification pipeline
just pre-push

# Build & Integration
cargo build --release                          # Release build
maturin develop                                # Build Python package via uv
