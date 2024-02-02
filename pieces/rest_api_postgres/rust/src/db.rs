use std::time::Duration;

use anyhow::Result;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use sqlx::{
    pool::PoolConnection,
    postgres::{PgPool, PgPoolOptions},
    Postgres,
};

/// Based on the axum sqlx-postgres example
/// https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs
struct DbManager(PoolConnection<Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DbManager
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub async fn create_pool(pg_uri: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(pg_uri)
        .await?;

    Ok(pool)
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
