use crate::domain::entities::SegmentationResult;
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
