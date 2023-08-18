use std::collections::HashMap;

use axum::{Router, routing::{get, delete, post}, extract::{State, Path, Query}, Json, http::StatusCode};
use uuid::Uuid;

use crate::{AppState, error::Error, users::db::{get_user_from_db, add_user_to_db, list_users_from_db, list_user_roles_from_db, delete_user_from_db}};

use super::model::{User, CreateUserRequest, UserRole, FullUser};

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/:id", delete(delete_user))
        .route("/users", get(list_users))
        .route("/users", post(add_user))
        .route("/users/roles", get(list_user_roles))
}

async fn get_user(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FullUser>, Error> {
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

async fn list_users(
    state: State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<FullUser>>, Error> {
    tracing::debug!("GET /users with query params: {:?}", params);

    match list_users_from_db(state).await {
        Ok(users) => {
            return Ok(Json(users))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        }
    }
}

async fn list_user_roles(
    state: State<AppState>,
) -> Result<Json<Vec<UserRole>>, Error> {
    tracing::debug!("GET /users/roles");

    match list_user_roles_from_db(state).await {
        Ok(user_roles) => {
            return Ok(Json(user_roles))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        }
    }
}

async fn add_user(
    state: State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, Error> {
    tracing::debug!("POST /users with params: {:?}", payload);
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username,
    };

    let user_role_id = payload.user_role_id;

    match add_user_to_db(state, user, user_role_id).await {
        Ok(user) => {
            return Ok(Json(user))
        },
        Err(err) => {
            tracing::warn!("{}", err);

            return Err(Error::server_issue());
        }
    }
}

async fn delete_user(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, Error> {
    tracing::debug!("DELETE /users with id: {:?}", id);

    match delete_user_from_db(state, Uuid::parse_str(&id).unwrap()).await {
        Ok(()) => {
            return Ok(StatusCode::NO_CONTENT)
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        }
    }
}