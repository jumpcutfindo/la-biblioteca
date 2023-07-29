use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Book {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize)]
pub struct BookAuthor {
    pub book_id: Uuid,
    pub author_id: Uuid,
}

#[derive(Serialize)]
pub struct BookCategory {
    pub book_id: Uuid,
    pub genre_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookRequest {
    pub name: String,
    pub description: String,

    pub author_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookRequest {
    pub name: String,
    pub description: String,

    pub author_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct Author {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub country: String,
    pub language: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAuthorRequest {
    pub name: String,
    pub description: String,
    pub country: String,
    pub language: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAuthorRequest {
    pub name: String,
    pub description: String,
    pub country: String,
    pub language: String,
}