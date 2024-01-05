import os
import random

import asyncpg
import pytest
from dotenv import load_dotenv

from main import perf_query


@pytest.fixture
async def get_connection():
    load_dotenv()
    PG_PASSWORD = os.environ.get("POSTGRES_PASSWORD")
    PG_URI = f"postgres://postgres:{PG_PASSWORD}@localhost:5432/etl"
    conn = await asyncpg.connect(PG_URI)
    yield conn


async def test_summary_query(get_connection):
    conn = get_connection
    count = await conn.fetchval("SELECT COUNT(*) AS count FROM persons")
    assert count > 0


async def test_perf_query(get_connection):
    conn = get_connection
    age_limits = [random.randint(22, 65) for _ in range(1000)]
    # This is a template test: in a real situation, we'd measure more meaningful counts
    count = await perf_query(conn, age_limits)
    assert count == 1000
