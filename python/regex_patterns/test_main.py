import pytest
from main import run


@pytest.fixture
def data():
    data = run()
    assert data.get("companies") is not None
    assert len(data["companies"]) == 4
    return data


def test_revenue_range(data):
    for company in data["companies"]:
        assert company.get("annual_revenue_lower") is not None
        assert company.get("annual_revenue_upper") is not None
        assert company["annual_revenue_lower"] < company["annual_revenue_upper"]
        assert company["annual_revenue_lower"] >= 0.0


def test_revenue_values(data):
    assert data["companies"][0]["annual_revenue_lower"] == 10000000.0
    assert data["companies"][0]["annual_revenue_upper"] == 20000000.0
    assert data["companies"][1]["annual_revenue_lower"] == 50000000.0
    assert data["companies"][1]["annual_revenue_upper"] == 100000000.0
    assert data["companies"][2]["annual_revenue_lower"] == 500000.0
    assert data["companies"][2]["annual_revenue_upper"] == 1000000.0
    assert data["companies"][3]["annual_revenue_lower"] == 800000000.0
    assert data["companies"][3]["annual_revenue_upper"] == 1000000000.0
