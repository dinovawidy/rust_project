use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
};
use crate::{
    auth::jwt::verify_token,
    error::AppError,
};

pub struct AuthUser {
    pub user_id: String,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized("Invalid Authorization format".into()))?;

        let claims = verify_token(token)
            .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

        Ok(AuthUser {
            user_id: claims.sub,
            role: claims.role,
        })
    }
}
