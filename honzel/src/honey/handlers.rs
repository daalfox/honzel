use std::sync::Arc;

use crate::{ApiError, extractor::ValidatedJson, honey::HoneyWithId};

use super::{Honey, service};
use axum::{Json, extract::State, http::StatusCode};
use uuid::Uuid;

type ServiceError = crate::service::Error;

#[utoipa::path(
    get,
    path = "",
    tag = "honey",
    responses(
        (status = 200, description = "Honey list", body = Vec<HoneyWithId>)
    )
)]
pub async fn get_honey(
    State(svc): State<Arc<dyn service::Service<Error = ServiceError>>>,
) -> Result<Json<Vec<HoneyWithId>>, ApiError> {
    let honey_list = svc.list().await?;
    Ok(Json(honey_list))
}

#[utoipa::path(
    post,
    path = "",
    tag = "honey",
    request_body = Honey,
    responses(
        (status = 201, description = "Honey created", body = Uuid),
    )
)]
#[axum::debug_handler]
pub async fn post_honey(
    State(svc): State<Arc<dyn service::Service<Error = ServiceError>>>,
    ValidatedJson(honey): ValidatedJson<Honey>,
) -> Result<(StatusCode, Json<Uuid>), ApiError> {
    let id = svc.create(honey).await?;
    Ok((StatusCode::CREATED, Json(id)))
}
