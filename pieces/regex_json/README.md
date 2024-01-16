# Regex parsing from JSON

Read in data from a JSON file, and convert the string values of annual revenue into
floats via captured regex patterns.

## Goal

The aim of this project is to convert a dataset of JSON blobs containing string values of companies'
annual revenue in dollars to their float ranges of lower/upper bounds using regex pattern matching.

An example scenario where this might be useful is to convert a dataset of companies and
their annual revenues into a form that can be queried easily in a SQL or NoSQL database downstream (the original data in raw string format cannot be easily queried).

## Sample input

The example input shown below contains the revenue field as a string that's human-readable but not
very machine-readable.

```json
{
  "company": "Initech Corp.",
  "annual_revenue": "$500K-$1M",
  "industry": "Technology"
}
```

## Desired output

Two new fields containing valid floats are added to each record that can be queried easily in a SQL
or NoSQL database downstream.

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
> It's assumed that the annual revenue field from the incoming data is always in the format of
> `$<lower><unit>-$<upper><unit>` where `<lower>` and `<upper>` are numbers and `<unit>` is a
> multiplier that's one of `K`, `M`, or `B`, representing a thousand, million or billion respectively. For simplicity, we do not deal with missing values in this example.

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

### Run script

```bash
python main.py
```

### Output

```python
[{'annual_revenue': '$10M-$20M',
  'annual_revenue_lower': 10000000.0,
  'annual_revenue_upper': 20000000.0,
  'company': 'Acme Inc.',
  'industry': 'Technology'},
 {'annual_revenue': '$7.5M-$8.5M',
  'annual_revenue_lower': 7500000.0,
  'annual_revenue_upper': 8500000.0,
  'company': 'Globex Corp.',
  'industry': 'Logistics'},
 {'annual_revenue': '$500K-$1M',
  'annual_revenue_lower': 500000.0,
  'annual_revenue_upper': 1000000.0,
  'company': 'Initech Corp.',
  'industry': 'Technology'},
 {'annual_revenue': '$800M-$1B',
  'annual_revenue_lower': 800000000.0,
  'annual_revenue_upper': 1000000000.0,
  'company': 'Umbrella Corp.',
  'industry': 'Retail'}]
```

### Run tests

Tests are run via pytest.

```bash
$ pytest -v
===================================================================================================== test session starts ======================================================================================================
platform darwin -- Python 3.11.6, pytest-7.4.3, pluggy-1.3.0 -- /code/rust-projects/python/regex_patterns/.venv/bin/python3.11
cachedir: .pytest_cache
rootdir: /code/rust-projects/python/regex_patterns
collected 2 items

test_main.py::test_revenue_range PASSED                                                                                                                                                                                  [ 50%]
test_main.py::test_revenue_values PASSED                                                                                                                                                                                 [100%]

====================================================================================================== 2 passed in 0.01s =======================================================================================================
```

## Rust Setup

Install dependencies via Cargo. Note that because we perform serialization/deserialization features
in `serde_json` via `serde`, we need to install it using the features flag.

```bash
cargo add regex
cargo add serde --features derive
cargo add serde_json
cargo add anyhow
```

### Run project

The provided `Makefile` runs the formatter, linter, tests and the main file all in sequence.

```bash
make all

# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
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

### Run linter and formatter only

Cargo provides out-of-the-box for formatting (`cargo fmt --all`), compile checks (`cargo check --all-targets`),
and linting (`cargo clippy --all-targets`). The following command runs both. It's highly recommended
to run both prior to pushing Rust code to a repository.

```bash
make format
make check
make lint
# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
```

### Run tests only

The Rust in-built test client allows tests to be defined within the same file as the code being tested. Because Rust is a compiled language, the compiler will know to ignore the tests when building the final binary for runtime.

Tests are run using `make test` or `cargo test --quiet`.

```bash
make test
cargo test --quiet

running 2 tests
test tests::test_revenue_range ... ok
test tests::test_run ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## Takeaways

In this example the [anyhow](https://github.com/dtolnay/anyhow) crate is used. This crate simplifies
error handling with the use of `anyhow::Result`, allowing the `?` operator to be used on all error
types. Additionally the return signature is simplified because the error is implied. This means instead
of returning `Result<T, Box<dyn std::error::Error>>`, you can return `Result<T>`.

In this example, the `bail!` macro is also used. This is a convenience macro that lets you exit the
function early returning an error containing the specified message. This is similar to using
`raise` in Python.
