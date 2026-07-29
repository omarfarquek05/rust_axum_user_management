use crate::{
    dto::user_dto::CreateUserRequest,
    error::AppError,
    models::user::User,
    repositories::user_repository::UserRepository,
    services::password_service,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(pool: &PgPool, req: CreateUserRequest) -> Result<User, AppError> {
    let password_hash = password_service::hash_password(&req.password)?;
    let id = Uuid::new_v4();
    let user = UserRepository::create(pool, id, &req.name, &req.email, &password_hash).await?;
    Ok(user)
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
    Ok(UserRepository::find_by_id(pool, id).await?)
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, AppError> {
    Ok(UserRepository::find_all(pool).await?)
}

pub async fn update_user_name(pool: &PgPool, id: Uuid, name: &str) -> Result<User, AppError> {
    Ok(UserRepository::update_name(pool, id, name).await?)
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let affected = UserRepository::delete(pool, id).await?;
    if affected == 0 {
        return Err(AppError::NotFound("ইউজার পাওয়া যায়নি".to_string()));
    }
    Ok(())
}