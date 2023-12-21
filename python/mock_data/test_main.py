from pathlib import Path

import pytest
from faker import Faker
from main import (
    convert_unicode_to_ascii,
    generate_fake_persons,
    get_locations,
    write_persons_to_csv,
)

AGE_LOWER, AGE_UPPER = 22, 65


@pytest.fixture
def faker():
    return Faker()


@pytest.fixture
def location_data():
    locations = get_locations(Path("./data/worldcities.csv"))
    return locations


@pytest.mark.parametrize(
    "test_input,expected",
    [
        ("São Paulo", "Sao Paulo"),
        ("İstanbul", "Istanbul"),
        ("Mahārāshtra", "Maharashtra"),
        ("Středočeský Kraj", "Stredocesky Kraj"),
        ("Dar‘ā", "Dara"),
        ("Île-de-France", "Ile-de-France"),
    ],
)
def test_convert_unicode_to_ascii(test_input, expected):
    assert convert_unicode_to_ascii(test_input) == expected


def test_get_locations(location_data):
    assert len(location_data) == 44691


def test_generate_fake_persons(faker, location_data):
    profiles = generate_fake_persons(faker, location_data, 10)
    for profile in profiles:
        assert profile["id"] > 0
        assert len(profile["name"].split()) > 1
        assert AGE_LOWER <= profile["age"] <= AGE_UPPER


def test_write_persons_to_csv(faker, location_data):
    profiles = generate_fake_persons(faker, location_data, 10)
    output_path = Path("./data/test_persons.csv")
    write_persons_to_csv(profiles, output_path)
    assert output_path.exists()
    # Delete the file
    Path("./data/test_persons.csv").unlink()
