use std::sync::Arc;

pub mod honey;

#[derive(Clone)]
pub struct AppState {
    pub honey_service: Arc<honey::ServiceV1>,
}
