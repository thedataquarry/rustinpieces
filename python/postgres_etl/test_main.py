import os
import random

import asyncio
import asyncpg
import pytest
from dotenv import load_dotenv

from load_data import run as loader
from main import perf_query, summary_query


load_dotenv()
PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"


async def test_main():
    counter = await loader()
    assert counter == 1000


async def test_summary_query():
    conn = await asyncpg.connect(PG_URI)
    count = await summary_query(conn)
    assert count == 1000


async def test_perf_query():
    conn = await asyncpg.connect(PG_URI)
    age_limits = [random.randint(22, 65) for _ in range(1000)]
    # This is a template test: in a real situation, we'd measure more meaningful counts
    count = await perf_query(conn, age_limits)
    assert count == 1000
