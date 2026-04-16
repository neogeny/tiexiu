---
apply: always
---

# AGENTS

## Project Overview

TieXiu is a Rust port of TatSu PEG parser generator with Python bindings via PyO3/maturin.

## Rules

* AI Agents will study `README.md`, `SYNTAX.md`, `./tatsu/README.rst` and all the
  `*.rst` documents in `./tatsu/docs/` to gain context about the projects.

* AI Agents will study all the files belonging to the current Rust project to understand its structure and semantics.

* AI Agents will study the Python source code for TatSu found in the modules under `./tatsu/tatsu/` and `./tatsu/tests/` to understand the background of the TieXiu project and its goals.

* AI Agents will read and study all the files under `./corpus/sessions/` to gain
  context from previous work sessions.

* AI Agents will not use text modification tools like `sed` or awk to modify program
  code. If done, the tool will be pointed to specific files, one by one, and
  never run then over a directory of a glob pattern.

* AI Agents will learn to use `ast-grep` by experimentation and by studying the
  documentation available at https://ast-grep.github.io

* AI Agents will follow strictly the rules described in `RULES.md`
* AI Agents will follow strictly the rules described all files in `./.aiassistant/rules/*.md`


## Dev Commands

```bash
# Quality gates (used by `just pre-push`)
cargo fmt --all                          # format check
cargo clippy --all-targets --all-features -- -D warnings  # strict lint
cargo test --verbose                     # tests

# All checks in order (default target)
just pre-push

# Other commands
cargo build --release                    # release build
cargo bench                              # benchmarks
cargo run                                # run CLI (requires cli feature)
maturin develop                           # build & install Python package
```

## Testing

- Tests use `cargo test --verbose` (not `cargo nextest` locally; CI uses default)
- Benchmark harness is criterion-based with codspeed compat
- Test fixtures in `tests/fixtures.rs` provide `boot_grammar()` and `parse_ebnf()`

## Python Bindings

- Build system: maturin (configured in `pyproject.toml`)
- `.cargo/config.toml` sets `PYO3_PYTHON` to `.venv/bin/python` for local venv
- Create venv: `uv venv`
- Install package: `maturin develop`

## Architecture

- `src/api.rs`: High-level API (`parse`, `compile`, `load`, `pretty`)
- `src/peg/`: Grammar model (Exp, Grammar, Rule)
- `src/state/`: Parser context and memoization
- `src/input/`: Cursor trait for text/byte streams
- `src/trees/`: CST/AST tree types
- `src/json/`: TatSu JSON grammar import

## Key Conventions

- Rust edition 2024
- Features: `default`, `bootstrap`, `transient`, `serde`, `serde_json`, `cli`, `fancy-regex`
- Python requires >=3.12
- Use `unindent` utility when inlining EBNF strings in tests

## CI Workflows

- `default.yml`: fmt → clippy → test (sequential)
- `codspeed.yml`: benchmark on main/PR
- `publish.yml`: PyPI release
