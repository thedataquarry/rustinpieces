# Categorize with Enums

Read in data from a CSV file, add people to age categories, then agrigate the data according to groups.

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
DemographicCount(minors=2, adults=8)
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
==================================================================================== test session starts =====================================================================================
platform linux -- Python 3.12.1, pytest-7.4.4, pluggy-1.3.0 -- /home/paul/development/rust/rustinpieces/python/age_groups_csv/.venv/bin/python
cachedir: .pytest_cache
rootdir: /home/paul/development/rust/rustinpieces/python/age_groups_csv
collected 10 items

test_main.py::test_age_bracket[1-AgeBracket.CHILD] PASSED                                                                                                                              [ 10%]
test_main.py::test_age_bracket[12-AgeBracket.CHILD] PASSED                                                                                                                             [ 20%]
test_main.py::test_age_bracket[13-AgeBracket.YOUTH] PASSED                                                                                                                             [ 30%]
test_main.py::test_age_bracket[17-AgeBracket.YOUTH] PASSED                                                                                                                             [ 40%]
test_main.py::test_age_bracket[18-AgeBracket.ADULT] PASSED                                                                                                                             [ 50%]
test_main.py::test_age_bracket[59-AgeBracket.ADULT] PASSED                                                                                                                             [ 60%]
test_main.py::test_age_bracket[60-AgeBracket.SENIOR] PASSED                                                                                                                            [ 70%]
test_main.py::test_age_bracket[None-None] PASSED                                                                                                                                       [ 80%]
test_main.py::test_construct_person_obj PASSED                                                                                                                                         [ 90%]
test_main.py::test_calculate_demographcs PASSED                                                                                                                                        [100%]

===================================================================================== 10 passed in 0.01s =====================================================================================
```
