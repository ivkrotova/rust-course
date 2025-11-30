# Commit Style Guide

Quick reference for writing useful commit messages.

## Structure

```
type: brief description (50 chars max)

Optional body explaining what and why.
Wrap at 72 characters.
```

## Types

Pick the one that fits:

- `feat` – new feature or capability
- `fix` – bug fix
- `refactor` – code restructure without behavior change
- `perf` – performance improvement
- `docs` – documentation only
- `test` – add or update tests
- `chore` – tooling, deps, config

## Style Rules

**Do:**
- Use imperative mood: "Add feature" not "Added feature"
- Be specific: "Fix panic in empty parser" not "Fix bug"
- Keep it under 50 chars
- Split unrelated changes into separate commits

**Don't:**
- End with a period
- Use vague terms: "Fix stuff", "Update code", "WIP"
- Combine multiple unrelated changes
- Write essays (save details for PR descriptions)

## Examples

Good:
```
feat: Add JWT token refresh
fix: Handle null pointer in config parser
refactor: Extract validation into separate module
perf: Cache compiled regex patterns
```

Bad:
```
Fixed some bugs and updated docs
WIP
Changes
Update main.rs
```

## When to Add a Body

Add context when the commit isn't self-explanatory:

```
fix: Prevent race condition in cache writes

Multiple threads were writing to cache simultaneously.
Added mutex lock around write operations. Performance
impact is negligible (~2ms per write).
```

Skip the body when the subject line says it all:
```
docs: Fix typo in README
```

## Atomic Commits

One logical change per commit. If you can't describe it in one line without using "and", split it up.

Good:
- Commit 1: `feat: Add user validation`
- Commit 2: `feat: Add error messages`

Bad:
- Commit 1: `feat: Add user validation and error messages and logging`

## Quick Commands

```bash
# Fix last commit message
git commit --amend

# Squash messy commits before pushing
git rebase -i HEAD~3

# Reference issues
git commit -m "fix: Handle edge case in parser (#42)"
```

---

The goal: make `git log --oneline` tell a clear story.

