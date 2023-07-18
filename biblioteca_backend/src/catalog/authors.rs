use axum::{Json, http::StatusCode, extract::Path};
use uuid::Uuid;

use super::model::{Author, CreateAuthorRequest, UpdateAuthorRequest};

pub async fn create_author(Json(payload): Json<CreateAuthorRequest>) -> (StatusCode, Json<Author>) {
    let author = Author {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        country: payload.country,
        language: payload.language,
    };

    (StatusCode::CREATED, Json(author))
}

pub async fn get_author(Path(id): Path<String>) {

}

pub async fn get_authors() {

}

pub async fn delete_author(Path(id): Path<String>) {

}

pub async fn update_author(Json(payload): Json<UpdateAuthorRequest>) {
    
}