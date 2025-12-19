use axum::{Router, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    order::handlers::{__path_post_order, post_order},
};

mod handlers;
mod repo;
mod service;

pub use repo::PgRepo;
pub use service::ServiceV1;

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Order {
    id: Uuid,
    items: Vec<OrderItem>,
}
#[derive(Serialize, FromRow, ToSchema)]
pub struct OrderItemWithId {
    id: Uuid,
    #[serde(flatten)]
    order_item: OrderItem,
}
#[derive(Serialize, Deserialize, FromRow, ToSchema, Validate)]
pub struct OrderItem {
    item_id: Uuid,
    #[validate(range(min = 1, message = "should be more than 1"))]
    qty: i32,
}

#[derive(OpenApi)]
#[openapi(paths(post_order))]
pub struct OrderApiDoc;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", post(post_order))
        .with_state(state.order_service)
}
