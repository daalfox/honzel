use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    honey::handlers::{__path_get_honey, get_honey},
    honey::handlers::{__path_post_honey, post_honey},
};

mod handlers;
mod repo;
mod service;

pub use repo::PgRepo;
pub use service::ServiceV1;

#[derive(Serialize, FromRow, ToSchema)]
pub struct HoneyWithId {
    id: Uuid,
    #[sqlx(flatten)]
    #[serde(flatten)]
    honey: Honey,
}
#[derive(Serialize, Deserialize, FromRow, ToSchema, Validate)]
pub struct Honey {
    #[validate(length(min = 1, message = "must be at least 1 character long"))]
    title: String,
}

#[derive(OpenApi)]
#[openapi(paths(get_honey, post_honey))]
pub struct HoneyApiDoc;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_honey).post(post_honey))
        .with_state(state.honey_service)
}
