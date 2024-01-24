# Lambdas vs. closures

Anonymous functions are functions that are not bound to a name. In Python, they are called
[lambdas](https://docs.python.org/3/tutorial/controlflow.html#lambda-expressions). In Rust, they are
called [closures](https://doc.rust-lang.org/book/ch13-01-closures.html). Both are useful for short,one-off functions that are not used anywhere else.

## Python

Recall from the [first example](../intro/protocols_traits.md#python-protocols) that we defined a
`Person` class with a name and an age attribute.

In the following example, we use the `sorted` function to sort a list of `Person` objects by their
age.

```py
def run5() -> None:
    persons = [Person("Aiko", 41), Person("Rohan", 18)]
    sorted_by_age = sorted(persons, key=lambda person: person.age)
    youngest_person = sorted_by_age[0]
    print(f"{youngest_person.name} is the youngest person at {youngest_person.age} years old")
```

The `sorted` function takes an optional `key` argument, which is a function that is called on each
item in the list to determine the value to sort by. In this case, we use a lambda to return the
`age` attribute of each `Person` object.

```bash
Rohan is the youngest person at 18 years old
```

## Rust

Recall from the [first example](../intro/protocols_traits.md#rust-traits) that we defined a `Person`
struct with a name and an age attribute, in a similar way to the Python example.

In the following example, we use the `sort_by_key` method to sort a vector of `Person` objects by
their age.

```rs
fn run5() {
    let mut persons = vec![Person::new("Aiko", 41), Person::new("Rohan", 18)];
    // Sort by age
    persons.sort_by_key(|p| p.age);
    let youngest_person = persons.first().unwrap();
    println!(
        "{} is the youngest person at {} years old",
        youngest_person.name, youngest_person.age
    );
```

The `sort_by_key` method takes a closure that is called on each item in the vector to determine the
value to sort by. In this case, we use a closure operator `||` to return the `age` attribute of each
`Person` object.

```bash
Rohan is the youngest person at 18 years old
```

## Takeaways

- Lambdas and closures are anonymous functions that are not bound to a name, or are passed as
  arguments to other functions.
- Lambdas and closures are useful for short, one-off functions that are not used anywhere else.
- Closures are more powerful than lambdas because they define higher-order functions that can
  capture their environment - this is out of scope for this book, but you can read more about it
  [here](https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment-with-closures).
