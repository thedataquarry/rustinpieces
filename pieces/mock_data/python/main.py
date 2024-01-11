from __future__ import annotations

import argparse
import csv
import random
import unicodedata
from pathlib import Path
from typing import TypedDict

from faker import Faker


class Person(TypedDict):
    id: int
    name: str
    age: int
    isMarried: bool
    city: str
    state: str
    country: str


class Location(TypedDict):
    city: str
    state: str
    country: str


def convert_unicode_to_ascii(text: str) -> str:
    text = unicodedata.normalize("NFKD", text)
    text_ascii = text.encode("ASCII", "ignore").decode("utf-8")
    return text_ascii


def get_locations(filename: Path) -> list[Location]:
    # Read city, state and country names from csv file
    locations = []
    with open(filename) as f:
        reader = csv.DictReader(f)
        for loc in reader:
            locations.append(
                Location(
                    city=loc["city_ascii"],
                    state=convert_unicode_to_ascii(loc["admin_name"]),
                    country=loc["country"],
                )
            )
    return locations


def generate_fake_persons(faker: Faker, locations: list[Location], num: int) -> list[Person]:
    # Generate fake persons with the desired structure and return a list of mappings
    profiles = []
    for i in range(1, num + 1):
        location = random.choice(locations)
        profile = Person(
            id=i,
            name=f"{faker.first_name()} {faker.last_name()}",
            age=random.randint(22, 65),
            isMarried=faker.random_element(elements=("true", "false")),
            city=location["city"],
            state=location["state"],
            country=location["country"],
        )
        profiles.append(profile)
    print(f"""Generated {num} fake profiles.""")
    return profiles


def write_persons_to_csv(profiles: list[Person], output_path: Path) -> None:
    with open(output_path, "w", newline="") as csvfile:
        fieldnames = list(profiles[0].keys())
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()
        for profile in profiles:
            writer.writerow(profile)


def main() -> None:
    # Set random seed
    random.seed(SEED)
    # Create faker object
    Faker.seed(SEED)
    faker = Faker()

    assert Path("../data/worldcities.csv").exists(), "worldcities.csv not found."
    locations = get_locations(Path("../data/worldcities.csv"))

    # Generate fake profiles
    profiles = generate_fake_persons(faker, locations, NUM)

    # Write to csv file
    write_persons_to_csv(profiles, Path("../data/persons.csv"))


if __name__ == "__main__":
    # fmt: off
    parser = argparse.ArgumentParser()
    parser.add_argument("--num", "-n", type=int, default=10, help="Number of fake profiles to generate")
    parser.add_argument("--seed", "-s", type=int, default=0, help="Random seed")
    args = parser.parse_args()
    # fmt: on

    SEED = args.seed
    NUM = args.num

    main()
