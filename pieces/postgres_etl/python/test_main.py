import os
import random

import asyncpg
import pytest
from dotenv import load_dotenv

from main import perf_query


@pytest.fixture
async def get_pool():
    load_dotenv()
    PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
    PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"
    pool = await asyncpg.create_pool(PG_URI, min_size=5, max_size=5)
    yield pool
    await pool.close()


async def test_summary_query(get_pool):
    pool = get_pool
    async with pool.acquire() as conn:
        count = await conn.fetchval("SELECT COUNT(*) AS count FROM persons")
    assert count > 0


@pytest.mark.parametrize("age_limit, expected", ((22, 10), (65, 0)))
async def test_perf_query(age_limit, expected, get_pool):
    pool = get_pool
    # This is a template test: in a real situation, we'd measure more meaningful counts
    count = await perf_query(pool, age_limit)
    assert count == expected
