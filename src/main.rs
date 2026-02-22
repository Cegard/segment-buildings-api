mod api;
mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;
use tokio::net::TcpListener;

use api::router::create_router;
use axum::serve;
use infrastructure::segmentation::inferenece_engine::OrtSegmenter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let segmenter = OrtSegmenter::new(true, Some(1))?;
    let inferer = Arc::new(segmenter);
    let app = create_router(inferer);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on http://{}", listener.local_addr()?);
    serve(listener, app).await?;
    Ok(())
}
