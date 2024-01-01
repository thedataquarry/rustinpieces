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

TBD

## Performance

TBD