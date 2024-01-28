from datetime import UTC, datetime
from uuid import uuid4

import pytest
from httpx import AsyncClient

from app.config import config
from app.db import db
from app.main import app
from app.models.book import Book, BookStatus
from app.services.book_service import add_book


@pytest.fixture(autouse=True)
async def prep_db():
    await db.create_pool()
    yield

    async with db.pool.acquire() as conn:
        await conn.execute(
            """
           DELETE FROM books;
           """,
        )
    await db.close_pool()


@pytest.fixture
async def test_client():
    async with AsyncClient(app=app, base_url=f"http://127.0.0.1{config.API_PREFIX}") as client:
        yield client


@pytest.fixture
def book():
    return Book(
        title=str(uuid4()),
        author_first_name="Imma",
        author_last_name="Author",
        book_status=BookStatus.READ,
        date_added=datetime.now(tz=UTC),
        date_read=datetime.now(tz=UTC),
        rating=3,
    )


@pytest.fixture
def book_json(book):
    book_json = book.model_dump(by_alias=True)
    book_json["bookStatus"] = book_json["bookStatus"].value
    book_json["dateAdded"] = book_json["dateAdded"].isoformat().replace("+00:00", "Z")
    book_json["dateRead"] = book_json["dateRead"].isoformat().replace("+00:00", "Z")

    return book_json


@pytest.fixture
async def book_in_db(book):
    book = await add_book(book, db)
    yield book
