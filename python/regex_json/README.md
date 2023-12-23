# Regex parsing from JSON

Converting string values of annual revenue into floats via regex patterns.

## Goal

The aim of this project is to convert a dataset of JSON blobs containing string values of companies' annual revenue in dollars to their float ranges of lower/upper bounds using regex pattern matching.

## Sample input

The example input shown below contains the revenue field as a string that's human-readable but not very machine-readable.

```json
{
    "company": "Initech Corp.",
    "annual_revenue": "$500K-$1M",
    "industry": "Technology"
}
```

## Desired output

Two new fields containing valid floats are added to each record that can be queried easily in a SQL or NoSQL database downstream.

```json
{
    "company": "Initech Corp.",
    "annual_revenue": "$500K-$1M",
    "annual_revenue_lower": 500000.0,
    "annual_revenue_upper": 1000000.0,
    "industry": "Technology"
}
```

## Assumptions

> [!NOTE]
> It's assumed that the annual revenue field from the incoming data is always in the format of `$<lower><unit>-$<upper><unit>` where `<lower>` and `<upper>` are numbers and `<unit>` is a multiplier that's one of `K`, `M`, or `B`, representing a thousand, million or billion respectively.

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

### Output

```python
[{'annual_revenue': '$10M-$20M',
  'annual_revenue_lower': 10000000.0,
  'annual_revenue_upper': 20000000.0,
  'company': 'Acme Inc.',
  'industry': 'Technology'},
 {'annual_revenue': '$7.5M-$8.5M',
  'annual_revenue_lower': 7500000.0,
  'annual_revenue_upper': 8500000.0,
  'company': 'Globex Corp.',
  'industry': 'Logistics'},
 {'annual_revenue': '$500K-$1M',
  'annual_revenue_lower': 500000.0,
  'annual_revenue_upper': 1000000.0,
  'company': 'Initech Corp.',
  'industry': 'Technology'},
 {'annual_revenue': '$800M-$1B',
  'annual_revenue_lower': 800000000.0,
  'annual_revenue_upper': 1000000000.0,
  'company': 'Umbrella Corp.',
  'industry': 'Retail'}]
```

## Run tests

Tests are run via pytest.

```bash
$ pytest -v
===================================================================================================== test session starts ======================================================================================================
platform darwin -- Python 3.11.6, pytest-7.4.3, pluggy-1.3.0 -- /code/rust-projects/python/regex_patterns/.venv/bin/python3.11
cachedir: .pytest_cache
rootdir: /code/rust-projects/python/regex_patterns
collected 2 items                                                                                                                                                                                                              

test_main.py::test_revenue_range PASSED                                                                                                                                                                                  [ 50%]
test_main.py::test_revenue_values PASSED                                                                                                                                                                                 [100%]

====================================================================================================== 2 passed in 0.01s =======================================================================================================
```
