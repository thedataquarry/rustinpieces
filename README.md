# Rust in Pieces

Journeys between two worlds: Python 🐍 and Rust 🦀.

This repo contains the code for an upcoming book series called "Rust in Pieces". The goal is to help Python and Rust developers learn either language by building small projects in that they wouldn't normally have any trouble doing in the language of their choice. Each solution starts off with a Python implementation for a specific problem and is accompanied by a Rust implementation that performs the same task.

Rust is a language that's known for its steep learning curve (yet, it's almost universally loved by those who become proficient with it). The book series that builds on this repo aims to make the initial steps for Python developers less daunting by providing a familiar environment for Python developers who are used to solving real world problems in their day-to-day work. 

In the reverse direction, Python is a language that's falsely known for its performance limitations. Python's rich package ecosystem allows one to write high-performance code, but sometimes, it makes sense to write parts of a project in Rust for performance reasons. In addition, a large portion of the AI/ML landscape is in Python. Thus, the other goal of this repo is to help Rust developers become familiar with Python's ecosystem and how to use it to their advantage.

The larger goal of the book is to help developers become proficient enough with software/data engineering use cases in either language to make an informed choice about whether to use one or the other language for parts of a larger project. This will come from a deeper understanding of Rust's performance benefits, and how to unite Python and Rust code bases via [PyO3](https://github.com/PyO3/pyo3).

## Pieces

Each *piece* is a small project that's implemented in both Python and Rust. The goal is to make it as easy as possible for Python developers to understand the Rust implementation, and vice versa.

### Intro
- [x] Hello World
- [x] Data structures and constructs

### File handling
- [x] Simple CSV
- [x] Regex JSON
- [x] Mock data generation CSV
- [x] Age groups CSV
- [x] Datetime parsing CSV

### Databases
- [x] Postgres ETL
- [ ] Meilisearch
- [ ] Qdrant

### APIs
- [ ] REST API on top of Postgres
- [ ] More to come...

### Data analysis
- [ ] Polars datetime handling
- [ ] Polars analysis

### Unification
- [ ] Blending 🐍 + 🦀 worlds via PyO3