from multiprocessing import Queue
from pathlib import Path

import pytest
from faker import Faker
from main import (
    convert_unicode_to_ascii,
    generate_data,
    generate_fake_persons,
    get_locations,
)

AGE_LOWER, AGE_UPPER = 22, 65


@pytest.fixture
def faker():
    return Faker()


@pytest.fixture
def location_data():
    locations = get_locations(Path("../data/worldcities.csv"))
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


def test_generate_data(faker, tmp_path):
    output_path = tmp_path / "test_persons.csv"
    message_queue = Queue()
    generate_data(faker, 10, output_path, message_queue)
