# Mock data generation

Generate a mock dataset using the [Faker](https://faker.readthedocs.io/en/master/) library in Python.

## Goal

In this project, we will generate a fake tabular dataset of people, their age, marital status and the city, state and country they last visited. The dataset should be in the following format:

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

Most importantly, the number of persons generated should be a configurable variable so that we can generate reproducible datasets of different sizes using a random seed.

## Inputs

Because we need to generate mock data with real locations, we use the [world cities dataset](https://www.kaggle.com/datasets/juanmah/world-cities?resource=download) from Kaggle. This is an accurate and up-to-date database of the world's cities and towns and more information, totalling to ~44k locations all over the world.

## Output

The output of this project is a CSV file `./data/persons.csv` with the desired schema shown above.

```csv
id,name,age,isMarried,city,state,country
1,Mateo Donnelly,38,true,Cuers,Provence-Alpes-Cote dAzur,France
2,Elian Walker,18,true,Santa Maria Texmelucan,Puebla,Mexico
3,Brycen Denesik,51,false,Gumia,Jharkhand,India
4,Aron O'Kon,49,true,Staffanstorp,Skane,Sweden
5,Judah Kling,57,true,Midvale,Utah,United States
6,Marjolaine Terry,55,false,Popesti-Leordeni,Ilfov,Romania
7,Alexander Conroy,32,true,Gorizia,Friuli Venezia Giulia,Italy
8,Tyreek Mertz,64,true,Manilva,Andalusia,Spain
9,Lurline Schoen,19,false,Monte Alegre de Minas,Minas Gerais,Brazil
10,Ethyl Deckow,47,false,Soasio,Maluku Utara,Indonesia
```

## Setup

Install dependencies via Cargo. Note that because we perform CSV serialization/deserialization via `serde`, we need to install it using the features flag.

```bash
cargo add csv
cargo add serde --features derive
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

In this case, we want to specify the number of mock persons to generate via an argument to `cargo run`. We can do this by passing the argument `-- 10` to `cargo run`.

```bash
cargo run -- 10
```

Once the debugging phase is over, it can be beneficial to run the optimized version via the `--release` flag.

```
cargo run --release -- 1000000
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
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Performance

Because the number of persons generated via this script is configurable, we can generate datasets of different sizes. And because it's Rust, we can expect the performance to be better than Python (even with minimal optimizations). 😀

> [!NOTE]
> The timing numbers shown below are the run times from a 2022 M2 Macbook Pro with 16GB of RAM.
> The Python version used was `3.11.6` and the Rust version used was `1.74.1`.
> The speedup over the Python code is shown in parentheses.


numPersons | Python | Rust (unoptimized) | Rust (Release)
--- | --- | --- | ---
10 | 0.21 sec | 0.35 sec (**0.6x**) | 0.15 sec (**1.4x**)
100 | 0.22 sec | 0.36 sec (**0.6x**) | 0.17 sec (**1.3x**)
1000 | 0.28 sec | 0.38 sec (**0.7x**) | 0.17 sec (**1.6x**)
10000 | 0.90 sec | 0.55 sec (**1.6x**)| 0.18 sec (**5.0x**)
100000 | 7.21 sec | 2.34 sec (**3.1x**) | 0.27 sec (**26.7x**)
1000000 | 70.91 sec | 20.29 sec (**3.5x**) | 1.16 sec (**61.1x**)

Even the unoptimized Rust code is multiple times faster than the Python code, the more data we're dealing with.