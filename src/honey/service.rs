use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;
use uuid::Uuid;

use crate::{AppState, honey::HoneyWithId, service};

use super::{Honey, repo::Repo};

#[async_trait]
pub trait Service: Send + Sync {
    type Error: std::error::Error + Send + Sync;

    async fn list(&self) -> Result<Vec<HoneyWithId>, Self::Error>;
    async fn create(&self, honey: Honey) -> Result<Uuid, Self::Error>;
}

impl FromRef<AppState> for ServiceV1 {
    fn from_ref(input: &AppState) -> Self {
        input.honey_service.as_ref().clone()
    }
}

#[derive(Clone)]
pub struct ServiceV1 {
    repo: Arc<dyn Repo<Error = sqlx::Error>>,
}

impl ServiceV1 {
    pub fn new(repo: Arc<dyn Repo<Error = sqlx::Error>>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for ServiceV1 {
    type Error = service::Error;

    async fn list(&self) -> Result<Vec<HoneyWithId>, Self::Error> {
        Ok(self.repo.get_all().await?)
    }
    async fn create(&self, honey: Honey) -> Result<Uuid, Self::Error> {
        Ok(self.repo.insert(honey).await?)
    }
}
