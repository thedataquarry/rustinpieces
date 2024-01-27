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
from app.services.book_service import add_book as add_book_service
from app.services.book_service import delete_book_by_id as delete_book_by_id_service
from app.services.book_service import get_books as get_books_service
from app.services.book_service import update_book as update_book_service
from app.utils import APIRouter

router = APIRouter(tags=["Books"], prefix=f"{config.API_PREFIX}/book")


@router.get("/")
async def get_books(db: Db) -> List[BookInDb]:
    """Retrieve all books."""
    books = await get_books_service(db)

    if not books:
        raise HTTPException(HTTP_404_NOT_FOUND, detail="No books found")

    return books


@router.delete("/{book_id}", status_code=HTTP_204_NO_CONTENT)
async def delete_book_by_id(book_id: int, db: Db) -> None:
    """Delete a book by it's id."""
    await delete_book_by_id_service(book_id, db)


@router.put("/")
async def update_book(book: BookInDb, db: Db) -> BookInDb:
    """Update a book."""
    return await update_book_service(book, db)


@router.post("/")
async def add_book(book: Book, db: Db) -> BookInDb:
    """Add a new book."""
    try:
        created_user = await add_book_service(book, db)
    except UniqueViolationError:
        raise HTTPException(
            status_code=HTTP_400_BAD_REQUEST,
            detail=f"A book with the user title {book.title} already exists",
        )
    except Exception:
        raise HTTPException(
            status_code=HTTP_500_INTERNAL_SERVER_ERROR,
            detail="An error occurred while adding the book",
        )

    return created_user
