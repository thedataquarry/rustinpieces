mod db;
mod errors;
mod models;
mod routes;

use axum::Router;
use dotenvy::dotenv;

use crate::db::create_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pg_uri = dotenvy::var("DATABASE_URL").expect("Invalid DB URI");
    let app = app(&pg_uri).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn app(pg_uri: &str) -> Router {
    let pool = create_pool(pg_uri)
        .await
        .expect("Unable to connect to the database");

    Router::new()
        .merge(routes::book::book_routes(pool.clone()))
        .with_state(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use chrono::prelude::*;
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use tower::ServiceExt; // for `oneshot`
    use uuid::Uuid;

    use crate::models::book::{BookInDb, BookStatus};

    fn pg_uri() -> String {
        dotenv().ok();
        dotenvy::var("DATABASE_URL").expect("Invalid DB URI")
    }

    async fn mock_book() -> BookInDb {
        let pool = create_pool(&pg_uri()).await.unwrap();
        sqlx::query_as!(
            BookInDb,
            r#"
            INSERT INTO books (title, author_first_name, author_last_name, "book_status", date_added, date_read, rating)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, title, author_first_name, author_last_name, book_status AS "book_status: BookStatus", date_added, date_read, rating
            "#,
            Uuid::new_v4().to_string(),
            "Duglas",
            "Adams",
            BookStatus::Read as BookStatus,
            Utc.with_ymd_and_hms(2024, 2, 2, 22, 2, 0).unwrap(),
            Utc.with_ymd_and_hms(2024, 2, 2, 22, 0, 0).unwrap(),
            5,

        )
        .fetch_one(&pool)
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_add_book() {
        let app = app(&pg_uri()).await;

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/book")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!(
                            {
                                "title": Uuid::new_v4().to_string(),
                                "authorFirstName": "First",
                                "authorLastName": "Last",
                                "bookStatus": "WantToRead",
                                "dateAdded": "2024-02-02T22:02:00Z",
                                "dateRead": null,
                                "rating": null
                            }
                        ))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_add_book_duplicate() {
        let book = mock_book().await;
        let app = app(&pg_uri()).await;

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/book")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!(
                            {
                                "title": book.title,
                                "authorFirstName": book.author_first_name,
                                "authorLastName": book.author_last_name,
                                "bookStatus": "WantToRead",
                                "dateAdded": "2024-02-02T22:02:00Z",
                                "dateRead": null,
                                "rating": null
                            }
                        ))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_book() {
        let book = mock_book().await;
        let app = app(&pg_uri()).await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/book/{}", book.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(book));
    }

    #[tokio::test]
    async fn test_get_book_not_found() {
        let app = app(&pg_uri()).await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/book/9999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_books() {
        let book = mock_book().await;
        let app = app(&pg_uri()).await;

        let response = app
            .oneshot(Request::builder().uri("/book").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Vec<BookInDb> = serde_json::from_slice(&body).unwrap();
        assert!(body.contains(&book));
    }

    #[tokio::test]
    async fn test_delete_book() {
        let book = mock_book().await;
        let app = app(&pg_uri()).await;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/book/{}", book.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let check = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/book/{}", book.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(check.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_book() {
        let mut book = mock_book().await;
        let app = app(&pg_uri()).await;

        book.title = Uuid::new_v4().to_string();

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri("/book")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!(
                            {
                                "id": book.id,
                                "title": book.title,
                                "authorFirstName": book.author_first_name,
                                "authorLastName": book.author_last_name,
                                "bookStatus": book.book_status,
                                "dateAdded": book.date_added,
                                "dateRead": book.date_read,
                                "rating": book.rating
                            }
                        ))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let check = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/book/{}", book.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = check.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(book));
    }

    #[tokio::test]
    async fn test_update_book_duplicate() {
        let mut book1 = mock_book().await;
        let book2 = mock_book().await;
        let app = app(&pg_uri()).await;

        book1.title = book2.title;
        book1.author_first_name = book2.author_first_name;
        book1.author_last_name = book2.author_last_name;

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri("/book")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!(
                            {
                                "id": book1.id,
                                "title": book1.title,
                                "authorFirstName": book1.author_first_name,
                                "authorLastName": book1.author_last_name,
                                "bookStatus": book1.book_status,
                                "dateAdded": book1.date_added,
                                "dateRead": book1.date_read,
                                "rating": book1.rating
                            }
                        ))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
