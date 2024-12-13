# Intro

The code in this directory illustrates the similarities of certain introductory Python constructs
to those in Rust.

## Goal

The following basic concepts (with their rough equivalents in Rust) are illustrated in this code:

| Python                    | Rust       |
| :------------------------ | :--------- |
| Protocols/special methods | Traits     |
| Enumerate                 | Enumerate  |
| Zip                       | Zip        |
| Tuple                     | Tuples     |
| Lambdas                   | Closures   |
| List comprehensions       | Map/filter |
| Dictionary                | HashMap    |
| Set                       | HashSet    |

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
Person 0: James is 33 years old
Person 1: Salima is 31 years old
[Person: Alice, 24, Person: Charlie, 45]
Youngest age: 18, oldest age: 65
Middle age: 41
Rohan is the youngest person at 18 years old
Josephine is 20 years old. Born in a leap year?: true
Wesley is 31 years old. Born in a leap year?: false
Persons born after 1995: [("Ibrahim", 26)]
Is "AMD Ryzen 3" in the hashmap of processors?: true
Key "13900KS" has the value "Intel Core i9"
Is "AMD Ryzen 3" in the hashset of processors?: true
```

## Python

### Setup and run

No dependencies are necessary -- simply navigate to the `intro/python` directory and run the script
using the `uv` package manager.

```bash
uv run main.py
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

### Differences to be aware of

In Python dictionaries and sets, you can have a mix of types. However, in Rust all of the types have
to be the same.

Below is an example for `dict` in Python and `HashMap` in Rust.

#### Python

```py
# This is fine in Python
example = {"first": "a value", "second": 1}
```

#### Rust

```rs
let mut example = HashMap::new();
// This is fine in Rust
example.insert("first", "a value");
// The following will fail to compile because the first value entered set the type as HashMap<&str, &str>
example.insert("second", 1);
```

The same is also true for `set` in Python and `HashSet` in Rust.
