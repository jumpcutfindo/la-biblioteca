use std::error::Error;

use axum::extract::State;
use rusqlite::Result;
use uuid::Uuid;
use chrono::prelude::*;

use crate::AppState;

use super::{model::{BookBorrowState, BookBorrowEntry}, error::LibraryError};

pub async fn add_borrow_entry_to_db(
    State(state): State<AppState>,
    user_id: Uuid,
    book_id: Uuid,
) -> Result<(), LibraryError> {
    let conn = state.db_pool.get().unwrap();

    // Check if the book is currently borrowed
    match get_latest_book_entry_from_db(State(state), book_id) {
        Ok(entry) => {
            let latest_entry = entry.action;

            // If entry exists, check if it's "Borrowed"
            match latest_entry {
                BookBorrowState::Borrowed => return Err(LibraryError::BookAlreadyBorrowed),
                BookBorrowState::Returned => {},
            }
        },
        Err(err) => {
            // If entry doesn't exist, we continue
            match err {
                rusqlite::Error::QueryReturnedNoRows => {},
                _ => return Err(LibraryError::DatabaseError(err))
            }
        },
    }

    match conn.execute(
        "INSERT INTO map_users_to_borrowed_books (id, user_id, book_id, timestamp, action) VALUES (?1, ?2, ?3, ?4, ?5)",
        (Uuid::new_v4(), user_id, book_id, Utc::now(), BookBorrowState::Borrowed),
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

pub async fn add_return_entry_to_db(
    State(state): State<AppState>,
    user_id: Uuid,
    book_id: Uuid,
) -> Result<(), LibraryError> {
    let conn = state.db_pool.get().unwrap();

    let mut borrower_id = Uuid::nil();
    let mut entry_id = Uuid::nil();

    // Check if the book is currently returned
    match get_latest_book_entry_from_db(State(state), book_id) {
        Ok(entry) => {
            let latest_entry = entry.action;

            // If entry exists, check if it's "Returned"
            match latest_entry {
                BookBorrowState::Borrowed => {},
                BookBorrowState::Returned => return Err(LibraryError::BookAlreadyReturned),
            }

            borrower_id = entry.user_id;
            entry_id = entry.id;
        },
        Err(err) => {
            // If entry doesn't exist, we continue
            match err {
                rusqlite::Error::QueryReturnedNoRows => {},
                _ => return Err(LibraryError::DatabaseError(err))
            }
        },
    }

    //  Check whether the borrower is the same user
    if user_id != borrower_id {
        return Err(LibraryError::BookNotBorrowedByUser)
    }

    match conn.execute(
        "INSERT INTO map_users_to_borrowed_books (id, user_id, book_id, timestamp, action) VALUES (?1, ?2, ?3, ?4, ?5)",
        (entry_id, user_id, book_id, Utc::now(), BookBorrowState::Returned),
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

pub fn get_latest_book_entry_from_db(
    State(state): State<AppState>,
    book_id: Uuid,
) -> Result<BookBorrowEntry, rusqlite::Error> {
    let conn = state.db_pool.get().unwrap();

    match conn.query_row(
        "SELECT * FROM map_users_to_borrowed_books WHERE book_id = ?1 ORDER BY timestamp DESC",
        [book_id], 
        |row| {
            Ok(BookBorrowEntry {
                id: row.get(0)?,
                user_id: row.get(1)?,
                book_id: row.get(2)?,
                timestamp: row.get(3)?,
                action: row.get(4)?,
            })
        }) {
            Ok(entry) => Ok(entry),
            Err(err) => Err(err),
        }
}