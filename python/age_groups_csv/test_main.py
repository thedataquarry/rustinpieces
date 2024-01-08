from pathlib import Path

import pytest

from main import AgeBracket, Person, load_csv


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
    file_path = Path("data/persons.csv")
    persons = load_csv(file_path)
    assert len(persons) == 10
    assert persons[0].id == 1
