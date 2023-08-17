use std::error::Error;

use axum::extract::State;
use rusqlite::Result;
use uuid::Uuid;
use chrono::prelude::*;

use crate::AppState;

use super::{model::BookBorrowState, error::LibraryError};

pub async fn add_borrow_entry_to_db(
    State(state): State<AppState>,
    user_id: Uuid,
    book_id: Uuid,
) -> Result<(), LibraryError> {
    let conn = state.db_pool.get().unwrap();

    // Check if the book is currently borrowed
    match conn.query_row(
        "SELECT * FROM map_users_to_borrowed_books WHERE book_id = ?1 ORDER BY timestamp",
        [book_id], 
        |row| {
            row.get(3)
        }) {
            Ok::<BookBorrowState, _>(latest_entry) => {
                // If entry exists, check if it's "Borrowed"
                match latest_entry {
                    BookBorrowState::Borrowed => return Err(LibraryError::BookBorrowed),
                    BookBorrowState::Returned => {},
                }
            },
            Err(err) => {
                // If entry doesn't exist, we continue
                match err {
                    rusqlite::Error::QueryReturnedNoRows => {},
                    _ => return Err(LibraryError::DatabaseError(err))
                }
            }
        }

    match conn.execute(
        "INSERT INTO map_users_to_borrowed_books (user_id, book_id, timestamp, action) VALUES (?1, ?2, ?3, ?4)",
        (user_id, book_id, Utc::now(), BookBorrowState::Borrowed),
    ) {
        Ok(_) => return Ok(()),
        Err(err) => {
            match err.sqlite_error_code().unwrap() {
                rusqlite::ErrorCode::ConstraintViolation => return Err(LibraryError::ResourceNotExists),
                _ => return Err(LibraryError::DatabaseError(err)),
            }
        },
    };
}

pub async fn add_return_entry_to_db() {

}

pub async fn get_latest_book_state_from_db() {
    // Takes the latest entry from the table that stores the past
    // book states
}