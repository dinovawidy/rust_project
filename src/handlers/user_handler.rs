use axum::{
    extract::{ State, Query },
    Json,
    response::Response,
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{    
    auth::{extractor::AuthUser, role::require_admin},
    models::user::{User, CreateUserRequest, UserQuery},
    state::AppState,
    error::AppError,
    utils::api_response::ApiResponse,
};

pub async fn get_users(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<UserQuery>,
) -> Result<Response, AppError> {

    require_admin(&auth.role)?;

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).min(100);
    let search = params.search.unwrap_or_default();
    let offset = (page - 1) * per_page;

    /*
    Count
    */
    
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE name ILIKE $1"
    )
    .bind(format!("%{}%", search))
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;


    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, email, role, created_at FROM users WHERE name ILIKE $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(format!("%{}%", search))
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let total_pages = (total.0 as f64 / per_page as f64).ceil() as i64;

    println!("SEARCH VALUE: {:?}", search);
    println!("OFFSET: {}", offset);
    println!("LIMIT: {}", per_page);
    Ok(ApiResponse::success(
        StatusCode::OK,
        "User Fetch successful",
        serde_json::json!({
            "items": users,
            "pagination": {
                "total": total.0,
                "page": page,
                "per_page": per_page,
                "total_pages": total_pages,
            }
        })
    ))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {

    // ✅ VALIDATION
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name, email)
         VALUES ($1, $2, $3)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(payload.name)
    .bind(payload.email)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") {
            AppError::Conflict("Email already exists".to_string())
        } else {
            AppError::Internal(e.to_string())
        }
    })?;

    Ok(Json(user))
}

