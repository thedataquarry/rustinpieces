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

Install dependencies via the `uv` package manager. All dependencies are listed in `pyproject.toml`.

### Run script

First, sync the dependencies from `pyproject.toml`.

```bash
uv sync
```

The script can be then run using the following command.

```bash
uv run main.py
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
$ uv run pytest -v
=================================================================================================== test session starts ====================================================================================================
platform darwin -- Python 3.12.5, pytest-8.3.4, pluggy-1.5.0 -- /Users/prrao/code/rustinpieces/src/regex_json/python/.venv/bin/python3
cachedir: .pytest_cache
rootdir: /Users/prrao/code/rustinpieces/src/regex_json/python
configfile: pyproject.toml
collected 2 items

test_main.py::test_revenue_range PASSED                                                                                                                                                                              [ 50%]
test_main.py::test_revenue_values PASSED                                                                                                                                                                             [100%]

==================================================================================================== 2 passed in 0.00s =====================================================================================================
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
types. When using the standard library `Result`, the error type that will be returned is specified
in the return signature. For example `Result<T, std::io::Error>`. In this case `?` can only be used
if the resulting error would be of type `std::io::Error`. Using `anyhow::Result` also simplifies the
return signature because the error type does not need to be provided, it is instead understood to be
`anyhow::Error`. This means instead of returning `Result<T, Box<dyn std::error::Error>>`, you can
return `Result<T>`.

In this example, the `bail!` macro is also used. This is a convenience macro that lets you exit the
function early returning the error in the `Result` containing the specified message. This is similar
to using `raise` in Python. The big difference between `bail!` and something like `panic!` is `panic!`
exits the program with an error, while `bail!` returns the error to the caller. To put this difference
in context, if the `get_data` function was called as part of a web app and we used `panic!` the app
would crash at this point and the server would have to be restarted to bring it back up. By using
`bail!` a message could instead be sent back to the user that an error occurred and the server can
keep running without issue.
