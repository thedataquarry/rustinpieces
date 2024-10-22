# PyO3 mock data generation

Generate a mock dataset using [PyO3](https://github.com/PyO3/pyo3) to create a module to generate
the data in Rust and call the function from Python. [Maturin](https://github.com/PyO3/maturin) will
be used to build the Rust module and make it available to Python.

## Goal

In this project, we will generate a mock tabular dataset of people, their age, marital status and
the city, state and country they last visited. The data will be genrated in a Rust module and called
from Python.

### Why might we do this in Rust?

As can be seen from the [mock_data](../mock_data/README.md) project, generating mock data in Python
can be slow as we increase the number of records. This is for two reasons:

1. Generating mock data is not straightforward to parallelize, because we need to maintain order
   across the fields along with the randomness in the data, and so, we need to resort to Python
   for loops, which are slow
2. The `Faker` library is used in Python, which is itself written in Python

Timing the mock data showed that Rust was up to 60x faster than Python as we approached 1M records
generated. In a real-world situation where we need to generate multiple mock datasets to test
different scenarios prior to production, this speedup can be significant.

However, Rust is not a language that a lot of developers are familiar with (or can learn), so it
makes sense to write a Rust extension that can be called from Python, which is the goal of this piece.

### Dataset schema

The dataset should be in the following format:

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

## Setup

Install the dependencies in a virtual environment via `requirements.txt`.

```bash
# First time setup
uv venv
source .venv/bin/activate
uv sync --frozen

# For subsequent runs, simply activate the environment
source .venv/bin/activate
```

### Build the Rust module

Maturin is used to build the rust module. The build can be run either with the provided Makefile
or directly.

```bash
# With make
make develop

# Directly with Maturin
uv run maturin develop
```

### Run script

Before running the python file make sure to rebuild the the Rust module if any changes have been
made. This is an easy step to forget because when working only with Rust cargo builds before running,
and when working with only Python there is no build step. When working with PyO3 the build doesn't
happen automatically so no changes to the Rust files will show up when called from Python without
the build step first. I know someone who has wasted way more time than he would care to admit trying
to figure out why changes aren't showing up only to realize the build step was forgotten...yes, I am
that someone :joy:.

```bash
# Generate 10 mock person profiles
uv run python main.py -n 10
# Generate 1000 mock person profiles
uv run python main.py -n 1000
```

### Run tests

Test can be run either with the provided Makefile or directly. The Makefile will run a build before
running the tests. If you run manually don't forget the build step before testing.

```bash
# With make
make test

# manually
uv run maturin develop
uv run pytest

==================================================================================== test session starts =====================================================================================
platform linux -- Python 3.12.2, pytest-8.0.0, pluggy-1.4.0
rootdir: /home/paul/development/rust/rustinpieces/src/pyo3_mock_data
collected 1 item

test_main.py::test_write_persons_to_csv PASSED

===================================================================================== 1 passed in 0.28s ======================================================================================
```

### Run linter and formatter only

```bash
make format
make check
make lint
# Runs the following
cargo fmt --all --quiet
cargo check --all-targets --quiet
cargo clippy --all-targets --quiet
```

### Release mode testing

When benchmarking, it can be beneficial to run the optimized version via the `--release` flag. This
can be done with either the provide Makefile or manually.

```bash
# With make
make release

# Manually
uv run maturin develop -r
```

## Performance

Because the number of persons generated via this script is configurable, we can generate datasets
of different sizes. Because the data is generated in Rust, we can expect the performance to be
better than Python alone (even with minimal optimizations).

Performance numbers here can be compared to the [mock data](../mock_data/README.md) generation piece where Python and Rust were individually used to generate the data.

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 32GB of RAM.
> The Python version used was `3.11.7` and the Rust version used was `1.75.0`.

| numPersons | Release mode |
| ---------- | ------------ |
| 10         | 0.06 sec     |
| 100        | 0.06 sec     |
| 1000       | 0.07 sec     |
| 10000      | 0.07 sec     |
| 100000     | 0.15 sec     |
| 1000000    | 0.91 sec     |

As can be seen, the Rust bindings called from Python produce near identical timings to
the pure Rust version's [results](../mock_data/README.md#performance) from the `mock_data`piece,
within 10%. The (very) slight difference is due to the overhead of calling Rust from Python.

## Takeaways

Rather than expecting users to install and learn Rust, it's possible to expose Rust
bindings to the Python interpreter, so that Python users can benefit from the performance
of Rust.

As can be seen. it is relatively easy to use PyO3 to build Python modules in Rust and get significant
speedups over pure Python.
There is some overhead involved with passing data between Rust and Python, and
because of this writing a Python extension is not likely to be identical in performance
to pure Rust code. However, in situations that are inherently difficult to parallelize in Python,
such as with this fake data generation, it makes sense to consider writing a Python extension in Rust.
