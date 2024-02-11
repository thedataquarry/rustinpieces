# PyO3 Mock data generation

Generate a mock dataset using [PyO3](https://github.com/PyO3/pyo3) to create a module to generate
the data in Rust and call the function from Python. [Maturin](https://github.com/PyO3/maturin) will
be used to build the Rust module and make it available to Python.

## Goal

In this project, we will generate a mock tabular dataset of people, their age, marital status and
the city, state and country they last visited. The data will be genrated in a Rust module and called
from Python. The dataset should be in the following format:

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
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# For subsequent runs, simply activate the environment
source venv/bin/activate
```

### Build the Rust module

Maturin is used to build the rust Module. The build can be run either with the provided Makefile
or directly.

```bash
# With make
make develop

# Directly with Maturin
maturin develop
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
python main.py -n 10
# Generate 1000 mock person profiles
python main.py -n 1000
```

### Run tests

Test can be run either with the provided Makefile or directly. The Makefile will run a build before
running the tests. If you run manually don't forget the build step before testing.

```bash
# With make
make test

# manually
maturin develop
pytest

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
maturin develop -r
```

## Performance

Because the number of persons generated via this script is configurable, we can generate datasets
of different sizes. Because the data is generated in Rust, we can expect the performance to be
better than Python alone (even with minimal optimizations).

Performance numbers here can be compared to the [mock data generation](../mock_data/README.md) where
Python and Rust were used individually to generate the data.

> [!NOTE]
> The timing numbers shown below are the run times from a 2022 M2 Macbook Pro with 16GB of RAM.
> The Python version used was `3.11.6` and the Rust version used was `1.74.1`.

| numPersons | unoptimized | Release |
| ---------- | ----------- | ------- |
| 10         |             |         |
| 100        |             |         |
| 1000       |             |         |
| 10000      |             |         |
| 100000     |             |         |
| 1000000    |             |         |

## Takeaways

It is realatively easy to use PyO3 to build Python modules in Rust, and get significant speedups.
There is some overhead involved with passing data back and forth between Python and Rust, and
because of this writing a module in Rust will not always be faster. In situations where Python is
slow, such as with this fake data generation, Rust can be significantly faster.
