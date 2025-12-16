use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::honey::HoneyWithId;

use super::Honey;

#[async_trait]
pub trait Repo: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn insert(&self, honey: Honey) -> Result<Uuid, Self::Error>;
    async fn get_all(&self) -> Result<Vec<HoneyWithId>, Self::Error>;
}

pub struct PgRepo {
    pool: PgPool,
}

impl PgRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repo for PgRepo {
    type Error = sqlx::Error;
    async fn insert(&self, honey: Honey) -> Result<Uuid, Self::Error> {
        let id = sqlx::query_scalar("INSERT INTO honey(title) VALUES ($1) RETURNING id")
            .bind(honey.title)
            .fetch_one(&self.pool)
            .await?;

        Ok(id)
    }
    async fn get_all(&self) -> Result<Vec<HoneyWithId>, Self::Error> {
        let honey_list = sqlx::query_as("SELECT * FROM honey")
            .fetch_all(&self.pool)
            .await?;

        Ok(honey_list)
    }
}
