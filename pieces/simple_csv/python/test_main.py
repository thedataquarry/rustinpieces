from pathlib import Path

import pytest
from main import convert_f_to_c, modify_fields, read_and_modify, write_csv

ABS_TOLERANCE = 0.05


@pytest.fixture
def city_temps():
    path_to_file = Path("../data/city_temps.csv")
    data = read_and_modify(path_to_file)
    return data


def test_read_and_modify(city_temps):
    assert len(city_temps) == 10


def test_modify_fields(city_temps):
    item = {
        "city": "New York City",
        "country": "United States",
        "avg_temp_jan_f": "33.0",
    }
    new_item = modify_fields(item)
    assert new_item.get("avg_temp_jan_c") == pytest.approx(0.6, abs=ABS_TOLERANCE)


@pytest.mark.parametrize(
    "temp_f,temp_c",
    [
        (70.2, 21.2),
        (-14.1, -25.6),
        (25.3, -3.7),
        (29.6, -1.3),
        (52.1, 11.2),
        (18.7, -7.4),
        (-37.5, -38.6),
    ],
)
def test_convert_f_to_c(temp_f, temp_c):
    assert convert_f_to_c(temp_f) == pytest.approx(temp_c, abs=ABS_TOLERANCE)


def test_write_csv(city_temps):
    output_path = Path("../data/test_city_temps.csv")
    write_csv(city_temps, output_path)
    assert output_path.exists()
    # Delete the file
    Path("../data/test_city_temps.csv").unlink()
