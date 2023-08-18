use std::net::SocketAddr;

mod app;
mod catalog;
mod database;
mod users;
mod library;
mod error;

use axum::Router;

use database::setup_db;

use crate::{app::AppState, catalog::{authors::authors_router, books::books_router}, users::users::users_router, library::library::library_router};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Create an in-memory db
    let pool = setup_db().unwrap();

    let state = AppState {
        db_pool: pool,
    };

    // Create router
    let app = Router::new()
        .merge(books_router())
        .merge(authors_router())
        .merge(users_router())
        .merge(library_router())
        .with_state(state);

    // Run app using hyper, listens on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("la-biblioteca server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
