---
type: Architecture
title: Grammar
description: PEG grammar representation with rules, expressions, and analysis.
tags: [grammar, peg, parsing, analysis]
timestamp: 2026-07-12T00:00:00Z
---

# Grammar

The grammar module implements PEG (Parsing Expression Grammar) representation, analysis, and parsing. TieXiu uses a boot grammar to parse EBNF grammars into its internal representation.

## Grammar Struct

```rust
pub struct Grammar {
    pub name: Str,
    pub analyzed: bool,
    pub directives: GrammarDirectives,
    pub keywords: GrammarKeywords,
    pub rules: RuleMap,
}
```

| Field | Type | Purpose |
|-------|------|---------|
| `name` | `Str` | Grammar name |
| `analyzed` | `bool` | Whether grammar has been linked and analyzed |
| `directives` | `Cfg` | Grammar-level directives |
| `keywords` | `Ref<[Str]>` | Sorted, deduplicated keywords |
| `rules` | `RuleMap` | Map from rule names to rules |

## Rule Struct

```rust
pub struct Rule {
    pub name: RuleName,
    pub params: Box<[Str]>,
    pub decorators: Box<[Str]>,
    pub flags: u8,
    pub exp: Exp,
}
```

### Flags

| Flag | Bit | Purpose |
|------|-----|---------|
| `FLAG_IS_NAME` | 0b0000_0001 | Rule is a name rule |
| `FLAG_IS_TOKN` | 0b0000_0010 | Rule is a token rule |
| `FLAG_IS_MEMO` | 0b0000_0100 | Rule should be memoized |
| `FLAG_IS_LREC` | 0b0000_1000 | Rule is left-recursive |
| `FLAG_NO_MEMO` | 0b0001_0000 | Disable memoization |
| `FLAG_NO_STAK` | 0b0010_0000 | Disable tracing |

## Expression Types

The `ExpKind` enum defines all PEG expression types:

### Terminals

| Kind | Description |
|------|-------------|
| `Nil` | No expression |
| `Cut` | Cut operator `!` |
| `Void` | Matches nothing |
| `Fail` | Always fails |
| `Dot` | Any single character |
| `Eof` | End of input |
| `Eol` | End of line |
| `Token(Str)` | Exact string match |
| `Pattern(Str)` | Regex pattern match |
| `Constant(Str)` | Constant value match |

### Non-Terminals

| Kind | Description |
|------|-------------|
| `Call { name, rule }` | Rule invocation |
| `Sequence(ERefArr)` | Concatenation |
| `Choice(ERefArr)` | Ordered alternatives |
| `Repeat { exp, min, max }` | Repetition (closure/optional) |
| `Join { exp, sep }` | Separated list |

### Grouping

| Kind | Description |
|------|-------------|
| `Group(ERef)` | Group expression |
| `SkipGroup(ERef)` | Skip grouping |
| `Named(Str, ERef)` | Name result |
| `NamedList(Str, ERef)` | Name as list |
| `Override(ERef)` | Override result |
| `OverrideList(ERef)` | Override as list |

### Lookahead

| Kind | Description |
|------|-------------|
| `Lookahead(ERef)` | Positive lookahead `&exp` |
| `NegativeLookahead(ERef)` | Negative lookahead `!exp` |
| `SkipTo(ERef)` | Skip until match |

### Meta-Expressions

| Kind | Description |
|------|-------------|
| `Name` | Match identifier `@name` |
| `Int` | Match signed integer `@int` |
| `UInt` | Match unsigned integer `@uint` |
| `Float` | Match floating-point `@float` |
| `Bool` | Match boolean `@bool` |

## Grammar Analysis

### Initialization

```rust
pub(crate) fn initialize(&mut self) -> Result<(), ParseFailure> {
    self.mark_left_recursion();
    self.link()?;
    self.analyzed = true;
    Ok(())
}
```

### Left-Recursion Detection

The grammar marks left-recursive rules for packrat parsing:

```rust
fn mark_left_recursion(&mut self) {
    // Detect and mark left-recursive rules
}
```

### Linking

Rules are linked to resolve rule references:

```rust
fn link(&mut self) -> Result<(), ParseFailure> {
    // Link Call expressions to their Rule definitions
}
```

## Parsing

### Start Rule

The start rule is "start" or the first rule:

```rust
pub fn start_rule(&self) -> Result<RuleName, ParseFailure> {
    if let Some(rule) = self.rules.get("start") {
        Ok(rule.name.clone())
    } else {
        self.rules.get_index(0)
            .map_or(Err(NoRulesInGrammar), |(_, r)| Ok(r.name.clone()))
    }
}
```

### Memoization

Rules are memoized based on flags:

```rust
pub fn is_memoizable(&self) -> bool {
    self.is_left_recursive() || self.flag(FLAG_IS_MEMO) && !self.flag(FLAG_NO_MEMO)
}
```

### Semantics

After parsing, semantics are applied:

```rust
pub fn parse_at<C: CtxSem>(&self, ctx: &mut C) -> ParseResult {
    match self.exp.parse_at(ctx) {
        Ok(tree) => {
            let folded = Tree::fold(tree);
            ctx.apply_semantics(folded, self.name, &self.params)
        }
        Err(nope) => Err(nope),
    }
}
```

## Boot Grammar

The boot grammar is self-hosted — TieXiu parses its own grammar format:

```rust
pub fn boot_grammar() -> Result<Grammar> {
    boot::boot_grammar()
}
```

## JSON Serialization

Grammars can be serialized to/from JSON:

```rust
// To JSON
let json = grammar.to_json_string()?;

// From JSON
let grammar = Grammar::from_json(&json)?;
```

## Source Files

- `src/peg/grammar.rs` — `Grammar` struct
- `src/peg/rule.rs` — `Rule` struct
- `src/peg/exp.rs` — `Exp`, `ExpKind`
- `src/peg/boot.rs` — Boot grammar
- `src/peg/parser.rs` — Parser trait
- `src/peg/analysis/` — Nullability, linking
- `src/peg/pretty.rs` — Pretty-printing
