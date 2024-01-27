from __future__ import annotations

from app.db import DatabaseManager
from app.exceptions import DeleteError, InsertError, UpdateError
from app.models.book import Book, BookInDb


async def add_book(book: Book, db: DatabaseManager) -> BookInDb:
    print(book.date_added)
    async with db.pool.acquire() as conn:
        result = await conn.execute(
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

        if result != "INSERT 0 1":
            raise InsertError("Error adding book")

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

    if not added_book:
        raise InsertError("Error adding book")

    return BookInDb(**added_book)


async def delete_book_by_id(book_id: int, db: DatabaseManager) -> None:
    async with db.pool.acquire() as conn:
        result = await conn.execute(
            """
            DELETE FROM books
            WHERE id = $1;
            """,
            book_id,
        )

        if result != "DELETE 1":
            raise DeleteError()


async def get_books(db: DatabaseManager) -> list[BookInDb] | None:
    async with db.pool.acquire() as conn:
        books = await conn.fetch(
            """
            SELECT id, title, author_first_name, author_last_name, book_status, date_added, date_read, rating
            FROM books;
            """
        )

    if not books:
        return None

    return [BookInDb(**book) for book in books]


async def update_book(book: BookInDb, db: DatabaseManager) -> BookInDb:
    async with db.pool.acquire() as conn:
        result = await conn.execute(
            """
            UPDATE books
            SET title = $1, author_first_name = $2, author_last_name = $3, book_status = $4, date_added = $5, date_read = $6, rating = $7;
            """,
            book.title,
            book.author_first_name,
            book.author_last_name,
            book.book_status,
            book.date_added,
            book.date_read,
            book.rating,
        )

        if result != "UPDATE 1":
            raise UpdateError()

        updated_book = await conn.fetchrow(
            """
            SELECT id, title, author_first_name, author_last_name, book_status, date_added, date_read, rating
            FROM books
            WHERE title = $1 and author_first_name = $2 and author_last_name = $3;
            """,
            book.title,
            book.author_first_name,
            book.author_last_name,
        )

    if not updated_book:
        raise UpdateError()

    return BookInDb(**updated_book)
