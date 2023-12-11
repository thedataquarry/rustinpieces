import json
import re
from enum import Enum
from typing import Any

# Compiled patterns
hyphenated_pattern = re.compile(r"\$(\d+\.?\d*)([KMB])?-?\$?(\d+\.?\d*)([KMB])?")
regular_pattern = re.compile(r"([\<\>])\$(\d+\.?\d*)([KMB])?")


def get_data() -> dict[str, Any]:
    data = {}
    with open("data/companies.json", "r") as f:
        data = json.load(f)
    return data


class Revenue(Enum):
    K = 1e3
    M = 1e6
    B = 1e9


def lower_bound_hyphenated(revenue: str) -> float:
    g = hyphenated_pattern.match(revenue).groups()
    if g[1] is None:
        return 0.0
    else:
        return float(g[0]) * Revenue[g[1]].value


def upper_bound_hyphenated(revenue: str) -> float:
    g = hyphenated_pattern.match(revenue).groups()
    if g[3] is None:
        return 0.0
    else:
        return float(g[2]) * Revenue[g[3]].value


def run() -> None:
    data = get_data()
    for company in data["companies"]:
        lower_bound = lower_bound_hyphenated(company["annual_revenue"])
        upper_bound = upper_bound_hyphenated(company["annual_revenue"])
        company["annual_revenue_lower"] = lower_bound
        company["annual_revenue_upper"] = upper_bound
    return data


if __name__ == "__main__":
    data = run()

    from pprint import pprint

    pprint(data)
