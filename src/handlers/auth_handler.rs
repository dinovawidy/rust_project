use axum::{ Json, 
            extract::State,
            http::StatusCode,
            response::Response,
        };
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use sqlx;
// use validator::Validate;
// use time::{OffsetDateTime, Duration};
use chrono::{Utc, NaiveDateTime};
use serde::Serialize;
use validator::Validate;
// use axum::extract::ConnectInfo;
// use std::net::SocketAddr;
use redis::AsyncCommands;

// use crate::auth::{
//     jwt::{generate_access_token, generate_refresh_token},
//     hash::hash_token,
// };

// use crate::{
//     models::auth::{RegisterRequest, LoginRequest, AuthResponse},
//     state::AppState,
//     error::AppError,
//     auth::jwt::generate_token,
// };

use crate::{
    utils::api_response::ApiResponse,
    state::AppState,
    error::AppError,
    models::auth::{
        RegisterRequest,
        LoginRequest,
        RefreshRequest,
        AuthResponse,
    },
    auth::{
        jwt::{
            verify_token,
            generate_access_token,
            generate_refresh_token,
        },
        hash::hash_token,
    },
};

#[derive(Serialize)]
pub struct RefreshResponse {
    pub refresh_token: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Response, AppError> {

    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let password_hash = bcrypt::hash(payload.password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::Internal("Hash password failed".into()))?;

    let user_id = Uuid::new_v4();
    let role = "user";

    sqlx::query!(
        "INSERT INTO users (id, name, email, password, role)
         VALUES ($1, $2, $3, $4, $5)",
        user_id,
        payload.name,
        payload.email,
        password_hash,
        role
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let access_token =
        generate_access_token(&user_id.to_string(), role);
    let refresh_token =
        generate_refresh_token(&user_id.to_string(), role);

    let refresh_id = Uuid::new_v4();

    let expires_at: NaiveDateTime =
        (Utc::now() + chrono::Duration::days(7)).naive_utc();

    sqlx::query!(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at)
        VALUES ($1, $2, $3, $4)",
        refresh_id,
        user_id,
        hash_token(&refresh_token),
        expires_at
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    // Ok(Json(AuthResponse {
    //     access_token,
    //     refresh_token,
    // }))

     Ok(ApiResponse::success(
        StatusCode::OK,
        "Register successfully",
        AuthResponse {
            access_token,
            refresh_token,
        },
    ))
}


pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response, AppError> {

    let record = sqlx::query!(
        "SELECT id, password, role FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or(AppError::Unauthorized("Invalid credentials".into()))?;

    let valid = bcrypt::verify(payload.password, &record.password)
        .map_err(|_| AppError::Internal("Password verify failed".into()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    let access_token =
        generate_access_token(&record.id.to_string(), &record.role);
    let refresh_token =
        generate_refresh_token(&record.id.to_string(), &record.role);
    let mut con = state.redis.get_async_connection().await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let key = format!("refresh_token:{}", refresh_token);
    let _: () = con.set_ex(
        format!("refresh_token:{}", refresh_token),
        record.id.to_string(),
        7 * 24 * 3600 // 7 days
    ).await
    .map_err(|_| AppError::ServiceUnavailable("Auth service unavailable".into()))?;
    // let refresh_id = Uuid::new_v4();

    // let expires_at: NaiveDateTime =
    //     (Utc::now() + chrono::Duration::days(7)).naive_utc();

    // sqlx::query!(
    //     "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at)
    //     VALUES ($1, $2, $3, $4)",
    //     refresh_id,
    //     record.id,
    //     hash_token(&refresh_token),
    //     expires_at
    // )
    // .execute(&state.db)
    // .await
    // .map_err(|e| AppError::Internal(e.to_string()))?;


    // Ok(Json(AuthResponse {
    //     access_token,
    //     refresh_token,
    // }))
    Ok(ApiResponse::success(
        StatusCode::OK,
        "Login successful",
        AuthResponse {
            access_token,
            refresh_token,
        },
    ))
}


pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Response, AppError> {

    let claims = verify_token(&payload.refresh_token)
        .map_err(|_| AppError::Unauthorized("Invalid refresh token".into()))?;

    if claims.typ != "refresh" {
        return Err(AppError::Unauthorized("Invalid token type".into()));
    }

    let mut conn = state.redis
        .get_async_connection()
        .await
        .map_err(|e| AppError::ServiceUnavailable("Auth unavailable".into()))?;

    let key = format!("refresh:{}", payload.refresh_token);

    let exists: Option<String> = conn.get(&key)
        .await
        .map_err(|e| AppError::ServiceUnavailable("Auth unavailable".into()))?;

    if exists.is_none() {
        return Err(AppError::Unauthorized("Token expired".into()));
    }

    let _: redis::RedisResult<()> = conn.del(&key).await.map(|_: i32| ());

    // let token_hash = hash_token(&payload.refresh_token);

    // let record = sqlx::query!(
    //     "SELECT user_id FROM refresh_tokens
    //      WHERE token_hash = $1 AND expires_at > now()",
    //     token_hash
    // )
    // .fetch_optional(&state.db)
    // .await
    // .map_err(|e| AppError::Internal(e.to_string()))?
    // .ok_or(AppError::Unauthorized("Refresh token revoked or expired".into()))?;

    // // ROTATE
    // sqlx::query!(
    //     "DELETE FROM refresh_tokens WHERE token_hash = $1",
    //     token_hash
    // )
    // .execute(&state.db)
    // .await
    // .map_err(|e| AppError::Internal(e.to_string()))?;

    let access_token = generate_access_token(&claims.sub, &claims.role);
    let new_refresh = generate_refresh_token(&claims.sub, &claims.role);

    // let refresh_id = Uuid::new_v4();
    
    // let expires_at: NaiveDateTime =
    //     (Utc::now() + chrono::Duration::days(7)).naive_utc();

    // sqlx::query!(
    //     "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at)
    //      VALUES ($1, $2, $3, $4)",
    //     refresh_id,
    //     record.user_id,
    //     hash_token(&new_refresh),
    //     expires_at
    // )
    // .execute(&state.db)
    // .await
    // .map_err(|e| AppError::Internal(e.to_string()))?;

    // Ok(Json(AuthResponse {
    //     access_token,
    //     refresh_token: new_refresh,
    // }))

     Ok(ApiResponse::success(
        StatusCode::OK,
        "Refresh successful",
        AuthResponse {
            access_token,
            refresh_token: new_refresh,
        },
    ))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<(), AppError> {

    // let token_hash = hash_token(&payload.refresh_token);

    // let result = sqlx::query!(
    //     "DELETE FROM refresh_tokens WHERE token_hash = $1",
    //     token_hash
    // )
    // .execute(&state.db)
    // .await
    // .map_err(|e| AppError::Internal(e.to_string()))?;


    // if result.rows_affected() == 0 {
    //     return Err(AppError::Unauthorized(
    //         "Refresh token already revoked".into()
    //     ));
    // }
    let mut conn = state.redis
        .get_async_connection()
        .await
        .map_err(|_| AppError::ServiceUnavailable("Auth unavailable".into()))?;

    let _: redis::RedisResult<()> = conn.del(format!("refresh:{}", payload.refresh_token))
        .await
        .map(|_: i32| ());

    Ok(())
}

