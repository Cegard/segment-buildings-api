use crate::{application::segment_image::SegmentImage, constants, domain::entities::ImageInput};
use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub use_case: Arc<SegmentImage>,
}

pub async fn segment_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut image_bytes = None;
    let image_field = env::var(constants::ENV_IMAGE_FILE_NAME)
        .expect("IMAGE_FILE_NAME environment variable not set");

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some(&image_field) {
            image_bytes = Some(field.bytes().await.unwrap().to_vec());
        }
    }

    let image_bytes = match image_bytes {
        Some(bytes) => bytes,
        None => return (StatusCode::BAD_REQUEST, "Missing image").into_response(),
    };

    let image_input = ImageInput { image_bytes };

    match state.use_case.execute(image_input).await {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Segmentation failed: {e}"),
        )
            .into_response(),
    }
}
