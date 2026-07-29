// src/routes/mod.rs
use crate::state::AppState;
use axum::Router;

mod users;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/users", users::routes())
        .with_state(state)
}