use dotenvy::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

mod test_main;

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
    // let pool = Arc::new(PgPool::connect(&pg_uri).await.unwrap());
    let pool = PgPoolOptions::new().min_connections(5).max_connections(5).connect(&pg_uri).await?;

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
