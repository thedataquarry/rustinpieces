import argparse
import asyncio
import os
import random

import asyncpg
from asyncpg.pool import Pool
from dotenv import load_dotenv


async def perf_query(pool: Pool, age_limit: int) -> int:
    async with pool.acquire() as conn:
        return await conn.fetchval(
            """
                SELECT COUNT(*) AS count
                FROM persons WHERE age > $1
                """,
            age_limit,
        )


async def main(limit: int) -> None:
    load_dotenv()

    PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
    PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"

    async with asyncpg.create_pool(PG_URI) as pool:
        # Fix seed
        random.seed(1)
        # Test performance
        tasks = []
        for age_limit in [random.randint(22, 65) for _ in range(limit)]:
            tasks.append(perf_query(pool, age_limit))
        await asyncio.gather(*tasks)
        print(f"Ran {limit} async queries")


if __name__ == "__main__":
    # fmt: off
    parser = argparse.ArgumentParser()
    parser.add_argument("--number", "-n", type=int, default=1000, help="Number of queries to run")
    args = parser.parse_args()
    # fmt: on

    asyncio.run(main(args.number))
