use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum BookStatus {
    Read,
    CurrentlyReading,
    WantToRead,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub title: String,
    pub author_first_name: String,
    pub author_last_name: String,
    pub book_status: BookStatus,
    pub date_added: DateTime<Utc>,
    pub date_read: Option<DateTime<Utc>>,
    pub rating: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct BookInDb {
    pub id: i32,
    pub title: String,
    pub author_first_name: String,
    pub author_last_name: String,
    pub book_status: BookStatus,
    pub date_added: DateTime<Utc>,
    pub date_read: Option<DateTime<Utc>>,
    pub rating: Option<i16>,
}

pub fn is_valid_rating(rating: &Option<i16>) -> bool {
    if let Some(r) = rating {
        if r > &5 {
            return false;
        }
    }

    true
}
