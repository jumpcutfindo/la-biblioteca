mod catalog;
mod database;
mod error;

use std::net::SocketAddr;

use axum::{
    Router,
    routing::get,
};

use catalog::{ 
    books::books_router,
    authors::authors_router,
};

use database::setup_db;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Create an in-memory db
    let conn = setup_db().unwrap();

    // Create router
    let app = Router::new()
        .merge(books_router())
        .merge(authors_router())
        .route("/", get(root));

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