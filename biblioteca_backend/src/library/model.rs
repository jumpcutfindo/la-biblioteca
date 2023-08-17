use std::{str::FromStr, convert::Infallible};

use rusqlite::{ToSql, types::{FromSql, FromSqlResult, FromSqlError, ValueRef}};
use serde::Deserialize;
use uuid::Uuid;

pub enum BookState {
    Borrowed, Returned
}

impl BookState {
    fn to_string(&self) -> String {
        match *self {
            BookState::Borrowed => String::from("Borrowed"),
            BookState::Returned => String::from("Returned"),
        }
    }
}

impl FromStr for BookState {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let "Borrowed" = s {
            Ok(BookState::Borrowed)
        } else {
            Ok(BookState::Returned)
        }
    }
}

impl ToSql for BookState {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for BookState {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str()?.parse()
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

#[derive(Debug, Deserialize)]
pub struct BorrowBookRequest {
    pub user_id: Uuid,
}