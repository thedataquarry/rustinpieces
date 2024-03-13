from typing import Any, Final

from pydantic import model_validator
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    API_PREFIX: Final[str] = "/api"

    postgres_password: str
    postgres_port: int = 5432
    postgres_uri: str
    model_config = SettingsConfigDict(env_file=".env", case_sensitive=False)

    @model_validator(mode="before")
    @classmethod
    def set_postgres_uri(cls, data: Any) -> str:
        if isinstance(data, dict):
            if not data.get("postgres_password"):
                raise ValueError("A POSTGRES_PASSWORD environment variable is required")

            port = data["postgres_port"] if data.get("postgres_port") else 5432
            data["postgres_uri"] = (
                f"postgres://postgres:{data['postgres_password']}@127.0.0.1:{port}/api"
            )

        return data


config = Settings()
