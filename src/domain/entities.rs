use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, ToSchema)]
pub struct SegmentationResult {
    pub mask: Vec<u8>,
    pub dimensions: Dimensions,
}

pub struct ImageInput {
    pub image_bytes: Vec<u8>,
}
