use axum::{Router, routing::{get, delete, post}, extract::{State, Path}, Json};
use uuid::Uuid;

use crate::{AppState, error::Error, users::db::get_user_from_db};

use super::model::User;

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/:id", delete(delete_user))
        .route("/users", get(get_users))
        .route("/users", post(add_user))
        .route("/users/roles", get(get_user_roles))
}

async fn get_user(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<User>, Error> {
    tracing::debug!("GET /users with id: {:?}", id);

    match get_user_from_db(state, Uuid::parse_str(&id).unwrap()).await {
        Ok(user) => {
            return Ok(Json(user))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::not_found())
        }
    }
}

async fn get_users() {

}

async fn get_user_roles() {

}

async fn add_user() {

}

async fn delete_user() {
    
}