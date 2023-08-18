use std::error::Error;

use axum::extract::State;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
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
    match get_latest_book_entry_from_db(state.db_pool.get().unwrap(), book_id) {
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

    // Check if the user can still borrow books
    let num_borrowed = get_num_borrowed_from_db(state.db_pool.get().unwrap(), user_id)?;
    let num_max_borrowable = get_num_user_can_borrow_from_db(state.db_pool.get().unwrap(), user_id)?;
    if num_borrowed > num_max_borrowable  {
        return Err(LibraryError::NumBorrowableExceeded(num_max_borrowable))
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
    match get_latest_book_entry_from_db(state.db_pool.get().unwrap(), book_id) {
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
    conn: PooledConnection<SqliteConnectionManager>,
    book_id: Uuid,
) -> Result<BookBorrowEntry, rusqlite::Error> {
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

pub fn get_num_borrowed_from_db(
    conn: PooledConnection<SqliteConnectionManager>,
    user_id: Uuid,
) -> Result<u32, rusqlite::Error> {
    conn.query_row::<u32, _, _>(
        "SELECT COUNT(*) FROM map_users_to_borrowed_books a
                WHERE a.action = 'Borrowed'
                AND a.user_id = $1
                AND a.id NOT IN (SELECT b.id FROM map_users_to_borrowed_books b WHERE b.action = 'Returned')", 
            [user_id], 
            |row| row.get(0)
    )
}

pub fn get_num_user_can_borrow_from_db(
    conn: PooledConnection<SqliteConnectionManager>,
    user_id: Uuid,
) -> Result<u32, rusqlite::Error> {
    conn.query_row::<u32, _, _>(
        "SELECT c.num_borrowable_books from users a 
                LEFT JOIN map_users_to_user_roles b ON a.id = b.user_id 
                LEFT JOIN user_roles c ON b.user_role_id = c.id;", 
            [user_id], 
            |row| row.get(0)
    )
}