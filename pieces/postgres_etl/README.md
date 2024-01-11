# PostgreSQL ETL

Insert data into a Postgres table and measure the performance of the async client.

## Goal

In this piece, we will use `asyncpg` in Python and the `sqlx` crate in Rust to load a tabular
dataset containing a million people, their age, marital status and the city, state and country they
last visited into a Postgres table. We will then measure the throughput of up to 1000 async queries
to the table.

## Inputs

The input is a CSV file `./data/persons.csv` generated from the [mock_data](../mock_data) project.
We use an input argument of `1000000` to generate a dataset of a million persons and their
information.

## Output

The outputs are the data loading time for 1M records and the query throughput (QPS) of 10, 100 and
1000 async queries via `asyncpg` in Python and `sqlx` in Rust.

## Python Setup

Install the dependencies in a virtual environment via `requirements.txt`.

```bash
# First time setup
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# For subsequent runs, simply activate the environment
source venv/bin/activate
```

### Run scripts

#### Load data into Postgres

The loader script is run just once as follows.

```bash
python load_data.py
```

#### Run queries

The query script is run as follows.

```bash
# Run for 10, 100 and 1000 queries
python main.py -n 10
python main.py -n 100
python main.py -n 1000
```

### Run tests

Tests can be run as follows:

```bash
$ pytest -v
=========================================== test session starts ============================================
platform darwin -- Python 3.11.7, pytest-7.4.4, pluggy-1.3.0 -- /code/rustinpieces/python/postgres_etl/.venv/bin/python3.11
cachedir: .pytest_cache
rootdir: /code/rustinpieces/python/postgres_etl
configfile: pyproject.toml
plugins: asyncio-0.23.3
asyncio: mode=Mode.AUTO
collected 2 items

test_main.py::test_summary_query PASSED                                                              [ 50%]
test_main.py::test_perf_query PASSED                                                                 [100%]

============================================ 2 passed in 0.26s =============================================
```

### Results

The results for the data loading and for the query throughput are shown below.

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 32GB of RAM.
> The Python version used was `3.11.7`.

#### Data loading

The data loading time is measured by running the `load_data.py` script.

| numPersons | Python  | Rust    |
| ---------- | ------- | ------- |
| 1000000    | 222 sec | 187 sec |

The run time for Python to load 1M records is around 3 min 42 seconds. Note that a sync for loop
was used to insert the records, so the insertion isn't truly non-blocking, and can be further
improved.

#### Query throughput

By specifying an argument to `main.py`, we can control the number of async queries that we're
running. The queries are aggregation queries that perform counts of persons whose age is greater
than a random number between 22 and 65.

| numPersons | Python                | Rust                  |
| ---------- | --------------------- | --------------------- |
| 10         | 0.510 sec (19.6 QPS)  | 0.677 sec (14.8 QPS)  |
| 100        | 3.786 sec (26.4 QPS)  | 3.977 sec (25.1 QPS)  |
| 1000       | 37.616 sec (26.5 QPS) | 37.895 sec (26.3 QPS) |

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
`src/bin/load_data.rs`. The loader is then run as follows:

```bash
cargo run --bin load_data
```

#### Run queries

The query script is run via `main.rs` and can be run multiple times. The query script is run as
follows:

```bash
# Run for 10, 100 and 1000 queries
cargo run -- 10
cargo run -- 100
cargo run -- 1000
```

### Run tests

The Rust in-built test client allows tests to be defined within the same file as the code being tested. Because Rust is a compiled language, the compiler will know to ignore the tests when building the final binary for runtime.

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
for just the unoptimized (dev) run, as this script is run just one time only to load the data.

| numPersons | Python  | Rust    |
| ---------- | ------- | ------- |
| 1000000    | 222 sec | 187 sec |

The Rust code takes about 16% less time than the Python code to insert 1M records into Postgres.
Note that a sync for loop was used to insert the records, so the insertion isn't truly non-blocking,
and can be further improved.

#### Query throughput

By specifying an argument to `main.rs`, we can control the number of async queries that we're
running. The queries are aggregation queries that perform counts of persons whose age is greater
than a random number between 22 and 65.

| numPersons | Python                | Rust                  |
| ---------- | --------------------- | --------------------- |
| 10         | 0.510 sec (19.6 QPS)  | 0.677 sec (14.8 QPS)  |
| 100        | 3.786 sec (26.4 QPS)  | 3.977 sec (25.1 QPS)  |
| 1000       | 37.616 sec (26.5 QPS) | 37.895 sec (26.3 QPS) |

## Takeaways

There isn't that much difference between the Rust and Python code when running many asynchronous
queries, because the bottleneck is the network overhead due to the client/server connection in
Postgres, which Python's `asyncpg` library also handles well (because it's implemented in C and
Cython under the hood). The Rust code is also not idiomatic, so there's a lot of room for
improvement overall.
