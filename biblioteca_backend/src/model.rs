use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateBookRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct Book {
    pub id: u64,
    pub name: String,
    pub description: String,
}