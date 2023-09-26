use axum::{extract::State, Router};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::{
    catalog::{authors::authors_router, books::books_router},
    library::controller::library_router,
    users::controller::users_router,
};

pub fn create_new_state(db_pool: Pool<SqliteConnectionManager>) -> AppState {
    AppState { db_pool }
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
