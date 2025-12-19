use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::{ApiError, extractor::ValidatedJson};

use super::{OrderItem, service};

type ServiceError = crate::service::Error;

#[utoipa::path(
    post,
    path = "",
    tag = "order",
    request_body = Vec<OrderItem>,
    responses((status = 201, body = Uuid))
)]
pub async fn post_order(
    State(svc): State<Arc<dyn service::Service<Error = ServiceError>>>,
    ValidatedJson(items): ValidatedJson<Vec<OrderItem>>,
) -> Result<(StatusCode, Json<Uuid>), ApiError> {
    Ok((StatusCode::CREATED, Json(svc.create(items).await?)))
}
