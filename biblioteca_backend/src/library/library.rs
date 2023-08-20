use axum::{extract::{Path, State}, http::StatusCode, Json, Router, routing::post};
use uuid::Uuid;

use crate::{error::Error, library::{db::{add_borrow_entry_to_db, is_book_exists_in_db, is_user_exists_in_db, get_latest_book_entry_from_db, get_num_borrowed_from_db, get_num_user_can_borrow_from_db}, error::LibraryError, model::BookBorrowState}};
use crate::app::AppState;

use super::{model::BorrowBookRequest, db::add_return_entry_to_db};

pub fn library_router() -> Router<AppState> {
    Router::new()
        .route("/books/:id/borrow", post(borrow_book))
        .route("/books/:id/return", post(return_book))
}

// TODO: Update all Path objects to be Uuid instead of string
pub async fn borrow_book(
    state: State<AppState>,
    Path(book_id): Path<Uuid>,
    Json(payload): Json<BorrowBookRequest>,
) -> Result<StatusCode, Error> {
    tracing::debug!("POST /books/:id/borrow for user_id {:?} and book_id {:?}", payload.user_id, book_id);

    // Check existence of book_id
    if !is_book_exists_in_db(&state, book_id).unwrap() {
        return Err(Error::bad_request(LibraryError::BookNotExists.to_string()))
    }

    // Check existence of user_id
    if !is_user_exists_in_db(&state, payload.user_id).unwrap() {
        return Err(Error::bad_request(LibraryError::UserNotExists.to_string()))
    }

    // Check whether user has exceeded borrow limit
    let num_borrowed = get_num_borrowed_from_db(&state, payload.user_id).unwrap();
    let num_max_borrowable = get_num_user_can_borrow_from_db(&state, payload.user_id).unwrap();
    if num_borrowed >= num_max_borrowable  {
        return Err(Error::bad_request(LibraryError::NumBorrowableExceeded(num_max_borrowable).to_string()))
    }

    // Check whether book is available for borrowing
    match get_latest_book_entry_from_db(&state, book_id) {
        Ok(entry) => {
            let latest_entry = entry.action;

            // If entry exists, check if it's "Borrowed"
            match latest_entry {
                // Can't borrow a borrowed book, return as error
                BookBorrowState::Borrowed => return Err(Error::bad_request(LibraryError::BookAlreadyBorrowed.to_string())),
                // Latest entry is that it's returned, so we can borrow it
                BookBorrowState::Returned => {},
            }
        },
        Err(err) => {
            tracing::warn!("{}", err);
            match err {
                // No entry found, so it's OK
                rusqlite::Error::QueryReturnedNoRows => {},
                _ => return Err(Error::server_issue()),
            }
        }
    }

    match add_borrow_entry_to_db(state, payload.user_id, book_id).await {
        Ok(()) => {
            return Ok(StatusCode::ACCEPTED)
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue());
        }
    }
}

pub async fn return_book(
    state: State<AppState>,
    Path(book_id): Path<Uuid>,
    Json(payload): Json<BorrowBookRequest>,
) -> Result<StatusCode, Error> {
    tracing::debug!("POST /books/:id/return for user_id {:?} and book_id {:?}", payload.user_id, book_id);

    // Check existence of book_id
    if !is_book_exists_in_db(&state, book_id).unwrap() {
        return Err(Error::bad_request(LibraryError::BookNotExists.to_string()))
    }

    // Check existence of user_id
    if !is_user_exists_in_db(&state, payload.user_id).unwrap() {
        return Err(Error::bad_request(LibraryError::UserNotExists.to_string()))
    }

    let mut borrower_id = Uuid::nil();
    let mut entry_id = Uuid::nil();

    // Check if the book is currently returned
    match get_latest_book_entry_from_db(&state, book_id) {
        Ok(entry) => {
            let latest_entry = entry.action;

            // If entry exists, check if it's "Returned"
            match latest_entry {
                BookBorrowState::Borrowed => {},
                BookBorrowState::Returned => return Err(Error::bad_request(LibraryError::BookAlreadyReturned.to_string())),
            }

            borrower_id = entry.user_id;
            entry_id = entry.id;
        },
        Err(err) => {
            tracing::warn!("{}", err);
            match err {
                rusqlite::Error::QueryReturnedNoRows => {},
                _ => return Err(Error::server_issue())
            }
        },
    }

    //  Check whether the borrower is the same user
    if payload.user_id != borrower_id {
        return Err(Error::bad_request(LibraryError::BookNotBorrowedByUser.to_string()));
    }
    
    match add_return_entry_to_db(state, entry_id, payload.user_id, book_id).await {
        Ok(()) => {
            return Ok(StatusCode::ACCEPTED)
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue());
        }
    }
}
