use anyhow::Result;
use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::{application::definitions::Inferer, domain::entities::ImageInput};

async fn extract_image(multipart: &mut Multipart) -> Result<ImageInput> {
    while let Some(field) = multipart.next_field().await? {
        if let Some(name) = field.name()
            && name == "image"
        {
            let bytes = field.bytes().await?;
            let image_bytes = bytes.to_vec();
            let dimensions = super::toolkit::extract_dimensions(&bytes)?;
            return Ok(ImageInput {
                image_bytes,
                dimensions,
            });
        }
    }

    anyhow::bail!("Image field not found in multipart data");
}

pub async fn segment_handler(
    State(inferer): State<Arc<dyn Inferer>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    match extract_image(&mut multipart).await {
        Ok(image_input) => match inferer.segment(image_input).await {
            Ok(result) => (StatusCode::OK, Json(result)).into_response(),
            Err(e) => {
                eprintln!("Segmentation error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}
