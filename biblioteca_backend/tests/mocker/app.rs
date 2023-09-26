use axum::{extract::State, Router};
use biblioteca_backend::app::{create_app, AppState};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn create_mock_app(db_pool: Pool<SqliteConnectionManager>) -> Router {
    let state = AppState { db_pool };

    create_app(State(state))
}
