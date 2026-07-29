use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("রিসোর্স পাওয়া যায়নি: {0}")]
    NotFound(String),

    #[error("অনুরোধ সঠিক না: {0}")]
    BadRequest(String),

    #[error("ভ্যালিডেশন ব্যর্থ")]
    Validation(garde::Report),

    #[error("ইতিমধ্যে বিদ্যমান: {0}")]
    Conflict(String),

    #[error("ডাটাবেস সমস্যা")]
    Database(sqlx::Error),

    #[error("পাসওয়ার্ড হ্যাশিং সমস্যা")]
    Hashing(String),
}

// ---- sqlx::Error থেকে কনভার্সন — এখানে nested logic দরকার, তাই #[from] না, ম্যানুয়াল ----
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_err) = &err {
            // PostgreSQL error code 23505 = unique_violation
            if db_err.code().as_deref() == Some("23505") {
                return AppError::Conflict("এই email দিয়ে ইতিমধ্যে অ্যাকাউন্ট আছে".to_string());
            }
        }
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("রেকর্ড পাওয়া যায়নি".to_string()),
            other => AppError::Database(other),
        }
    }
}


impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::Hashing(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if let AppError::Validation(report) = &self {
            let errors: Vec<String> = report
                .iter()
                .map(|(path, err)| format!("{}: {}", path, err))
                .collect();
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "errors": errors })),
            )
                .into_response();
        }

        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Database(_) | AppError::Hashing(_) => {
                tracing::error!(error = %self, "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "সার্ভারে একটা সমস্যা হয়েছে".to_string())
            }
            AppError::Validation(_) => unreachable!(),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}