---
apply: always
---

# AGENTS

## Research First

Study [README.md](README.md) to understand the project context before making changes. Analyze the current project structure.

## Plan Before Action

Never change files without a plan and user authorization. Always consult with the User before making changes that impact multiple files. Evaluate changes before applying them — trial and error is not allowed. Don't act on assumptions; always verify with the User.

## Scope

Never change any files that were not explicitly named in the authorization. Never try to access or modify a file or directory outside the current project's directory tree.

## Ownership

The User is the sole owner of files and other assets. Never modify any file or asset without explicit consent from the User.

## Testing

Tests may be marked to skip or ignore, but must compile. Never delete a failing test — it is a red flag that indicates a bug in the project.

## Privileged Actions

The following git operations require explicit user approval and must never be executed autonomously:

- `git merge` — merging branches into main or any shared branch
- `git push` — pushing commits to any remote
- `git commit` — committing to main (commits to feature branches are permitted after test passes)

Agents may create branches, stage files, and commit on feature branches without approval. The user performs all merges and pushes.

## Shared Understanding

Interview the User about every aspect of a plan until there is a shared understanding. Walk down each branch of the design tree resolving dependencies between decisions one-by-one.
