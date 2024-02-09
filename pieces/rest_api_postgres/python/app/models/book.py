from datetime import datetime
from enum import Enum
from typing import Union

from camel_converter.pydantic_base import CamelBase
from pydantic import field_validator


class BookStatus(str, Enum):
    READ = "read"
    CURRENTLY_READING = "currently_reading"
    WANT_TO_READ = "want_to_read"


class Book(CamelBase):
    title: str
    author_first_name: str
    author_last_name: str
    book_status: BookStatus
    date_added: datetime
    date_read: Union[datetime, None] = None
    rating: Union[int, None] = None

    @field_validator("rating")
    @classmethod
    def validate_rating(cls, v: Union[int, None]) -> Union[int, None]:
        if not v:
            return v

        if v < 0 or v > 5:
            raise ValueError("Rating must be between 0 and 5")

        return v


class BookInDb(Book):
    id: int
