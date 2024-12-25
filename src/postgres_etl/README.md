# PostgreSQL ETL

Insert data into a Postgres table and measure the performance of the async client.

## Goal

In this piece, we will use `asyncpg` in Python and the `sqlx` + `tokio` crates in Rust to
asynchronously load a tabular dataset containing 100K people, their age, marital status and the
city, state and country they last visited into a Postgres table. We will then measure the throughput
of up to 100K async queries to the table.

## Inputs

The input is a CSV file `./data/persons.csv` generated from the [mock_data](../mock_data) project.
We use an input argument of `100000` to generate a dataset of 100K persons and their
metadata.

## Output

The outputs are the ingestion time for 100K records and the query throughput (QPS) of
100K async queries via `asyncpg` in Python and `sqlx` + `tokio` in Rust.

## Python Setup

Install dependencies via the `uv` package manager. All dependencies are listed in `pyproject.toml`.

If you want to manually add the latest versions of the dependencies yourself, run the following commands.

```bash
uv add asyncpg
uv add python-dotenv
uv add --dev pytest
uv add --dev pytest-asyncio
```

### Run script

First, sync the dependencies from `pyproject.toml`.

```bash
uv sync
```

### Run scripts

#### Load data into Postgres

The loader script is run just once as follows.

```bash
uv run load_data.py
```

This will ingest the 100K records via gathered async tasks.

#### Run queries

The query script is run as follows.

```bash
uv run main.py -n 100000
```

### Run tests

Tests can be run as follows:

```bash
$ uv run pytest -v
=========================================== test session starts ============================================
platform darwin -- Python 3.12.5, pytest-7.4.4, pluggy-1.3.0 -- /Users/prrao/code/rustinpieces/src/postgres_etl/python/.venv/bin/python3.12
cachedir: .pytest_cache
rootdir: /Users/prrao/code/rustinpieces/src/postgres_etl/python
configfile: pyproject.toml
plugins: asyncio-0.25.0
asyncio: mode=Mode.AUTO
collected 2 items

test_main.py::test_summary_query PASSED                                                              [ 50%]
test_main.py::test_perf_query PASSED                                                                 [100%]

============================================ 2 passed in 0.26s =============================================
```

### Results

The results for the data loading and for the query throughput are shown below.

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 36GB of RAM.
> The Python version used was `3.12.5`.

#### Data loading

The data loading time is measured by running the `load_data.py` script.

| numPersons | Python   | Rust    |
| ---------- | -------- | ------- |
| 100000     | 12.6 sec | 7.1 sec |

The Rust version takes ~44% less time than the Python version to insert 100K records into Postgres.
20 min/max threads were used in the `sqlx` connection pool. We will see more on this below.

#### Query throughput

By specifying an argument to `main.py`, we can control the number of async queries that we're
running. The queries are aggregation queries that perform counts of persons whose age is greater
than a random number between 22 and 65. An example query is shown below.

```sql
SELECT COUNT(*) FROM persons WHERE age > 22;
```

| numQueries | Python                  | Rust                    |
| ---------- | ----------------------- | ----------------------- |
| 100000     | 1 min 27 sec (1149 QPS) | 0 min 32 sec (3125 QPS) |

It can be seen that for 100K such aggregation queries with random age bounds, the Rust code is about
2.7x faster than the Python code. We will see more on this below.

## Rust Setup

Install dependencies via Cargo. In this piece, we use the `sqlx`, `postgres` and `tokio` crates to
perform async queries to the Postgres database. In addition, `serde`, `csv` and `dotenvy` crates
are used for managing the CSV file and environment variables.

```bash
cargo add csv
cargo add dotenvy
cargo add sqlx
cargo add postgres
cargo add rand
cargo add serde --features derive
cargo add tokio --features full
```

Note that we need to install all tokio features in order to use the `tokio::spawn` method
that allows us to run async tasks truly concurrently.

### Run scripts

The provided `Makefile` runs the formatter, linter, tests for `main.rs` file all in sequence.

```bash
make all

# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
cargo test --quiet
cargo run --quiet
```

#### Load data into Postgres

The loader script is run just once, via the `src/bin` directory that's external to `main.rs`. In
order to do this, the Rust code that loads data to the Postgres database is situated in
`src/bin/load_data.rs`. The loader is then run in release-mode as follows:

```bash
cargo run --release --bin load_data
# or
cargo run -r --bin load_data
```

#### Run queries

The query script is run via `main.rs` and can be run multiple times. The query script is run in
release-mode as follows:

```bash
cargo run -r -- 100000
```

### Run tests

The Rust in-built test client allows tests to be defined within the same file as the code being
tested. Because Rust is a compiled language, the compiler will know to ignore the tests when
building the final binary for runtime.

Tests are run using `make test` or `cargo test --quiet`.

```bash
make test
cargo test --quiet
```

```bash
running 2 tests
test test_main::test_summary_query ... ok
test test_main::test_perf_query ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
```

### Results

The results for the data loading and for the query throughput are shown below.

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 32GB of RAM.
> The Rust version used was `1.74.1`.

#### Data loading

The data loading time is measured by running the `load_data.rs` script. The results are shown below
for just the release-mode run, as this script is run just one time only to load the data.

| numPersons | Python   | Rust    |
| ---------- | -------- | ------- |
| 100000     | 12.6 sec | 7.1 sec |

The Rust code takes about 16% less time than the Python code to insert 1M records into Postgres.
Note that a sync for loop was used to insert the records, so the insertion isn't truly non-blocking,
and can be further improved.

#### Query throughput

By specifying an argument to `main.rs`, we can control the number of async queries that we're
running. The queries are very simple aggregation queries that perform counts of persons whose age
is greater than a random number between 22 and 65. An example query is shown below.

```sql
SELECT COUNT(*) FROM persons WHERE age > 22;
```

| numQueries | Python                  | Rust                    |
| ---------- | ----------------------- | ----------------------- |
| 100000     | 1 min 27 sec (1149 QPS) | 0 min 32 sec (3125 QPS) |

It can be seen that for 100K such aggregation queries with random age bounds, the Rust code is about
2.7x faster than the Python code. Note that the QPS and the run time depend on the CPU, OS, and the
number of min/max threads that are allowed to run concurrently as specified to the `sqlx`
connection pool in Rust, so depending on all these factors, your mileage may vary. However, it's
clear that Rust will be the faster option when it comes to async queries.

## Takeaways

Python's `asyncpg` library has its internals written in Cython, and as can be seen from the results,
it performs admirably well and is really performant. However, Rust's `sqlx` + `tokio`
combination is even more performant, largely because of the fact that it's a lot more low-level
and deals purely in optimized Rust objects once they're compiled.

Although Rust is way faster than Python here, it's important to note how much more
complex the Rust code is compared to the Python version. For example, the Postgres connection pool
created in `sqlx` is initialized with an `Arc` object, which stands for "atomic reference count" in
Rust. This is a way to share ownership of the pool object across multiple threads, allowing for very
cheap cloning of the underlying object. The way Rust shares ownership of objects across threads is
rather complex for a beginner, and it's important to understand the ownership model in Rust, and the
impact of cloning on performance, before attempting to write async code in Rust.

> [!NOTE]
> This example showed how to concurrently ingest data, record-by-record, into a Postgres table.
> For much larger datasets in a realistic production environment, it would be more efficient to
> batch-insert the data. This would require a slightly different approach, as we'd need
> to handle the batching logic and potentially pass the entire vector of records to Postgres as
> a temporary table. This would be a great exercise to try out in the future for larger datasets!

When is it beneficial to use Python for such ETL tasks? Looking at how concise and readable the
`asyncpg` Python code is compared to the Rust version, it's clear that Python is a great choice for
rapid prototyping and iteration, for example, in early stages of a project when the data types and
schemas are still evolving. In such cases, Python's dynamic typing and the ability to quickly
string together async data ingestion code that performs well is a huge advantage.

Rust's `sqlx` + `tokio` combination is a great choice for production-grade ETL tasks that
require high performance and throughput. Not only is `sqlx` async-only, but it also allows us to
perform compile-time SQL query validation (via the `query!` macro), allowing the developer to
catch SQL errors at compile-time rather than at runtime. In a production environment, this is a
huge advantage, as it allows developers to have the confidence that their production code will
not fail at runtime due to simple syntax errors in their SQL.

To summarize, in data engineering and ETL tasks in Postgres,, Python (via `asyncpg`) is a great
choice as it's really fast, and the code is concise and easy to implement. Rust (via `sqlx` +
`tokio`) is a great choice for production-grade ETL tasks that require high performance and
reliability enabled by its compile-time checks and SQL query validation, though this comes at the
cost of added code complexity.
