# Intro

The code in this directory illustrates the similarities of certain introductory Python constructs to those in Rust.

## Setup and run

No dependencies are necessary -- simply install the latest version of Python and run the script `main.py`.

```bash
python main.py
```

## Concepts illustrated

The following basic concepts (with their rough Rust equivalents) are illustrated in this code:

- Protocols
- Enumerate
- Zip
- Tuple unpacking and indexing
- Anonymous functions
- Single-line if statements
- List comprehensions

We start with defining a simple data structure, `Person`, that has a name and an age. We then define a list of `Person` objects, and use the above constructs to illustrate how to work with them. See the file `main.py` for implementation details.

## Inputs

There aren't any inputs to the script: the data is generated in the script itself, and is handled by the data structures within.

## Output

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

## Takeaways

Once you read the Python code, take a look at the equivalent Rust code in the [Rust script](../../rust/intro/src/main.rs) and hopefully, you'll start appreciating some of the similarities!
