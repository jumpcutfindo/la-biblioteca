use axum::{extract::{Path, State}, http::StatusCode, Json, Router, routing::post};
use uuid::Uuid;

use crate::{error::Error, library::{db::add_borrow_entry_to_db, error::LibraryError}};
use crate::app::AppState;

use super::{model::BorrowBookRequest, db::add_return_entry_to_db};

pub fn library_router() -> Router<AppState> {
    Router::new()
        .route("/books/:id/borrow", post(borrow_book))
        .route("/books/:id/return", post(return_book))
}

pub async fn borrow_book(
    state: State<AppState>,
    Path(book_id): Path<String>,
    Json(payload): Json<BorrowBookRequest>,
) -> Result<StatusCode, Error> {
    tracing::debug!("POST /books/:id/borrow for user_id {:?} and book_id {:?}", payload.user_id, book_id);

    match add_borrow_entry_to_db(state, payload.user_id, Uuid::parse_str(&book_id).unwrap()).await {
        Ok(()) => {
            return Ok(StatusCode::ACCEPTED)
        },
        Err(err) => {
            tracing::warn!("{}", err);

            match err {
                LibraryError::NumBorrowableExceeded(_) | LibraryError::BookAlreadyBorrowed | LibraryError::ResourceNotExists => 
                    return Err(Error::bad_request(String::from(err.to_string()))),
                _ => return Err(Error::server_issue()),
            }
        }
    }
}

pub async fn return_book(
    state: State<AppState>,
    Path(book_id): Path<String>,
    Json(payload): Json<BorrowBookRequest>,
) -> Result<StatusCode, Error> {
    tracing::debug!("POST /books/:id/return for user_id {:?} and book_id {:?}", payload.user_id, book_id);
    
    match add_return_entry_to_db(state, payload.user_id, Uuid::parse_str(&book_id).unwrap()).await {
        Ok(()) => {
            return Ok(StatusCode::ACCEPTED)
        },
        Err(err) => {
            tracing::warn!("{}", err);

            match err {
                LibraryError::BookNotBorrowedByUser | LibraryError::BookAlreadyReturned | LibraryError::ResourceNotExists => 
                    return Err(Error::bad_request(String::from(err.to_string()))),
                _ => return Err(Error::server_issue()),
            }
        }
    }
}
