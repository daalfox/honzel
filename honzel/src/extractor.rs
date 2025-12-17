use axum::{
    Form, Json,
    extract::{FromRequest, Request, rejection::FormRejection},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::ApiError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| ApiError::BadRequest(format!("Unexpected payload: {err}")))?;

        value.validate()?;

        Ok(ValidatedJson(value))
    }
}
