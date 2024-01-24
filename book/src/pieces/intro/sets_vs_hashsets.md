# Sets vs. hashsets

Python's `set` is an unordered collection of unique items, where duplicate items are not allowed.
Rust's `HashSet` performs the same function.

## Python

Consider the following function in which we define a set of processors and their corresponding
market names.

```py
def run9() -> None:
    processors = {
        "Intel Core i9",
        "Intel Core i7",
        "Intel Core i5",
        "AMD Ryzen 7",
        "AMD Ryzen 5",
        "AMD Ryzen 3",
    }
    # Duplicate values are ignored
    processors.add("Intel Core i7")
    processors.add("AMD Ryzen 5")
    # Check for presence of value
    is_item_in_set = "AMD Ryzen 3" in processors
    print(f'Is "AMD Ryzen 3" in the set of processors?: {is_item_in_set}')
```

The purpose of the above function is to check for the presence of a value in the set of processors.
When we add duplicate values to the set, they are ignored.

Running the above function via `main.py` gives us the following output:

```bash
Is "AMD Ryzen 3" in the set of processors?: True
```

## Rust

We define the below function in Rust, where we define a hashset of processors and their
corresponding market names.

```rs
use std::collections::HashSet;

fn run9() {
    let mut processors = HashSet::new();
    processors.insert("Intel Core i9");
    processors.insert("Intel Core i7");
    processors.insert("Intel Core i5");
    processors.insert("AMD Ryzen 7");
    processors.insert("AMD Ryzen 5");
    processors.insert("AMD Ryzen 3");
    // Duplicate values are ignored
    processors.insert("Intel Core i7");
    processors.insert("AMD Ryzen 5");
    // Check for presence of value
    let value = "AMD Ryzen 3";
    println!(
        "Is \"AMD Ryzen 3\" in the hashset of processors?: {}",
        processors.contains(&value)
    );
}
```

The purpose of the above function is to check for the presence of a value in the hashset of
processors. When we add duplicate values to the hashset, they are ignored.

Running the function via `main.rs` gives us the same output as in Python:

```bash
Is "AMD Ryzen 3" in the hashset of processors?: true
```

## Takeaways

- Python and Rust contain collections that allow for the storage of unique items.
- Python's `set` can contain items of any type, while Rust's `HashSet` can only contain items of the
  same type that were specified at the time of initialization.
