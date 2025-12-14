use std::sync::Arc;

use super::{Honey, service};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn get_honey(State(svc): State<Arc<dyn service::Service>>) -> impl IntoResponse {
    Json(svc.list().await)
}
pub async fn post_honey(
    State(svc): State<Arc<dyn service::Service>>,
    Json(honey): Json<Honey>,
) -> impl IntoResponse {
    svc.create(honey).await;
    StatusCode::CREATED
}
