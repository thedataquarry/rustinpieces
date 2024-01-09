from __future__ import annotations

import csv
from enum import Enum
from pathlib import Path
from typing import NamedTuple


class AgeBracket(Enum):
    CHILD = "child"
    YOUTH = "youth"
    ADULT = "adult"
    SENIOR = "senior"


class DemographicCount(NamedTuple):
    minors: int
    adults: int


class Person:
    def __init__(
        self,
        id: int | None,
        name: str | None,
        age: int | None,
    ):
        self.id = id
        self.name = name
        self.age = age
        self.age_bracket = self._set_age_bracket(age)

    @staticmethod
    def _set_age_bracket(age: int | None) -> AgeBracket | None:
        if not age:
            return None

        if age < 13:
            return AgeBracket.CHILD

        if age >= 13 and age <= 17:
            return AgeBracket.YOUTH

        if age >= 18 and age <= 59:
            return AgeBracket.ADULT

        return AgeBracket.SENIOR


def calculate_demographcs(persons: list[Person]) -> DemographicCount:
    minors = 0
    adults = 0

    for person in persons:
        if person.age_bracket in (AgeBracket.CHILD, AgeBracket.YOUTH):
            minors += 1
        else:
            adults += 1

    return DemographicCount(minors=minors, adults=adults)


def load_csv(file_path: Path = Path("data/persons.csv")) -> list[Person]:
    persons = []
    with open(file_path) as f:
        reader = csv.DictReader(f)

        for person in reader:
            persons.append(
                Person(
                    id=int(person["id"]) if person.get("id") else None,
                    name=person["name"] if person.get("name") else None,
                    age=int(person["age"]) if person.get("age") else None,
                )
            )

    return persons


def main() -> int:
    persons = load_csv()
    demographics = calculate_demographcs(persons)
    print(demographics)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
