# Mock data generation

Generate a mock dataset using the [Faker](https://faker.readthedocs.io/en/master/) library in
Python, and the [`fake-rs`](https://github.com/cksac/fake-rs) crate in Rust.

## Goal

In this project, we will generate a mock tabular dataset of people, their age, marital status and
the city, state and country they last visited. The dataset should be in the following format:

```json
{
  "id": "int",
  "name": "string",
  "age": "integer",
  "isMarried": "boolean",
  "city": "string",
  "state": "string",
  "country": "string"
}
```

The fields in the dataset should meet the following requirements:

- The name of the person must be a valid unicode string of the form `Firstname Lastname`
- The age of persons must be between 22-65
- The `isMarried` field must be a boolean string of the form `true` or `false`
- The city, state and country must be valid locations on planet Earth 🌏

Most importantly, the number of persons generated should be a configurable variable so that we can
generate reproducible datasets of different sizes using a random seed.

## Inputs

Because we need to generate mock data with real locations, we use the
[world cities dataset](https://www.kaggle.com/datasets/juanmah/world-cities?resource=download) from
Kaggle. This is an accurate and up-to-date database of the world's cities and towns and more
information, totalling to ~44k locations all over the world.

## Output

The output of this project is a CSV file `./data/persons.csv` with the desired schema shown above.

```csv
id,name,age,isMarried,city,state,country
1,Megan Chang,48,false,Fredonia,Antioquia,Colombia
2,Billy Sheppard,38,false,Campeche,Campeche,Mexico
3,Richard Bowers,53,false,Tahannawt,Marrakech-Safi,Morocco
4,Tammy Howard,41,true,Somandepalle,Andhra Pradesh,India
5,William Campbell,44,true,Dimiao,Bohol,Philippines
6,Christine King,35,true,Kanur,Karnataka,India
7,Kyle Blair,30,false,Ettapur,Tamil Nadu,India
8,Thomas Garcia,30,false,Gurpinar,Van,Turkey
9,Leslie Bowman,61,true,Madaba,Madaba,Jordan
10,Tammy Woods,56,false,Vernon,British Columbia,Canada
```

## Python Setup

Install dependencies via the `uv` package manager. All dependencies are listed in `pyproject.toml`.

If you want to manually add the dependencies yourself, run the following commands.

```bash
uv add faker
uv add --dev pytest
```

### Run script

First, sync the dependencies from `pyproject.toml`.

```bash
uv sync
```

The script can be then run using the following command.

```bash
# Generate 10 mock person profiles
uv run main.py -n 10
# Generate 1000 mock person profiles
uv run main.py -n 1000
```

### Run tests

```bash
$ uv run pytest -v
================================================================================================= test session starts =================================================================================================
platform darwin -- Python 3.12.5, pytest-8.3.4, pluggy-1.5.0 -- /Users/prrao/code/rustinpieces/src/mock_data/python/.venv/bin/python3
cachedir: .pytest_cache
rootdir: /Users/prrao/code/rustinpieces/src/mock_data/python
configfile: pyproject.toml
plugins: Faker-33.1.0
collected 9 items

test_main.py::test_convert_unicode_to_ascii[S\xe3o Paulo-Sao Paulo] PASSED                                                                                                                                      [ 11%]
test_main.py::test_convert_unicode_to_ascii[\u0130stanbul-Istanbul] PASSED                                                                                                                                      [ 22%]
test_main.py::test_convert_unicode_to_ascii[Mah\u0101r\u0101shtra-Maharashtra] PASSED                                                                                                                           [ 33%]
test_main.py::test_convert_unicode_to_ascii[St\u0159edo\u010desk\xfd Kraj-Stredocesky Kraj] PASSED                                                                                                              [ 44%]
test_main.py::test_convert_unicode_to_ascii[Dar\u2018\u0101-Dara] PASSED                                                                                                                                        [ 55%]
test_main.py::test_convert_unicode_to_ascii[\xcele-de-France-Ile-de-France] PASSED                                                                                                                              [ 66%]
test_main.py::test_get_locations PASSED                                                                                                                                                                         [ 77%]
test_main.py::test_generate_fake_persons PASSED                                                                                                                                                                 [ 88%]
test_main.py::test_write_persons_to_csv PASSED                                                                                                                                                                  [100%]

================================================================================================== 9 passed in 0.28s ==================================================================================================
```

## Rust Setup

Install dependencies via Cargo. Note that because we perform CSV serialization/deserialization via
`serde`, we need to install it using the features flag. Multiple other crates, like `fake` for
generating fake data, `unicode-normalization` for normalizing unicode strings and `csv` for
reading/writing CSV files are also installed.

```bash
cargo add csv
cargo add serde --features derive
cargo add fake --features derive
cargo add unicode-normalization
cargo rand
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

In this case, we want to specify the number of mock persons to generate via an argument to
`cargo run`. We can do this by passing the argument `-- 10` to `cargo run`.

```bash
cargo run -- 10
```

Once the debugging phase is over, it can be beneficial to run the optimized version via the
`--release` flag.

```
cargo run --release -- 1000000
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

### Run tests only

The Rust in-built test client allows tests to be defined within the same file as the code being tested. Because Rust is a compiled language, the compiler will know to ignore the tests when building the final binary for runtime.

Tests are run using `make test` or `cargo test --quiet`.

```bash
make test
cargo test --quiet


running 3 tests
test tests::test_convert_unicode_to_ascii ... ok
test tests::test_read_cities ... ok
test tests::test_construct_person ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
```

---

## Performance

Because the number of persons generated via this script is configurable, we can generate datasets of
different sizes.

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 32GB of RAM.
> The Python version used was `3.11.7` and the Rust version used was `1.75.0`.

| numPersons | Python    | Rust (Release mode) | Speedup factor over Python |
| ---------- | --------- | ------------------- | ---------------------------|
| 10         | 0.18 sec  | 0.04 sec | 4.5x |
| 100        | 0.18 sec  | 0.05 sec | 3.6x |
| 1000       | 0.23 sec  | 0.05 sec | 4.6x |
| 10000      | 0.80 sec  | 0.06 sec | 13.3x |
| 100000     | 6.35 sec  | 0.13 sec | 48.8x |
| 1000000    | 62.27 sec | 0.89 sec | 70.1x |

Generating mock data with Rust in release mode is order of magnitudes faster than the Python code.
The more data we're dealing with, the bigger this difference will be.
