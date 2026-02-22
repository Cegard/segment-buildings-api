use anyhow::Result;
use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::{
    application::definitions::Inferer,
    domain::entities::{ImageInput, SegmentationResult},
};

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

// pub async fn segment_handler(
//     State(Inferer): State<Arc<dyn Inferer>>,
//     Multipart(mut multipart): Multipart,
//
