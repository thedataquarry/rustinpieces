# Rust in Pieces

A top-down approach to learning Rust coming from Python (and vice-versa).

## Introduction

This book's goal is to bring the Python and Rust developer communities closer together, and to help more developers from one language learn how to leverage the benefits of the other.

The book and accompanying [code](https://github.com/thedataquarry/rustinpieces) are organized into a collection of small projects, termed _pieces_. Each piece is a self-contained task with Python and Rust implementations that each perform the same task. The aim is to help Python developers gain familiarity with Rust, and vice-versa, by comparing and contrasting the two languages in a top-down manner.

Importantly, the pieces in this book build towards **unifying Python and Rust code bases** via [PyO3](https://github.com/PyO3/pyo3), a highly popular open source library that allows you to call Rust bindings from Python or the Python interpreter from Rust. _Using one language does not preclude using the other!_

As you go through the pieces, you'll find yourself becoming proficient in writing clean, tested, production-worthy code using engineering best practices in either language, moving between them at will. Over time, you can make a more informed choice regarding when to use one or the other language for parts of a larger project.

We believe that Rust 🦀 is the among the most approachable lower-level programming languages for Python developers, and that Python is one of the most valuable high-level languages for Rust developers who are looking to build tooling for the burgeoning data, AI and ML ecosystems. The arrival of tools like PyO3 has made it highly feasible for a developer to straddle both worlds, combining their best parts, thus helping build more efficient and scalable software.

## What's covered in this book?

Rust's learning curve is considerably steeper than Python's, so the table below is provided to show a mapping between each piece and its corresponding concept in Rust. As can be seen, structs, serialization, deserialization, vectors and traits are ubiquitous concepts in Rust.

| Piece                        | Category      | New Rust concepts                        |
| ---------------------------- | ------------- | ---------------------------------------- |
| Hello world                  | Intro         | macros                                   |
| Data structures & constructs | Intro         | crates, structs, traits, implementations |
| Simple CSV parsing           | File-handling | serde, vec                               |
| Regex JSON                   | File-handling | match, regex                             |
| Mock data generation         | File-handling | RNG, sampling                            |
| Age grouping                 | File-handling | enums                                    |
| Datetime parsing             | File-handling | chrono, lifetimes                        |
| Extract pronouns from text   | Parallelism   | rayon, parallelism                       |
| Polars datetimes             | DataFrames    | datetimes                                |
| Polars EDA                   | DataFrames    | TBD                                      |
| Postgres                     | Databases     | async, sqlx, tokio                       |
| Meilisearch                  | Databases     | async, async-std, clap                   |
| REST API to Postgres         | APIs          | axum, async, tokio                       |
| REST API to local LLM        | APIs          | axum, LLMs                               |
| PyO3 mock data generation    | Unification   | TBD                                      |
| PyO3 query local LLM         | Unification   | TBD                                      |
