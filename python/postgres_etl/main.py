import asyncio
import csv
import json
import os
import random
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


async def summary_query(conn: Connection) -> None:
    count = await conn.fetchval("SELECT COUNT(*) FROM persons")
    data = await conn.fetch("SELECT * FROM persons limit 10")
    for record in data[:2]:
        print(json.dumps(dict(record), indent=4))
    print(f"Total records: {count}")


async def perf_query(conn: Connection, age_limits: list[int]) -> None:
    for age_limit in age_limits:
        _ = await conn.fetchval(
            """
            SELECT COUNT(*) AS count
            FROM persons WHERE age > $1
            """,
            age_limit
        )


async def main() -> None:
    conn = await asyncpg.connect(PG_URI)
    await create_tables(conn)
    data = read_data(Path("data/persons.csv"))
    for row in data:
        await conn.execute(
            """
                INSERT INTO persons (id, name, age, isMarried, city, state, country)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            """,
            int(row["id"]),
            row["name"],
            int(row["age"]),
            bool(row["isMarried"]),
            row["city"],
            row["state"],
            row["country"],
        )
    print("Data loaded successfully!")

    # Get summary of dataset
    await summary_query(conn)

    # Test performance
    age_limits = [random.randint(22, 65) for _ in range(1000)]
    await perf_query(conn, age_limits)

    await conn.close()


if __name__ == "__main__":
    load_dotenv()
    # Fix seed
    random.seed(1)

    PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
    PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"

    asyncio.run(main())
