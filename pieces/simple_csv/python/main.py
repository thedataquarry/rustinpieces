from __future__ import annotations

import csv
from pathlib import Path
from typing import Any


def convert_f_to_c(temp_f: float) -> float:
    """
    Convert from Fahrenheit to Celsius to 1 decimal precision
    """
    temp_c = (temp_f - 32) * 5 / 9
    rounded_temp_c = round(temp_c, 1)
    return rounded_temp_c


def modify_fields(item: dict) -> dict:
    new_item: dict[str, Any] = dict()
    new_item["city"] = item["city"]
    new_item["country"] = item["country"]
    new_item["avg_temp_jan_f"] = item["avg_temp_jan_f"]
    # Convert temp to Celsius
    new_item["avg_temp_jan_c"] = convert_f_to_c(float(item["avg_temp_jan_f"]))
    return new_item


def read_and_modify(filepath: Path) -> list[dict[str, Any]]:
    cities = []
    with open(filepath) as f:
        reader = csv.DictReader(f)
        cities = [modify_fields(item) for item in reader]
    print(f"Read {len(cities)} records from {str(filepath)}")
    return cities


def write_csv(persons: list, output_path: Path) -> None:
    with open(output_path, "w") as f:
        fieldnames = list(persons[0].keys())
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        for person in persons:
            writer.writerow(person)


def main() -> None:
    file_path = Path("../data/city_temps.csv")
    cities_modified = read_and_modify(file_path)
    output_path = Path("../data/city_temps_modified.csv")
    write_csv(cities_modified, output_path)
    print(f"Wrote {len(cities_modified)} records to {str(output_path)}")


if __name__ == "__main__":
    main()
