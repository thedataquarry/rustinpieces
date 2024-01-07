# Modify a CSV file

Read in data from a CSV file and write it back to a new CSV file with modifications.

## Goal

In this project, we will add a new column to a CSV file that contains data on people, their age, marital status and the city, state and country they last visited. The goal is to write an integer `id` for each person, starting from 1 and incrementing by 1 for each person, and store that as a new CSV file.

The dataset is in the following format:

```json
{
  "name": "string",
  "dob": "string",
  "age": "integer",
  "isMarried": "boolean",
  "city": "string",
  "state": "string",
  "country": "string"
}
```

## Inputs

The input CSV file is `./data/people.csv` with the following data.

```csv
name,dob,age,isMarried,city,state,country
Michelle Lewis,,49,true,San Borja,El Beni,Bolivia
Jack Garrison,05-01-1987,36,true,Lakeland North,Washington,United States
Rebecca Hawkins,11-21-1987,36,false,Houghton le Spring,Sunderland,United Kingdom
Erik Nelson,03-14-1970,53,true,Talagante,Region Metropolitana,Chile
Stephanie Morgan,12-25-1977,46,false,Shankou,Guangdong,China
Daniel Prince,02-02-1997,26,false,Audubon,Pennsylvania,United States
```

## Output

The output is also a CSV file `./data/people_modified.csv` with an additional column `id` that has an incrementally rising integer ID for each person.
Additionally the dob column is converted from mm-dd-yyyy format to yyyy-mm-dd format.

```csv
id,name,dob,age,isMarried,city,state,country
1,Michelle Lewis,,49,true,San Borja,El Beni,Bolivia
2,Jack Garrison,1987-05-01,36,true,Lakeland North,Washington,United States
3,Rebecca Hawkins,1987-11-21,36,false,Houghton le Spring,Sunderland,United Kingdom
4,Erik Nelson,1970-03-14,53,true,Talagante,Region Metropolitana,Chile
5,Stephanie Morgan,1977-12-25,46,false,Shankou,Guangdong,China
6,Daniel Prince,1997-02-02,26,false,Audubon,Pennsylvania,United States
```

## Setup

Install dependencies via Cargo. Note that because we perform CSV serialization/deserialization via `serde`, we need to install it using the features flag.

```bash
cargo add csv
cargo add serde --features derive
cargo add chrono
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

