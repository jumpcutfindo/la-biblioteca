use axum::Router;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::{catalog::{books::books_router, authors::authors_router}, users::users::users_router, library::library::library_router, database::setup_db};

pub fn app() -> Router {
    // Create an in-memory db
    let pool = setup_db().unwrap();

    let state = AppState {
        db_pool: pool,
    };

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