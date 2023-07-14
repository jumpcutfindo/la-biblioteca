#[path = "./model.rs"]
mod model;

#[path = "./db.rs"]
mod database;

use model::{Author, Book, CreateAuthorRequest, CreateBookRequest};

use database::{ add_book_to_db, get_all_books_from_db };
use uuid::Uuid;

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
        id: Uuid::new_v4().to_string(),
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

    (StatusCode::OK, Json(get_all_books_from_db().await.unwrap()))
}

// Creates a new book
pub async fn create_book(Json(payload): Json<CreateBookRequest>) -> (StatusCode, Json<Book>) {
    let book = Book {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        description: payload.description,
    };

    (StatusCode::CREATED, Json(add_book_to_db(book).await.unwrap()))
}

// Deletes a specific book
pub async fn delete_book(Path(id): Path<String>) -> (StatusCode, Json<Book>) {
    tracing::debug!("DELETE /books with id: {:?}", id);

    let book = Book {
        id: Uuid::new_v4().to_string(),
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

pub async fn get_author(Path(_id): Path<String>) -> (StatusCode, Json<Author>) {
    let author = Author {
        id: 1,
        name: "Bill Gates".to_owned(),
        country: "US".to_owned(),
    };

    (StatusCode::OK, Json(author))
}