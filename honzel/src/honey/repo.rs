use async_trait::async_trait;
use crusader::{Creatable, Listable};
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::honey::HoneyWithId;

use super::Honey;

pub trait Repo:
    Creatable<Input = Honey, Output = Uuid, Error = sqlx::Error>
    + Listable<Output = Vec<HoneyWithId>, Error = sqlx::Error>
    + Send
    + Sync
    + 'static
{
}

pub struct PgRepo {
    pool: PgPool,
}

impl PgRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repo for PgRepo {}

#[async_trait]
impl Creatable for PgRepo {
    type Input = Honey;
    type Output = Uuid;
    type Error = sqlx::Error;

    async fn create(&self, honey: Honey) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar("INSERT INTO honey(title) VALUES ($1) RETURNING id")
            .bind(honey.title)
            .fetch_one(&self.pool)
            .await?;

        Ok(id)
    }
}

#[async_trait]
impl Listable for PgRepo {
    type Output = Vec<HoneyWithId>;
    type Error = sqlx::Error;

    async fn list(&self) -> Result<Vec<HoneyWithId>, sqlx::Error> {
        let honey_list = sqlx::query_as("SELECT * FROM honey")
            .fetch_all(&self.pool)
            .await?;

        Ok(honey_list)
    }
}
