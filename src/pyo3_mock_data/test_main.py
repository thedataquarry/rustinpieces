from pyo3_mock_data import generate_fake_persons


def test_write_persons_to_csv(tmp_path):
    output_path = tmp_path / "persons.csv"
    generate_fake_persons("./data/worldcities.csv", 10, output_path)
    assert output_path.exists()
