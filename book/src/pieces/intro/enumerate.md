# Enumerate

In both Python and Rust, the `enumerate` function exists to
iterate over a list while keeping track of the index of the current item.

## Python

Recall from the [first example](../intro/protocols_traits.md#python-protocols) that we defined a
`Person` class with a name and an age attribute.

We can instantiate a list of `Person` objects and iterate over them using `enumerate`.

```py
def run2() -> None:
    persons = [Person("James", 33), Person("Salima", 31)]
    for i, person in enumerate(persons):
        print(f"Person {i}: {str(person)}")
```

Running the above function via `main.py` gives us the same output as in Rust:

```bash
Person 0: James is 33 years old
Person 1: Salima is 31 years old
```

The `enumerate` method returns a tuple of `(index, item)` for each item in the list,
allowing us to access the index of the current item as we iterate over the list in a `for` loop.

## Rust

Recall from the [first example](../intro/protocols_traits.md#rust-traits) that we defined a `Person` struct with a
name and an age attribute, in a similar way to the Python example.

We can instantiate a vector of `Person` objects and iterate over them using `enumerate`. In Rust, a vector is a
dynamic array allocated on the heap, provided by the standard library. It performs a similar function to a Python list.
However, unlike in Python, a vector can only contain objects of the same type, in this case, `Person`.

```rs
fn run2() {
    let persons = vec![Person::new("James", 33), Person::new("Salima", 31)];
    for (i, p) in persons.iter().enumerate() {
        println!("Person {}: {}", i, p)
    }
}
```

The `vec!` macro is syntactic sugar for `Vec::new()`, which creates a new vector of `Person` objects.
Additionally, the `iter` method returns an iterator over the vector, which is required before we can
call the `enumerate` method on it.

Running the above function via `main.rs` gives us the same output as in Python:

```bash
Person 0: James is 33 years old
Person 1: Salima is 31 years old
```

## Takeaways

- Both Python and Rust contain a convenience method called `enumerate` to iterate over a list while keeping
  track of the index of the current item.
- Python lists are dynamic arrays that can contain objects of _any_ type.
- Rust vectors are heap-allocated dynamic arrays that can only contain objects of the _same_ type.
