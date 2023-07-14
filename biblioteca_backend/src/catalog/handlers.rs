#[path = "./model.rs"]
mod model;

use model::{Author, Book, CreateAuthorRequest, CreateBookRequest};

use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};

// Retrieves a specific book, by id
pub async fn get_book(Path(id): Path<String>) -> (StatusCode, Json<Book>) {
    tracing::debug!("GET /books with id: {:?}", id);

    let book = Book {
        id: 1,
        name: "Alice in Wonderland".to_owned(),
        description: "Lorem ipsum et amor de fulcus merudo".to_owned(),
    };

    (StatusCode::OK, Json(book))
}

// Retrieves all books
pub async fn get_books(
    Query(params): Query<HashMap<String, String>>,
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
pub async fn create_book(Json(payload): Json<CreateBookRequest>) -> (StatusCode, Json<Book>) {
    let book = Book {
        id: 1,
        name: payload.name,
        description: payload.description,
    };

    (StatusCode::CREATED, Json(book))
}

// Deletes a specific book
pub async fn delete_book(Path(id): Path<String>) -> (StatusCode, Json<Book>) {
    tracing::debug!("DELETE /books with id: {:?}", id);

    let book = Book {
        id: 1,
        name: "Alice in Wonderland".to_owned(),
        description: "Lorem ipsum et amor de fulcus merudo".to_owned(),
    };

    (StatusCode::OK, Json(book))
}

pub async fn create_author(Json(payload): Json<CreateAuthorRequest>) -> (StatusCode, Json<Author>) {
    let author = Author {
        id: 1,
        name: payload.name,
        country: payload.country,
    };

    (StatusCode::CREATED, Json(author))
}

pub async fn get_author(Path(id): Path<String>) -> (StatusCode, Json<Author>) {
    let author = Author {
        id: 1,
        name: "Bill Gates".to_owned(),
        country: "US".to_owned(),
    };

    (StatusCode::OK, Json(author))
}