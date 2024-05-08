use dotenvy::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;

async fn get_pool(pg_uri: &str) -> Result<Arc<PgPool>, sqlx::Error> {
    let pool = Arc::new(
        PgPoolOptions::new()
            .min_connections(20)
            .max_connections(20)
            .connect(pg_uri)
            .await
            .expect("Cannot obtain connection from pool"),
    );
    Ok(pool)
}

async fn perf_query(pool: Arc<PgPool>, age: i16) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        SELECT COUNT(*) AS count
        FROM persons WHERE age > $1
        "#,
    )
    .bind(age)
    .fetch_one(&*pool)
    .await?;
    Ok(())
}

async fn get_age_limits() -> Result<Vec<i16>, sqlx::Error> {
    // Generate array of random numbers of length 100, with values between 22 and 65
    let mut rng = StdRng::seed_from_u64(1);
    // Collect args
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        // If no args are provided, length is 1 because the first arg is the program name
        1 => {
            println!("No arguments provided. Defaulting to generating 1000 random age values.");
            let ages = (0..1000).map(|_| rng.gen_range(22..65)).collect();
            Ok(ages)
        }
        _ => {
            let limit: i32 = args[1].parse::<i32>().expect("Invalid limit provided");
            let ages = (0..limit).map(|_| rng.gen_range(22..65)).collect();
            Ok(ages)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    // Obtain connection
    let pg_uri = dotenvy::var("DATABASE_URL").expect("Invalid DB URI");
    let pool = get_pool(&pg_uri).await?;
    let ages = get_age_limits().await?;
    let mut tasks = Vec::new();

    // Create async tasks to query data
    for &age in ages.iter() {
        let task = tokio::spawn(perf_query(Arc::clone(&pool), age));
        tasks.push(task);
    }
    // Run async tasks
    for task in tasks {
        _ = task.await.expect("Error running async task");
    }
    println!("Number of queries executed: {}", ages.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Row;

    // Get database connection pool for test
    pub async fn get_pool() -> Arc<PgPool> {
        dotenv().ok();
        let pg_uri = dotenvy::var("DATABASE_URL").expect("Invalid DB URI");
        Arc::new(
            PgPool::connect(&pg_uri)
                .await
                .expect("Could not connect to DB"),
        )
    }

    #[sqlx::test]
    async fn test_summary_query() {
        let pool = get_pool().await;
        let query = sqlx::query("SELECT COUNT(*) AS count FROM persons");
        let result = query
            .fetch_one(&*pool)
            .await
            .expect("Query did not execute");
        let count: i64 = result.get("count");
        println!("{count}");
        assert!(count > 0);
    }

    #[sqlx::test]
    async fn test_perf_query() {
        let pool = get_pool().await;
        let mut rng = StdRng::seed_from_u64(1);
        let ages: Vec<i16> = (0..1000).map(|_| rng.gen_range(22..65)).collect();
        // This is a template test: in a real situation, we'd measure more meaningful counts
        for age in ages {
            let result = super::perf_query(pool.clone(), age).await;
            assert!(result.is_ok());
        }
    }
}
