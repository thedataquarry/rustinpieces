from __future__ import annotations

import argparse
import csv
import random
import unicodedata
from multiprocessing import Process, Queue
from pathlib import Path
from time import sleep
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


def generate_data(faker: Faker, num: int, output_path: Path, message_queue: Queue) -> None:
    file_path = Path("../data/worldcities.csv")

    try:
        if not file_path.exists():
            raise ValueError("worldcities.csv not found.")

        locations = get_locations(file_path)
        profiles = generate_fake_persons(faker, locations, num)
        write_persons_to_csv(profiles, output_path)
    finally:
        message_queue.put("done")


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
    return profiles


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


def write_persons_to_csv(profiles: list[Person], output_path: Path) -> None:
    with open(output_path, "w", newline="") as csvfile:
        fieldnames = list(profiles[0].keys())
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()
        for profile in profiles:
            writer.writerow(profile)


def spinner(message_queue: Queue) -> None:
    rate = 0.2
    while True:
        try:
            message = message_queue.get_nowait()
        except Exception:
            message = None

        if message == "done":
            break

        print("Generating data |", end="\r")
        sleep(rate)
        print("Generating data /", end="\r")
        sleep(rate)
        print("Generating data -", end="\r")
        sleep(rate)
        print("Generating data \\", end="\r")
        sleep(rate)


def main() -> int:
    # Set random seed
    random.seed(SEED)
    # Create faker object
    Faker.seed(SEED)
    faker = Faker()
    message_queue: Queue[str] = Queue()
    output_path = Path("../data/persons.csv")

    data_generation_process = Process(
        target=generate_data,
        args=(
            faker,
            NUM,
            output_path,
            message_queue,
        ),
    )
    data_generation_process.start()
    spinner_process = Process(target=spinner, args=(message_queue,))
    spinner_process.start()

    data_generation_process.join()
    spinner_process.join()

    print(f"""Generated {NUM} fake profiles.""")

    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--num", "-n", type=int, default=1000, help="Number of fake profiles to generate"
    )
    parser.add_argument("--seed", "-s", type=int, default=0, help="Random seed")
    args = parser.parse_args()

    SEED = args.seed
    NUM = args.num

    raise SystemExit(main())
