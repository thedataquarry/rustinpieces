use dotenvy::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    id: i32,
    name: String,
    age: i16,
    is_married: bool,
    city: String,
    state: String,
    country: String,
}

async fn get_age_limits() -> Vec<i16> {
    // Generate array of random numbers of length 100, with values between 22 and 65
    let mut rng = StdRng::seed_from_u64(1);
    // Collect args
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        // If no args are provided, length is 1 because the first arg is the program name
        1 => {
            println!("No arguments provided. Defaulting to generating 1000 random age values.");
            (0..1000).map(|_| rng.gen_range(22..65)).collect()
        }
        _ => {
            let limit: i32 = args[1].parse::<i32>().unwrap();
            (0..limit).map(|_| rng.gen_range(22..65)).collect()
        }
    }
}

async fn perf_query(pool: PgPool, age: i16) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
        SELECT COUNT(*) AS count
        FROM persons WHERE age > $1
        "#,
        age
    );
    query.fetch_one(&pool).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    // Obtain connection
    let pg_uri = dotenvy::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(5)
        .connect(&pg_uri)
        .await?;

    let ages = get_age_limits().await;
    let mut tasks = Vec::new();

    for &age in ages.iter() {
        let task = tokio::spawn(perf_query(pool.clone(), age));
        tasks.push(task);
    }
    for task in tasks {
        _ = task.await.expect("Error running task");
    }
    pool.close().await;
    println!("Number of queries executed: {}", ages.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Get database connection pool for test
    pub async fn get_pool() -> PgPool {
        dotenv().ok();
        let pg_uri = dotenvy::var("DATABASE_URL").expect("Invalid DB URI");
        PgPoolOptions::new()
            .min_connections(5)
            .max_connections(5)
            .connect(&pg_uri)
            .await
            .expect("Could not connect to DB")
    }

    #[sqlx::test]
    async fn test_summary_query() {
        let pool = get_pool().await;
        let query = sqlx::query!("SELECT COUNT(*) AS count FROM persons");
        let result = query.fetch_one(&pool).await.expect("Query did not execute");
        assert!(result.count.unwrap() > 0);
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
