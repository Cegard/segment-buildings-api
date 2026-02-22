use serde::Serialize;

#[derive(Serialize)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize)]
pub struct SegmentationResult {
    pub mask: Vec<u8>,
    pub dimensions: Dimensions,
}

pub struct ImageInput {
    pub image_bytes: Vec<u8>,
}
