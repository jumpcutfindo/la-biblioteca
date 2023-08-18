use std::{str::FromStr, convert::Infallible};

use chrono::{DateTime, Utc};
use rusqlite::{ToSql, types::{FromSql, FromSqlResult, FromSqlError, ValueRef}};
use serde::Deserialize;
use uuid::Uuid;

pub enum BookBorrowState {
    Borrowed, Returned
}

impl BookBorrowState {
    fn to_string(&self) -> String {
        match *self {
            BookBorrowState::Borrowed => String::from("Borrowed"),
            BookBorrowState::Returned => String::from("Returned"),
        }
    }
}

impl FromStr for BookBorrowState {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let "Borrowed" = s {
            Ok(BookBorrowState::Borrowed)
        } else {
            Ok(BookBorrowState::Returned)
        }
    }
}

impl ToSql for BookBorrowState {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for BookBorrowState {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str()?.parse()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

pub struct BookBorrowEntry {
    pub id: Uuid,
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub action: BookBorrowState,
}

#[derive(Debug, Deserialize)]
pub struct BorrowBookRequest {
    pub user_id: Uuid,
}