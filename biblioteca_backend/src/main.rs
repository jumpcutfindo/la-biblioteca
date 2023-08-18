use std::net::SocketAddr;

mod app;
mod catalog;
mod database;
mod users;
mod library;
mod error;

use axum::{
    Router,
    routing::{get, delete, put, post},
};

use catalog::{authors::{get_author, delete_author, update_author, list_authors, create_author}, books::{get_book, delete_book, update_book, list_books, create_book}};
use database::setup_db;
use library::library::{borrow_book, return_book};
use users::users::{get_user, delete_user, list_users, add_user, list_user_roles};

use crate::app::AppState;

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
        .route("/", get(root))
        .with_state(state);

    // Run app using hyper, listens on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("la-biblioteca server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn authors_router() -> Router<AppState> {
    Router::new()
        .route("/authors/:id", get(get_author))
        .route("/authors/:id", delete(delete_author))
        .route("/authors/:id", put(update_author))
        .route("/authors", get(list_authors))
        .route("/authors", post(create_author))
}

pub fn books_router() -> Router<AppState> {
    Router::new()
        .route("/books/:id", get(get_book))
        .route("/books/:id", delete(delete_book))
        .route("/books/:id", put(update_book))
        .route("/books", get(list_books))
        .route("/books", post(create_book))
}

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/:id", delete(delete_user))
        .route("/users", get(list_users))
        .route("/users", post(add_user))
        .route("/users/roles", get(list_user_roles))
}

pub fn library_router() -> Router<AppState> {
    Router::new()
        .route("/books/:id/borrow", post(borrow_book))
        .route("/books/:id/return", post(return_book))
}

// Sample handler that responds with static string
async fn root() -> &'static str {
    "Hello World!"
}