## Dicts vs. hashmaps

Python's `dict` is essentially a hash table, which is a data structure that maps keys to values.
Rust's `HashMap` performs the same function. Both are collections of key-value pairs where the
keys must be unique, but the values can be duplicated. The purpose of dicts and hashmaps is to
allow for fast lookup of values by key.

## Python

Consider the below function in Python, where we define a dictionary of processors and their
corresponding market names.

```py
    processors = {
        "13900KS": "Intel Core i9",
        "13700K": "Intel Core i7",
        "13600K": "Intel Core i5",
        "1800X": "AMD Ryzen 7",
        "1600X": "AMD Ryzen 5",
        "1300X": "AMD Ryzen 3",
    }

    # Check for presence of value
    is_item_in_dict = "AMD Ryzen 3" in processors.values()
    print(f'Is "AMD Ryzen 3" in the dict of processors?: {is_item_in_dict}')
    # Lookup by key
    key = "13900KS"
    lookup_by_key = processors[key]
    print(f'Key "{key}" has the value "{lookup_by_key}"')
```

The first portion checks for the presence of a value in the dictionary, while the second portion
looks up the value by key.

Running the above function via `main.py` gives us the following output:

```bash
Is "AMD Ryzen 3" in the dict of processors?: True
Key "13900KS" has the value "Intel Core i9"
```

## Rust

We define the below function in Rust, where we define a hashmap of processors and their
corresponding market names.

```rs
use std::collections::HashMap;

fn run8() {
    let mut processors = HashMap::new();
    processors.insert("13900KS", "Intel Core i9");
    processors.insert("13700K", "Intel Core i7");
    processors.insert("13600K", "Intel Core i5");
    processors.insert("1800X", "AMD Ryzen 7");
    processors.insert("1600X", "AMD Ryzen 5");
    processors.insert("1300X", "AMD Ryzen 3");

    // Check for presence of value
    let value = "AMD Ryzen 3";
    let mut values = processors.values();
    println!(
        "Is \"AMD Ryzen 3\" in the hashmap of processors?: {}",
        values.any(|v| v == &value)
    );
    // Lookup by key
    let key = "13900KS";
    let lookup_by_key = processors.get(key);
    println!(
        "Key \"{}\" has the value \"{}\"",
        key,
        lookup_by_key.unwrap()
    );
}
```

Just like in the Python version, the first portion checks for the presence of a value in the
hashmap, while the second portion looks up the value by key.

Running the function via `main.rs` gives us the same output as in Python:

```bash
Is "AMD Ryzen 3" in the hashmap of processors?: true
Key "13900KS" has the value "Intel Core i9"
```

## Takeaways

Python and Rust contain collections that store key-value pairs for fast lookups. A key difference is
that Python's `dict` keys can be any hashable type and values can be of any
type, but in Rust, _both_ the keys and values of a `HashMap` must be of the same type.

In Python, this `dict` is perfectly valid:

```python
# You can have a dict with keys of different types
example = {
    "a": 1,
    1: 2
}
```

In Rust, the compiler will enforce that the keys and values are of the same type, based on
the first entry's inferred types.

```rs
let mut example = HashMap::new();
example.insert("a", 1);
// This errors because the previous values specified the key as &str
example.insert(1, 2);
// This is valid
example.insert("b", 2);
```
