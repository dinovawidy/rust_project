use axum:: {
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub error: bool,
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
    pub error_message: Option<serde_json::Value>,
}

impl<T> ApiResponse<T>
where 
    T: Serialize,
{
    pub fn success(status: StatusCode, message: &str, data: T) -> Response {
        let response = ApiResponse {
            error: false,
            status: status.as_u16(),
            message: message.to_string(),
            data: Some(data),
            error_message: None,
        };
        (status, Json(response)).into_response()
    }

    pub fn error(status: StatusCode, message: &str, error_message: Option<serde_json::Value>,) -> Response {
        let response = ApiResponse::<()> {
            error: true,
            status: status.as_u16(),
            message: message.to_string(),
            data: None,
            error_message,
        };
        (status, Json(response)).into_response()
    }
}