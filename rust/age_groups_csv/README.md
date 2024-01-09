# Categorize with Enums

Read in data from a CSV file, add people to age categories, then agrigate the data according to
groups.

## Goal

In this project, we will add users to age groups using enums. Users that are less than 13 years
old will be placed in the child group, 13 to 17 year olds will be in the youth group, 18 to 59 will
be in the adult group, and 60 or greater will be in the senior group. Once all the users are loaded
and categorized, a count of minors (people in the child and youth groups), and adults (people in the
adult and senior groups) will be calculated.

Enum use in Python is fairly rare, while in Rust is is common to have multiple enums. The Goal
of this project is to show one of the reasons they are so useful in Rust. Looking at the programs
the reason will probably not be immediately obvious, in both cases the enums are used to restrict
the `age_bracket` to specific values. Now lets say we decide to add a new geriatric value to the
`AgeBracket` enum for people 80 and older, which also changes the senior group to 60 to 79 years
old.

To do this we need to add the new value to the enum, update the setting of the age bracket, and
update the calculating of the demographics to include the new group. In the example here this update
seems trivial, everything in is one file so what needs updating is fairly obvious. Let's imaging
another scenario: you are new to a team working on an application with 100,000 lines of code, split
into 25 modules, and the `AgeBracket` enum is used in 15 different places. In this real world
scenario mistakes are much more likely.

In the Python version imaging you add the geriatric value to the enum, then update all the places
`AgeBracket` is used, but accidently miss updating `calculate_demographcs`. In this scenario the
type checker, linter, and tests will all pass with no errors. The problem is `calculate_demographcs`
will now ignore a whole group of people. The only way to know there is a problem is to notice the
counts that use the `calculate_demographcs` results are incorrect, but if you knew to check this
you probably wouldn't have missed the update in the first place.

Now let's consider the same scenario in Rust. The `match` statement requires all values in an enum to
be used. This means as soon as the geriatric group is added to the enum, the program will no longer
compile until the geriactric group is added to the match statement in the calculation. With this,
it is impossible to have the same problem that we had in the Python program, and the compiler will
tell you exactly where you need to make updates.

The dataset is in the following format:

```json
{
  "id": "integer",
  "name": "string",
  "age": "integer"
}
```

## Inputs

The input CSV file is `./data/people.csv` with the following data.

```csv
id,name,age
1,Megan Chang,8
2,Billy Sheppard,38
3,Richard Bowers,53
4,Tammy Howard,41
5,William Campbell,64
6,Christine King,35
7,Kyle Blair,13
8,Thomas Garcia,30
9,Leslie Bowman,61
10,Tammy Woods,56
```

## Output

The results of the demographics calcuation will be printed to the screen.

```console
DemographicCount { minors: 2, adults: 8 }
```

## Setup

Install dependencies via a Cargo. Note that because we perform CSV deserialization via `serde`, we
need to install it using the features flag.

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

To run just the main file, use the following command.

```bash
make run
# or, simply run via cargo
cargo run --quiet
```

## Run linter and formatter only

Cargo provides out-of-the-box for formatting (`cargo fmt`) and linting (`cargo clippy`). The
following command runs both. It's highly recommended to run both prior to pushing Rust code to a
repository.

```bash
make format
make lint
# Runs the following
cargo fmt --quiet
cargo clippy --quiet
```

## Run tests only

Using Rust's inbuilt client, tests can either be within `main.rs` or in a separate file
`test_main.rs` made accessible to `main.rs` via `mod test_main`.

Tests are run using `make test` or `cargo test --quiet`.

```bash
make test
cargo test --quiet

running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
