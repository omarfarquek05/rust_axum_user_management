use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        id: Uuid,
        name: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email, password_hash)
             VALUES ($1, $2, $3, $4)
             RETURNING id, name, email, password_hash, created_at",
        )
        .bind(id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, created_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, created_at FROM users ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update_name(pool: &PgPool, id: Uuid, name: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET name = $1 WHERE id = $2
             RETURNING id, name, email, password_hash, created_at",
        )
        .bind(name)
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}