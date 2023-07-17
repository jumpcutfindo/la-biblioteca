#[path = "./catalog/model.rs"] mod catalog_model;
#[path = "./catalog/handlers.rs"] mod catalog_handlers;

#[path = "./db/database.rs"] mod db;
#[path = "./error.rs"] mod errors;

use std::net::SocketAddr;

use axum::routing::{get, post};

use catalog_handlers::{
    create_book, get_book, get_books, delete_book, create_author, get_author
};

use db::setup_db;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Create an in-memory db
    setup_db().unwrap();

    // Create router
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/books/:id", get(get_book).delete(delete_book))
        .route("/books", get(get_books).post(create_book))
        .route("/authors/:id", get(get_author))
        .route("/authors", post(create_author));

    // Run app using hyper, listens on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("la-biblioteca server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Sample handler that responds with static string
async fn root() -> &'static str {
    "Hello World!"
}