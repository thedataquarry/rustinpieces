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
    let pool = create_pool(&pg_uri)
        .await
        .expect("Unable to connect to the database");
    let app = Router::new()
        .merge(routes::book::book_routes(pool.clone()))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
