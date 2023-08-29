use axum::{extract::State, Router};
use biblioteca_backend::app::{create_new_state, AppState, create_app};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_mock_state(db_pool: Pool<SqliteConnectionManager>) -> AppState {
    return create_new_state(db_pool);
}

pub fn create_mock_app(state: AppState) -> Router {
    return create_app(State(state));
}