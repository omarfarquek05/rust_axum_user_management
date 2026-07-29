use crate::models::user::User;
use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---- Request: signup/create-এ client যা পাঠাবে ----
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[garde(length(min = 2, max = 50))]
    pub name: String,

    #[garde(email)]
    pub email: String,

    #[garde(length(min = 8, max = 100), custom(has_uppercase_and_digit))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[garde(length(min = 2, max = 50))]
    pub name: String,
}

fn has_uppercase_and_digit(value: &str, _ctx: &()) -> garde::Result {
    let has_upper = value.chars().any(|c| c.is_uppercase());
    let has_digit = value.chars().any(|c| c.is_ascii_digit());

    if has_upper && has_digit {
        Ok(())
    } else {
        Err(garde::Error::new(
            "পাসওয়ার্ডে অন্তত একটা বড় হাতের অক্ষর এবং একটা সংখ্যা থাকতে হবে",
        ))
    }
}

// ---- Response: client যা দেখবে (password_hash বাদ) ----
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        }
    }
}