# Protocols vs. Traits

Python has a concept called protocols, sometimes referred to as special methods, or "dunder methods" implemented on
[built-in types](https://docs.python.org/3/library/stdtypes.html) in the standard library.
For example, the `__str__` method is used to implement the `str()` function, which returns the string representation of an object.
Th `__repr__` method is used to implement the `repr()` function, which returns a string containing a _printable representation_ of an object.

## Python: Protocols

In Python, we start by defining a simple `Person` class that has a name and an age attribute.
To make the output of the `print` statement more interesting, we implement the following `__str__` and `__repr__` methods
that are translated to the `str()` and `repr()` functions respectively.

```python
class Person:
    def __init__(self, name: str, age: int) -> None:
        self.name = name
        if age > 0 and isinstance(age, int):
            self.age = age
        else:
            raise ValueError("Age must be a positive integer")

    def __str__(self) -> str:
        return f"{self.name} is {self.age} years old"

    def __repr__(self) -> str:
        return f"Person: {self.name}, {self.age}"
```

One limitation of Python's type system that's worth noting is that it treats _all_ integers as `int` types, even if they
are unsigned. In this case, the age of a person should be a positive integer, so we need to check for this by using
an `if` statement in the class constructor defined in the `__init__` block. Rust's type system, as we'll see, is
more powerful, while also being stricter than Python's.

We can now create a `Person` object via a function and print it to the console by running the code via `main.py`.

```python
def run1() -> None:
    person = Person("Megan", 28)
    print(person)
    print(repr(person))
    """
    Megan is 28 years old
    Person: Megan, 28
    """
```

When we print the `person` object, the `__str__` method is called, and when we print the `repr` object,
the `__repr__` method is called, thus producing slightly different outputs depending on what we want to display.
Generally, `repr()` is used for debugging a stack trace, and `str()` is used for displaying something to the user.

## Rust: Traits

In Rust, we start by defining a `Person` struct with a name and an age attribute, in a similar way to the Python example.

```rs
struct Person {
    name: String,
    age: u8,
}
```

Unlike a Python class which always provides `__init__`, Rust doesn't provide constructors on structs, so we
need to define an implementation block (shown below) for the `Person` struct via the `impl` keyword.

As noted earlier, Rust allows us to declare the `age` variable as an unsigned integer, which is more
appropriate for this use case, eliminating the need to check for positive integers in the constructor.
This makes the code more concise and easier to read in this case.

```rs
impl Person {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}
```

Two things stand out in the `impl` block defined. We provide an argument `&str`, which represents a string _slice_,
and we use the `to_string()` method to convert the string slice to a `String` type.

Because Rust is a statically typed language, it needs to know the type and allocation of all variables at compile time.
When we input a person's name during initialization, we don't know how long the name will be. However, arguments to
functions and methods in Rust are passed by reference, so we'd typically use a string slice to represent the name. The
compiler keeps a track of all this, so if you forget to call the `to_string()` method, you'll get a nice compiler error!

Rust has its own versions of Python's `__str__` and `__repr__` methods, but they're called `Display` and `Debug` traits.
A trait is similar to an interface in other languages, and vaguely similar to a protocol in Python, because it describes
an object's _behavior_.

```rs
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person: {}, {}", self.name, self.age)
    }
}
```

`Display` and `Debug` traits are typically not automatically defined on every object type in Rust, because its
strict type system needs to know upfront what the user wants to do with the object's types prior to displaying them.

With these bits in place, we can now create a `Person` object via a function and print it to the console by running the code via `main.rs`.

```rs
fn run1() {
    let p = Person::new("Megan", 28);
    println!("{}", p);
    println!("{:?}", p);
    /*
    Megan is 28 years old
    Person: Megan, 28
    */
}
```

Note that in Rust, for printing `Debug` traits, we use the `{:?}` format specifier, whereas for `Display` traits, we can just use `{}`.

The above output is identical to the Python output!

## Takeaways

- Rust's type system is stricter and more powerful than Python's, allowing us to define unsigned integers and other types
  that are not available in Python's standard library.
- Python is object-oriented, so it uses classes in many cases to keep related data and methods together
- Rust isn't an object-oriented language and doesn't use classes, but it does have the concept of traits and implementations
- In Rust, constructors aren't defined on custom structs, so we need to define the struct's constructor via an implementation
  block using the `impl` keyword.
