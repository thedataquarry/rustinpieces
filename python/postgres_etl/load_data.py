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


async def insert(conn: Connection, data: dict[str, Any]) -> None:
    await conn.execute(
        """
            INSERT INTO persons (id, name, age, isMarried, city, state, country)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        """,
        int(data["id"]),
        data["name"],
        int(data["age"]),
        bool(data["isMarried"]),
        data["city"],
        data["state"],
        data["country"],
    )


async def run() -> int:
    PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
    PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"

    conn = await asyncpg.connect(PG_URI)
    await create_tables(conn)
    data = read_data(Path("data/persons.csv"))
    for counter, record in enumerate(data, 1):
        await insert(conn, record)
    print(f"Finished loading {counter} records")

    await conn.close()
    return counter


if __name__ == "__main__":
    load_dotenv()
    _ = asyncio.run(run())
