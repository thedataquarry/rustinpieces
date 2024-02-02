mod db;
mod errors;
mod models;
mod routes;

use std::time::Duration;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

use crate::db::create_pool;
use crate::routes::book;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pg_uri = dotenvy::var("DATABASE_URL").expect("Invalid DB URI");
    let pool = create_pool(&pg_uri).await.unwrap();
    // .expect("Unable to connect to the database");
    let app = Router::new()
        .merge(routes::book::book_routes(pool.clone()))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    // Using unwrap here because if these error the server doesn't start so better error handling
    // isn't really needed
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
