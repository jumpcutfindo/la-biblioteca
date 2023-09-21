use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{app::AppState, users::db::{get_user_role_from_db, add_user_role_to_db, delete_user_role_from_db}};
use crate::{
    error::Error,
    users::db::{
        add_user_to_db, delete_user_from_db, get_user_from_db, list_user_roles_from_db,
        list_users_from_db,
    },
};

use super::model::{CreateUserRequest, FullUser, User, UserRole, CreateUserRoleRequest};

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/:id", delete(delete_user))
        .route("/users", get(list_users))
        .route("/users", post(add_user))
        .route("/users/roles/:id", get(get_user_role))
        .route("/users/roles/:id", delete(delete_user_role))
        .route("/users/roles", post(add_user_role))
        .route("/users/roles", get(list_user_roles))
        
}

pub async fn get_user(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FullUser>, Error> {
    tracing::debug!("GET /users with id: {:?}", id);

    match get_user_from_db(state, Uuid::parse_str(&id).unwrap()).await {
        Ok(user) => return Ok(Json(user)),
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::not_found());
        }
    }
}

pub async fn list_users(
    state: State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<FullUser>>, Error> {
    tracing::debug!("GET /users with query params: {:?}", params);

    match list_users_from_db(state).await {
        Ok(users) => return Ok(Json(users)),
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue());
        }
    }
}

pub async fn add_user(
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
        Ok(user) => return Ok(Json(user)),
        Err(err) => {
            tracing::warn!("{}", err);

            return Err(Error::server_issue());
        }
    }
}

pub async fn delete_user(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, Error> {
    tracing::debug!("DELETE /users with id: {:?}", id);

    match delete_user_from_db(state, Uuid::parse_str(&id).unwrap()).await {
        Ok(()) => return Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue());
        }
    }
}

pub async fn get_user_role(state: State<AppState>, Path(id): Path<String>) -> Result<Json<UserRole>, Error> {
    tracing::debug!("GET /users/roles with id: {:?}", id);

    match get_user_role_from_db(state, Uuid::parse_str(&id).unwrap()).await {
        Ok(user_role) => return Ok(Json(user_role)),
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::not_found());
        }
    }
}

pub async fn list_user_roles(state: State<AppState>) -> Result<Json<Vec<UserRole>>, Error> {
    tracing::debug!("GET /users/roles");

    match list_user_roles_from_db(state).await {
        Ok(user_roles) => return Ok(Json(user_roles)),
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue());
        }
    }
}

pub async fn add_user_role(state: State<AppState>, Json(payload): Json<CreateUserRoleRequest>) -> Result<Json<UserRole>, Error> {
    tracing::debug!("POST /users/roles with params: {:?}", payload);

    let user_role = UserRole {
        id: Uuid::new_v4(),
        name: payload.name,
        num_borrowable_books: payload.num_borrowable_books,
    };

    match add_user_role_to_db(state, user_role).await {
        Ok(user_role) => return Ok(Json(user_role)),
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue());
        }
    }
}

pub async fn delete_user_role(state: State<AppState>, Path(id): Path<String>) -> Result<StatusCode, Error> {
    tracing::debug!("DELETE /users/roles with id: {:?}", id);

    match delete_user_role_from_db(state, Uuid::parse_str(&id).unwrap()).await {
        Ok(()) => return Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue());
        }
    }
}
