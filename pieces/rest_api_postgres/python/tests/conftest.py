from uuid import uuid4

import pytest
from asyncpg import create_pool
from httpx import AsyncClient

from app.config import config
from app.main import app
from app.models.user import UserCreate
from app.services.user_service import create_user, delete_user


@pytest.fixture
async def test_client():
    async with AsyncClient(app=app, base_url=f"http://127.0.0.1{config.API_PREFIX}") as client:
        yield client


@pytest.fixture
async def user():
    pool = await create_pool(config.postgres_uri, min_size=1, max_size=1)
    user = await create_user(UserCreate(user_name=str(uuid4()), password="test"))
    yield user
    await delete_user(user.id)
    await pool.close()
