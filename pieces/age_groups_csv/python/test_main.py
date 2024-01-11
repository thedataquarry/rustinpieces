from pathlib import Path

import pytest
from main import AgeBracket, Person, calculate_demographcs, load_csv


@pytest.mark.parametrize(
    "age, expected",
    (
        (1, AgeBracket.CHILD),
        (12, AgeBracket.CHILD),
        (13, AgeBracket.YOUTH),
        (17, AgeBracket.YOUTH),
        (18, AgeBracket.ADULT),
        (59, AgeBracket.ADULT),
        (60, AgeBracket.SENIOR),
        (None, None),
    ),
)
def test_age_bracket(age, expected):
    person = Person(id=1, name="Arthur Dent", age=age)
    assert person.age_bracket == expected


def test_construct_person_obj():
    file_path = Path("../data/persons.csv")
    persons = load_csv(file_path)
    assert len(persons) == 10
    assert persons[0].id == 1


def test_calculate_demographcs():
    persons = [
        Person(id=1, name="Arthur Dent", age=28),
        Person(id=2, name="Ford Prefect", age=72),
        Person(id=3, name="Trillian", age=19),
        Person(id=4, name="Marvin", age=2),
        Person(id=5, name="Mr. Prosser", age=14),
    ]

    result = calculate_demographcs(persons)
    assert result.minors == 2
    assert result.adults == 3
