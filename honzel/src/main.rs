use std::{env, sync::Arc};

use axum::Router;
use honzel::{AppState, honey};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let honey_repo = Arc::new(honey::PgRepo::new(pool));

    let honey_service = Arc::new(honey::ServiceV1::new(honey_repo));

    let state = AppState { honey_service };

    let router = Router::new().nest("/honey", honey::router(state.clone()));

    Ok(axum::serve(listener, router).await?)
}
