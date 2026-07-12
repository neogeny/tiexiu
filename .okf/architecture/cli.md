---
type: Architecture
title: CLI
description: Command-line interface built with clap for grammar operations and parsing.
tags: [cli, command-line, clap, ui]
timestamp: 2026-07-12T00:00:00Z
---

# CLI

TieXiu provides a command-line interface built with clap for grammar operations, parsing, and transformations.

## Commands

### Boot

Display the internal boot grammar:

```bash
tiexiu boot                    # JSON format (default)
tiexiu boot --pretty           # Pretty-printed EBNF
tiexiu boot --railroads        # Railroad diagram (APL characters)
tiexiu boot --model            # Rust code for boot model
```

### Run

Execute a grammar against input files:

```bash
tiexiu run grammar.ebnf input1.txt input2.txt
tiexiu run grammar.json input.txt --json
tiexiu run grammar.ebnf input.txt --short
tiexiu run grammar.ebnf input.txt -n 4  # 4 concurrent tasks
```

#### Options

| Option | Description |
|--------|-------------|
| `grammar` | Path to compiled TatSu JSON grammar |
| `inputs` | Files to parse (one or more) |
| `--json` | Output tree as JSON |
| `--model` | Output Rust code for tree construction |
| `--short` | Output tree in "short" notation |
| `-n, --nproc` | Number of concurrent parse tasks |

### Grammar

Transform grammars:

```bash
tiexiu grammar input.ebnf              # Pretty-print (default)
tiexiu grammar input.ebnf --json       # Output as JSON
tiexiu grammar input.json --pretty     # JSON to EBNF
tiexiu grammar input.ebnf --railroads  # Railroad diagram
tiexiu grammar input.ebnf --model      # Rust code for grammar model
```

## Global Options

| Option | Description |
|--------|-------------|
| `-o, --output` | Write output to file instead of stdout |
| `--color` | Color control: `auto`, `always`, `never` |
| `--trace` | Display detailed parse trace |

## Output Formats

| Format | Extension | Description |
|--------|-----------|-------------|
| JSON | `.json` | Minified JSON object |
| Pretty | `.ebnf` | Pretty-printed EBNF |
| Railroads | `.apl` | Railroad diagram (APL characters) |
| Model | `.rs` | Rust code for construction |

## Syntax Highlighting

Output is syntax-highlighted using syntect:

```rust
pub fn pygmentize(content: &str, extension: &str, use_color: bool) -> Result<String> {
    // Uses base16-eighties.dark theme
}
```

## Color Strategy

Color is auto-detected based on terminal capabilities:

```rust
fn configure_color(color: clap::ColorChoice) -> bool {
    match color {
        clap::ColorChoice::Always => true,
        clap::ColorChoice::Never => false,
        clap::ColorChoice::Auto => {
            std::io::IsTerminal::is_terminal(&std::io::stdout())
                && std::io::IsTerminal::is_terminal(&std::io::stderr())
        }
    }
}
```

## Progress Reporting

The CLI uses a heartbeat-based progress system:

```rust
pub(crate) fn load_grammar_from_path(
    grammar: &PathBuf,
    progress: &ProgressUI,
    cfga: &CfgA,
) -> Result<Grammar> {
    let loader = progress.loading("loading grammar...");
    // ... load and compile grammar
    loader.finish();
}
```

## Error Handling

- `BrokenPipe` — silently ignored
- Other errors — displayed with debug format in debug mode, plain format in release

## Source Files

- `src/main.rs` — Entry point
- `src/ui/cli.rs` — CLI definition and dispatch
- `src/ui/cmd_run.rs` — Run command implementation
- `src/ui/heartbeat.rs` — Progress reporting
- `src/ui/progress.rs` — Progress UI
