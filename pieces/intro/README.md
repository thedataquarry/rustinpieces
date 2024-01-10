# Intro

The code in this directory illustrates the similarities of certain introductory Python constructs
to those in Rust.

## Goal

The following basic concepts (with their rough equivalents in Rust) are illustrated in this code:

Python | Rust
:--- | :---
Protocols/special methods | Traits
Enumerate | Enumerate
Zip | Zip
Tuple | Tuples
Lambdas | Closures
List comprehensions | Map/filter

We start with defining a simple data structure, `Person` (a `class` in Python and a `struct` in Rust with an `impl` block). The `Person` has a name and an age. We then
define a list of `Person` objects, and use the above constructs to illustrate how to work with
them.

### Inputs

There aren't any inputs in this piece: the data is generated in the script itself, and is handled
by the data structures within.

### Output

```console
Megan is 28 years old
Person: Megan, 28
0: James is 33 years old
1: Salima is 31 years old
[Person: Alice, 24, Person: Charlie, 45]
Youngest age: 18, oldest age: 65
Middle age: 41
Rohan is the youngest person at 18 years old
Josephine is 20 years old. Born in a leap year?: True
Wesley is 31 years old. Born in a leap year?: False
Persons born after 1995: [('Ibrahim', 26)]
```

## Python

### Setup and run

No dependencies are necessary -- simply install the latest version of Python and run the script
`main.py`.

```bash
python main.py
```

## Rust

### Setup and run

The `chrono` crate is required to handle datetimes in Rust. In Rust, dependencies are managed using
the `cargo` tool. To add the `chrono` crate as a dependency, run the following command.

```bash
cargo add chrono
```

Then, run the script `main.rs` as follows.

```bash
cargo run
```

## Takeaways

Once you read the Python code, take a look at the equivalent Rust code and hopefully, you'll start
appreciating some of the similarities!
