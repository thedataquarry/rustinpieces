# Modify a CSV file

Read in data from a CSV file and write it back to a new CSV file with modifications.

## Goal

In this project, we will add a new column to a CSV file that contains data on people, their age, marital status and the city, state and country they last visited. The goal is to write an integer `id` for each person, starting from 1 and incrementing by 1 for each person, and store that as a new CSV file.

The dataset is in the following format:

```json
{
    "name": "string",
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
name,age,isMarried,city,state,country
Michelle Lewis,49,true,San Borja,El Beni,Bolivia
Jack Garrison,36,true,Lakeland North,Washington,United States
Rebecca Hawkins,36,false,Houghton le Spring,Sunderland,United Kingdom
Erik Nelson,53,true,Talagante,Region Metropolitana,Chile
Stephanie Morgan,46,false,Shankou,Guangdong,China
Daniel Prince,26,false,Audubon,Pennsylvania,United States
```

## Output

The output is also a CSV file `./data/people_modified.csv` with an additional column `id` that has an incrementally rising integer ID for each person.

```csv
id,name,age,isMarried,city,state,country
1,Michelle Lewis,49,true,San Borja,El Beni,Bolivia
2,Jack Garrison,36,true,Lakeland North,Washington,United States
3,Rebecca Hawkins,36,false,Houghton le Spring,Sunderland,United Kingdom
4,Erik Nelson,53,true,Talagante,Region Metropolitana,Chile
5,Stephanie Morgan,46,false,Shankou,Guangdong,China
6,Daniel Prince,26,false,Audubon,Pennsylvania,United States
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
