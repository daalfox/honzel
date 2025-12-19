use async_trait::async_trait;
use crusader::Creatable;
use sqlx::PgPool;
use uuid::Uuid;

use super::OrderItem;

pub trait Repo:
    Creatable<Input = Vec<OrderItem>, Output = Uuid, Error = sqlx::Error> + Send + Sync
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
    type Input = Vec<OrderItem>;
    type Output = Uuid;
    type Error = sqlx::Error;

    async fn create(&self, items: Vec<OrderItem>) -> Result<Uuid, sqlx::Error> {
        let mut item_ids = Vec::with_capacity(items.len());
        let mut item_qtys = Vec::with_capacity(items.len());
        for v in items {
            item_ids.push(v.item_id);
            item_qtys.push(v.qty);
        }
        let id = sqlx::query_scalar!(
            r#"
                WITH x AS (
                    INSERT INTO customer_order DEFAULT VALUES RETURNING id
                ), y AS (
                    INSERT INTO order_item(item_id, qty) SELECT * FROM UNNEST($1::uuid[], $2::int[])
                ) SELECT id FROM x
            "#,
            &item_ids,
            &item_qtys
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }
}
