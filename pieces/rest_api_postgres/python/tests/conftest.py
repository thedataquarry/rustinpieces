from datetime import UTC, datetime
from uuid import uuid4

import pytest
from httpx import AsyncClient

from app.config import config
from app.db import db
from app.main import app
from app.models.book import Book, BookInDb, BookStatus


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
    async with db.pool.acquire() as conn:
        await conn.execute(
            """
            INSERT INTO books(title, author_first_name, author_last_name, book_status, date_added, date_read, rating)
            VALUES ($1, $2, $3, $4, $5, $6, $7);
            """,
            book.title,
            book.author_first_name,
            book.author_last_name,
            book.book_status,
            book.date_added,
            book.date_read,
            book.rating,
        )

        added_book = await conn.fetchrow(
            """
            SELECT id, title, author_first_name, author_last_name, book_status, date_added, date_read, rating
            FROM books
            WHERE title = $1 and author_first_name = $2 and author_last_name = $3;
            """,
            book.title,
            book.author_first_name,
            book.author_last_name,
        )

    yield BookInDb(**added_book)
