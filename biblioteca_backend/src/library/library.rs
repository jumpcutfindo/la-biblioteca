use axum::{Router, routing::{delete, post}, extract::{Path, State}, http::StatusCode, Json};
use uuid::Uuid;

use crate::{AppState, error::Error, library::db::add_borrow_entry_to_db};

use super::model::BorrowBookRequest;

pub fn library_router() -> Router<AppState> {
    Router::new()
        .route("/books/:id/borrow", post(borrow_book))
        .route("/books/:id/return", delete(return_book))
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
            return Err(Error::server_issue())
        }
    }
}

async fn can_borrow_book() {

}

async fn return_book() {

}

async fn can_return_book() {

}