from __future__ import annotations

from asyncpg import create_pool

from app.config import config


class DatabaseManager:
    __slots__ = "pool"

    async def create_pool(self) -> None:
        self.pool = await create_pool(config.postgres_uri, min_size=10, max_size=10)

    async def close_pool(self) -> None:
        await self.pool.close()


db = DatabaseManager()
