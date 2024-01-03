use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{error::Error, fs, path::Path};

#[derive(Serialize, Deserialize)]
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

fn read_sql(path: &Path) -> Result<String, Box<dyn Error>> {
    let sql: String = fs::read_to_string(path)?.parse()?;
    println!("{}", sql);
    Ok(sql)
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

async fn get_connection_pool(pg_uri: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(pg_uri)
        .await?;
    Ok(pool)
}

async fn create_tables(
    persons_table_sql_file: &Path,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    let persons_table_sql = read_sql(persons_table_sql_file).expect("Cannot read SQL file.");
    sqlx::query(&persons_table_sql).execute(pool).await?;
    // Truncate table once it exists
    sqlx::query("TRUNCATE TABLE persons").execute(pool).await?;
    println!("Created persons table");
    Ok(())
}

async fn insert_person(person: &Person, pool: &Pool<Postgres>) {
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
    .execute(pool)
    .await
    .expect("Cannot insert data into persons table");
}

async fn run() -> Result<u32, Box<dyn Error>> {
    dotenv().ok();
    // Get files
    let persons_csv_file = Path::new("data/persons.csv");
    let persons_table_sql_file = Path::new("sql/create_persons_table.sql");
    // Create pool
    let pg_uri = format!(
        "postgres://postgres:{}@localhost:5432/etl",
        dotenv::var("POSTGRES_PASSWORD").unwrap()
    );
    let pool = get_connection_pool(&pg_uri)
        .await
        .expect("Cannot create pool.");

    let persons: Vec<Person> =
        read_data(persons_csv_file).expect("Did not obtain valid output from CSV");
    println!("Number of persons: {}", persons.len());
    // create table

    create_tables(persons_table_sql_file, &pool)
        .await
        .expect("Cannot create table in database, please check SQL files.");
    // Populate database
    let mut counter: u32 = 0;
    for person in persons.iter() {
        insert_person(person, &pool).await;
        counter += 1;
    }
    println!("Finished loading {} records", counter);
    Ok(counter)
}

#[tokio::main]
async fn main() {
    _ = run().await;
}
