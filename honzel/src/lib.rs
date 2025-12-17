use std::sync::Arc;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

mod extractor;
pub mod honey;
mod service;

#[derive(Clone)]
pub struct AppState {
    pub honey_service: Arc<honey::ServiceV1>,
}

pub enum ApiError {
    Internal(InternalError),
    BadRequest(String),
    UnprocessableEntity(Vec<ValidationError>),
}

#[derive(Serialize)]
pub struct InternalError {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // TODO: Log the error if necessary
        match self {
            ApiError::Internal(internal_error) => Json(internal_error).into_response(),
            ApiError::BadRequest(err) => (StatusCode::BAD_REQUEST, err).into_response(),
            ApiError::UnprocessableEntity(validation_errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"errors": validation_errors})),
            )
                .into_response(),
        }
    }
}

impl From<service::Error> for ApiError {
    fn from(value: service::Error) -> Self {
        match value {
            service::Error::Storage(_) => Self::Internal(InternalError {
                message: "Internal server error".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let mut validation_errors = Vec::new();

        for (field, error_msgs) in errors.field_errors() {
            for error in error_msgs {
                let message = error
                    .message
                    .as_ref()
                    .map(|cow| cow.to_string())
                    .unwrap_or_else(|| format!("Validation failed on {field}"));

                validation_errors.push(ValidationError {
                    message,
                    field: field.to_string(),
                });
            }
        }

        Self::UnprocessableEntity(validation_errors)
    }
}
