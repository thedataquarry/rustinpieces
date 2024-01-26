import logging
from typing import Any, Final

from pydantic import model_validator
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    API_PREFIX: Final[str] = "/api"

    # 60 minutes * 24 hours * 8 days = 8 days
    ACCESS_TOKEN_EXPIRE_MINUTES: Final[int] = 60 * 24 * 8
    SECRET_KEY: Final[str] = "92d287efde6adf42fe3f4f73eb987472642e085f0a2e88b0ad4640f53d169a0f"

    postgres_password: str
    postgres_port: int = 5432
    postgres_uri: str
    log_level: int = logging.INFO
    model_config = SettingsConfigDict(env_file=".env", case_sensitive=False)

    @model_validator(mode="before")
    @classmethod
    def set_postgres_uri(cls, data: Any) -> str:
        if isinstance(data, dict):
            if not data.get("postgres_password"):
                raise ValueError("A POSTGRES_PASSWORD environment variable is required")

            port = data["postgres_port"] if data.get("postgres_port") else 5432
            data[
                "postgres_uri"
            ] = f"postgres://postgres:{data['postgres_password']}@127.0.0.1:{port}/api"

        return data


config = Settings()
