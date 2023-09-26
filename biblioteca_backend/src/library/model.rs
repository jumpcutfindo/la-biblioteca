use std::{convert::Infallible, fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ValueRef},
    ToSql,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub enum BookBorrowState {
    Borrowed,
    Returned,
}

impl Display for BookBorrowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookBorrowState::Borrowed => write!(f, "Borrowed"),
            BookBorrowState::Returned => write!(f, "Returned"),
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
        value
            .as_str()?
            .parse()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

pub struct BookBorrowEntry {
    pub id: Uuid,
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub state: BookBorrowState,
}

#[derive(Debug, Deserialize)]
pub struct BorrowBookRequest {
    pub user_id: Uuid,
}
