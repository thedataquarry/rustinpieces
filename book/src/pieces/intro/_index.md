# Introduction

This piece is meant to be a quick introduction to simple constructs that are more or less similar between Python and Rust.

The following constructs are covered:

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

The code is available in the `src/intro` directory of the [repo](https://github.com/thedataquarry/rustinpieces/tree/main/src/intro).

Rust's traits don't have a direct equivalent in Python, but they are similar enough to protocols or special methods in that they
allow us to define a set of methods that a type must implement, allowing us to customize the behavior of the type.

Rust embraces functional programming more than Python does, so it has a number of functional constructs that are commonly used.
Where Python prefers list comprehensions, Rust prefers map/filter. Rust's closures, are, at the surface level, similar
enough to Python's lambda functions, but they are also a lot more complex and can be viewed as a superset of anonymous functions.

Hopefully, as you read through the examples, you'll see that Rust and Python are not as different as they may seem at first glance!
