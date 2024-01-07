from __future__ import annotations

import csv
from datetime import datetime
from pathlib import Path
from typing import Any


def read_and_modify(filepath: Path):
    persons = []
    with open(filepath, "r") as f:
        reader = csv.DictReader(f)
        for i, item in enumerate(reader, 1):
            new_item: dict[str, Any] = dict()
            new_item["id"] = i
            new_item["name"] = item["name"]
            new_item["dob"] = (
                datetime.strptime(item["dob"], "%m-%d-%Y").date() if item.get("dob") else None
            )
            new_item["age"] = item["age"]
            new_item["isMarried"] = item["isMarried"]
            new_item["city"] = item["city"]
            new_item["state"] = item["state"]
            new_item["country"] = item["country"]
            persons.append(new_item)
    print(f"Read {len(persons)} lines from {filepath}")
    return persons


def write_csv(persons: list, output_path: Path) -> None:
    with open(output_path, "w") as f:
        fieldnames = list(persons[0].keys())
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        for person in persons:
            writer.writerow(person)


def main() -> None:
    file_path = Path("data/persons.csv")
    persons = read_and_modify(file_path)
    output_path = Path("data/persons_modified.csv")
    write_csv(persons, output_path)


if __name__ == "__main__":
    main()
