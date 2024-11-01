# Modify a CSV file with simple types

Read in data from a CSV file with simple types (strings, floats) and write it back to a new CSV file with modified values.

## Goal

In this project, we will add a new column to a CSV file that contains data on cities, the country they
belong to, and their average temperatures for the month of January in Fahrenheit. The goal is to write a new CSV file containing
the same data but an added column containing the average temperatures converted to Celsius. The raw
data is obtained from [this Wikipedia page](https://en.wikipedia.org/wiki/List_of_cities_by_average_temperature).

The converted temperature in Celsius is calculated up to 1 decimal precision, because that's
the same precision as the input data. We will also test the results of the conversion
to be within 0.05 of the expected value. This is because numerical operations with floats
are always subject to [floating point errors](https://docs.python.org/3/tutorial/floatingpoint.html).

## Inputs

The input CSV file is `./data/city_temps.csv` with the following data.

```csv
city,country,avg_temp_jan_f
Sydney,Australia,72.1
Wellington,New Zealand,62.4
Honolulu,United States,73.1
Santiago,Chile,70.2
Yellowknife,Canada,-14.1
Toronto,Canada,25.3
Salt Lake City,United States,29.6
San Francisco,United States,52.1
Nuuk,Greenland,18.7
Yakutsk,Russia,-37.5
```

We have a combination of positive and negative temperatures for these places in Fahrenheit.
In the northern hemisphere in January, certain cities go _brrr_ 🥶.

## Output

The output is also a CSV file `./data/city_temps-modified.csv` with an additional column `avg_temp_jan_c`
that has the converted temperature value in Celsius.

```csv
city,country,avg_temp_jan_f,avg_temp_jan_c
Sydney,Australia,72.1,22.3
Wellington,New Zealand,62.4,16.9
Honolulu,United States,73.1,22.8
Santiago,Chile,70.2,21.2
Yellowknife,Canada,-14.1,-25.6
Toronto,Canada,25.3,-3.7
Salt Lake City,United States,29.6,-1.3
San Francisco,United States,52.1,11.2
Nuuk,Greenland,18.7,-7.4
Yakutsk,Russia,-37.5,-38.6
```

## Python Setup

Install dependencies via the `uv` package manager. All dependencies are listed in `pyproject.toml`.
For this project, we only need the standard library of Python, so there are no dependencies to install.

### Run script

First, sync the dependencies from `pyproject.toml`.

```bash
uv sync
```

The script can be then run using the following command.

```bash
uv run main.py
```

### Run tests

```bash
$ uv run pytest -v
===================================================================================================== test session starts =====================================================================================================
platform darwin -- Python 3.12.5, pytest-8.3.2, pluggy-1.5.0 -- /Users/prrao/.pyenv/versions/3.12.5/bin/python3.12
cachedir: .pytest_cache
rootdir: /Users/prrao/code/rustinpieces/src/simple_csv/python
configfile: pyproject.toml
plugins: anyio-4.4.0
collected 10 items

test_main.py::test_read_and_modify PASSED                                                                                                                                                                               [ 10%]
test_main.py::test_modify_fields PASSED                                                                                                                                                                                 [ 20%]
test_main.py::test_convert_f_to_c[70.2-21.2] PASSED                                                                                                                                                                     [ 30%]
test_main.py::test_convert_f_to_c[-14.1--25.6] PASSED                                                                                                                                                                   [ 40%]
test_main.py::test_convert_f_to_c[25.3--3.7] PASSED                                                                                                                                                                     [ 50%]
test_main.py::test_convert_f_to_c[29.6--1.3] PASSED                                                                                                                                                                     [ 60%]
test_main.py::test_convert_f_to_c[52.1-11.2] PASSED                                                                                                                                                                     [ 70%]
test_main.py::test_convert_f_to_c[18.7--7.4] PASSED                                                                                                                                                                     [ 80%]
test_main.py::test_convert_f_to_c[-37.5--38.6] PASSED                                                                                                                                                                   [ 90%]
test_main.py::test_write_csv PASSED                                                                                                                                                                                     [100%]

===================================================================================================== 10 passed in 0.01s ======================================================================================================
```

## Rust Setup

Install dependencies via Cargo. Note that because we perform CSV serialization/deserialization via
`serde`, we need to install it using the features flag. In addition, we need to install the `approx`
crate to perform approximate floating point comparisons during testing (all floating point comparisons
are subject to floating point errors, so approximate comparisons are required).

```bash
cargo add csv
cargo add serde --features derive
# Only add the approx crate for testing with the --dev flag
cargo add --dev approx
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

### Run linter and formatter only

Cargo provides out-of-the-box for formatting (`cargo fmt --all`), compile checks (`cargo check --all-targets`),
and linting (`cargo clippy --all-targets`). The following command runs both. It's highly recommended
to run both prior to pushing Rust code to a
repository.

```bash
make format
make check
make lint
# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
```

## Run tests only

The Rust in-built test client allows tests to be defined within the same file as the code being tested. Because Rust is a compiled language, the compiler will know to ignore the tests when building the final binary for runtime.

Tests are run using `make test` or `cargo test --quiet`.

```bash
make test
cargo test --quiet


running 3 tests
test tests::test_conversion_to_celsius ... ok
test tests::test_read_csv ... ok
test tests::test_write_csv ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Takeaways

Notice that in `read_csv` in the Rust file the return type is `Result<Vec<CityTemps>, Box<dyn std::error::Error>>`.
In Python, exceptions can be raised either explicitly or silently at any time. However, this is not
the case in Rust. If a function can error it, _must_ return the `Result` type, specifying the type
for success, the `Ok` value, and the type for errors, the `Err`. Any time a function is called
that returns a `Result` both the `Ok` and the `Err` possiblities have to be handled. The `?` operator
can be used when a function returns a `Result`, and specfifies that if the result of the function
call is an error that error should be returned, otherwise use the `Ok` value.

You will also notice that `expect` and `unwrap` are used. These are similiar to using `?`, but can
be used in functions that don't return a `Result`. The difference between `expect` and `unwrap` is
`expect` allows you to provide a specific message when a panic, while `unwrap` uses a generaic message.
Because `expect` allows a custom message, it can make it easier to understand where an error is happening
since you will be to compare the error message to the messages you have provided. `expect` and `unwrap`
are good for prototyping and tests, but are almost never what you want to use in the final application
or library. If the result of the `expect`/`unwrap` is an error, the program will panic, meaning it
exits without the possiblity of recovering, so it's considered a best practice to clean up your code
with proper error handling using `Result` prior to running any code in production.
