# Parallel processing intro

Demonstrate parallel processing.

## Goal

In this project, we will build on the [mock data generation](../mock_data/) example by displaying
a spinner while the data is being generated in parallel. This will make it clear that the program
is still running and not frozen while generating large amounts of data.

This exercise is meant as a simple introduction to parallel processing and message passing. If you
want to add spinners or progress bars to you application we recommend a purpose built library such
as [TQDM](https://github.com/tqdm/tqdm) or [Rich](https://github.com/Textualize/rich) in Python,
and [Indicatif](https://github.com/console-rs/indicatif) in Rust.

## Inputs

Because we need to generate mock data with real locations, we use the
[world cities dataset](https://www.kaggle.com/datasets/juanmah/world-cities?resource=download) from
Kaggle. This is an accurate and up-to-date database of the world's cities and towns and more
information, totalling to ~44k locations all over the world.

## Output

The output of this project is a CSV file `./data/persons.csv`

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

Install the dependencies in a virtual environment via `requirements.txt`.

```bash
# First time setup
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# For subsequent runs, simply activate the environment
source venv/bin/activate
```

### Run script

```bash
# Generate 10 mock person profiles
python main.py -n 10
# Generate 1000 mock person profiles
python main.py -n 1000
```

### Run tests

==================================================================================== test session starts ====================================================================================
platform linux -- Python 3.12.1, pytest-7.4.4, pluggy-1.3.0
rootdir: /home/paul/development/rust/rustinpieces/pieces/parallel_processing_intro/python
plugins: Faker-21.0.1
collected 9 items

test_main.py ......... [100%]

===================================================================================== 9 passed in 1.03s =====================================================================================

## Rust Setup
