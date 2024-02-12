import pytest

from pyo3_mock_data import generate_fake_persons


@pytest.mark.parametrize("limit", (10, 20))
def test_write_persons_to_csv(limit, tmp_path):
    output_path = tmp_path / "persons.csv"
    generate_fake_persons("./data/worldcities.csv", limit=limit, output_filename=output_path)
    assert output_path.exists()
    with open(output_path) as f:
        lines = len(f.readlines()) - 1  # Remove the trailing new line
    assert lines == limit


def test_write_persons_to_csv_default_limit(tmp_path):
    output_path = tmp_path / "persons.csv"
    generate_fake_persons("./data/worldcities.csv", output_filename=output_path)
    assert output_path.exists()
    with open(output_path) as f:
        lines = len(f.readlines()) - 1  # Remove the trailing new line
    assert lines == 10
