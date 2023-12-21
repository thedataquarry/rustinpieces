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

Install dependencies via a virtual environment.

```bash
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

## Run script

```bash
# Generate 10 mock person profiles
python main.py -n 10
# Generate 1000 mock person profiles
python main.py -n 1000
```

## Run tests

```bash
$ pytest -v
============================================================= test session starts =============================================================
platform darwin -- Python 3.11.6, pytest-7.4.3, pluggy-1.3.0 -- /Users/prrao/.pyenv/versions/3.11.6/bin/python3.11
cachedir: .pytest_cache
rootdir: /code/rustinpieces/python/mock_data
plugins: anyio-4.0.0, Faker-21.0.0
collected 9 items                                                                                                                             

test_main.py::test_convert_unicode_to_ascii[S\xe3o Paulo-Sao Paulo] PASSED                                                              [ 11%]
test_main.py::test_convert_unicode_to_ascii[\u0130stanbul-Istanbul] PASSED                                                              [ 22%]
test_main.py::test_convert_unicode_to_ascii[Mah\u0101r\u0101shtra-Maharashtra] PASSED                                                   [ 33%]
test_main.py::test_convert_unicode_to_ascii[St\u0159edo\u010desk\xfd Kraj-Stredocesky Kraj] PASSED                                      [ 44%]
test_main.py::test_convert_unicode_to_ascii[Dar\u2018\u0101-Dara] PASSED                                                                [ 55%]
test_main.py::test_convert_unicode_to_ascii[\xcele-de-France-Ile-de-France] PASSED                                                      [ 66%]
test_main.py::test_get_locations PASSED                                                                                                 [ 77%]
test_main.py::test_generate_fake_persons PASSED                                                                                         [ 88%]
test_main.py::test_write_persons_to_csv PASSED                                                                                          [100%]

============================================================== 9 passed in 0.32s ==============================================================
```

## Performance

Because the number of persons generated via this script is configurable, we can generate datasets of different sizes.

> [!NOTE]
> The timing numbers shown below are the run times from a 2022 M2 Macbook Pro with 16GB of RAM.
> The Python version used was `3.11.6`.

numPersons | Python
--- | ---
10 | 0.21 sec
100 | 0.22 sec
1000 | 0.29 sec
10000 | 0.91 sec
100000 | 7.28 sec
1000000 | 69.91 sec