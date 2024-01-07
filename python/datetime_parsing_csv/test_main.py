from pathlib import Path

import pytest
from main import read_and_modify, write_csv


@pytest.fixture
def persons():
    path_to_file = Path("./data/persons.csv")
    data = read_and_modify(path_to_file)
    return data


def test_read_and_modify(persons):
    assert len(persons) == 6
    assert persons[0].get("id") is not None


def test_write_csv(persons):
    output_path = Path("./data/test_persons.csv")
    write_csv(persons, output_path)
    assert output_path.exists()
    # Delete the file
    Path("./data/test_persons.csv").unlink()
