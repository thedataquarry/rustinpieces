# PostgreSQL ETL

Insert data into a Postgres table and measure the performance of async querying.

## Goal

In this project, we will insert a tabular dataset containing a million people, their age, marital status and the city, state and country they last visited into a Postgres table. We will then measure the performance of 1000 async queries to the table.

## Inputs

The input is a CSV file `./data/persons.csv` generated from the [mock_data](../mock_data) project. We use an input argument of `1000000` to generate a dataset of a million persons and their information.

## Output

The output is the runtime performance (in seconds) of 1000 async queries to the Postgres table.

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

Tests can be run as follows:

```bash
$ pytest -v
================================ test session starts ================================
platform darwin -- Python 3.11.7, pytest-7.4.4, pluggy-1.3.0 -- /code/rustinpieces/python/postgres_etl/.venv/bin/python3.11
cachedir: .pytest_cache
rootdir: /code/rustinpieces/python/postgres_etl
plugins: asyncio-0.23.3
asyncio: mode=Mode.STRICT
collected 3 items                                                                   

test_main.py::test_main PASSED                                                [ 33%]
test_main.py::test_summary_query PASSED                                       [ 66%]
test_main.py::test_perf_query PASSED                                          [100%]

================================= 3 passed in 0.59s =================================
```

## Performance

By specifying an argument to `main.py`, we can control the number of async queries that we're running. The queries are aggregation queries that perform counts of persons whose age is greater than a random number between 22 and 65.

> [!NOTE]
> The timing numbers shown below are the run times from a 2023 M3 Macbook Pro with 32GB of RAM.
> The Python version used was `3.11.7`.

numPersons | Python
--- | ---
10 | 0.123 sec
100 | 0.150 sec
1000 | 0.299 sec
10000 | 1.761 sec
100000 | 15.999 sec