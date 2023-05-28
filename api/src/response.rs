use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApiResponse<T: Serialize> {
    data: T,
}

impl<T: Serialize> ApiResponse<T>
where
    T: Serialize,
{
    pub fn send(data: T) -> Self {
        ApiResponse { data }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiErrorResponse {
    message: String,

    #[serde(with = "http_serde::status_code")]
    status: StatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

impl ApiErrorResponse {
    pub fn send(status: StatusCode, message: String, code: Option<String>) -> Response {
        ApiErrorResponse {
            message,
            status,
            code,
        }
        .into_response()
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}
