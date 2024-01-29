use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::InvalidRatingError;

#[derive(Serialize, Deserialize, Debug)]
pub enum BookStatus {
    Read,
    CurrentlyReading,
    WantToRead,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub title: String,
    pub author_first_name: String,
    pub author_last_name: String,
    pub book_status: BookStatus,
    pub date_added: DateTime<Utc>,
    pub date_read: Option<DateTime<Utc>>,
    pub rating: u8,
}

impl Book {
    pub fn new(
        title: String,
        author_first_name: String,
        author_last_name: String,
        book_status: BookStatus,
        date_added: DateTime<Utc>,
        date_read: Option<DateTime<Utc>>,
        rating: u8,
    ) -> Result<Self> {
        if !is_valid_rating(&rating) {
            bail!(InvalidRatingError());
        }

        Ok(Self {
            title,
            author_first_name,
            author_last_name,
            book_status,
            date_added,
            date_read,
            rating,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BookInDb {
    pub id: usize,
    pub title: String,
    pub author_first_name: String,
    pub author_last_name: String,
    pub book_status: BookStatus,
    pub date_added: DateTime<Utc>,
    pub date_read: Option<DateTime<Utc>>,
    pub rating: u8,
}

impl BookInDb {
    pub fn new(
        id: usize,
        title: String,
        author_first_name: String,
        author_last_name: String,
        book_status: BookStatus,
        date_added: DateTime<Utc>,
        date_read: Option<DateTime<Utc>>,
        rating: u8,
    ) -> Result<Self> {
        if !is_valid_rating(&rating) {
            bail!(InvalidRatingError());
        }

        Ok(Self {
            id,
            title,
            author_first_name,
            author_last_name,
            book_status,
            date_added,
            date_read,
            rating,
        })
    }
}

fn is_valid_rating(rating: &u8) -> bool {
    if rating > &5 {
        return false;
    }

    true
}
