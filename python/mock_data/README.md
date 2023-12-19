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

The output of this project is a CSV file with the desired schema shown above.

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
pytest -v
```
