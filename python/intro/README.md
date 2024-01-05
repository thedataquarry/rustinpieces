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
- First-class functions
- Anonymous functions
- Single-line if statements
- List comprehensions

We start with defining a simple data structure, `Person`, that has a name and an age. We then define a list of `Person` objects, and use the above constructs to illustrate how to use them. See the file `main.py` for implementation details.

## Inputs

There aren't any inputs to the script: the data is generated in the script itself, and is handled by the data structures within.

## Output

```console
Megan is 28 years old
Person: Megan, 28
0: James is 33 years old
1: Salima is 31 years old
Khalil is 50 years old. They are estimated to be born in 1974
Rohan is the youngest person at 18 years old
Josephine is 20 years old. Born in leap year?: True
Will is 31 years old. Born in leap year?: False
Adebayo is estimated to be born in 1995
Ibrahim is estimated to be born in 1998
```

## Takeaways

Once you look at the Python code, take a look at the equivalent Rust code in the `rust` directory and hopefully, you'll start appreciating some of the similarities!
