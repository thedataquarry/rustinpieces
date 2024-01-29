mod errors;
mod models;
mod routes;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::routes::book;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().merge(routes::book::book_routes());
    //.route("/", get(book::get_book));

    // run our app with hyper, listening globally on port 3000
    // Using unwrap here because if these error the server doesn't start so better error handling
    // isn't really needed
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
