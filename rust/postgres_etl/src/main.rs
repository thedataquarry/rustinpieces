use dotenvy::dotenv;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;
use sqlx::{Connection, PgConnection};

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

async fn perf_query(mut conn: PgConnection, ages: Vec<i16>) -> i32 {
    let mut count = 0;
    for age in ages {
        let query = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM persons WHERE age > $1
            "#,
            age
        );
        _ = query.fetch_one(&mut conn).await.unwrap();
        count += 1;
    }
    count
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    // Obtain connection
    let pg_uri = dotenvy::var("DATABASE_URL").unwrap();
    let conn = PgConnection::connect(&pg_uri).await.unwrap();

    let ages = get_age_limits().await;
    perf_query(conn, ages).await;
    Ok(())
}
