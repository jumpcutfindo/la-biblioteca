use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    ServerIssue(#[from] ServerIssue),
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            // 4XXs
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40001),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40004),

            // 5XXs
            Error::ServerIssue(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
        }
    }

    pub fn not_found() -> Self {
        return Error::NotFound(NotFound {  })
    }

    pub fn bad_request() -> Self {
        return Error::BadRequest(BadRequest {  })
    }

    pub fn server_issue() -> Self {
        return Error::ServerIssue(ServerIssue {  })
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_codes();

        let message = self.to_string();
        let body = Json(json!({ "code": code, "message": message }));

        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Bad request!")]
pub struct BadRequest {

}

#[derive(thiserror::Error, Debug)]
#[error("Resource not found!")]
pub struct NotFound {

}

#[derive(thiserror::Error, Debug)]
#[error("Internal server error -- check logs for more details!")]
pub struct ServerIssue {

}