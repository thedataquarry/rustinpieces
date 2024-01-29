use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;

use crate::models::book::{Book, BookInDb, BookStatus};

pub fn book_routes() -> Router {
    let prefix = "/book";
    Router::new()
        .route(prefix, post(add_book))
        .route(prefix, get(get_books))
        .route(&format!("{prefix}/:book_id"), get(get_book))
        .route(&format!("{prefix}/:book_id"), delete(delete_book))
        .route(prefix, put(update_book))
}

async fn add_book(Json(book): Json<Book>) -> (StatusCode, Json<BookInDb>) {
    let created_book = BookInDb::new(
        1,
        book.title,
        book.author_first_name,
        book.author_last_name,
        book.book_status,
        book.date_added,
        book.date_read,
        book.rating,
    )
    .unwrap();

    (StatusCode::CREATED, Json(created_book))
}

async fn delete_book(Path(book_id): Path<usize>) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn get_books() -> Json<Vec<BookInDb>> {
    let book1 = BookInDb::new(
        1,
        "title".to_string(),
        "first".to_string(),
        "last".to_string(),
        BookStatus::Read,
        Utc::now(),
        None,
        3,
    )
    .unwrap();

    let book2 = BookInDb::new(
        2,
        "title2".to_string(),
        "first".to_string(),
        "last".to_string(),
        BookStatus::Read,
        Utc::now(),
        None,
        3,
    )
    .unwrap();

    Json(vec![book1, book2])
}

async fn get_book(Path(book_id): Path<usize>) -> Json<BookInDb> {
    let book = BookInDb::new(
        book_id,
        "title".to_string(),
        "first".to_string(),
        "last".to_string(),
        BookStatus::Read,
        Utc::now(),
        None,
        3,
    )
    .unwrap();

    Json(book)
}

async fn update_book(Json(book): Json<BookInDb>) -> (StatusCode, Json<BookInDb>) {
    let updated_book = BookInDb::new(
        book.id,
        book.title,
        book.author_first_name,
        book.author_last_name,
        book.book_status,
        book.date_added,
        book.date_read,
        book.rating,
    )
    .unwrap();

    (StatusCode::CREATED, Json(updated_book))
}
