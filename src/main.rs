use axum::{
    Json, Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use utoipa::OpenApi;

mod api;
mod application;
mod constants;
mod domain;
mod infrastructure;

use api::handlers::{AppState, health_handler, segment_handler};
use application::segment_image::SegmentImage;
use infrastructure::segmentation::inferenece_engine::OrtSegmenter;

/// OpenAPI specification for the Buildings Digitalizer API
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Buildings Digitalizer API",
        version = "0.1.0",
        description = "REST API for building image segmentation using ONNX Runtime",
        contact(name = "API Support", email = "support@buildingsdigitalizer.com"),
        license(name = "MIT")
    ),
    servers(
        (url = "http://localhost:8080", description = "Development server"),
        (url = "https://api.buildingsdigitalizer.com", description = "Production server")
    ),
    paths(
        api::handlers::segment_handler,
        api::handlers::health_handler,
        openapi_spec
    ),
    components(
        schemas(
            api::handlers::ErrorResponse,
            api::handlers::HealthResponse,
            api::handlers::SegmentRequest,
            domain::entities::SegmentationResult,
            domain::entities::Dimensions,
        )
    )
)]
pub struct ApiDoc;

/// # OpenAPI Specification
///
/// Returns the complete OpenAPI 3.0 specification of the API in JSON format.
///
/// ## Use Cases
/// - Import into API clients (Postman, Insomnia, etc.)
/// - Generate client SDKs
/// - Display in Swagger UI or ReDoc
/// - API documentation generation
/// - Integration with API management platforms
///
/// ## Consuming the Specification
///
/// ### View in browser with Swagger UI:
/// ```
/// https://editor.swagger.io/?url=http://localhost:8080/api/openapi.json
/// ```
///
/// ### Import into Postman:
/// 1. Open Postman
/// 2. File → Import
/// 3. Paste URL: http://localhost:8080/api/openapi.json
/// 4. Click Import
///
/// ### Generate TypeScript client:
/// ```bash
/// npx openapi-generator-cli generate \
///   -i http://localhost:8080/api/openapi.json \
///   -g typescript-fetch \
///   -o ./generated-client
/// ```
///
/// ### Generate Python client:
/// ```bash
/// openapi-generator-cli generate \
///   -i http://localhost:8080/api/openapi.json \
///   -g python \
///   -o ./python_client
/// ```
///
/// ### Generate Go client:
/// ```bash
/// openapi-generator-cli generate \
///   -i http://localhost:8080/api/openapi.json \
///   -g go \
///   -o ./go-client
/// ```
///
/// ### Using curl:
/// ```bash
/// curl http://localhost:8080/api/openapi.json | jq
/// ```
#[utoipa::path(
    get,
    path = "/api/openapi.json",
    responses(
        (status = 200, description = "OpenAPI 3.0 specification in JSON format. Contains endpoint definitions, schemas, examples, and documentation.", body = serde_json::Value)
    ),
    tag = "API Documentation"
)]
async fn openapi_spec() -> Json<serde_json::Value> {
    Json(serde_json::json!(ApiDoc::openapi()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let address = env::var(constants::ENV_ON_ADDRESS)?;
    let port = env::var(constants::ENV_PORT)?.parse::<usize>()?;
    let will_run_on_cuda = env::var(constants::ENV_ON_CUDA)?.parse::<bool>()?;
    let max_con_calls = env::var(constants::ENV_MAX_CON_CALLS)?.parse::<usize>()?;

    let inferer = Arc::new(OrtSegmenter::new(will_run_on_cuda, Some(max_con_calls))?);
    let use_case = Arc::new(SegmentImage::new(inferer));
    let state = AppState { use_case };

    // Create router with segmentation and health endpoints
    let app = Router::new()
        .route("/segment", post(segment_handler))
        .route("/health", get(health_handler))
        .route("/api/openapi.json", get(openapi_spec))
        .with_state(state);

    let listener = TcpListener::bind(format!("{address}:{port}")).await?;
    println!("Server running on http://{}", listener.local_addr()?);
    println!(
        "API OpenAPI spec available at http://{}/api/openapi.json",
        listener.local_addr()?
    );
    println!("Press Ctrl+C to stop the server");

    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    server.await?;

    Ok(())
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    println!("\nShutting down server...");
}
