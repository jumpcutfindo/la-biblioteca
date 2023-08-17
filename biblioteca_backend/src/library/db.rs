use axum::extract::State;
use rusqlite::Result;
use uuid::Uuid;
use chrono::prelude::*;

use crate::AppState;

use super::model::BookState;

pub async fn add_borrow_entry_to_db(
    State(state): State<AppState>,
    user_id: Uuid,
    book_id: Uuid,
) -> Result<()> {
    state.db_pool.get().unwrap().execute(
        "INSERT INTO map_users_to_borrowed_books (user_id, book_id, timestamp, action) VALUES (?1, ?2, ?3, ?4)",
        (user_id, book_id, Utc::now(), BookState::Borrowed),
    )?;

    Ok(())
}

pub async fn add_return_entry_to_db() {

}

pub async fn get_latest_book_state_from_db() {
    // Takes the latest entry from the table that stores the past
    // book states
}