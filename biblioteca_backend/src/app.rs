use axum::{Router, extract::State};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::{catalog::{books::books_router, authors::authors_router}, users::users::users_router, library::library::library_router};

pub fn create_new_state(db_pool: Pool<SqliteConnectionManager>) -> AppState {
    return AppState {
        db_pool,
    }
}

pub fn create_app(State(state): State<AppState>) -> Router {
    // Create router
    Router::new()
        .merge(books_router())
        .merge(authors_router())
        .merge(users_router())
        .merge(library_router())
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<SqliteConnectionManager>,
}