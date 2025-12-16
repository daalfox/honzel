use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::AppState;

mod handlers;
mod repo;
mod service;

pub use repo::PgRepo;
pub use service::ServiceV1;

#[derive(Serialize, Deserialize, FromRow)]
pub struct HoneyWithId {
    id: Uuid,
    #[sqlx(flatten)]
    #[serde(flatten)]
    honey: Honey,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct Honey {
    title: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::get_honey).post(handlers::post_honey))
        .with_state(state.honey_service)
}
