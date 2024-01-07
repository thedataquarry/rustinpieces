# Modify a CSV file

Read in data from a CSV file and write it back to a new CSV file with modifications.

## Goal

In this project, we will add two new columns to a CSV file that contains data on people, their age,
their date of birth, marital status and the city, state and country they last visited. The goal is to
write a new CSV file containing an integer `id` for each person, starting from 1 and incrementing by 1
for each person, and to convert the dob column from the `mm-dd-yyyy` format to `yyyy-mm-dd` format.

One difference you will notice between the Rust and Python versions in working with dates is in the
Python version, no external packages are required, while in Rust we install the
[chrono](https://github.com/chronotope/chrono) crate to work with datetimes. Python is often referred to
as a "batteries included" language because it has a large standard library with a lot of functionality
built in. Rust takes the opposite approach with a small standard library, requiring the use of
external libraries, known as *crates*, to handle a lot of the functionality.

Each of these approaches has its own set of advantages and disadvantages. The Python approach focuses on
simplicity and productivity: you just need to import `datetime` from the standard library and you are
good to go. The downside to this approach is that new functionality can only be added to the standard library
modules during the release of new Python versions, which currently happens yearly. Additionally, older
versions of Python will not be able to use the new functionality.

Rust's philosophy allows for more flexibility on the developer's end and faster adding of features to existing
projects based on the ever-growing crates ecosystem. This however means that
you need to know about which external crates to to use to handle specific functionality, making things less straightforward
if you're new to Rust. In this example we choose the chrono crate because it is currently the most popular option based on
the [number of downloads on crates.io](https://crates.io/search?q=chrono), but it's improtant to remember that it's
not the only option. We could have, for example, used the [time](https://github.com/time-rs/time)
crate to achieve the same thing. It's recommended to look up the catalog of crates on [crates.io](https://crates.io/)
and choose the one that best fits your needs.

Either language's approach is not better or worse than the other, it is just something you need to be aware of when moving
between the two languages.

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

Install dependencies via a virtual environment.

```bash
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

## Run script

```bash
python main.py
```

## Run tests

```bash
$ pytest -v
================================================================== test session starts ===================================================================
platform darwin -- Python 3.11.6, pytest-7.4.3, pluggy-1.3.0 -- /Users/prrao/.pyenv/versions/3.11.6/bin/python3.11
cachedir: .pytest_cache
rootdir: /code/rustinpieces/python/modify_csv
plugins: anyio-4.0.0, Faker-21.0.0
collected 2 items

test_main.py::test_read_and_modify PASSED                                                                                                          [ 50%]
test_main.py::test_write_csv PASSED                                                                                                                [100%]

=================================================================== 2 passed in 0.02s ====================================================================
```
