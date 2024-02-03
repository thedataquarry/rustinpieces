use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use crate::models::book::{is_valid_rating, Book, BookInDb, BookStatus};

#[derive(Deserialize, Serialize, Debug)]
struct GenericMessage {
    detail: String,
}

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

async fn add_book(State(pool): State<PgPool>, Json(book): Json<Book>) -> Response {
    if !is_valid_rating(&book.rating) {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericMessage {
                detail: format!(
                    "{:?} is not a valid rating. Ratings must be between 0 and 5",
                    book.rating
                ),
            }),
        )
            .into_response();
    }
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
    .await;

    match book {
        Ok(b) => (StatusCode::CREATED, Json(b)).into_response(),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.code().map(|code| code == "23505").unwrap_or(false) {
                (
                    StatusCode::BAD_REQUEST,
                    Json(GenericMessage {
                        detail: "Book already exists".to_string(),
                    }),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(GenericMessage {
                        detail: "An error occurred while adding the record".to_string(),
                    }),
                )
                    .into_response()
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenericMessage {
                detail: "An error occurred while adding the record".to_string(),
            }),
        )
            .into_response(),
    }
}

async fn delete_book(State(pool): State<PgPool>, Path(book_id): Path<i32>) -> StatusCode {
    let result = sqlx::query!(
        r#"
    DELETE FROM books
    WHERE id = $1
    "#,
        book_id,
    )
    .execute(&pool)
    .await;

    match result {
        Ok(r) => {
            if r.rows_affected() > 0 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn get_books(State(pool): State<PgPool>) -> Response {
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
    .await;

    match books {
        Ok(b) => (StatusCode::OK, Json(b)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenericMessage {
                detail: "Error getting records".to_string(),
            }),
        )
            .into_response(),
    }
}

async fn get_book(State(pool): State<PgPool>, Path(book_id): Path<i32>) -> Response {
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
    .fetch_optional(&pool)
    .await;

    if let Ok(book_result) = book {
        match book_result {
            Some(b) => (StatusCode::OK, Json(b)).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json(GenericMessage {
                    detail: format!("No record with the id {book_id} found"),
                }),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenericMessage {
                detail: "An error occurred while retrieving the record".to_string(),
            }),
        )
            .into_response()
    }
}

async fn update_book(State(pool): State<PgPool>, Json(book): Json<BookInDb>) -> Response {
    if !is_valid_rating(&book.rating) {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericMessage {
                detail: format!(
                    "{:?} is not a valid rating. Ratings must be between 0 and 5",
                    book.rating
                ),
            }),
        )
            .into_response();
    }
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
        book.author_last_name,
        book.book_status as BookStatus,
        book.date_added,
        book.date_read,
        book.rating,
    )
    .fetch_one(&pool)
    .await;

    match updated_book {
        Ok(b) => (StatusCode::CREATED, Json(b)).into_response(),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.code().map(|code| code == "23505").unwrap_or(false) {
                (
                    StatusCode::BAD_REQUEST,
                    Json(GenericMessage {
                        detail: "Book already exists".to_string(),
                    }),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(GenericMessage {
                        detail: "An error occurred while updating the record".to_string(),
                    }),
                )
                    .into_response()
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(GenericMessage {
                detail: "An error occurred while updating the record".to_string(),
            }),
        )
            .into_response(),
    }
}
