use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;

use crate::{AppState, honey::HoneyWithId};

use super::{Honey, repo::Repo};

#[async_trait]
pub trait Service: Send + Sync {
    async fn list(&self) -> Vec<HoneyWithId>;
    async fn create(&self, honey: Honey);
}

impl FromRef<AppState> for ServiceV1 {
    fn from_ref(input: &AppState) -> Self {
        input.honey_service.as_ref().clone()
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
    async fn list(&self) -> Vec<HoneyWithId> {
        self.repo.get_all().await
    }
    async fn create(&self, honey: Honey) {
        self.repo.insert(honey).await;
    }
}
