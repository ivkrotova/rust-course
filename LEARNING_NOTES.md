# Rust Learning Notes

A collection of concepts, tools, and techniques learned during the Yandex Rust course.

---

## Table of Contents
- [Development Tools](#development-tools)
- [Core Concepts](#core-concepts)
- [Best Practices](#best-practices)
- [Common Patterns](#common-patterns)
- [Other](#other)

---

## Development Tools

### Essential Rust Components

#### Clippy - Linter

**Command:** `rustup component add clippy`

**What it does:** Installs Clippy, Rust's official linter that analyzes your code and suggests improvements for correctness, performance, and style.

**Usage:**
```bash
cargo clippy                    # Run linter
cargo clippy -- -D warnings     # Treat warnings as errors
```

Helps to maintain code more idiomatic.



#### Rustfmt - Formatter

**Command:** `rustup component add rustfmt`

**What it does:** Installs rustfmt, Rust's official code formatter that automatically formats your code according to the Rust style guide.

**Usage:**
```bash
cargo fmt                      # Format all code in project, use in the root folder
cargo fmt --check              # Check if code is formatted (CI/CD)
rustfmt src/main.rs            # Format specific file
```

If you want to use custom rules, not the default ones, you can add ```rustfmt.toml``` or ```.rustfmt.toml``` to the root folder and store there your custom rules.

Run ```cargo fmt``` before commiting.

---

### Pre-commit Hooks

Pre-commit hooks automatically run checks before allowing a commit, helping maintain code quality.

#### Option 1: Manual Git Hook (Simplest)

Create a script at `.git/hooks/pre-commit`:

```bash
#!/bin/bash

# Run cargo fmt
echo "Running cargo fmt..."
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "❌ Code is not formatted. Please run 'cargo fmt' before committing."
    exit 1
fi

# Run cargo clippy
echo "Running cargo clippy..."
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy found issues. Please fix them before committing."
    exit 1
fi

echo "✅ All checks passed!"
exit 0
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

**Pros:** Simple, no dependencies
**Cons:** Not committed to git (each developer must set up manually)

#### Option 2: Using `pre-commit` Framework

Install the framework:
```bash
pip install pre-commit
# or
brew install pre-commit
```

Create `.pre-commit-config.yaml` in project root:
```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt
        args: ['--', '--check']
        language: system
        types: [rust]
        pass_filenames: false
      
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy
        args: ['--', '-D', 'warnings']
        language: system
        types: [rust]
        pass_filenames: false
```

Install hooks:
```bash
pre-commit install
```

**Pros:** Cross-platform, committed to repo, team-wide consistency
**Cons:** Requires Python/pre-commit installation

#### Option 3: Using `rusty-hook` (Rust-native)

Add to `Cargo.toml`:
```toml
[dev-dependencies]
rusty-hook = "0.11"
```

Create `.rusty-hook.toml`:
```toml
[hooks]
pre-commit = "cargo fmt -- --check && cargo clippy -- -D warnings"
```

Initialize:
```bash
cargo test  # Automatically installs hooks on first build
```

**Pros:** Rust-native, committed to repo
**Cons:** Requires adding dev dependency

#### Bypassing Hooks (When Necessary)
```bash
git commit --no-verify  # Skip all hooks (use sparingly!)
```

#### Undoing Commits

**Remove last commit but keep changes (soft reset):**
```bash
git reset --soft HEAD~1
```

**Remove last commit and discard changes (hard reset):**
```bash
git reset --hard HEAD~1
```

**Remove multiple commits:**
```bash
git reset --hard HEAD~N  # Replace N with number of commits to remove
```

**Remove the very first commit (when it's the only commit):**
```bash
git update-ref -d HEAD
```

**Unstage files:**
```bash
git rm --cached <file>     # Unstage a specific file
git reset                  # Unstage all files
```

#### Setting Up Remote Repository

**1. Configure Git Profile (one-time setup):**
```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# Verify configuration
git config --list | grep user
```

**2. Create Remote Repository:**
- Go to GitHub/GitLab and create a new repository
- Copy the repository URL (HTTPS or SSH)

**3. Connect Local to Remote:**
```bash
# Add remote origin
git remote add origin <repository-url>

# Verify remote is added
git remote -v

# Example:
# git remote add origin https://github.com/username/repo.git
# or
# git remote add origin git@github.com:username/repo.git
```

**4. First Push to Remote:**
```bash
# Stage all files
git add .

# Commit with message
git commit -m "Initial commit"

# Push to remote (first time)
git push -u origin master
# or for newer repos with 'main' as default branch:
# git push -u origin main

# Subsequent pushes (after first time)
git push
```

**5. Check Remote Connection:**
```bash
git remote -v              # Show remote URLs
git remote show origin     # Detailed remote info
```

**6. Change Remote URL (if needed):**
```bash
git remote set-url origin <new-url>
```

---

## Core Concepts

### Comments in Rust

Rust has two main types of comments:

#### 1. Regular Comments
Used for internal code explanations, notes within functions, and general comments.

**Single-line comments:**
```rust
// This is a single-line comment
let x = 5; // Comments can also be placed after code
```

**Multi-line comments:**
```rust
/* This is a multi-line comment
   that spans multiple lines
   and can contain more detailed explanations */
```

#### 2. Documentation Comments (Doc Comments)
Used to document items like functions, structs, modules, etc. These appear in generated documentation (via `cargo doc`).

**Outer doc comments** (most common - document the item that follows):
```rust
/// This function adds two numbers.
/// 
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Inner doc comments** (document the enclosing item):
```rust
mod my_module {
    //! This module contains utility functions.
    //! It provides basic mathematical operations.
}
```

#### Key Rules:
- Doc comments (`///` or `//!`) must be placed **before** the item they document (or inside for `//!`)
- Regular comments (`//` or `/* */`) can be placed anywhere
- Doc comments on macro invocations (like `println!`) don't work - use regular comments instead
- Doc comments support Markdown formatting

### Ownership & Borrowing
*Notes to be added...*

### Lifetimes
*Notes to be added...*

### Error Handling
*Notes to be added...*

---

## Best Practices

*Notes to be added...*

---

## Common Patterns

*Notes to be added...*

## Other

### Unit Type () in Rust

Python: the function doesn't return the explicit value -> implicitly returns None
Rust: returns (), the unit type.
Similar to void type in Java and C.



---

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Documentation](https://github.com/rust-lang/rust-clippy)
- [Rustfmt Documentation](https://github.com/rust-lang/rustfmt)

---

**Last Updated:** November 30, 2025

