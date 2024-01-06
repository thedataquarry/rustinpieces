#![cfg(test)]

use dotenvy::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sqlx::{PgPool, postgres::PgPoolOptions};

// Get database connection pool for test
pub async fn get_pool() -> PgPool {
    dotenv().ok();
    let pg_uri = dotenvy::var("DATABASE_URL").expect("Invalid DB URI");
    PgPoolOptions::new().min_connections(5).max_connections(5).connect(&pg_uri)
        .await
        .expect("Could not connect to DB")
}

#[sqlx::test]
async fn test_summary_query() {
    let pool = get_pool().await;
    let query = sqlx::query!("SELECT COUNT(*) AS count FROM persons");
    let result = query
        .fetch_one(&pool)
        .await
        .expect("Query did not execute");
    assert!(result.count.unwrap() > 0);
}

#[sqlx::test]
async fn test_perf_query() {
    let pool = get_pool().await;
    let mut rng = StdRng::seed_from_u64(1);
    let ages: Vec<i16> = (0..1000).map(|_| rng.gen_range(22..65)).collect();
    // This is a template test: in a real situation, we'd measure more meaningful counts
    for age in ages {
        let result = super::perf_query(pool.clone(), age)
            .await;
        assert!(result.is_ok());
    }
}
