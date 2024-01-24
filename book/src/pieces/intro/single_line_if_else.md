# Single line if-else

Both Python and Rust support single line if-else statements. This is especially useful when
performing simple operations on a value, allowing for more concise code.

## Python

Consider the following function in which we print a message depending on whether a person is
born in a leap year or not.

To do this, we first define a function `approx_year_of_birth` that returns the approximate year.

```py
def approx_year_of_birth(person: Person) -> int:
    birth_year_approx = datetime.now().year - person.age
    return birth_year_approx
```

<div class="warning">

The leap year logic used above is simplistic and does not account for edge cases.
It's used here purely for the purposes of illustration.

</div>

We can use this function after initializing a list of `Person` objects.

```py
def run6() -> None:
    persons = [Person("Josephine", 20), Person("Wesley", 31)]
    for person in persons:
        # Check if person is born in a leap year using simplistic leap year logic
        birth_year = approx_year_of_birth(person)
        person_is_born_in_leap_year = True if birth_year % 4 == 0 else False
        print(f"{person}. Born in a leap year?: {person_is_born_in_leap_year}")
```

Running the above function via `main.py` gives us the following output:

```bash
Josephine is 20 years old. Born in leap year?: True
Wesley is 31 years old. Born in leap year?: False
```

## Rust

We can define the below function in Rust, where we print a message depending on whether a person is
born in a leap year or not.

```rs
use chrono::prelude::*;

fn approx_year_of_birth(person: &Person) -> u16 {
    let now = chrono::Utc::now();
    let year = now.year() - (person.age as i32);
    year as u16
}
```

Note that in Rust, we need to use the `chrono` crate to handle datetimes, unlike in Python where
the `datetime` module comes with the standard library.

We then use this function after initializing a vector of `Person` objects.

```rs
fn run6() {
    let persons = vec![Person::new("Josephine", 20), Person::new("Wesley", 31)];
    for person in persons {
        // check if person is born in a leap year using simplistic leap year logic
        let birth_year = approx_year_of_birth(&person);
        let person_is_born_in_leap_year = birth_year % 4 == 0;
        println!(
            "{}. Born in a leap year?: {}",
            person, person_is_born_in_leap_year
        );
    }
```

Running the function via `main.rs` gives us the same output as in Python:

```bash
Josephine is 20 years old. Born in a leap year?: true
Wesley is 31 years old. Born in a leap year?: false
```

## Takeaways

- Single line if-else statements are useful for performing simple operations on a value while
  remaining concise.
- In certain cases in Rust, we have to use external crates to handle certain functionality that
  comes with the standard library in Python.
