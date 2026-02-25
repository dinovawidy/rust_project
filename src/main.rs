mod routes;
mod handlers;
mod models;
mod state;
mod error;
mod auth;
mod utils;

use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use state::AppState;
use redis::Client;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::EnvFilter;




#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
    .with(
        tracing_subscriber::fmt::layer()
            .json() // JSON log production ready
            .with_target(false)
    )
    .with(EnvFilter::from_default_env())
    .init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

        tracing::info!("Database connected successfully.");
        println!("Database connected successfully.");

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        let redis_client = Client::open(redis_url)
        .expect("Failed to create Redis client");
    let state = AppState {
        db: db_pool,
        redis: redis_client,
    };


    let app = Router::new()
        .nest("/users", routes::user::user_routes())
        .nest("/api/v1/auth", routes::auth::auth_routes())
        .with_state(state);


    
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    tracing::info!("🚀 Server running at http://localhost:8000");
    axum::serve(listener, app)
    .await
    .expect("server failed");

}


// use axum::{routing::get, Json, Router,};
// use serde::Serialize;

// #[derive(Serialize)]
// struct ApiResponse {
//     message: String,
//     status: bool,
// }

// #[tokio::main]
// async fn main() {
//     // routing
//     let app = Router::new()
//         .route("/", get(hello))
//         .route("/json", get(hello_json));

//     // run server
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
//         .await
//         .unwrap();

//     println!("🚀 Server running at http://localhost:3000");
//     axum::serve(listener, app).await.unwrap();
// }

// async fn hello() -> &'static str {
//     "Hello from Rust API 🚀"
// }

// async fn hello_json() -> Json<ApiResponse> {
//     Json(ApiResponse {
//         message: "Hello JSON from Rust".to_string(),
//         status: true,
//     })
// }


