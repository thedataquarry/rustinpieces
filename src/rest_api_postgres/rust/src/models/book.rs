use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type, PartialEq, Eq)]
#[sqlx(rename_all = "snake_case")]
pub enum BookStatus {
    Read,
    CurrentlyReading,
    WantToRead,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, PartialEq, Eq)]
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
        if !(&0..=&5).contains(&r) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ratings() {
        let ratings = vec![Some(0), Some(1), Some(2), Some(3), Some(4), Some(5)];

        for rating in ratings {
            assert!(is_valid_rating(&rating));
        }
    }

    #[test]
    fn test_invalid_ratings() {
        let ratings = vec![Some(-1), Some(6)];

        for rating in ratings {
            assert!(!is_valid_rating(&rating));
        }
    }
}
