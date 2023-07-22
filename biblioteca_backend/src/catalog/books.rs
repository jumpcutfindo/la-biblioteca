use crate::AppState;
use crate::catalog::error::CatalogError;

use super::model::{Book, CreateBookRequest, UpdateBookRequest};
use super::db::{ get_all_books_from_db, get_book_from_db, add_book_to_db, delete_book_from_db, update_book_in_db };
use super::super::error::Error;

use axum::extract::State;
use uuid::Uuid;

use std::{collections::HashMap, str::FromStr};

use axum::{
    Router,
    routing::{get, delete, put, post},
    extract::{Path, Query},
    http::StatusCode,
    Json,
};

pub fn books_router() -> Router<AppState> {
    Router::new()
        .route("/books/:id", get(get_book))
        .route("/books/:id", delete(delete_book))
        .route("/books/:id", put(update_book))
        .route("/books", get(get_books))
        .route("/books", post(create_book))
}

// Retrieves a specific book, by id
async fn get_book(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Book>, Error> {
    tracing::debug!("GET /books with id: {:?}", id);
    
    match get_book_from_db(state, Uuid::from_str(&id).unwrap()).await {
        Ok(book) => {
            return Ok(Json(book))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::not_found())
        }
    };
}

// Retrieves all books
async fn get_books(
    state: State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Book>>, Error> {
    tracing::debug!("GET /books with query params: {:?}", params);

    match get_all_books_from_db(state).await {
        Ok(books) => {
            return Ok(Json(books))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        },
    }
}

// Creates a new book
async fn create_book(
    state: State<AppState>,
    Json(payload): Json<CreateBookRequest>,
) -> Result<Json<Book>, Error> {
    tracing::debug!("POST /books with params: {:?}", payload);
    let book = Book {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
    };

    match add_book_to_db(state, book, payload.author_id).await {
        Ok(book) => {
            return Ok(Json(book))
        },
        Err(err) => {
            tracing::warn!("{}", err);

            match err {
                CatalogError::AuthorNotFound => 
                    return Err(Error::bad_request(err.to_string())),
                CatalogError::DatabaseError(_) => 
                    return Err(Error::server_issue()),
            }
        }
    }
}

// Deletes a specific book
async fn delete_book(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, Error> {
    tracing::debug!("DELETE /books with id: {:?}", id);

    match delete_book_from_db(state, Uuid::from_str(&id).unwrap()).await {
        Ok(()) => {
            return Ok(StatusCode::NO_CONTENT)
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        },
    }
}

// Updates a specific book
async fn update_book(
    state: State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBookRequest>,
) -> Result<StatusCode, Error> {
    tracing::debug!("PUT /books with id: {:?}", id);

    let book = Book {
        id: Uuid::from_str(&id).unwrap(),
        name: payload.name,
        description: payload.description,
    };

    match update_book_in_db(state, book).await {
        Ok(()) => {
            return Ok(StatusCode::NO_CONTENT)
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        }
    }
}