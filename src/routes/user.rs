use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::user_handler::{
    get_users,
    create_user,
};

use crate::state::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
}
