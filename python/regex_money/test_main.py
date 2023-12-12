import pytest
from main import run


@pytest.fixture
def data():
    data = run()
    assert data
    assert len(data) == 4
    return data


def test_revenue_range(data):
    for company in data:
        assert company.get("annual_revenue_lower") is not None
        assert company.get("annual_revenue_upper") is not None
        assert company["annual_revenue_lower"] < company["annual_revenue_upper"]
        assert company["annual_revenue_lower"] >= 0.0


def test_revenue_values(data):
    assert data[0]["annual_revenue_lower"] == 10000000.0
    assert data[0]["annual_revenue_upper"] == 20000000.0
    assert data[1]["annual_revenue_lower"] == 7500000.0
    assert data[1]["annual_revenue_upper"] == 8500000.0
    assert data[2]["annual_revenue_lower"] == 500000.0
    assert data[2]["annual_revenue_upper"] == 1000000.0
    assert data[3]["annual_revenue_lower"] == 800000000.0
    assert data[3]["annual_revenue_upper"] == 1000000000.0
