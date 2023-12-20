# Regex: Money

Converting string values of annual revenue into floats via regex patterns.

## Goal

The aim of this project is to convert a dataset of JSON blobs containing string values of companies' annual revenue in dollars to their float ranges of lower/upper bounds using regex pattern matching.

## Sample input

The example input shown below contains the revenue field as a string that's human-readable but not very machine-readable.

```json
{
    "company": "Initech Corp.",
    "annual_revenue": "$500K-$1M",
    "industry": "Technology"
}
```

## Desired output

Two new fields containing valid floats are added to each record that can be queried easily in a SQL or NoSQL database downstream.

```json
{
    "company": "Initech Corp.",
    "annual_revenue": "$500K-$1M",
    "annual_revenue_lower": 500000.0,
    "annual_revenue_upper": 1000000.0,
    "industry": "Technology"
}
```

## Assumptions

> [!NOTE]
> It's assumed that the annual revenue field from the incoming data is always in the format of `$<lower><unit>-$<upper><unit>` where `<lower>` and `<upper>` are numbers and `<unit>` is a multiplier that's one of `K`, `M`, or `B`, representing a thousand, million or billion respectively.

## Setup

Install dependencies via Cargo. Note that because we perform serialization/deserialization features in `serde_json` via `serde`, we need to install it using the features flag.

```bash
cargo add regex
cargo add serde --features derive
cargo add serde_json
```

## Run project

The provided `Makefile` runs the formatter, linter, tests and the main file all in sequence.

```bash
make all

# Runs the following
cargo fmt --quiet
cargo clippy --quiet
cargo test --quiet
cargo run --quiet
```

To run just the main file, use the following command.

```bash
make run
# or, simply run via cargo
cargo run --quiet
```

### Output

```sh
[
  {
    "company": "Acme Inc.",
    "industry": "Technology",
    "annual_revenue": "$10M-$20M",
    "annual_revenue_lower": 10000000.0,
    "annual_revenue_upper": 20000000.0
  },
  {
    "company": "Globex Corp.",
    "industry": "Logistics",
    "annual_revenue": "$7.5M-$8.5M",
    "annual_revenue_lower": 7500000.0,
    "annual_revenue_upper": 8500000.0
  },
  {
    "company": "Initech Corp.",
    "industry": "Technology",
    "annual_revenue": "$500K-$1M",
    "annual_revenue_lower": 500000.0,
    "annual_revenue_upper": 1000000.0
  },
  {
    "company": "Umbrella Corp.",
    "industry": "Retail",
    "annual_revenue": "$800M-$1B",
    "annual_revenue_lower": 800000000.0,
    "annual_revenue_upper": 1000000000.0
  }
]
```

## Run linter and formatter only

Cargo provides out-of-the-box for formatting (`cargo fmt`) and linting (`cargo clippy`). The following command runs both. It's highly recommended to run both prior to pushing Rust code to a repository.

```bash
make format
make lint
# Runs the following
cargo fmt --quiet
cargo clippy --quiet
```

## Run tests only

Using Rust's inbuilt client, tests can either be within `main.rs` or in a separate file `test_main.rs` made accessible to `main.rs` via `mod test_main`.

Tests are run using `make test` or `cargo test --quiet`.

```bash
make test
cargo test --quiet

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
