use async_trait::async_trait;
use sqlx::PgPool;

use crate::honey::HoneyWithId;

use super::Honey;

#[async_trait]
pub trait Repo: Send + Sync {
    async fn insert(&self, honey: Honey);
    async fn get_all(&self) -> Vec<HoneyWithId>;
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
    async fn insert(&self, honey: Honey) {
        let _ = sqlx::query("INSERT INTO honey(title) VALUES ($1)")
            .bind(honey.title)
            .execute(&self.pool)
            .await;
    }
    async fn get_all(&self) -> Vec<HoneyWithId> {
        let honey_list: Vec<HoneyWithId> = sqlx::query_as("SELECT * FROM honey")
            .fetch_all(&self.pool)
            .await
            .unwrap();

        honey_list
    }
}
