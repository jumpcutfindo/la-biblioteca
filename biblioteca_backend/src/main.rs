mod catalog;
mod db;
mod error;

use std::net::SocketAddr;

use axum::routing::{get, post, delete, put};

use catalog::{ 
    books::{
        create_book, get_book, get_books, delete_book, update_book
    },
    authors::{
        create_author, get_author
    }
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
        .route("/books/:id", get(get_book))
        .route("/books/:id", delete(delete_book))
        .route("/books/:id", put(update_book))
        .route("/books", get(get_books))
        .route("/books", post(create_book))
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