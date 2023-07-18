use axum::{Json, http::StatusCode, extract::Path};
use uuid::Uuid;

use super::model::{Author, CreateAuthorRequest};

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

pub async fn get_authors() {

}

pub async fn delete_author() {

}

pub async fn update_author() {
    
}