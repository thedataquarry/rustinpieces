use dotenvy::dotenv;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::path::Path;
use std::sync::Arc;

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

fn read_data(persons_csv_file: &Path) -> Result<Vec<Person>, csv::Error> {
    // Read from persons.csv file;
    let mut rdr = csv::Reader::from_path(persons_csv_file).expect("Cannot read persons CSV file.");
    let mut persons: Vec<Person> = Vec::new();
    for record in rdr.records() {
        let record = record.expect("Did not obtain valid record from CSV file.");
        persons.push(Person {
            id: record[0].parse::<i32>().expect("Cannot parse ID."),
            name: record[1].to_string(),
            age: record[2].parse::<i16>().expect("Cannot parse age."),
            is_married: record[3].parse::<bool>().expect("Cannot parse isMarried."),
            city: record[4].to_string(),
            state: record[5].to_string(),
            country: record[6].to_string(),
        });
    }
    Ok(persons)
}

async fn truncate_table(pool: Arc<PgPool>) -> Result<(), sqlx::Error> {
    // Truncate table
    sqlx::query("TRUNCATE TABLE persons")
        .execute(&*pool)
        .await?;
    println!("Created persons table");
    Ok(())
}

async fn insert(person: Person, pool: Arc<PgPool>) {
    // Populate database
    sqlx::query(
        r#"
            INSERT INTO persons (id, name, age, isMarried, city, state, country)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
    )
    .bind(person.id)
    .bind(&person.name)
    .bind(person.age)
    .bind(person.is_married)
    .bind(&person.city)
    .bind(&person.state)
    .bind(&person.country)
    .execute(&*pool)
    .await
    .expect("Cannot insert data into persons table");
}

async fn run() -> Result<usize, sqlx::Error> {
    dotenv().ok();
    // Get files
    let persons_csv_file = Path::new("../data/persons.csv");
    let persons: Vec<Person> =
        read_data(persons_csv_file).expect("Did not obtain valid output from CSV");
    let counter = persons.len();
    println!("Number of persons: {}", counter);

    // Obtain connection pool
    let pg_uri = dotenvy::var("DATABASE_URL").expect("Invalid DB URI");
    let pool = get_pool(&pg_uri).await?;

    // Truncate table
    truncate_table(Arc::clone(&pool)).await?;

    // Run async data load
    let mut tasks = Vec::new();
    // Create async tasks to insert data
    for person in persons.into_iter() {
        let task = tokio::spawn(insert(person, Arc::clone(&pool)));
        tasks.push(task);
    }
    // Run async tasks
    for task in tasks {
        task.await.expect("Error running async task");
    }
    println!("Finished loading {:?} records", counter);
    Ok(counter)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    _ = run().await;
}
