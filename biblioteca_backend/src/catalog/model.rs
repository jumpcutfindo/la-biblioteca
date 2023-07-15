use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub country: String,
}

#[derive(Serialize)]
pub struct BookAuthor {
    pub book_id: String,
    pub author_id: String,
}

#[derive(Serialize)]
pub struct BookGenre {
    pub book_id: String,
    pub genre_id: String,
}

#[derive(Deserialize)]
pub struct CreateBookRequest {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateAuthorRequest {
    pub name: String,
    pub country: String,
}