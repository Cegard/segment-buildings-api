use axum::{Router, routing::post};
use std::sync::Arc;

pub fn create_router(inferer: Arc<dyn crate::application::definitions::Inferer>) -> Router {
    Router::new()
        .route("/segment", post(crate::api::handlers::segment_handler))
        .with_state(inferer)
}
