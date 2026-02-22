use axum::{Router, routing::post};
use std::sync::Arc;
use tokio::net::TcpListener;

mod api;
mod application;
mod domain;
mod infrastructure;

use api::handlers::{AppState, segment_handler};
use application::segment_image::SegmentImage as SegmentUseCase;
use infrastructure::segmentation::inferenece_engine::OrtSegmenter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let inferer = Arc::new(OrtSegmenter::new(true, Some(2))?);

    let use_case = Arc::new(SegmentUseCase::new(inferer));

    let state = AppState { use_case };

    let app = Router::new()
        .route("/segment", post(segment_handler))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server running on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
