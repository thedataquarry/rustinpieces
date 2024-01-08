from __future__ import annotations

import csv
from enum import Enum
from pathlib import Path


class AgeBracket(Enum):
    CHILD = "child"
    YOUTH = "youth"
    ADULT = "adult"
    SENIOR = "senior"


class Person:
    def __init__(
        self,
        id: int | None,
        age: int | None,
        is_married: bool | None,
        city: str | None,
        state: str | None,
        country: str | None,
    ):
        self.id = id
        self.age = age
        self.is_married = is_married
        self.city = city
        self.state = state
        self.country = country
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


def load_csv(file_path: Path = Path("data/persons.csv")) -> list[Person]:
    persons = []
    with open(file_path) as f:
        reader = csv.DictReader(f)

        for person in reader:
            if not person.get("isMarried"):
                is_married = None
            else:
                is_married = True if person["isMarried"].lower() == "true" else False

            persons.append(
                Person(
                    id=int(person["id"]) if person.get("id") else None,
                    age=int(person["age"]) if person.get("age") else None,
                    is_married=is_married,
                    city=person.get("city"),
                    state=person.get("state"),
                    country=person.get("country"),
                )
            )

    return persons


def main() -> int:
    persons = load_csv()
    print(persons[0].__dict__)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
