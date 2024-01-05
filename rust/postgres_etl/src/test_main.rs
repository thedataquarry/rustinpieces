#![cfg(test)]

use dotenvy::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sqlx::{Connection, PgConnection};

// Get database connection for test
pub async fn get_connection() -> PgConnection {
    dotenv().ok();
    let pg_uri = dotenvy::var("DATABASE_URL").expect("Invalid DB URI");
    let conn = PgConnection::connect(&pg_uri)
        .await
        .expect("Could not connect to DB");
    conn
}

#[sqlx::test]
async fn test_summary_query() {
    let mut conn = get_connection().await;
    let query = sqlx::query!("SELECT COUNT(*) AS count FROM persons");
    let result = query
        .fetch_one(&mut conn)
        .await
        .expect("Query did not execute");
    assert!(result.count.unwrap() > 0);
}

#[sqlx::test]
async fn test_perf_query() {
    let conn = get_connection().await;
    let mut rng = StdRng::seed_from_u64(1);
    let ages = (0..1000).map(|_| rng.gen_range(22..65)).collect();
    // This is a template test: in a real situation, we'd measure more meaningful counts
    let result = super::perf_query(conn, ages)
        .await
        .expect("Query did not execute");
    assert_eq!(result, 1000);
}
