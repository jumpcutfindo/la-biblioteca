use std::{collections::HashMap, str::FromStr};

use crate::{AppState, error::Error, catalog::db::{list_authors_from_db, get_author_from_db, delete_author_from_db, update_author_in_db}};

use axum::{Json, http::StatusCode, extract::{Path, Query, State}, Router, routing::{get, post, put, delete}};
use uuid::Uuid;

use super::{model::{Author, CreateAuthorRequest, UpdateAuthorRequest}, db::add_author_to_db};

pub fn authors_router() -> Router<AppState> {
    Router::new()
        .route("/authors/:id", get(get_author))
        .route("/authors/:id", delete(delete_author))
        .route("/authors/:id", put(update_author))
        .route("/authors", get(list_authors))
        .route("/authors", post(create_author))
}
async fn get_author(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Author>, Error> {
    tracing::debug!("GET /authors with id: {:?}", id);

    match get_author_from_db(state, Uuid::from_str(&id).unwrap()).await {
        Ok(author) => {
            return Ok(Json(author))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::not_found())
        }
    }
}

async fn list_authors(
    state: State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Author>>, Error> {
    tracing::debug!("GET /authors with query params: {:?}", params);

    match list_authors_from_db(state, params).await {
        Ok(authors) => {
            return Ok(Json(authors))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        }
    }
}

async fn create_author(
    state: State<AppState>,
    Json(payload): Json<CreateAuthorRequest>,
) -> Result<Json<Author>, Error> {
    tracing::debug!("POST /authors with params: {:?}", payload);

    let author = Author {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        country: payload.country,
        language: payload.language,
    };

    match add_author_to_db(state, author).await {
        Ok(author) => {
            return Ok(Json(author))
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        }
    }
}

async fn delete_author(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, Error> {
    tracing::debug!("DELETE /authors with id: {:?}", id);

    match delete_author_from_db(state, Uuid::from_str(&id).unwrap()).await {
        Ok(()) => {
            return Ok(StatusCode::NO_CONTENT)
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        }
    }
}

async fn update_author(
    state: State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAuthorRequest>,
) -> Result<StatusCode, Error> {
    tracing::debug!("PUT /books with id: {:?}", id);

    let author = Author {
        id: Uuid::from_str(&id).unwrap(),
        name: payload.name,
        description: payload.description,
        country: payload.country,
        language: payload.language,
    };

    match update_author_in_db(state, author).await {
        Ok(()) => {
            return Ok(StatusCode::NO_CONTENT)
        },
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(Error::server_issue())
        }
    }
    
}