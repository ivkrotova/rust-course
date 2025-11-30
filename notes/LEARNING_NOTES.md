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

### fmt vs. check vs. clippy

#### cargo fmt
**What it does:** Automatically formats your code to match Rust style guidelines (spacing, indentation, line breaks). Doesn't change logic, just makes code consistent and readable.

**Usage:**
```bash
cargo fmt                      # Format all code in project
cargo fmt --check              # Check if code is formatted (CI/CD)
```

#### cargo check
**What it does:** Compiles your code to check for errors but doesn't produce an executable. Much faster than `cargo build` because it skips the final steps. Use this constantly while coding to catch type errors, borrowing issues, etc.

**Usage:**
```bash
cargo check                    # Quick compilation check
```

#### cargo clippy
**What it does:** A linter that analyzes your code for common mistakes, non-idiomatic patterns, and performance issues. It catches things like unnecessary clones, overly complex logic, or better ways to write something. Goes beyond what the compiler checks.

**Usage:**
```bash
cargo clippy                   # Run linter
cargo clippy -- -D warnings    # Treat warnings as errors
```

#### Typical Workflow
Run `check` frequently while coding → run `clippy` before committing → run `fmt` to clean up formatting before pushing.

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

### Modules

**The basic idea:** A module is just a namespace that contains definitions - functions, structs, traits, and even other modules. Use the `mod` keyword to create them.

Let's say we're building a simple app and want to organize user-related code:

```rust
// You can define a module inline
mod users {
    pub fn create_user(name: &str) -> String {
        format!("Created user: {}", name)
    }
    
    fn internal_helper() {
        // This is private by default
    }
}

fn main() {
    let result = users::create_user("Alice");
    println!("{}", result);
}
```

**Key things to know:**

Everything in a module is **private by default**. If you want something to be accessible from outside the module, you need to mark it with `pub` (like `pub fn create_user` above). This is Rust's way of enforcing encapsulation - you decide what's part of your public API.

You can also **split modules into separate files**, which is what you'll do in real projects. If you have a file called `users.rs`, you can use it as a module:

```rust
// In main.rs
mod users;  // This looks for users.rs

fn main() {
    users::create_user("Alice");
}
```

The `use` keyword lets you **bring things into scope** so you don't have to write the full path every time:

```rust
use users::create_user;

fn main() {
    create_user("Alice");  // No need for users:: prefix
}
```

For larger projects, you can nest modules (like folders within folders), and you use `mod.rs` files or the newer style with folder names to organize them.

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

