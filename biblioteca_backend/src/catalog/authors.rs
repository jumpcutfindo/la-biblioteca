use std::{collections::HashMap, str::FromStr};

use crate::app::AppState;
use crate::catalog::db::{list_authors_from_db, get_author_from_db, delete_author_from_db, update_author_in_db};
use crate::error::Error;

use axum::{Json, http::StatusCode, extract::{Path, Query, State}};
use uuid::Uuid;

use super::{model::{Author, CreateAuthorRequest, UpdateAuthorRequest}, db::add_author_to_db};

pub async fn get_author(
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

pub async fn list_authors(
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

pub async fn create_author(
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

pub async fn delete_author(
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

pub async fn update_author(
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