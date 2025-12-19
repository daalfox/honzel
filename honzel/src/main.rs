use std::{env, sync::Arc};

use axum::Router;
use honzel::{
    AppState,
    honey::{self, HoneyApiDoc},
    order::{self, OrderApiDoc},
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/honey", api = HoneyApiDoc),
        (path = "/orders", api = OrderApiDoc)
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let honey_repo = Arc::new(honey::PgRepo::new(pool.clone()));
    let order_repo = Arc::new(order::PgRepo::new(pool));

    let honey_service = Arc::new(honey::ServiceV1::new(honey_repo));
    let order_service = Arc::new(order::ServiceV1::new(order_repo));

    let state = AppState {
        honey_service,
        order_service,
    };

    let router = Router::new()
        .nest("/honey", honey::router(state.clone()))
        .nest("/orders", order::router(state.clone()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()));

    Ok(axum::serve(listener, router).await?)
}
