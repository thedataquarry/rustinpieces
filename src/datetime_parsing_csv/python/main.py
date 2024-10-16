from __future__ import annotations

import csv
from datetime import datetime
from pathlib import Path
from typing import Any


def modify_fields(item: dict, i: int) -> dict:
    """
    Add an id field and convert dob to an ISO date
    """
    new_item: dict[str, Any] = dict()
    new_item["id"] = i
    new_item["name"] = item["name"]
    new_item["dob"] = datetime.strptime(item["dob"], "%m-%d-%Y").date() if item.get("dob") else None
    new_item["age"] = item["age"]
    new_item["isMarried"] = item["isMarried"]
    new_item["city"] = item["city"]
    new_item["state"] = item["state"]
    new_item["country"] = item["country"]
    return new_item


def read_and_modify(filepath: Path) -> list[dict[str, Any]]:
    persons = []
    with open(filepath) as f:
        reader = csv.DictReader(f)
        for i, item in enumerate(reader, 1):
            new_item = modify_fields(item, i)
            persons.append(new_item)
    print(f"Read {len(persons)} records from {str(filepath)}")
    return persons


def write_csv(persons: list, output_path: Path) -> None:
    with open(output_path, "w") as f:
        fieldnames = list(persons[0].keys())
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        for person in persons:
            writer.writerow(person)


def main() -> None:
    file_path = Path("../data/persons.csv")
    persons_modified = read_and_modify(file_path)
    output_path = Path("../data/persons_modified.csv")
    write_csv(persons_modified, output_path)
    print(f"Wrote {len(persons_modified)} records to {str(output_path)}")


if __name__ == "__main__":
    main()
