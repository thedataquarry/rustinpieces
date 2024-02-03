# Zip

In both Python and Rust, the `zip` function exists to construct an iterator over two or more iterables.

## Python

Recall from the [first example](../intro/protocols_traits.md#python-protocols) that we defined a
`Person` class with a name and an age attribute.

If we have two lists, one containing names and one containing ages. `zip` conveniently allows us
to iterate over both lists.

```py
def run3() -> None:
    names = ["Alice", "Charlie"]
    ages = [24, 45]
    persons = []
    for name, age in zip(names, ages):
        person = Person(name, age)
        persons.append(person)
    print(f"{repr(persons)}")
```

The `append` method is used to add a new item to the end of the list, similar to `push` in Rust.

Running the above function via `main.py` gives us the following output:

```bash
[Person('Alice', 24), Person('Charlie', 45)]
```

Note that the `zip` method returns an iterator over tuples of the same length as the _shortest_ iterable passed to it.
So, if we'd passed one list with 3 items and one list with 2 items, the resulting iterator would have 2 items.

## Rust

Recall from the [first example](../intro/protocols_traits.md#rust-traits) that we defined a `Person` struct with a
name and an age attribute, in a similar way to the Python example.

Consider that we have two vectors, one containing names and one containing ages. `zip` conveniently allows us
to iterate over both vectors.

```rs
fn run3() {
    let names = ["Alice", "Charlie"];
    let ages = [24, 45];
    let mut persons = vec![];
    for (name, age) in names.iter().zip(ages.iter()) {
        persons.push(Person::new(name, *age));
    }
    println!("{:?}", persons);

}
```

- The `zip` method can only called on an iterator, so we need to call `iter` on both vectors before we can call `zip`.
- The `push` method is used to add a new item to the end of the vector, just like `append` in Python.

Again, there's no need to "remember" any of this: the Rust compiler is super helpful in calling you out on common
mistakes, while offering a helpful solution!

Running the function via `main.rs` gives us the same output as in Python:

```bash
[Person: Alice, 24, Person: Charlie, 45]
```

## Takeaways

The functionality of `zip` is the largely the same in both Python and Rust.

There really aren't too many differences, but it's worth noting that Rust's `zip` is held to account by the strict type system,
so it's typically only available on iterators (unless you implement your own traits or macros). Python's `zip` method, on the
other hand, can be called on any iterable (lists, tuples, dictionaries, and so on) because of its dynamic, loosely typed nature.
