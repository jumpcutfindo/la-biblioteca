use axum::{Router, routing::{get, delete, post}};

use crate::AppState;

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/:id", delete(delete_user))
        .route("/users", get(get_users))
        .route("/users", post(add_user))
        .route("/users/roles", get(get_user_roles))
}

async fn get_user() {

}

async fn get_users() {

}

async fn get_user_roles() {

}

async fn add_user() {

}

async fn delete_user() {
    
}