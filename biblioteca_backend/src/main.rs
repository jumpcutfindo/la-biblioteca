use std::{
    net::SocketAddr,
    collections::HashMap
};

use axum::{
    routing::{get, post},
    http::Uri, http::StatusCode,
    response::IntoResponse,
    Json, Router,
    extract::{self, Query}
};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Create router
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/books", get(get_books))
        .route("/books", post(create_book));

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

// Retrieves all books
async fn get_books(
    extract::Query(params): Query<HashMap<String, String>>    
) -> (StatusCode, Json<Vec<Book>>) {
    tracing::debug!("GET /books with query params: {:?}", params);
    
    let a = Book {
        id: 1,
        name: "Alice in Wonderland".to_owned(),
        description: "Lorem ipsum et amor de fulcus merudo".to_owned(),
    };

    let b = Book {
        id: 2,
        name: "Harry Potter".to_owned(),
        description: "Lorem ipsum et amor de fulcus merudo".to_owned(),
    };

    let vec = vec![a, b];

    (StatusCode::OK, Json(vec))
}

// Creates a new book
async fn create_book(
    Json(payload): Json<CreateBookRequest>,
) -> (StatusCode, Json<Book>) {
    let book = Book {
        id: 1,
        name: payload.name,
        description: payload.description,
    };

    (StatusCode::CREATED, Json(book))
}

#[derive(Deserialize)]
struct CreateBookRequest {
    name: String,
    description: String,
}

#[derive(Serialize)]
struct Book {
    id: u64,
    name: String,
    description: String,
}