# META_UPDATE: @name, @int, @uint, @float, @bool for TieXiu

## Session Context

TatSu v5.21.1 added `@name`, `@int`, `@uint`, `@float`, `@bool` meta-expressions.
OGoPEGo (Go port) has a complete implementation at `pkg/peg/model_meta.go` + cursor
methods in `pkg/input/cursor_rune.go` / `cursor_str.go` + 40 tests. TieXiu (Rust port)
does not yet have any meta expression support.

---

## Implementation Plan (bottom-up)

### Layer 1 — ExpKind enum variants + constructors

**File: `src/peg/exp.rs`** at line 117 (before closing `}` of ExpKind enum)

Add 5 unit variants to the ExpKind enum:

```rust
/// Match a name/identifier (like `@name`).
NameMeta,
/// Match a signed integer (like `@int`).
IntMeta,
/// Match an unsigned integer (like `@uint`).
UIntMeta,
/// Match a floating-point literal (like `@float`).
FloatMeta,
/// Match a boolean literal (like `@bool`).
BoolMeta,
```

Place them near `EmptyClosure`/`Fail`/`Dot`/`Eof`/`Eol` — they are leaf
expressions with no children, like those.

**File: `src/peg/build.rs`**

Add 5 constructor methods following the pattern of existing leaf constructors:

```rust
pub fn name_meta() -> Self { Self::new(ExpKind::NameMeta) }
pub fn int_meta() -> Self { Self::new(ExpKind::IntMeta) }
pub fn uint_meta() -> Self { Self::new(ExpKind::UIntMeta) }
pub fn float_meta() -> Self { Self::new(ExpKind::FloatMeta) }
pub fn bool_meta() -> Self { Self::new(ExpKind::BoolMeta) }
```

---

### Layer 2 — Cursor trait + StrCursor implementation

**File: `src/input/cursor.rs`** — add 5 methods to the `Cursor` trait:

```rust
fn match_name(&mut self) -> Option<String>;
fn match_int(&mut self) -> Option<i64>;
fn match_uint(&mut self) -> Option<u64>;
fn match_float(&mut self) -> Option<f64>;
fn match_bool(&mut self) -> Option<bool>;
```

**File: `src/input/strcursor.rs`** — implement the 5 methods.

> **ADVICE**: Port the character-by-character logic from OGoPEGo's
> `pkg/input/cursor_rune.go` lines 293–419 and `cursor_str.go` lines 401–531.
> The algorithms are pure character scanning — no regex. Key helpers:
>
> - `consume_uint` / `consume_signed_int` / `consume_sign` / `clean_number`
> - Name chars: `is_alphabetic()` + `namechars` set (already available via `self.is_name_char()`)
>
> Key behaviors (must match TatSu exactly):
> - `@name`: first char `is_alphabetic()` or `_` or `namechars`; rest `is_alphanumeric()` or `namechars`
> - `@int`: optional `+`/`-`, then digits with optional internal `_`
> - `@uint`: digits only with optional internal `_`
> - `@float`: optional sign, digits, optional `.` + digits, optional `e`/`E` + optional sign + digits
> - `@bool`: match `true`/`false`/`True`/`False` (case-sensitive)
>
> Return `None` on no match, `Some(...)` on match (and advance offset).

---

### Layer 3 — CtxSem wrapper methods

**File: `src/context/ctx.rs`** — add 5 methods to the `CtxSem` trait:

```rust
fn match_name(&mut self) -> Option<Str> {
    self.next_token();
    let result = self.cursor_mut().match_name();
    if let Some(ref name) = result {
        self.tracer().trace_match(self, name, "@name");
        Some(self.intern(name))
    } else {
        self.tracer().trace_no_match(self, "", "@name");
        None
    }
}
// match_int, match_uint, match_float, match_bool follow the same pattern
// For typed values, convert to string representation for the tree
```

> **ADVICE**: For `@int`, `@uint`, `@float`, `@bool`, convert the typed value
> to a string and return as `Str` wrapped in `Tree::Text(...)`. TieXiu's Tree
> enum has no `Number`/`Bool` variants — `Tree::Text` is sufficient. TatSu returns
> typed Python values but the PEG model only cares about the match result. Keep it
> simple: return the matched string.

> **ADVICE**: Add the failure variants to `ParseFailure` in
> `src/peg/error/failure.rs`, or use existing `Fail`/`ExpectedToken` variants.
> Simplest: use a catch-all `Fail` for meta failures since the error message will
> still show the expression name through the trace.

---

### Layer 4 — Runtime parsing dispatch

**File: `src/peg/parsing/expressions.rs`** in `do_parse_at()` (lines 65–256).

Add 5 match arms after the `ExpKind::Eol` / `ExpKind::Eof` section:

```rust
ExpKind::NameMeta => {
    if let Some(name) = ctx.match_name() {
        Ok(Tree::Text(name).into())
    } else {
        Err(ctx.failure(start, Fail))
    }
}
ExpKind::IntMeta => {
    if let Some(n) = ctx.match_int() {
        Ok(Tree::Text(n.to_string().into()).into())
    } else {
        Err(ctx.failure(start, Fail))
    }
}
ExpKind::UIntMeta => {
    if let Some(n) = ctx.match_uint() {
        Ok(Tree::Text(n.to_string().into()).into())
    } else {
        Err(ctx.failure(start, Fail))
    }
}
ExpKind::FloatMeta => {
    if let Some(f) = ctx.match_float() {
        Ok(Tree::Text(f.to_string().into()).into())
    } else {
        Err(ctx.failure(start, Fail))
    }
}
ExpKind::BoolMeta => {
    if let Some(b) = ctx.match_bool() {
        Ok(Tree::Text(b.to_string().into()).into())
    } else {
        Err(ctx.failure(start, Fail))
    }
}
```

---

### Layer 5 — EBNF compilation (recognize at parse time)

**File: `src/peg/analysis/compiler.rs`** in `parse_exp()` (lines 178–293).

Add cases for the 5 TatSu JSON `__class__` names before the `_ =>` fallback
(currently line 292), OR handle them via the EBNF tree's `"Meta"` typename:

```rust
"NameMeta" => Exp::name_meta(),
"IntMeta" => Exp::int_meta(),
"UIntMeta" => Exp::uint_meta(),
"FloatMeta" => Exp::float_meta(),
"BoolMeta" => Exp::bool_meta(),
```

> **ADVICE**: The boot grammar parses `@name` etc. from EBNF and produces a tree
> node with typename `"Meta"`. Check what the actual typename is — it may be
> `"Meta"` with a `text` child. If so, add a case like:
> ```rust
> "Meta" => {
>     let text = tree.value();
>     match text.as_str() {
>         "name" => Exp::name_meta(),
>         "int" => Exp::int_meta(),
>         "uint" => Exp::uint_meta(),
>         "float" => Exp::float_meta(),
>         "bool" => Exp::bool_meta(),
>         _ => return Err(...)
>     }
> }
> ```
> Refer to how OGoPEGo handles this in `pkg/peg/compile.go` lines 520–531.

---

### Layer 6 — JSON serialization/deserialization

**File: `src/json/export.rs`** — add 5 arms in `ExpKind::to_json_value()`:

```rust
ExpKind::NameMeta => { obj[&tag] = JsonValue::String("NameMeta".into()); }
ExpKind::IntMeta => { obj[&tag] = JsonValue::String("IntMeta".into()); }
ExpKind::UIntMeta => { obj[&tag] = JsonValue::String("UIntMeta".into()); }
ExpKind::FloatMeta => { obj[&tag] = JsonValue::String("FloatMeta".into()); }
ExpKind::BoolMeta => { obj[&tag] = JsonValue::String("BoolMeta".into()); }
```

**File: `src/json/import.rs`** — add 5 arms in `from_json_with_path()`:

```rust
"NameMeta" => Ok(Exp::name_meta()),
"IntMeta" => Ok(Exp::int_meta()),
"UIntMeta" => Ok(Exp::uint_meta()),
"FloatMeta" => Ok(Exp::float_meta()),
"BoolMeta" => Ok(Exp::bool_meta()),
```

---

### Layer 7 — Analysis passes

**File: `src/peg/analysis/nullability.rs`** — three functions to update:

In `is_nullable()`: add to the `Fail | Dot | Token(_) => false` group:
```rust
Self::Fail | Self::Dot | Self::Token(_) | Self::NameMeta
| Self::IntMeta | Self::UIntMeta | Self::FloatMeta | Self::BoolMeta => false,
```

In `callable_from()`: add to the leaf group returning `vec![]`:
```rust
Self::NameMeta | Self::IntMeta | Self::UIntMeta
| Self::FloatMeta | Self::BoolMeta => vec![],
```

In `callable_from_mut()`: same addition as above.

**File: `src/peg/analysis/defines.rs`** — meta expressions define nothing,
handled by `_ => {}` catch-all at line 65. No changes needed.

**File: `src/peg/analysis/leftrec.rs`** — add to the leaf catch-all at lines 53–64:
```rust
ExpKind::NameMeta | ExpKind::IntMeta | ExpKind::UIntMeta
| ExpKind::FloatMeta | ExpKind::BoolMeta => Vec::new(),
```

**File: `src/peg/analysis/linker.rs`** — meta expressions have no children to link,
handled by `_ => {}` catch-all at line 66. No changes needed.

---

### Layer 8 — Fold, pretty-print, railroads

**File: `src/peg/fold.rs`** in `children()` — add to leaf group:
```rust
ExpKind::NameMeta | ExpKind::IntMeta | ExpKind::UIntMeta
| ExpKind::FloatMeta | ExpKind::BoolMeta => vec![],
```

**File: `src/peg/pretty.rs`** in `PrettyPrint for ExpKind` — add 5 arms:
```rust
ExpKind::NameMeta => "@name".into(),
ExpKind::IntMeta => "@int".into(),
ExpKind::UIntMeta => "@uint".into(),
ExpKind::FloatMeta => "@float".into(),
ExpKind::BoolMeta => "@bool".into(),
```

**File: `src/tools/rails.rs`** in `walk_exp()` — add 5 arms:
```rust
ExpKind::NameMeta => vec![make_rail("@name")],
ExpKind::IntMeta => vec![make_rail("@int")],
ExpKind::UIntMeta => vec![make_rail("@uint")],
ExpKind::FloatMeta => vec![make_rail("@float")],
ExpKind::BoolMeta => vec![make_rail("@bool")],
```

---

### Layer 9 — Tests

**File: `src/input/strcursor.rs`** — add unit tests for the 5 new cursor methods:

```rust
#[cfg(test)]
mod tests {
    // ...
    #[test]
    fn test_match_name() { ... }
    #[test]
    fn test_match_int() { ... }
    #[test]
    fn test_match_uint() { ... }
    #[test]
    fn test_match_float() { ... }
    #[test]
    fn test_match_bool() { ... }
}
```

> **ADVICE**: Port the 40 cursor-level tests from OGoPEGo:
> - `pkg/input/cursor_rune_test.go` lines 562–710
> - `pkg/input/cursor_str_test.go` lines 285–432
>
> Each OGoPEGo test exercises match success, match failure, and edge cases
> (underscores in numbers, leading `_` in names, signs, exponent forms, etc.).

**File: `tests/parsing_test.rs`** — add integration tests:
- Parse a grammar like `start: value=@int` on input `"42"` → success with value
- Parse `start: flag=@bool` on `"true"` → success
- Parse `start: name=@name` on `"hello_world"` → success
- Parse `start: val=@float` on `"3.14e-2"` → success
- Parse `start: val=@uint` on `"007"` → success
- Each with failure case (wrong input type)

---

## Build & Verification

```bash
cargo build              # Must compile without errors
cargo test               # All existing tests pass + new tests pass
just clippy              # No new warnings
```

The tally of new tests should be 40+ for cursor methods + 5–10 integration tests.

---

## Testing Approach Summary

| Test level | Location | Count | What it covers |
|-----------|----------|-------|----------------|
| Unit (cursor) | `strcursor.rs` | ~40 | Individual match methods: success, failure, edge cases |
| Integration | `tests/parsing_test.rs` | ~10 | Full pipeline: compile grammar with @meta → parse input → verify Tree result |

---

## Key Design Decisions

1. **No typed Tree variants**: Return matched strings as `Tree::Text(...)`.
   Don't add `Tree::Number` or `Tree::Bool` — the PEG Tree enum doesn't need
   them and it would require changing many downstream patterns.

2. **No new ParseFailure variants**: Use `Fail` for error in `do_parse_at()`.
   The error tracing already shows which expression failed. Add specific
   variants only if tests require distinct error messages.

3. **Metadata for integer/float naming**: Use `ExpKind::IntMeta` etc. as the
   variant names, not `ExpKind::Int` (too generic), not `ExpKind::@Int` (not
   valid Rust). The `Meta` suffix distinguishes from any future Int/Float
   literal types.

4. **No interface/trait for meta types**: Unlike OGoPEGo which uses separate
   struct types (Go doesn't have enums), Rust uses enum variants directly.
   No `MetaExp` base type needed — `ExpKind::NameMeta` etc. are natural unit
   variants.

5. **TatSu JSON round-trip**: Use `"NameMeta"` etc. as `__class__` in JSON
   export (match TatSu's format), so `grammar.to_json()` → `Grammar::from_json()`
   round-trips correctly.

## OGoPEGo Reference Files (study these)

| Concept | File |
|---------|------|
| Cursor interface (5 match methods) | `pkg/input/cursor.go:63-72` |
| RuneCursor implementation | `pkg/input/cursor_rune.go:293-419` |
| StrCursor implementation | `pkg/input/cursor_str.go:401-531` |
| Ctx interface + CoreCtx | `pkg/context/ctx.go` + `ctx_core.go` |
| EBNF compile dispatch | `pkg/peg/compile.go:377-390,520-531` |
| JSON import | `pkg/peg/import.go:582-595` |
| JSON export | `pkg/peg/export.go:58-67` |
| Cursor tests | `pkg/input/cursor_rune_test.go:562-710` |
| StrCursor tests | `pkg/input/cursor_str_test.go:285-432` |
