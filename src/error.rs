use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use crate::utils::api_response::ApiResponse;


pub enum AppError {
    // BadRequest(String),
    // NotFound(String),
    Conflict(String),
    Forbidden(String),
    Validation(String),
    Unauthorized(String),
    NotFound(String),
    Internal(String),
    ServiceUnavailable(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(msg) => {
                ApiResponse::<()>::error(
                    StatusCode::BAD_REQUEST,
                    "Validation error",
                    Some(json!(msg)),
                )
            }
            AppError::Unauthorized(msg) => {
                ApiResponse::<()>::error(
                    StatusCode::UNAUTHORIZED,
                    &msg,
                    None,
                )
            }
            AppError::NotFound(msg) => {
                ApiResponse::<()>::error(
                    StatusCode::NOT_FOUND,
                    &msg,
                    None,
                )
            }
            AppError::Internal(msg) => {
                ApiResponse::<()>::error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    Some(json!(msg)),
                )
            }
            AppError::Conflict(msg) => {
                ApiResponse::<()>::error(
                    StatusCode::CONFLICT,
                    &msg,
                    None,
                )
            }
            AppError::Forbidden(msg) => {
                ApiResponse::<()>::error(
                    StatusCode::FORBIDDEN,
                    &msg,
                    None,
                )
            }
            AppError::ServiceUnavailable(msg) => {
                ApiResponse::<()>::error(
                    StatusCode::SERVICE_UNAVAILABLE,
                    &msg,
                    None,
                )
            }
        }
    }
}

// #[derive(Serialize)]
// struct ErrorResponse {
//     message: String,
// }

// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         let (status, message) = match self {
//             AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
//             AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
//             AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
//             AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
//             AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
//             AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
//         };

//         let body = Json(ErrorResponse { message });
//         (status, body).into_response()
//     }

// }

// impl AppError {
//     pub fn from_validation_error(errors: ValidationErrors) -> Self {
//         let message = errors
//             .field_errors()
//             .iter()
//             .next()
//             .map(|(_, errors)| {
//                 errors
//                     .first()
//                     .and_then(|e| e.message.clone())
//                     .unwrap_or_else(|| "Invalid request".into())
//             })
//             .unwrap_or_else(|| "Invalid request".into());

//         AppError::BadRequest(message.to_string())
//     }
// }
