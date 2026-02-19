use super::definitions::Inferer;
use crate::domain::entities::{ImageInput, SegmentationResult};
use anyhow::Result;

pub struct SegmentImage<I: Inferer> {
    inferer: I,
}

impl<I: Inferer> SegmentImage<I> {
    pub fn new(segmenter: I) -> Self {
        Self { inferer: segmenter }
    }

    pub async fn execute(&self, image_btytes: ImageInput) -> Result<SegmentationResult> {
        self.inferer.segment(image_btytes).await
    }
}
