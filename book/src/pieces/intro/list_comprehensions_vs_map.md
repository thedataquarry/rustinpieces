# List comprehensions vs map/filter

One of Python's most popular features is its list comprehensions. They are a concise way to create
lists from existing lists. Rust is more functional than Python, so it has a similar feature called
`map` and `filter`. Although map and filter functions are availble in Python, they are not as
commonly used as list comprehensions.

## Python

Consider the following function in which we print a message depending on which persons from
a list of `Person` objects are born after the year 1995, based on their current age.

```py
def run7() -> None:
    """
    1. List comprehensions
    """
    persons = [Person("Issa", 39), Person("Ibrahim", 26)]
    persons_born_after_1995 = [
        (person.name, person.age) for person in persons if approx_year_of_birth(person) > 1995
    ]
    print(f"Persons born after 1995: {persons_born_after_1995}")
```

The list comprehension in the above function essentially does the following:

1. Iterate over the list of `Person` objects
2. Unpack each `Person` tuple into their name and age
3. For each person, check if their approximate year of birth is greater than 1995

Running the above function via `main.py` gives us the following output:

```bash
Persons born after 1995: [('Ibrahim', 26)]
```

## Rust

We can define the below function in Rust, where we print a message depending on which persons from
a vector of `Person` objects are born after the year 1995, based on their current age.

```rs
fn run7() {
    let persons = vec![Person::new("Issa", 39), Person::new("Ibrahim", 26)];
    let result = persons
        .into_iter()
        .filter(|p| approx_year_of_birth(p) > 1995)
        .map(|p| (p.name, p.age))
        .collect::<Vec<(String, u8)>>();
    println!("Persons born after 1995: {:?}", result)
```

The `filter` and `map` functions in the above function essentially do the following:

1. Turn the `persons` vector into an iterator and iterate over the `Person` objects
2. For each person, check if their approximate year of birth is greater than 1995
3. If the above condition is true, then create a tuple of their name and age
4. Collect all the tuples into a vector of unsigned 8-bit integers

Running the function via `main.rs` gives us the same output as in Python:

```bash
Persons born after 1995: [("Ibrahim", 26)]
```

The Rust version is a little more verbose than the Python version, but it's still quite readable.

## Takeaways

- Both Python and Rust have convenient ways to create iterables without having to use explicit loops.
- Python's list comprehensions are more concise than Rust's `map` and `filter` functions in
  most cases.
- Rust's `map` and `filter` functions show that Rust is more functional than Python in its syntax.
