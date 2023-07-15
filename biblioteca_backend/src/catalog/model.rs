use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
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
pub struct Author {
    pub id: Uuid,
    pub name: String,
    pub country: String,
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
}

#[derive(Debug, Deserialize)]
pub struct CreateAuthorRequest {
    pub name: String,
    pub country: String,
}