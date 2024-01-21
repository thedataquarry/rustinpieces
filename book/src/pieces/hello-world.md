# Hello world!

This is the conventional first program that you write when learning a new programming language. It's a simple program that prints the text `Hello, world!` to the console.

Navigate to the `pieces/hello_world` directory in the [repo](https://github.com/thedataquarry/rustinpieces/tree/main/pieces/hello_world) to get started.

## Python

The file `main.py` has just one line of code:

```python
print("Hello, world!")
```

The program is run as follows:

```bash
python main.py
```

## Rust

The file `main.rs` has just three lines of code:

```rs
fn main() {
    println!("Hello, world!");
}
```

The program is run via `cargo`:

```bash
cargo run
```

## Output

```bash
Hello, world!
```

## Takeaways

Rust's `println!` is similar to Python's `print` function, but it's a **macro**, not a function. It simply prints the standard output to the console followed by a newline character.

Macros are a powerful Rust feature that allow you to write code that writes other code. We'll see more examples of macros in later pieces, but for now, it's enough to know that in Rust, macros are invoked with an exclamation mark `!` at the end of their name.
