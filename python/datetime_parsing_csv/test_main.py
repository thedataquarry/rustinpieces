from datetime import datetime
from pathlib import Path

import pytest
from main import modify_fields, read_and_modify, write_csv


@pytest.fixture
def persons():
    path_to_file = Path("./data/persons.csv")
    data = read_and_modify(path_to_file)
    return data


def test_modify_fields():
    item = {
        "name": "Alicia Watson",
        "dob": "09-21-1989",
        "age": "34",
        "isMarried": "true",
        "city": "New York",
        "state": "New York",
        "country": "United States",
    }
    new_item = modify_fields(item, 1)
    assert new_item.get("id") == 1
    assert new_item.get("dob") == datetime(1989, 9, 21).date()


def test_read_and_modify(persons):
    assert len(persons) == 6
    assert persons[0].get("id") > 0


def test_write_csv(persons):
    output_path = Path("./data/test_persons.csv")
    write_csv(persons, output_path)
    assert output_path.exists()
    # Delete the file
    Path("./data/test_persons.csv").unlink()
