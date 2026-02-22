use crate::{application::segment_image::SegmentImage, constants, domain::entities::ImageInput};
use axum::{
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
    let image_field = match env::var(constants::ENV_IMAGE_FILE_NAME) {
        Ok(name) => name,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "IMAGE_FILE_NAME environment variable not set",
            )
                .into_response();
        }
    };

    let mut image_bytes: Option<Vec<u8>> = None;

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid multipart form data").into_response();
            }
        };

        if field.name() == Some(&image_field) {
            match field.bytes().await {
                Ok(bytes) => {
                    image_bytes = Some(bytes.to_vec());
                }
                Err(_) => {
                    return (StatusCode::BAD_REQUEST, "Failed to read image bytes").into_response();
                }
            }
        }
    }

    let image_bytes = match image_bytes {
        Some(bytes) => bytes,
        None => {
            return (StatusCode::BAD_REQUEST, "Missing image field").into_response();
        }
    };

    let image_input = ImageInput { image_bytes };

    let result = match state.use_case.execute(image_input).await {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Segmentation failed: {e}"),
            )
                .into_response();
        }
    };

    let png_bytes = match super::toolkit::to_png(result) {
        Ok(bytes) => bytes,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to convert segmentation result to PNG: {e}"),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        [
            ("Content-Type", "image/png"),
            (
                "Content-Disposition",
                "attachment; filename=\"segmentation.png\"",
            ),
        ],
        png_bytes,
    )
        .into_response()
}
