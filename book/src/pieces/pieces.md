# Pieces

## What is the purpose of each piece?

A piece is a self-contained project with Python and Rust implementations that each perform the same tasks. The purpose of each piece is to help Python developers gain familiarity with Rust, and vice-versa, by comparing and contrasting the two languages in a top-down manner.

A piece's directory structure is organized as follows:

```
pieces
├── piece-1
│   ├── python
│   │   ├── main.py
│   │   └── test_main.py
│   └── rust
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── piece-2
├── piece-3
└── ...
```

Each piece comes with Python and Rust source code, and their associated tests. When using Rust's test client, test code is placed in the same file as the code it's testing, and is marked with the `#[cfg(test)]` attribute. When using Python's test client, test code is placed in a separate file, and is marked with the `test_` prefix.

For Python, `pip` is the package manager of choice, and `pytest` is the test client used throughout the code. For Rust, `cargo` is the package manager of choice, and Rust's inbuilt test client, invoked by `cargo test`, is used throughout.
