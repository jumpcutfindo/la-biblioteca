#[path = "./model.rs"]
mod model;

#[path = "./db.rs"]
mod database;

use model::{Author, Book, CreateAuthorRequest, CreateBookRequest};

use database::{ get_all_books_from_db, get_book_from_db, add_book_to_db };
use uuid::Uuid;

use std::{collections::HashMap, str::FromStr};

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};

use serde_json::{Value, json};

// Retrieves a specific book, by id
pub async fn get_book(
    Path(id): Path<String>
) -> (StatusCode, Json<Value>) {
    tracing::debug!("GET /books with id: {:?}", id);
    
    match get_book_from_db(Uuid::from_str(&id).unwrap()).await {
        Ok(book) => {
            return (StatusCode::OK, Json(json!(book)))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return (StatusCode::NOT_FOUND, Json(json!({})))
        }
    };
}

// Retrieves all books
pub async fn get_books(
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    tracing::debug!("GET /books with query params: {:?}", params);

    match get_all_books_from_db().await {
        Ok(books) => {
            return (StatusCode::OK, Json(json!(books)))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})))
        },
    }
}

// Creates a new book
pub async fn create_book(
    Json(payload): Json<CreateBookRequest>
) -> (StatusCode, Json<Value>) {
    let book = Book {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
    };

    match add_book_to_db(book).await {
        Ok(book) => {
            return (StatusCode::CREATED, Json(json!(book)))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})))
        }
    }
}

// Deletes a specific book
pub async fn delete_book(Path(id): Path<String>) -> (StatusCode, Json<Book>) {
    tracing::debug!("DELETE /books with id: {:?}", id);

    let book = Book {
        id: Uuid::new_v4(),
        name: "Alice in Wonderland".to_owned(),
        description: "Lorem ipsum et amor de fulcus merudo".to_owned(),
    };

    (StatusCode::OK, Json(book))
}

pub async fn create_author(Json(payload): Json<CreateAuthorRequest>) -> (StatusCode, Json<Author>) {
    let author = Author {
        id: Uuid::new_v4(),
        name: payload.name,
        country: payload.country,
    };

    (StatusCode::CREATED, Json(author))
}

pub async fn get_author(Path(_id): Path<String>) -> (StatusCode, Json<Author>) {
    let author = Author {
        id: Uuid::new_v4(),
        name: "Bill Gates".to_owned(),
        country: "US".to_owned(),
    };

    (StatusCode::OK, Json(author))
}