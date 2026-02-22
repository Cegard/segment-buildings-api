use crate::domain::entities::Dimensions;
use anyhow::Result;
use image::GenericImageView;

pub fn extract_dimensions(bytes: &[u8]) -> Result<Dimensions> {
    let img = image::load_from_memory(bytes)?;
    let (width, height) = img.dimensions();
    Ok(Dimensions {
        width: width as usize,
        height: height as usize,
    })
}
