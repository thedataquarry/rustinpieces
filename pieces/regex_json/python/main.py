from __future__ import annotations

import json
import re
from enum import Enum
from pathlib import Path
from typing import Any

# Compiled patterns
hyphenated_pattern = re.compile(r"\$(\d+\.?\d*)([KMB])?-?\$?(\d+\.?\d*)([KMB])?")


def get_data() -> list[dict[str, Any]]:
    data = []
    with open(Path("../data/companies.json"), "r") as f:
        data = json.load(f)
    return data


class Revenue(Enum):
    K = 1e3
    M = 1e6
    B = 1e9


def calculate_range(revenue: str) -> tuple[float, float]:
    re_match = hyphenated_pattern.match(revenue)
    if not re_match:
        raise ValueError("No hyphenated patterns found")

    captures = re_match.groups()
    left_match_num = captures[0] if captures[0] else 0.0
    right_match_num = captures[2] if captures[2] else 0.0
    # Multiply by enum value to get ranges
    annual_revenue_lower = float(left_match_num) * Revenue[captures[1]].value
    annual_revenue_upper = float(right_match_num) * Revenue[captures[3]].value
    return annual_revenue_lower, annual_revenue_upper


def run() -> list[dict[str, Any]]:
    data = get_data()
    if not data:
        raise ValueError("No data found")

    for company in data:
        annual_revenue_lower, annual_revenue_upper = calculate_range(
            company["annual_revenue"]
        )
        # Append to existing dict
        company["annual_revenue_lower"] = annual_revenue_lower
        company["annual_revenue_upper"] = annual_revenue_upper
    return data


if __name__ == "__main__":
    data = run()

    from pprint import pprint

    pprint(data)
