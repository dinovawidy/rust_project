use axum::{
    Router,
    routing::post,
    http::StatusCode,
    response::IntoResponse,
};
use std::{sync::Arc, env};
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
};

use crate::{
    handlers::auth_handler::{register, login, refresh, logout},
    state::AppState,
};

pub fn auth_routes() -> Router<AppState> {
    let enable_rate_limit =
        env::var("ENABLE_RATE_LIMIT").unwrap_or_else(|_| "false".into()) == "true";

    let router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout));

    // 🔕 DEV MODE → NO RATE LIMIT
    if !enable_rate_limit {
        return router;
    }

    // 🔐 PROD MODE → RATE LIMIT ON
    let rate_limit = GovernorLayer {
        config: Arc::new(
            GovernorConfigBuilder::default()
                .per_second(5)
                .burst_size(10)
                .error_handler(|_| {
                    (
                        StatusCode::TOO_MANY_REQUESTS,
                        "Too many requests",
                    )
                        .into_response()
                })
                .finish()
                .expect("rate limit config"),
        ),
    };

    router.route_layer(rate_limit)
}
