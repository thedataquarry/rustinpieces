from typing import List

from asyncpg.exceptions import UniqueViolationError
from fastapi import HTTPException
from starlette.status import (
    HTTP_204_NO_CONTENT,
    HTTP_400_BAD_REQUEST,
    HTTP_404_NOT_FOUND,
    HTTP_500_INTERNAL_SERVER_ERROR,
)

from app.config import config
from app.deps import Db
from app.models.book import Book, BookInDb
from app.utils import APIRouter

router = APIRouter(tags=["Books"], prefix=f"{config.API_PREFIX}/book")


@router.get("/")
async def get_books(db: Db) -> List[BookInDb]:
    """Retrieve all books."""
    async with db.pool.acquire() as conn:
        books = await conn.fetch(
            """
            SELECT id, title, author_first_name, author_last_name, book_status, date_added, date_read, rating
            FROM books;
            """
        )

    if not books:
        raise HTTPException(HTTP_404_NOT_FOUND, detail="No books found")

    return [BookInDb(**book) for book in books]


@router.get("/{book_id}")
async def get_book(book_id: int, db: Db) -> BookInDb:
    """Retrieve a book by it's id."""
    async with db.pool.acquire() as conn:
        book = await conn.fetchrow(
            """
            SELECT id, title, author_first_name, author_last_name, book_status, date_added, date_read, rating
            FROM books
            WHERE id = $1;
            """,
            book_id,
        )

    if not book:
        raise HTTPException(HTTP_404_NOT_FOUND, detail=f"No book with id {book_id} found")

    return BookInDb(**book)


@router.delete("/{book_id}", status_code=HTTP_204_NO_CONTENT)
async def delete_book_by_id(book_id: int, db: Db) -> None:
    """Delete a book by it's id."""
    async with db.pool.acquire() as conn:
        result = await conn.execute(
            """
            DELETE FROM books
            WHERE id = $1;
            """,
            book_id,
        )

    if result != "DELETE 1":
        raise HTTPException(HTTP_500_INTERNAL_SERVER_ERROR, detail="Error deleging book")


@router.put("/")
async def update_book(book: BookInDb, db: Db) -> BookInDb:
    """Update a book."""
    async with db.pool.acquire() as conn:
        try:
            result = await conn.execute(
                """
                UPDATE books
                SET title = $1, author_first_name = $2, author_last_name = $3, book_status = $4, date_added = $5, date_read = $6, rating = $7
                WHERE id = $8;
                """,
                book.title,
                book.author_first_name,
                book.author_last_name,
                book.book_status,
                book.date_added,
                book.date_read,
                book.rating,
                book.id,
            )
        except UniqueViolationError:
            raise HTTPException(
                status_code=HTTP_400_BAD_REQUEST,
                detail=f"A book with the title {book.title} by author {book.author_first_name} {book.author_last_name} already exists",
            )

        if result != "UPDATE 1":
            raise HTTPException(
                status_code=HTTP_500_INTERNAL_SERVER_ERROR,
                detail="An error occurred while updating the book",
            )

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
            raise HTTPException(
                status_code=HTTP_500_INTERNAL_SERVER_ERROR,
                detail="An error occurred while updating the book",
            )

    return BookInDb(**updated_book)


@router.post("/")
async def add_book(book: Book, db: Db) -> BookInDb:
    """Add a new book."""
    async with db.pool.acquire() as conn:
        try:
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
        except UniqueViolationError:
            raise HTTPException(
                status_code=HTTP_400_BAD_REQUEST,
                detail=f"A book with the title {book.title} by author {book.author_first_name} {book.author_last_name} already exists",
            )

        if result != "INSERT 0 1":
            raise HTTPException(
                status_code=HTTP_500_INTERNAL_SERVER_ERROR,
                detail="An error occurred while adding the book",
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

        if not added_book:
            raise HTTPException(
                status_code=HTTP_500_INTERNAL_SERVER_ERROR,
                detail="An error occurred while adding the book",
            )

    return BookInDb(**added_book)
