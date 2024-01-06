import asyncio
import csv
import os
from pathlib import Path
from typing import Any

import asyncpg
from asyncpg.pool import Pool
from dotenv import load_dotenv


def read_sql(filename: Path) -> str:
    with open(filename) as f:
        sql = f.read()
    return sql


def read_data(filename: Path) -> list[dict[str, Any]] | None:
    with open(filename) as f:
        reader = csv.DictReader(f)
        data = list(reader)
    return data


async def insert(pool: Pool, person: dict[str, Any]) -> None:
    async with pool.acquire() as conn:
        await conn.execute(
            """
                INSERT INTO persons (id, name, age, isMarried, city, state, country)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            """,
            int(person["id"]),
            person["name"],
            int(person["age"]),
            bool(person["isMarried"]),
            person["city"],
            person["state"],
            person["country"],
        )


async def run() -> int:
    PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
    PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"

    persons = read_data(Path("data/persons.csv"))
    if not persons:
        raise ValueError("No people found")
    counter = len(persons)

    # Insert data
    tasks = []
    async with asyncpg.create_pool(PG_URI) as pool:
        for person in persons:
            tasks.append(insert(pool, person))

        await asyncio.gather(*tasks)
    print(f"Finished loading {counter} records")

    return counter


if __name__ == "__main__":
    load_dotenv()
    _ = asyncio.run(run())
