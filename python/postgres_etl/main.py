import argparse
import asyncio
import os
import random

import asyncpg
from asyncpg.connection import Connection
from dotenv import load_dotenv


async def perf_query(conn: Connection, age_limits: list[int]) -> int:
    for count, age_limit in enumerate(age_limits, 1):
        _ = await conn.fetchval(
            """
            SELECT COUNT(*) AS count
            FROM persons WHERE age > $1
            """,
            age_limit,
        )
    return count


async def main(limit: int) -> None:
    load_dotenv()

    PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
    PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"

    conn = await asyncpg.connect(PG_URI)

    # Fix seed
    random.seed(1)
    # Test performance
    age_limits = [random.randint(22, 65) for _ in range(limit)]
    await perf_query(conn, age_limits)
    print(f"Ran {limit} async queries")

    await conn.close()


if __name__ == "__main__":
    # fmt: off
    parser = argparse.ArgumentParser()
    parser.add_argument("--number", "-n", type=int, default=1000, help="Number of queries to run")
    args = parser.parse_args()
    # fmt: on

    asyncio.run(main(args.number))
