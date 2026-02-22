use axum::{Router, routing::post};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

mod api;
mod application;
mod constants;
mod domain;
mod infrastructure;

use api::handlers::{AppState, segment_handler};
use application::segment_image::SegmentImage;
use infrastructure::segmentation::inferenece_engine::OrtSegmenter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let addres = env::var(constants::ENV_ON_ADDRESS)?;
    let port = env::var(constants::ENV_PORT)?.parse::<usize>()?;
    let will_run_on_cuda = env::var(constants::ENV_ON_CUDA)?.parse::<bool>()?;
    let max_con_calls = env::var(constants::ENV_MAX_CON_CALLS)?.parse::<usize>()?;
    let inferer = Arc::new(OrtSegmenter::new(will_run_on_cuda, Some(max_con_calls))?);
    let use_case = Arc::new(SegmentImage::new(inferer));
    let state = AppState { use_case };
    let app = Router::new()
        .route("/segment", post(segment_handler))
        .with_state(state);
    let listener = TcpListener::bind(format!("{addres}:{port}")).await?;

    println!("Server running on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
