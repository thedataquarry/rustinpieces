from typing import Annotated

from asyncpg import Pool
from fastapi import Depends

from app.config import Settings, config
from app.db import DatabaseManager, db


def get_config() -> Settings:
    return config


def get_db() -> DatabaseManager:
    return db


Config = Annotated[Settings, Depends(get_config)]
Db = Annotated[Pool, Depends(get_db)]
