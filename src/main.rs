// src/main.rs
mod config;
mod dto;
mod error;
mod extractors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use sqlx::postgres::PgPoolOptions;
use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::AppConfig::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Neon ডাটাবেসে কানেক্ট করা যায়নি");

    let port = config.port;
    let state = AppState {
        pool,
        config: std::sync::Arc::new(config),
    };

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    tracing::info!("Server চলছে -> http://0.0.0.0:{}", port);
    axum::serve(listener, app).await.unwrap();
}