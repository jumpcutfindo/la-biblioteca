use crate::AppState;

use axum::{Json, http::StatusCode, extract::Path, Router, routing::{get, post, put, delete}};
use uuid::Uuid;

use super::model::{Author, CreateAuthorRequest, UpdateAuthorRequest};

pub fn authors_router() -> Router<AppState> {
    Router::new()
        .route("/authors/:id", get(get_author))
        .route("/authors", post(create_author))
}

async fn create_author(Json(payload): Json<CreateAuthorRequest>) -> (StatusCode, Json<Author>) {
    let author = Author {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        country: payload.country,
        language: payload.language,
    };

    (StatusCode::CREATED, Json(author))
}

async fn get_author(Path(id): Path<String>) {

}

async fn get_authors() {

}

async fn delete_author(Path(id): Path<String>) {

}

async fn update_author(Json(payload): Json<UpdateAuthorRequest>) {
    
}