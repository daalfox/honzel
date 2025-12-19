use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;
use uuid::Uuid;

use crate::{AppState, service};

use super::{OrderItem, repo::Repo};

#[async_trait]
pub trait Service: Send + Sync {
    type Error: std::error::Error + Send + Sync;

    async fn create(&self, items: Vec<OrderItem>) -> Result<Uuid, Self::Error>;
}

impl FromRef<AppState> for ServiceV1 {
    fn from_ref(input: &AppState) -> Self {
        input.order_service.as_ref().clone()
    }
}

#[derive(Clone)]
pub struct ServiceV1 {
    repo: Arc<dyn Repo>,
}

impl ServiceV1 {
    pub fn new(repo: Arc<dyn Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for ServiceV1 {
    type Error = service::Error;

    async fn create(&self, items: Vec<OrderItem>) -> Result<Uuid, Self::Error> {
        Ok(self.repo.create(items).await?)
    }
}
