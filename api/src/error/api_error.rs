use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    // #[error(transparent)]
    // RequestError(#[from] RequestError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {}
    }
}
