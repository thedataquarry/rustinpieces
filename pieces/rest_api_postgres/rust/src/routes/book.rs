use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use sqlx::postgres::PgPool;

use crate::db::internal_error;
use crate::models::book::{Book, BookInDb, BookStatus};

pub fn book_routes(pool: PgPool) -> Router<PgPool> {
    let prefix = "/book";
    Router::new()
        .route(prefix, post(add_book))
        .route(prefix, get(get_books))
        .route(&format!("{prefix}/:book_id"), get(get_book))
        .route(&format!("{prefix}/:book_id"), delete(delete_book))
        .route(prefix, put(update_book))
        .with_state(pool)
}

async fn add_book(
    State(pool): State<PgPool>,
    Json(book): Json<Book>,
) -> (StatusCode, Json<BookInDb>) {
    let book = sqlx::query_as!(
        BookInDb,
        r#"
        INSERT INTO books (title, author_first_name, author_last_name, "book_status", date_added, date_read, rating)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, title, author_first_name, author_last_name, book_status AS "book_status: BookStatus", date_added, date_read, rating
        "#,
        book.title,
        book.author_first_name,
        book.author_last_name,
        book.book_status as BookStatus,
        book.date_added,
        book.date_read,
        book.rating,

    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(book))
}

async fn delete_book(State(pool): State<PgPool>, Path(book_id): Path<i32>) -> StatusCode {
    sqlx::query!(
        r#"
    DELETE FROM books
    WHERE id = $1
    "#,
        book_id,
    )
    .execute(&pool)
    .await
    .unwrap();

    StatusCode::NO_CONTENT
}

async fn get_books(State(pool): State<PgPool>) -> Json<Vec<BookInDb>> {
    let books = sqlx::query_as!(
        BookInDb,
        r#"
        SELECT
            id,
            title,
            author_first_name,
            author_last_name,
            book_status AS "book_status: BookStatus",
            date_added,
            date_read,
            rating
        FROM books
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap(); //map_err(internal_error);

    Json(books)
}

async fn get_book(State(pool): State<PgPool>, Path(book_id): Path<i32>) -> Json<BookInDb> {
    let book = sqlx::query_as!(
        BookInDb,
        r#"
        SELECT
            id,
            title,
            author_first_name,
            author_last_name,
            book_status AS "book_status: BookStatus",
            date_added,
            date_read,
            rating
        FROM books
        WHERE id = $1
        "#,
        book_id,
    )
    .fetch_one(&pool)
    .await
    .unwrap(); // .map_err(internal_error);

    Json(book)
}

async fn update_book(
    State(pool): State<PgPool>,
    Json(book): Json<BookInDb>,
) -> (StatusCode, Json<BookInDb>) {
    let updated_book = sqlx::query_as!(
        BookInDb,
        r#"
        UPDATE books
        SET
            title = $2,
            author_first_name = $3,
            author_last_name = $4,
            book_status = $5,
            date_added = $6,
            date_read = $7,
            rating = $8
        WHERE id = $1
        RETURNING id, title, author_first_name, author_last_name, book_status AS "book_status: BookStatus", date_added, date_read, rating
        "#,
        book.id,
        book.title,
        book.author_first_name,
        book.author_first_name,
        book.book_status as BookStatus,
        book.date_added,
        book.date_read,
        book.rating,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(updated_book))
}
