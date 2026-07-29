use crate::error::AppError;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use garde::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: Validate + serde::de::DeserializeOwned,
    T::Context: Default,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e: JsonRejection| AppError::BadRequest(e.body_text()))?;

        data.validate_with(&T::Context::default())
            .map_err(AppError::Validation)?;

        Ok(ValidatedJson(data))
    }
}