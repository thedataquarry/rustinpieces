# Rust in Pieces

Journeys between two worlds: Python 🐍 and Rust 🦀.

This repo contains the code for an upcoming book series called "Rust in Pieces". The goal is to help Python and Rust developers learn either language by building small projects in that they wouldn't normally have any trouble doing in the language of their choice. Each solution starts off with a Python implementation for a specific problem and is accompanied by a Rust implementation that performs the same task.

Rust is a language that's known for its steep learning curve (yet, it's almost universally loved by those who become proficient with it). The book series that builds on this repo aims to make the initial steps for Python developers less daunting by providing a familiar environment for Python developers who are used to solving real world problems in their day-to-day work.

In the reverse direction, Python is a language that's falsely known for its performance limitations. Nowadays, Python's rich package ecosystem allows one to write high-performance code, but sometimes, it makes sense to write only _parts_ of a Python project in Rust for performance reasons. A large portion of the AI/ML landscape is in Python. Thus, the other goal of this repo is to help Rust developers become familiar with Python's ecosystem and how to use it to their advantage.

The larger focus of the book is to help developers become proficient enough with software/data engineering use cases in either language to make an informed choice about whether to use one or the other language for parts of a larger project. This will come from a deeper understanding of Rust's performance benefits, and how to unite Python and Rust code bases via [PyO3](https://github.com/PyO3/pyo3).

## Pieces

Each _piece_ is a small project that's implemented in both Python and Rust. The goal is to make it as easy as possible for Python developers to understand the Rust implementation, and vice versa.

The code for the pieces is in the [pieces](./pieces) directory. Each piece is accompanied by a README that explains the problem statement, and how to run the code in both languages.

One of the challenges with learning (and teaching) Rust, is that certain concepts such as ownership, borrowing, traits and lifetimes can be quite challenging to grasp for a new learner, but these concepts are ubiquitous in the language, such that they appear all at once. But, because the learning approach provided here is top-down, the best way to get familiar with these concepts is to try and apply them to your own projects, as done in each piece listed below.

As such, the concepts are introduced in a way that's as gradual as possible, though it's still possible that you may find yourself having to refer to the [Rust book](https://doc.rust-lang.org/book/) or other resources to understand certain concepts from the bottom-up as you go along.

A roadmap for upcoming pieces is shown below. Stay tuned!

| Piece                        | Category      | New Rust concepts                        |
| ---------------------------- | ------------- | ---------------------------------------- |
| Hello world                  | Intro         | macros                                   |
| Data structures & constructs | Intro         | crates, structs, traits, implementations |
| Simple CSV parsing           | File-handling | serde, vec                               |
| Regex JSON                   | File-handling | match, regex                             |
| Mock data generation         | File-handling | RNG, sampling                            |
| Age grouping                 | File-handling | enums                                    |
| Datetime parsing             | File-handling | chrono, lifetimes                        |
| Preprocessing data for NLP   | Parallelism   | rayon, parallelism                       |
| Polars datetimes             | DataFrames    | datetimes                                |
| Polars EDA                   | DataFrames    | TBD                                      |
| Postgres                     | Databases     | async, sqlx, tokio                       |
| DuckDB                       | Databases     | arrow, in-memory DB                      |
| Meilisearch                  | Databases     | async, async-std, clap                   |
| Qdrant                       | Databases     | async, tokio, gRPC                       |
| KùzuDB                       | Databases     | async, graph                             |
| REST API to Postgres         | APIs          | axum, async, tokio                       |
| REST API to local LLM        | APIs          | axum, LLMs                               |
| PyO3 mock data generation    | Unification   | TBD                                      |
| PyO3 query local LLM         | Unification   | TBD                                      |
