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

