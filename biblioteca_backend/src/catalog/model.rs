use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateBookRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateAuthorRequest {
    pub name: String,
    pub country: String,
}

#[derive(Serialize)]
pub struct Author {
    pub id: u64,
    pub name: String,
    pub country: String,
}