# PostgreSQL ETL

Insert data into a Postgres table and measure the performance of async querying.

## Goal

In this project, we will insert a tabular dataset containing a million people, their age, marital status and the city, state and country they last visited into a Postgres table. We will then measure the performance of 1000 async queries to the table.

## Inputs

The input is a CSV file `./data/persons.csv` generated from the [mock_data](../mock_data) project. We use an input argument of `1000000` to generate a dataset of a million persons and their information.

## Output

The output is the runtime performance (in seconds) of 1000 async queries to the Postgres table.

## Setup

Install dependencies via Cargo. In this piece, we use the `sqlx`, `postgres` and `tokio` crates to perform async queries to the Postgres database. In addition, `serde`, `csv` and `dotenvy` crates are used for managing the CSV file and environment variables.

```bash
cargo add csv
cargo add dotenvy
cargo add sqlx
cargo add postgres
cargo add serde --features derive
cargo add tokio --features full
```

## Run script

The provided `Makefile` runs the formatter, linter, tests and the main file all in sequence.

```bash
make all

# Runs the following
cargo fmt --quiet
cargo clippy --quiet
cargo test --quiet
cargo run --quiet
```

### Load data into Postgres

The loader script is run just once, via the `src/bin` directory that's external to `main.rs`. In order to do this, the Rust code that loads data to the Postgres database is situated in `src/bin/load_data.rs`. The loader is then run as follows:

```bash
cargo run --bin load_data
```

## Run tests

Using Rust's inbuilt client, tests can either be within `main.rs` or in a separate file `test_main.rs` made accessible to `main.rs` via `mod test_main`.

Tests are run using `make test` or `cargo test --quiet`.

```bash
make test
cargo test --quiet
```

TODO: Add test results here

## Performance

By specifying an argument to `main.rs`, we can control the number of async queries that we're running. The queries are aggregation queries that perform counts of persons whose age is greater than a random number between 22 and 65.

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 32GB of RAM.
> The Rust version used was `1.74.1`.

numPersons | Python | Rust (unoptimized) | Rust (release)
--- | --- | --- | ---
10 | 0.123 sec | 0.295 | 0.220
100 | 0.150 sec | 0.315 | 0.237
1000 | 0.299 sec | 0.469 | 0.370
10000 | 1.761 sec | 1.760 | 1.660
100000 | 15.999 sec | 15.266 | 14.319

There isn't that much difference between the Rust and Python code when running many asynchronous queries, because the bottleneck is the network overhead due to the client/server connection in Postgres, which Python's `asyncpg` library also handles well (because it's implemented in C and Cython under the hood). The Rust code is faster when compiled in release mode, but not by much.
