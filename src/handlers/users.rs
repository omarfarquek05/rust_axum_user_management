use crate::{
    dto::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse},
    error::AppError,
    extractors::validated_json::ValidatedJson,
    services::user_service,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = user_service::create_user(&state.pool, req).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_service::get_user(&state.pool, id).await?;
    Ok(Json(user.into()))
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = user_service::list_users(&state.pool).await?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    ValidatedJson(req): ValidatedJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_service::update_user_name(&state.pool, id, &req.name).await?;
    Ok(Json(user.into()))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    user_service::delete_user(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}