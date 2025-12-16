use std::sync::Arc;

use axum::{Json, response::IntoResponse};
use serde::Serialize;

pub mod honey;
mod service;

#[derive(Clone)]
pub struct AppState {
    pub honey_service: Arc<honey::ServiceV1>,
}

enum ApiError {
    Internal(InternalError),
}

#[derive(Serialize)]
struct InternalError {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // TODO: Log the error if necessary
        match self {
            ApiError::Internal(internal_error) => Json(internal_error).into_response(),
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
