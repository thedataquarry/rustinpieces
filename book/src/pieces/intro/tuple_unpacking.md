# Tuple unpacking

Both Python and Rust support tuple unpacking in similar ways.

### Python

Consider the following function in which we unpack the youngest and oldest age from a sorted
list of ages:

```python
def run4() -> None:
    sorted_ages = (18, 41, 65)
    youngest, _, oldest = sorted_ages
    print(f"Youngest age: {youngest}, oldest age: {oldest}")
    print(f"Middle age: {sorted_ages[1]}")
```

The `_` is a special variable name in Python that indicates that we don't care about the value,
allowing the unused value to be cleared by the Python memory manager during runtime. We can
still access the middle age via the index operator for tuples, `sorted_ages[1]`.

Running the above function via `main.py` gives us the following output:

```bash
Youngest age: 18, oldest age: 65
Middle age: 41
```

### Rust

We can write the following function in which we unpack the youngest and oldest age from a sorted
list of ages:

```rs
fn run4() {
    let sorted_ages: (u8, u8, u8) = (18, 41, 65);
    let (youngest, _, oldest) = sorted_ages;
    println!("Youngest age: {}, oldest age: {}", youngest, oldest);
    println!("Middle age: {}", sorted_ages.1);
}
```

Just like in Python, the `_` indicates that we don't care about the middle value. The difference
is that in Rust, there isn't a garbage collector (or reference counter) like in Python, so the
unused value is only kept in scope till the function is exited. Also, we need to explicitly
declare the type of each age element as unsigned 8-bit integers.

The index operator for tuples in Rust is `.`, so we can access the middle age via `sorted_ages.1`.

Running the function via `main.rs` gives us the same output as in Python:

```bash
Youngest age: 18, oldest age: 65
Middle age: 41
```

## Takeaways

- Tuple unpacking is largely the same in Python and Rust.
- There are some minor differences between Python and Rust tuples:
  - In rust, elements of a tuple are mutable, while in Python, they are immutable (lists are
    mutable in Python).
  - In Rust, the index operator for tuples is `.`, while in Python, it's `[]`.
