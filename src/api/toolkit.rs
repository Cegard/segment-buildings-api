use crate::domain::entities::SegmentationResult;
use axum::{extract::Multipart, http::StatusCode};
use image::{ImageBuffer, Luma};

pub(super) fn to_png(result: SegmentationResult) -> anyhow::Result<Vec<u8>> {
    let img: ImageBuffer<Luma<u8>, _> = ImageBuffer::from_raw(
        result.dimensions.width as u32,
        result.dimensions.height as u32,
        result.mask.clone(),
    )
    .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer from segmentation result"))?;
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png,
    )?;
    Ok(bytes)
}

pub(super) async fn extract_image_bytes(
    mut multipart: Multipart,
    image_field: &str,
) -> Result<Vec<u8>, (StatusCode, &'static str)> {
    let mut image_bytes: Option<Vec<u8>> = None;

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid multipart form data")),
        };

        if field.name() == Some(image_field) {
            match field.bytes().await {
                Ok(bytes) => {
                    image_bytes = Some(bytes.to_vec());
                }
                Err(_) => return Err((StatusCode::BAD_REQUEST, "Failed to read image bytes")),
            }
        }
    }

    image_bytes.ok_or((StatusCode::BAD_REQUEST, "Missing image field"))
}