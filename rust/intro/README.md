# Intro

The code in this directory illustrates the similarities of certain introductory Rust constructs to those in Python.

## Setup and run

The `chrono` crate is required to handle datetimes in Rust. In Rust, dependencies are managed using the `cargo` tool. To add the `chrono` crate as a dependency, run the following command.

```bash
cargo add chrono
```

Then, run the script `main.rs` as follows.

```bash
cargo run
```

## Concepts illustrated

The following basic concepts (with their rough Rust equivalents) are illustrated in this code:

- Traits (perform the role of Python's protocols and magic methods)
- Enumerate
- Zip
- Tuple unpacking and indexing
- Anonymous functions
- Single-line if statements
- Map and filter (perform the role of Python's list comprehensions)

We start with defining a struct, `Person`, that has a name and an age. We then define an array of `Person` structs, and use the above constructs to illustrate how to work with them. See the file `main.rs` for implementation details.

## Inputs

There aren't any inputs to the script: the data is generated in the script itself, and is handled by the data structures within.

## Output

```console
Megan is 28 years old
Person: Megan, 28
Person 0: James is 33 years old
Person 1: Salima is 31 years old
[Person: Alice, 24, Person: Charlie, 45]
Youngest age: 18, oldest age: 65
Middle age: 41
Rohan is the youngest person at 18 years old
Josephine is 20 years old. Born in a leap year?: true
Wesley is 31 years old. Born in a leap year?: false
Persons born after 1995: [("Ibrahim", 26)]
```

## Takeaways

Once you read the Rust code, take a look at the equivalent Python code in the [Python script](../../python/intro/main.py) and hopefully, you'll start appreciating some of the similarities!
