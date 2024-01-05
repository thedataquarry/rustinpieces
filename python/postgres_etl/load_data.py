import asyncio
import csv
import os
from pathlib import Path
from typing import Any

import asyncpg
from asyncpg.connection import Connection
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


async def create_tables(conn: Connection) -> None:
    await conn.execute(read_sql(Path("sql/create_persons_table.sql")))
    # Truncate table once it exists
    await conn.execute("TRUNCATE TABLE persons")


async def insert(conn: Connection, persons: list[dict[str, Any]]) -> None:
    for counter, person in enumerate(persons, 1):
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
    return counter


async def run() -> int:
    PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
    PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"

    conn = await asyncpg.connect(PG_URI)
    # Create table and truncate it once it exists
    await create_tables(conn)
    persons = read_data(Path("data/persons.csv"))
    # Insert data
    counter = await insert(conn, persons)
    print(f"Finished loading {counter} records")

    await conn.close()
    return counter


if __name__ == "__main__":
    load_dotenv()
    _ = asyncio.run(run())
