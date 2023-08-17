use axum::extract::State;
use rusqlite::Result;
use uuid::Uuid;

use crate::AppState;

pub async fn add_borrow_entry_to_db(
    State(state): State<AppState>,
    user_id: Uuid,
    book_id: Uuid,
) -> Result<()> {
    state.db_pool.get().unwrap().execute(
        "INSERT INTO map_users_to_borrowed_books (user_id, book_id) VALUES (?1, ?2)",
        (user_id, book_id),
    )?;

    Ok(())
}

pub async fn add_return_entry_to_db() {

}

pub async fn get_latest_book_state_from_db() {
    // Takes the latest entry from the table that stores the past
    // book states
}