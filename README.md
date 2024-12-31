# Rust in Pieces

Journeys between two worlds: Python 🐍 and Rust 🦀.

This repo contains the code for the book _[Rust in Pieces](https://rustinpieces.dev)_. The goal is to help developers move between the worlds of Python and Rust by building small projects that they wouldn't normally have any trouble doing in the language of their choice. Each solution starts off with a Python implementation for a specific problem and is accompanied by a Rust implementation that performs the same task.

Rust is a language that's known for its steep learning curve -- yet, it's almost universally loved by those who become proficient with it. The book series that builds on this repo aims to make the initial steps for Python developers less daunting by providing a familiar environment for Python developers who are used to solving real world problems in their day-to-day work.

In the reverse direction, Python is a language that's falsely known for its performance limitations. Python's rich package ecosystem allows one to write high-performance code by only writing _parts_ of a project in Rust for performance reasons. These days, a large portion of the AI/ML landscape is in Python, and it's become one of the most popular programming languages on GitHub. Thus, the other goal of this repo is to help Rust developers become familiar with Python's ecosystem and how to use it to their advantage.

The larger focus of the book and this project is to help developers become proficient enough with software/data engineering use cases in either language to make an informed choice about whether to use one or the other language for parts of a larger project. This will come from a deeper understanding of Rust's performance benefits, and how to unite Python and Rust code bases via [PyO3](https://github.com/PyO3/pyo3).

## Pieces

Each _piece_ is a small project that's implemented in both Python and Rust. The goal is to make it as easy as possible for Python developers to understand the Rust implementation, and vice versa.

The code for the pieces is in the [src](./src) directory. Each piece is accompanied by a README that explains the problem statement, and how to run the code in both languages.

One of the challenges with learning (and teaching) Rust, is that concepts such as ownership, borrowing, traits and lifetimes that can be quite challenging to grasp for a new learner, are ubiquitous in the language, so it's not straightforward to introduce them gradually. The learning approach applied here is top-down, and so the best way to get familiar with these concepts is to try and apply them to your own problems, as we do in each piece.

Although the implementation concepts are introduced as gradually as possible, it's still possible that you may find yourself referring to the [Rust book](https://doc.rust-lang.org/book/) or other resources to understand certain concepts from the bottom-up as you go along.

The following pieces are covered:

| Piece                        | Category    | Key Rust concepts                        |
| ---------------------------- | ----------- | ---------------------------------------- |
| Hello world                  | Intro       | macros                                   |
| Data structures & constructs | Intro       | crates, structs, traits, implementations |
| Simple CSV parsing           | Files       | serde, vec                               |
| Regex JSON                   | Files       | match, regex                             |
| Mock data generation         | Files       | RNG, sampling                            |
| Age grouping                 | Files       | enums                                    |
| Datetime parsing             | Files       | chrono, lifetimes                        |
| Extract pronouns from text   | Files       | rayon, parallelism                       |
| Postgres                     | Databases   | async, sqlx, tokio                       |
| Meilisearch                  | CLIs        | async, async-std, clap                   |
| REST API to Postgres         | APIs        | axum, async, tokio                       |
| PyO3 mock data generation    | Unification | PyO3, Maturin                            |
| PyO3 parallel computation    | Unification | PyO3, Maturin                            |

## Contributing

Contributions and improvements from the community are welcome! Please see the [contributing guidelines](./CONTRIBUTING.md).
