# Rules

## Interaction

- Never change files without a plan and user authorization
- Always consult with the User before making changes that impact multiple files
- Evaluate changes before applying them (no "apply and see" approach)
- Do not act on assumptions. Always verify assumptions with the User.

## Testing

- Tests can be marked with `#[ignore]` but must compile
- Run tests with `cargo test --verbose`

## Code Tools

- Do not use `sed`, `awk`, or similar text tools to modify code
- Use `ast-grep` for code search and rewrite tasks (see https://ast-grep.github.io)
- When using `rg`, explicitly name the directories to search (e.g., `rg -l 'pattern' src tests`) instead of excluding directories
- Create `./tmp/` for temporary files instead of using `/tmp`
