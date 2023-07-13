use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    http::Uri, http::StatusCode,
    response::IntoResponse,
    Json, Router
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