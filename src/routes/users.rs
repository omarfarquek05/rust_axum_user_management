// src/routes/users.rs
use crate::{handlers::users, state::AppState};
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(users::list_users).post(users::create_user))
        .route(
            "/{id}",
            get(users::get_user)
                .patch(users::update_user)
                .delete(users::delete_user),
        )
}