// src/state.rs
use sqlx::PgPool;
use std::sync::Arc;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Arc<AppConfig>,
}