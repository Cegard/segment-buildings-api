use super::definitions::Segmenter;
use crate::domain::entities::*;
use anyhow::Result;

pub struct SegmentUseCase<S: Segmenter> {
    segmenter: S,
}

impl<S: Segmenter> SegmentUseCase<S> {
    pub fn new(segmenter: S) -> Self {
        Self { segmenter }
    }

    pub async fn execute(&self, image_btytes: SegmentationInput) -> Result<SegmentationResult> {
        self.segmenter.segment(image_btytes).await
    }
}
