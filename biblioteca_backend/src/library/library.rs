use axum::{Router, routing::{delete, post}};

use crate::AppState;

pub fn library_router() -> Router<AppState> {
    Router::new()
        .route("/books/:id/borrow", post(borrow_book))
        .route("/books/:id/return", delete(return_book))
}

pub async fn borrow_book() {

}

async fn can_borrow_book() {

}

async fn return_book() {

}

async fn can_return_book() {

}